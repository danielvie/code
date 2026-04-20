import asyncio
import time
import json
import math
import numpy as np
import websockets
import flatbuffers
import scipy.linalg
from Simulation import Message, MessageType, PendulumState, Parameters as FbParameters, Command as FbCommand

# Physical Constants & State
class Simulation:
    def __init__(self):
        # Default Parameters
        self.mass_cart = 1.0
        self.mass_pole = 0.1
        self.length = 1.0
        self.gravity = 9.81
        self.dt = 1.0 / 60.0
        
        self.q_pos = 10.0
        self.q_vel = 1.0
        self.q_ang = 100.0
        self.q_omg = 10.0
        self.r_ctrl = 0.01
        
        self.target_x = 0.0
        self.K = None
        
        self.reset_state()
        self._recompute_lqr()
        
    def reset_state(self):
        self.x = 0.0
        self.v = 0.0
        self.theta = 0.1 # Small nudge to start stabilization
        self.omega = 0.0
        self.is_valid = True

    def reset(self):
        # This is the remote reset from the UI
        self.reset_state()

    def _recompute_lqr(self):
        M = self.mass_cart
        m = self.mass_pole
        l = self.length
        L = l / 2.0
        g = self.gravity
        I = (1.0/12.0) * m * l**2
        p = (M + m) * (I + m * L**2) - (m * L)**2
        
        A = np.array([
            [0, 1, 0, 0],
            [0, 0, (-m**2 * L**2 * g) / p, 0],
            [0, 0, 0, 1],
            [0, 0, (m * g * L * (M + m)) / p, 0]
        ])
        B = np.array([[0], [(I + m * L**2) / p], [0], [(-m * L) / p]])
        Q = np.diag([self.q_pos, self.q_vel, self.q_ang, self.q_omg]) 
        R = np.array([[self.r_ctrl]])
        
        try:
            P = scipy.linalg.solve_continuous_are(A, B, Q, R)
            self.K = np.linalg.inv(R) @ (B.T @ P)
            self.K = self.K.flatten()
        except:
            self.K = np.array([-1, -1, 10, 1]) 

    def step(self):
        if not self.is_valid:
            return

        M, m, l, g = self.mass_cart, self.mass_pole, self.length, self.gravity
        state_vec = np.array([self.x, self.v, self.theta, self.omega])
        target_vec = np.array([self.target_x, 0, 0, 0])
        
        force = -np.dot(self.K, state_vec - target_vec)
        L = l / 2.0
        sin_t, cos_t = np.sin(self.theta), np.cos(self.theta)
        
        temp = (force + m * L * self.omega**2 * sin_t) / (M + m)
        theta_acc = (g * sin_t - cos_t * temp) / (L * (4.0/3.0 - m * cos_t**2 / (M + m)))
        x_acc = temp - (m * L * theta_acc * cos_t) / (M + m)
        
        self.v += x_acc * self.dt
        self.x += self.v * self.dt
        self.omega += theta_acc * self.dt
        self.theta += self.omega * self.dt
        self.v *= 0.999
        self.omega *= 0.999
        self.theta = (self.theta + np.pi) % (2 * np.pi) - np.pi

        # Stability Check (float32 safe range)
        LIMIT = 1e30 
        state_vals = [self.x, self.v, self.theta, self.omega]
        if not all(math.isfinite(v) and abs(v) < LIMIT for v in state_vals):
            self.is_valid = False
            # Zero out to prevent any possible overflow in packing
            self.x, self.v, self.theta, self.omega = 0.0, 0.0, 0.0, 0.0
            print("SIMULATION FAILED: Numerical instability detected.", flush=True)

    def serialize_state(self):
        inner_builder = flatbuffers.Builder(512)
        PendulumState.Start(inner_builder)
        # Use safe values if invalid to avoid OverflowError on packing
        px = self.x if self.is_valid else 0.0
        pv = self.v if self.is_valid else 0.0
        pt = self.theta if self.is_valid else 0.0
        po = self.omega if self.is_valid else 0.0
        
        PendulumState.AddCartPosition(inner_builder, px)
        PendulumState.AddCartVelocity(inner_builder, pv)
        PendulumState.AddPendulumAngle(inner_builder, pt)
        PendulumState.AddPendulumAngularVelocity(inner_builder, po)
        PendulumState.AddTimestamp(inner_builder, int(time.time() * 1000))
        PendulumState.AddIsValid(inner_builder, self.is_valid)
        state_offset = PendulumState.End(inner_builder)
        inner_builder.Finish(state_offset)
        inner_data = inner_builder.Output()
        
        builder = flatbuffers.Builder(1024)
        data_vector = Message.MessageCreateDataVector(builder, inner_data)
        Message.Start(builder)
        Message.AddType(builder, MessageType.MessageType.State)
        Message.AddData(builder, data_vector)
        msg_offset = Message.End(builder)
        builder.Finish(msg_offset)
        return builder.Output()

async def handle_client(websocket):
    current_sim = Simulation()
    print("Client connected", flush=True)
    
    send_task = None
    try:
        async def send_loop():
            next_step = asyncio.get_event_loop().time()
            while True:
                current_sim.step()
                data = current_sim.serialize_state()
                await websocket.send(data)
                
                next_step += current_sim.dt
                sleep_time = next_step - asyncio.get_event_loop().time()
                if sleep_time > 0:
                    await asyncio.sleep(sleep_time)
                else:
                    # We are drifting, reset next_step to now
                    next_step = asyncio.get_event_loop().time()
                    await asyncio.sleep(0) # Yield for other tasks

        # Start send_loop as a background task
        send_task = asyncio.create_task(send_loop())

        # Receive loop (blocking)
        async for message in websocket:
            buf = bytearray(message)
            msg = Message.Message.GetRootAsMessage(buf, 0)
            
            if msg.Type() == MessageType.MessageType.Params:
                params_buf = msg.DataAsNumpy().tobytes()
                fb_params = FbParameters.Parameters.GetRootAsParameters(params_buf, 0)
                current_sim.length = fb_params.Length()
                current_sim.mass_cart = fb_params.MassCart()
                current_sim.mass_pole = fb_params.MassPole()
                current_sim.q_pos = fb_params.QPos()
                current_sim.q_vel = fb_params.QVel()
                current_sim.q_ang = fb_params.QAng()
                current_sim.q_omg = fb_params.QOmg()
                current_sim.r_ctrl = fb_params.RCtrl()
                current_sim._recompute_lqr()
                
            elif msg.Type() == MessageType.MessageType.Cmd:
                cmd_buf = msg.DataAsNumpy().tobytes()
                fb_cmd = FbCommand.Command.GetRootAsCommand(cmd_buf, 0)
                current_sim.target_x = fb_cmd.TargetPosition()
            
            elif msg.Type() == MessageType.MessageType.Reset:
                current_sim.reset()
                print("Simulation Reset", flush=True)

    except websockets.exceptions.ConnectionClosed:
        pass
    except Exception as e:
        print(f"Error: {e}", flush=True)
    finally:
        if send_task:
            send_task.cancel()
        print("Client disconnected", flush=True)

async def main():
    async with websockets.serve(handle_client, "localhost", 8765):
        print("Backend Server started on ws://localhost:8765", flush=True)
        await asyncio.Future()

if __name__ == "__main__":
    asyncio.run(main())
