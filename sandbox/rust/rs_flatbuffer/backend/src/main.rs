//! WebSocket server — the backend entry point.
//!
//! Each browser tab that connects gets its own independent simulation.
//! Two concurrent tasks run per connection:
//!
//!   ┌─────────────────────────────────────────────────────────────┐
//!   │  SEND LOOP (60 Hz)                                          │
//!   │  step physics → serialize to FlatBuffer → send binary frame │
//!   └─────────────────────────────────────────────────────────────┘
//!   ┌─────────────────────────────────────────────────────────────┐
//!   │  RECEIVE LOOP                                               │
//!   │  receive binary frame → decode FlatBuffer → update sim      │
//!   └─────────────────────────────────────────────────────────────┘
//!
//! Both tasks share the simulation state through an Arc<Mutex<Simulation>>.

mod generated; // auto-generated FlatBuffers types (from schema/simulation.fbs)
mod physics;   // cart-pole dynamics and LQR controller

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use flatbuffers::FlatBufferBuilder;
use futures_util::{SinkExt, StreamExt};
use generated::{Command, Message, MessageArgs, MessageType, Parameters, PendulumState, PendulumStateArgs};
use physics::{Params, Simulation, DT_SECONDS};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};

const BIND_ADDR: &str = "127.0.0.1:8765";

// ─── Entry point ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let listener = TcpListener::bind(BIND_ADDR)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind {BIND_ADDR}: {e}"));

    log::info!("Server listening on ws://{BIND_ADDR}");

    // Accept connections forever; each gets its own async task.
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => { tokio::spawn(handle_client(stream, addr)); }
            Err(e)             => log::error!("Accept error: {e}"),
        }
    }
}

// ─── Per-client handler ───────────────────────────────────────────────────────

async fn handle_client(stream: TcpStream, addr: SocketAddr) {
    log::info!("Client connected: {addr}");

    // Upgrade the raw TCP connection to a WebSocket.
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => { log::error!("WebSocket handshake failed for {addr}: {e}"); return; }
    };

    // Split into independent write (tx) and read (rx) halves.
    let (ws_tx, mut ws_rx) = ws.split();

    // The write half is shared with the send-loop task below.
    let ws_tx = Arc::new(Mutex::new(ws_tx));

    // One simulation instance per client — isolated, no shared global state.
    let sim = Arc::new(Mutex::new(Simulation::new()));

    // ── Task 1: Send loop — runs the physics and streams state at 60 Hz ───────
    tokio::spawn({
        let sim   = Arc::clone(&sim);
        let ws_tx = Arc::clone(&ws_tx);

        async move {
            // One FlatBufferBuilder reused every frame (reset, not reallocated).
            let mut builder = FlatBufferBuilder::with_capacity(512);

            // Absolute-deadline timer: tracks when the next frame is due.
            // This prevents drift — any processing time is automatically
            // subtracted from the next sleep, keeping a steady 60 Hz.
            let mut next_frame_due = tokio::time::Instant::now();
            let frame_interval     = Duration::from_secs_f64(DT_SECONDS);

            loop {
                // 1. Advance the physics simulation by one timestep.
                sim.lock().await.step();

                // 2. Encode the current state as a FlatBuffer binary payload.
                let payload = {
                    let guard = sim.lock().await;
                    encode_state(&guard, &mut builder)
                };

                // 3. Send the binary frame over the WebSocket.
                //    If the send fails the client has disconnected — stop the loop.
                let send_result = ws_tx.lock().await
                    .send(WsMessage::Binary(payload.into()))
                    .await;

                if send_result.is_err() {
                    break;
                }

                // 4. Sleep until the next frame deadline.
                //    If we're already behind, snap forward and yield instead
                //    of trying to "catch up" with a burst of frames.
                next_frame_due += frame_interval;
                let now = tokio::time::Instant::now();
                if next_frame_due > now {
                    tokio::time::sleep_until(next_frame_due).await;
                } else {
                    next_frame_due = now;
                    tokio::task::yield_now().await;
                }
            }

            log::info!("Send loop stopped for {addr}");
        }
    });

    // ── Task 2: Receive loop — decodes incoming FlatBuffer messages ───────────
    while let Some(msg) = ws_rx.next().await {
        let msg = match msg {
            Ok(m)  => m,
            Err(e) => { log::debug!("Recv error for {addr}: {e}"); break; }
        };

        if msg.is_binary() {
            decode_and_apply(msg.into_data().to_vec(), &sim).await;
        }
        // Text / ping / pong frames are silently ignored.
    }

    log::info!("Client disconnected: {addr}");
}

