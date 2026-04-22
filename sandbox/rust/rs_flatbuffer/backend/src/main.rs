//! WebSocket server - the backend entry point.
//!
//! All connected browser tabs share a single, global simulation.
//!
//! Architecture:
//! 1. Global Physics Task: Runs at 60Hz, advances simulation, and broadcasts state.
//! 2. Client Task:
//!    - RECEIVE: decodes binary frames from WS and updates the global simulation.
//!    - SEND: subscribes to the broadcast channel and forwards state to the WS.

mod generated; 
mod physics;   

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use flatbuffers::FlatBufferBuilder;
use futures_util::{SinkExt, StreamExt};
use generated::{Message, MessageArgs, MessageType, PendulumState, PendulumStateArgs};
use physics::{Simulation, DT_SECONDS};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message as WsMessage};

const BIND_ADDR: &str = "127.0.0.1:8765";

/// Application entry point: initializes logging, shared state, and background tasks.
#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // The single source of truth shared by all clients.
    let sim = Arc::new(Mutex::new(Simulation::new()));

    // Broadcast channel for sending the 60Hz state updates to all clients.
    let (tx, _) = broadcast::channel::<Vec<u8>>(16);

    // Spawn the global physics loop.
    tokio::spawn(simulation_run(Arc::clone(&sim), tx.clone()));

    // Sets the listenner for incoming TCP traffic
    let listener = TcpListener::bind(BIND_ADDR)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind {BIND_ADDR}: {e}"));

    log::info!("Server listening on ws://{BIND_ADDR}");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                tokio::spawn(websocket_handler(stream, addr, Arc::clone(&sim), tx.clone()));
            }
            Err(e) => log::error!("Accept error: {e}"),
        }
    }
}

/// Simulation driver: steps the physics engine and broadcasts state updates at 60Hz.
async fn simulation_run(sim: Arc<Mutex<Simulation>>, tx: broadcast::Sender<Vec<u8>>) {
    let mut builder = FlatBufferBuilder::with_capacity(512);
    let mut next_frame_due = tokio::time::Instant::now();
    let frame_interval = Duration::from_secs_f64(DT_SECONDS);

    loop {
        sim.lock().await.step();

        let payload = {
            let guard = sim.lock().await;
            encode_state(&guard, &mut builder)
        };

        let _ = tx.send(payload);

        next_frame_due += frame_interval;
        let now = tokio::time::Instant::now();
        if next_frame_due > now {
            tokio::time::sleep_until(next_frame_due).await;
        } /* else {
            next_frame_due = now;
            tokio::task::yield_now().await;
        } */
    }
}

/// Manages the WebSocket lifecycle and message routing for a single client.
async fn websocket_handler(
    stream: TcpStream,
    addr: SocketAddr,
    sim: Arc<Mutex<Simulation>>,
    tx: broadcast::Sender<Vec<u8>>,
) {
    log::info!("Client connected: {addr}");

    // 1. WebSocket Handshake
    let ws = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            log::error!("WebSocket handshake failed for {addr}: {e}");
            return;
        }
    };

    // 2. Split into sink (TX) and stream (RX)
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    // 3. Spawn task to forward simulation updates to this client
    let send_task = tokio::spawn(async move {
        while let Ok(payload) = rx.recv().await {
            if let Err(e) = ws_tx.send(WsMessage::Binary(payload.into())).await {
                log::debug!("Send error for {addr}: {e}");
                break;
            }
        }
    });

    // 4. Inbound message loop (Params, Cmd, Reset)
    while let Some(msg) = ws_rx.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                log::debug!("Recv error for {addr}: {e}");
                break;
            }
        };

        if msg.is_binary() {
            // Dispatch binary data to simulation handlers
            simulation_process_message(msg.into_data().to_vec(), &sim).await;
        }
    }

    // 5. Cleanup on disconnect
    send_task.abort();
    log::info!("Client disconnected: {addr}");
}

/// Decodes FlatBuffer frames and delegates to specific handlers.
async fn simulation_process_message(data: Vec<u8>, sim: &Arc<Mutex<Simulation>>) {
    let msg = match flatbuffers::root::<Message>(&data) {
        Ok(m) => m,
        Err(e) => {
            log::warn!("Malformed FlatBuffer: {e}");
            return;
        }
    };

    match msg.type_() {
        MessageType::Params => simulation_apply_params(msg, sim).await,
        MessageType::Cmd => simulation_apply_cmd(msg, sim).await,
        MessageType::Reset => simulation_apply_reset(sim).await,
        _ => log::debug!("Unknown message type: {:?}", msg.type_()),
    }
}

/// Parameter handler: updates physical constants like mass and length in the simulation.
async fn simulation_apply_params(msg: Message<'_>, sim: &Arc<Mutex<Simulation>>) {
    let Some(raw) = msg.data() else {
        return;
    };

    let fb = match flatbuffers::root::<generated::Parameters>(raw.bytes()) {
        Ok(p) => p,
        Err(e) => {
            log::warn!("Malformed Parameters: {e}");
            return;
        }
    };

    let new_params = physics::Params {
        mass_cart: fb.mass_cart() as f64,
        mass_pole: fb.mass_pole() as f64,
        length: fb.length() as f64,
        q_pos: fb.q_pos() as f64,
        q_vel: fb.q_vel() as f64,
        q_ang: fb.q_ang() as f64,
        q_omg: fb.q_omg() as f64,
        r_ctrl: fb.r_ctrl() as f64,
        integration_method: fb.integration_method(),
    };

    sim.lock().await.apply_params(new_params);
}

/// Command handler: updates the target horizontal position of the cart.
async fn simulation_apply_cmd(msg: Message<'_>, sim: &Arc<Mutex<Simulation>>) {
    let Some(raw) = msg.data() else {
        return;
    };

    let fb = match flatbuffers::root::<generated::Command>(raw.bytes()) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Malformed Command: {e}");
            return;
        }
    };

    sim.lock().await.target_x = fb.target_position() as f64;
}

/// Reset handler: restores the simulation to its starting state.
async fn simulation_apply_reset(sim: &Arc<Mutex<Simulation>>) {
    sim.lock().await.reset_state();
    log::info!("Global simulation reset.");
}

/// State encoder: serializes current simulation data into a binary FlatBuffer payload.
fn encode_state(sim: &physics::Simulation, builder: &mut FlatBufferBuilder) -> Vec<u8> {
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    builder.reset();

    let state_offset = PendulumState::create(
        builder,
        &PendulumStateArgs {
            cart_position: sim.x as f32,
            cart_velocity: sim.v as f32,
            pendulum_angle: sim.theta as f32,
            pendulum_angular_velocity: sim.omega as f32,
            timestamp: timestamp_ms,
            is_valid: sim.is_valid,
        },
    );
    builder.finish(state_offset, None);

    let inner_bytes = builder.finished_data().to_vec();

    builder.reset();

    let data_vector = builder.create_vector(&inner_bytes);
    let msg_offset = Message::create(
        builder,
        &MessageArgs {
            type_: MessageType::State,
            data: Some(data_vector),
        },
    );
    builder.finish(msg_offset, None);

    builder.finished_data().to_vec()
}
