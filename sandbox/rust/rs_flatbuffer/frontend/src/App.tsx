import React, { useEffect, useRef, useState, useCallback } from "react";
import * as flatbuffers from "flatbuffers";
import { Message } from "./generated/simulation/message";
import { MessageType } from "./generated/simulation/message-type";
import { PendulumState } from "./generated/simulation/pendulum-state";
import { Parameters } from "./generated/simulation/parameters";
import { Command } from "./generated/simulation/command";
import { Reset } from "./generated/simulation/reset";
import { IntegrationMethod } from "./generated/simulation/integration-method";
import { Activity, Cpu, Layers, AlertTriangle, Undo2 } from "lucide-react";
import { Agentation } from "agentation";
import SimulationCanvas from "./components/SimulationCanvas";
import ChartCanvas from "./components/ChartCanvas";
import Header from "./components/Header";

const DEFAULT_PARAMS = {
  length: 1.0,
  massCart: 1.0,
  massPole: 0.1,
  qPos: 10.0,
  qVel: 1.0,
  qAng: 100.0,
  qOmg: 10.0,
  rCtrl: 0.1,
  integrationMethod: IntegrationMethod.Euler,
};

const App: React.FC = () => {
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [connected, setConnected] = useState(false);
  const [isValid, setIsValid] = useState(true);
  const [pps, setPps] = useState(0);
  const packetCountRef = useRef(0);
  const historyRef = useRef<{ x: number; target: number }[]>([]);

  // Monitoring Throughput
  useEffect(() => {
    const timer = setInterval(() => {
      setPps(packetCountRef.current);
      packetCountRef.current = 0;
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const [state, setState] = useState({
    x: 0,
    v: 0,
    theta: 0,
    omega: 0,
  });

  const [targetX, setTargetX] = useState(0);
  const [params, setParams] = useState(DEFAULT_PARAMS);

  const targetRef = useRef(0);
  useEffect(() => {
    targetRef.current = targetX;
  }, [targetX]);

  const sendMsg = useCallback(
    (type: MessageType, dataBuilder: (b: flatbuffers.Builder) => number) => {
      if (!wsRef.current || wsRef.current.readyState !== WebSocket.OPEN) return;
      const innerBuilder = new flatbuffers.Builder(256);
      const offset = dataBuilder(innerBuilder);
      innerBuilder.finish(offset);
      const bytes = innerBuilder.asUint8Array();
      const outerBuilder = new flatbuffers.Builder(512);
      const dataVector = Message.createDataVector(outerBuilder, bytes);
      Message.startMessage(outerBuilder);
      Message.addType(outerBuilder, type);
      Message.addData(outerBuilder, dataVector);
      const msgOffset = Message.endMessage(outerBuilder);
      outerBuilder.finish(msgOffset);
      wsRef.current.send(outerBuilder.asUint8Array() as BufferSource);
    },
    [],
  );

  const handleReset = () => {
    sendMsg(MessageType.Reset, (b) => {
      Reset.startReset(b);
      return Reset.endReset(b);
    });
    setIsValid(true);
    historyRef.current = [];
  };

  const handleRestoreDefaults = () => {
    setParams(DEFAULT_PARAMS);
    setTargetX(0);
  };

  // WebSocket Setup
  useEffect(() => {
    let reconnectDelay = 1000;

    const connectWs = () => {
      const ws = new WebSocket("ws://localhost:8765");
      ws.binaryType = "arraybuffer";
      wsRef.current = ws;

      ws.onopen = () => {
        setConnected(true);
        reconnectDelay = 1000;
      };

      ws.onclose = () => {
        setConnected(false);
        wsRef.current = null;
        reconnectTimerRef.current = setTimeout(() => {
          connectWs();
        }, reconnectDelay);
        reconnectDelay = Math.min(reconnectDelay * 2, 30000);
      };

      ws.onmessage = (event) => {
        packetCountRef.current++;
        const buf = new Uint8Array(event.data);
        const builder = new flatbuffers.ByteBuffer(buf);
        const msg = Message.getRootAsMessage(builder);
        if (msg.type() === MessageType.State) {
          const stateData = msg.dataArray();
          if (stateData) {
            const stateBuf = new flatbuffers.ByteBuffer(stateData);
            const pState = PendulumState.getRootAsPendulumState(stateBuf);
            const valid = pState.isValid();
            setIsValid(valid);
            if (valid) {
              const currentX = pState.cartPosition();
              setState({
                x: currentX,
                v: pState.cartVelocity(),
                theta: pState.pendulumAngle(),
                omega: pState.pendulumAngularVelocity(),
              });
              historyRef.current.push({
                x: currentX,
                target: targetRef.current,
              });
              if (historyRef.current.length > 300) historyRef.current.shift();
            }
          }
        }
      };
    };
    connectWs();

    return () => {
      if (reconnectTimerRef.current) clearTimeout(reconnectTimerRef.current);
      wsRef.current?.close();
    };
  }, []);

  const syncParams = useCallback(() => {
    sendMsg(MessageType.Params, (b) => {
      Parameters.startParameters(b);
      Parameters.addLength(b, params.length);
      Parameters.addMassCart(b, params.massCart);
      Parameters.addMassPole(b, params.massPole);
      Parameters.addQPos(b, params.qPos);
      Parameters.addQVel(b, params.qVel);
      Parameters.addQAng(b, params.qAng);
      Parameters.addQOmg(b, params.qOmg);
      Parameters.addRCtrl(b, params.rCtrl);
      Parameters.addIntegrationMethod(b, params.integrationMethod);
      return Parameters.endParameters(b);
    });
  }, [params, sendMsg]);

  useEffect(() => {
    sendMsg(MessageType.Cmd, (b) => {
      Command.startCommand(b);
      Command.addTargetPosition(b, targetX);
      return Command.endCommand(b);
    });
  }, [targetX, sendMsg]);

  // Send updated params to the backend whenever any slider changes.
  // This triggers LQR gain recomputation on the Rust side.
  useEffect(() => {
    syncParams();
  }, [syncParams]);

  return (
    <>
      <div className="dashboard">
        <aside className="sidebar">
          <div className="sidebar-content">
            <div>
              <button
                className="icon-btn"
                onClick={handleRestoreDefaults}
                title="Restore Default Settings"
                aria-label="Restore Default Settings"
                style={{
                  width: "100%",
                  justifyContent: "center",
                  background: "#181b21",
                  border: "1px solid rgb(37, 42, 51)",
                  gap: "0.5rem",
                }}
              >
                <Undo2 size={16} />
                <span style={{ fontSize: "0.7rem", fontWeight: 700 }}>
                  Restore Defaults
                </span>
              </button>
            </div>

            {!isValid && (
              <div className="alert-box">
                <AlertTriangle size={16} />
                <span>Simulation Halted</span>
              </div>
            )}

            <div className="control-group">
              <div className="control-group-title">Physics</div>
              <div className="control-item">
                <label htmlFor="length-slider">
                  <Layers size={10} style={{ marginRight: 4 }} /> Pole Length
                </label>
                <div className="control-row">
                  <input
                    id="length-slider"
                    type="range"
                    min="0.5"
                    max="3"
                    step="0.1"
                    value={params.length}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        length: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">
                    {params.length.toFixed(1)}m
                  </div>
                </div>
              </div>

              <div className="control-item">
                <label htmlFor="mass-slider">
                  <Layers size={10} style={{ marginRight: 4 }} /> Pole Mass
                </label>
                <div className="control-row">
                  <input
                    id="mass-slider"
                    type="range"
                    min="0.01"
                    max="1"
                    step="0.01"
                    value={params.massPole}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        massPole: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">
                    {params.massPole.toFixed(2)}kg
                  </div>
                </div>
              </div>
            </div>

            <div className="control-group">
              <div className="control-group-title">LQR Penalties</div>
              <div className="control-item">
                <label htmlFor="qpos-slider">
                  <Cpu size={10} style={{ marginRight: 4 }} /> Position (Q1)
                </label>
                <div className="control-row">
                  <input
                    id="qpos-slider"
                    type="range"
                    min="1"
                    max="200"
                    step="1"
                    value={params.qPos}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        qPos: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">{params.qPos.toFixed(0)}</div>
                </div>
              </div>

              <div className="control-item">
                <label htmlFor="qvel-slider">
                  <Cpu size={10} style={{ marginRight: 4 }} /> Velocity (Q2)
                </label>
                <div className="control-row">
                  <input
                    id="qvel-slider"
                    type="range"
                    min="1"
                    max="200"
                    step="1"
                    value={params.qVel}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        qVel: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">{params.qVel.toFixed(0)}</div>
                </div>
              </div>

              <div className="control-item">
                <label htmlFor="qang-slider">
                  <Cpu size={10} style={{ marginRight: 4 }} /> Angle (Q3)
                </label>
                <div className="control-row">
                  <input
                    id="qang-slider"
                    type="range"
                    min="1"
                    max="200"
                    step="1"
                    value={params.qAng}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        qAng: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">{params.qAng.toFixed(0)}</div>
                </div>
              </div>

              <div className="control-item">
                <label htmlFor="qomg-slider">
                  <Cpu size={10} style={{ marginRight: 4 }} /> Ang.Vel (Q4)
                </label>
                <div className="control-row">
                  <input
                    id="qomg-slider"
                    type="range"
                    min="1"
                    max="50"
                    step="1"
                    value={params.qOmg}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        qOmg: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">{params.qOmg.toFixed(0)}</div>
                </div>
              </div>

              <div className="control-item">
                <label htmlFor="r-slider">
                  <Activity size={10} style={{ marginRight: 4 }} /> Control
                  Effort (R)
                </label>
                <div className="control-row">
                  <input
                    id="r-slider"
                    type="range"
                    min="0.01"
                    max="100"
                    step="0.1"
                    value={params.rCtrl}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        rCtrl: parseFloat(e.target.value),
                      }))
                    }
                  />
                  <div className="control-value">{params.rCtrl.toFixed(3)}</div>
                </div>
              </div>
            </div>

            <div className="control-group">
              <div className="control-group-title">Solver</div>
              <div className="control-item">
                <label htmlFor="solver-select">
                  <Cpu size={10} style={{ marginRight: 4 }} /> Integration
                  Method
                </label>
                <div className="control-row">
                  <select
                    id="solver-select"
                    value={params.integrationMethod}
                    onChange={(e) =>
                      setParams((p) => ({
                        ...p,
                        integrationMethod: parseInt(e.target.value),
                      }))
                    }
                  >
                    <option value={IntegrationMethod.Euler}>
                      Euler (Fast)
                    </option>
                    <option value={IntegrationMethod.ODE3}>
                      ODE3 (Balanced)
                    </option>
                    <option value={IntegrationMethod.RK4}>
                      Runge-Kutta 4 (Precise)
                    </option>
                  </select>
                </div>
              </div>
            </div>
          </div>

          <div className="sidebar-footer">
            <div className="control-item">
              <label
                style={{ paddingLeft: "1.25rem", paddingRight: "1.25rem" }}
              >
                Chart (pos)
              </label>
              <ChartCanvas historyRef={historyRef} />
            </div>
          </div>
        </aside>

        <main className="viewport">
          <Header
            connected={connected}
            isValid={isValid}
            pps={pps}
            targetX={targetX}
            state={state}
            onReset={handleReset}
          />
          <SimulationCanvas
            state={state}
            target_x={targetX}
            params={params}
            is_valid={isValid}
            on_target_change={setTargetX}
          />
        </main>
      </div>
      {process.env.NODE_ENV === "development" && (
        <Agentation endpoint="http://localhost:4747" />
      )}
    </>
  );
};

export default App;