// ─── Incoming message decoder ────────────────────────────────────────────────
//
// The frontend wraps every message in a Message envelope that carries a `type`
// discriminant and a nested `data` byte vector containing the actual payload.
//
//   Message { type: Params | Cmd | Reset, data: [ubyte] }
//
// We decode the outer envelope first, then the inner payload.

async fn decode_and_apply(data: Vec<u8>, sim: &Arc<Mutex<Simulation>>) {
    // Decode the outer Message envelope.
    let msg = match flatbuffers::root::<Message>(&data) {
        Ok(m)  => m,
        Err(e) => { log::warn!("Malformed FlatBuffer: {e}"); return; }
    };

    match msg.type_() {
        MessageType::Params => apply_params(msg, sim).await,
        MessageType::Cmd    => apply_cmd(msg, sim).await,
        MessageType::Reset  => apply_reset(sim).await,
        _                   => log::debug!("Unknown message type: {:?}", msg.type_()),
    }
}

// Decode a Parameters message and update the simulation's physical + LQR params.
async fn apply_params(msg: Message<'_>, sim: &Arc<Mutex<Simulation>>) {
    let Some(raw) = msg.data() else {
        log::warn!("Params message has no data.");
        return;
    };

    // Decode the inner Parameters table from the nested byte vector.
    let fb = match flatbuffers::root::<Parameters>(raw.bytes()) {
        Ok(p)  => p,
        Err(e) => { log::warn!("Malformed Parameters: {e}"); return; }
    };

    // Map FlatBuffer fields (f32) → Rust Params struct (f64).
    let new_params = Params {
        mass_cart:          fb.mass_cart() as f64,
        mass_pole:          fb.mass_pole() as f64,
        length:             fb.length()    as f64,
        q_pos:              fb.q_pos()     as f64,
        q_vel:              fb.q_vel()     as f64,
        q_ang:              fb.q_ang()     as f64,
        q_omg:              fb.q_omg()     as f64,
        r_ctrl:             fb.r_ctrl()    as f64,
        integration_method: fb.integration_method(),
    };

    // Log when the integration method changes (not on every param update).
    {
        let current = sim.lock().await.params.integration_method;
        if current != new_params.integration_method {
            log::info!("Integration method changed: {:?} → {:?}", current, new_params.integration_method);
        }
    }

    // Applying params also recomputes the LQR gain matrix internally.
    sim.lock().await.apply_params(new_params);
}


// Decode a Command message and update the cart's target position.
async fn apply_cmd(msg: Message<'_>, sim: &Arc<Mutex<Simulation>>) {
    let Some(raw) = msg.data() else {
        log::warn!("Cmd message has no data.");
        return;
    };

    let fb = match flatbuffers::root::<Command>(raw.bytes()) {
        Ok(c)  => c,
        Err(e) => { log::warn!("Malformed Command: {e}"); return; }
    };

    sim.lock().await.target_x = fb.target_position() as f64;
}

// Reset the physical state (position, velocity, angle) without touching params.
async fn apply_reset(sim: &Arc<Mutex<Simulation>>) {
    sim.lock().await.reset_state();
    log::info!("Simulation reset.");
}

// ─── Outgoing message encoder ─────────────────────────────────────────────────
//
// We encode state using a two-layer FlatBuffer:
//
//   Layer 1 — PendulumState table  (cart & pole physics values)
//   Layer 2 — Message envelope     (type: State, data: <layer-1 bytes>)
//
// This two-layer design mirrors the schema's multiplexed message format and
// allows the frontend to distinguish State from any future server-push types
// using the same `type` discriminant.
//
// The builder is reset (not reallocated) each frame to avoid heap allocations
// in the hot path.

fn encode_state(sim: &Simulation, builder: &mut FlatBufferBuilder) -> Vec<u8> {
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    // ── Layer 1: build PendulumState ─────────────────────────────────────────
    builder.reset();

    let state_offset = PendulumState::create(builder, &PendulumStateArgs {
        cart_position:             sim.x     as f32,
        cart_velocity:             sim.v     as f32,
        pendulum_angle:            sim.theta as f32,
        pendulum_angular_velocity: sim.omega as f32,
        timestamp:                 timestamp_ms,
        is_valid:                  sim.is_valid,
    });
    builder.finish(state_offset, None);

    // Copy the finished bytes — we need to reset the builder for layer 2,
    // but the inner bytes must survive to be embedded as the data field.
    let inner_bytes = builder.finished_data().to_vec();

    // ── Layer 2: wrap in Message envelope ────────────────────────────────────
    builder.reset();

    let data_vector = builder.create_vector(&inner_bytes);
    let msg_offset  = Message::create(builder, &MessageArgs {
        type_: MessageType::State,
        data:  Some(data_vector),
    });
    builder.finish(msg_offset, None);

    builder.finished_data().to_vec()
}
