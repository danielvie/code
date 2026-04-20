use flatbuffers::FlatBufferBuilder;
use futures_util::{SinkExt, StreamExt};
use prost::Message;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::tungstenite::Message as WsMessage;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod sensor_fbs_gen {
    include!("generated/sensor_generated.rs");
}
pub use sensor_fbs_gen::sensor_fbs;

pub mod sensor_proto {
    include!("generated/sensor_proto.rs");
}

#[derive(Clone, serde::Serialize)]
struct RawSensor {
    id: i32,
    value: f32,
    x: f32,
    y: f32,
    z: f32,
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Listening on: {}", addr);

    let (proto_tx, _) = broadcast::channel::<Vec<u8>>(16);
    let (flat_tx, _) = broadcast::channel::<Vec<u8>>(16);
    let (json_tx, _) = broadcast::channel::<Vec<u8>>(16);
    let (update_tx, mut update_rx) = mpsc::channel::<usize>(16);

    let proto_tx_clone = proto_tx.clone();
    let flat_tx_clone = flat_tx.clone();
    let json_tx_clone = json_tx.clone();

    // Simulation Task
    tokio::spawn(async move {
        let mut builder = FlatBufferBuilder::with_capacity(1024 * 1024);
        let mut sensors = vec![RawSensor { id: 0, value: 0.0, x: 0.0, y: 0.0, z: 0.0 }; 5000];

        let mut ticker = tokio::time::interval(Duration::from_millis(1000 / 60)); // 60 Hz

        let mut step: f32 = 0.0;
        loop {
            ticker.tick().await;
            
            while let Ok(new_size) = update_rx.try_recv() {
                let size = new_size.clamp(1, 500_000);
                sensors.resize(size, RawSensor { id: 0, value: 0.0, x: 0.0, y: 0.0, z: 0.0 });
            }

            step += 0.01;

            for (i, sensor) in sensors.iter_mut().enumerate() {
                sensor.id = i as i32;
                sensor.value = (step + i as f32).sin();
                sensor.x = (step * 0.5 + i as f32).cos();
                sensor.y = (step * 0.8 + i as f32).sin();
                sensor.z = (step * 1.2 + i as f32).cos();
            }

            // Serialize Protobuf
            let mut proto_sensors = Vec::with_capacity(sensors.len());
            for s in &sensors {
                proto_sensors.push(sensor_proto::Sensor {
                    id: s.id as u32,
                    value: s.value,
                    x: s.x,
                    y: s.y,
                    z: s.z,
                });
            }
            let proto_list = sensor_proto::SensorList { sensors: proto_sensors };
            let mut proto_buf = Vec::new();
            proto_list.encode(&mut proto_buf).unwrap();

            // Broadcast Protobuf
            let _ = proto_tx_clone.send(proto_buf);

            // Serialize FlatBuffers
            builder.reset();
            let mut fbs_sensors = Vec::with_capacity(sensors.len());
            for s in &sensors {
                fbs_sensors.push(sensor_fbs::Sensor::new(s.id as u32, s.value, s.x, s.y, s.z));
            }
            let vec = builder.create_vector(&fbs_sensors);
            let mut fbs_list_builder = sensor_fbs::SensorListBuilder::new(&mut builder);
            fbs_list_builder.add_sensors(vec);
            let fbs_list = fbs_list_builder.finish();
            builder.finish(fbs_list, None);
            let flat_buf = builder.finished_data().to_vec();

            // Broadcast FlatBuffers
            let _ = flat_tx_clone.send(flat_buf);

            // 3. Serialize to JSON
            if json_tx_clone.receiver_count() > 0 {
                if let Ok(json_data) = serde_json::to_vec(&sensors) {
                    let _ = json_tx_clone.send(json_data);
                }
            }
        }
    });

    while let Ok((stream, addr)) = listener.accept().await {
        let proto_rx = proto_tx.subscribe();
        let flat_rx = flat_tx.subscribe();
        let json_rx = json_tx.subscribe();
        let upd_tx = update_tx.clone();
        tokio::spawn(handle_connection(stream, addr, proto_rx, flat_rx, json_rx, upd_tx));
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    mut proto_rx: broadcast::Receiver<Vec<u8>>,
    mut flat_rx: broadcast::Receiver<Vec<u8>>,
    mut json_rx: broadcast::Receiver<Vec<u8>>,
    update_tx: mpsc::Sender<usize>,
) {
    let mut ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error accepting websocket: {}", e);
            return;
        }
    };

    println!("New WebSocket connection: {}", addr);

    // Initial message to get the path
    let msg = match ws_stream.next().await {
        Some(Ok(msg)) => msg,
        _ => return,
    };

    let path = msg.into_text().unwrap_or_default();
    let is_proto = path == "/proto";
    let is_flat = path == "/flat";
    let is_json = path == "/json";

    println!("Client requested path: {}", path);

    if is_proto {
        loop {
            tokio::select! {
                Ok(buf) = proto_rx.recv() => {
                    if ws_stream.send(WsMessage::Binary(buf)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(msg)) = ws_stream.next() => {
                    if let Ok(text) = msg.into_text() {
                        if let Ok(val) = text.parse::<usize>() {
                            let _ = update_tx.send(val).await;
                        }
                    }
                }
                else => break,
            }
        }
    } else if is_flat {
        loop {
            tokio::select! {
                Ok(buf) = flat_rx.recv() => {
                    if ws_stream.send(WsMessage::Binary(buf)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(msg)) = ws_stream.next() => {
                    if let Ok(text) = msg.into_text() {
                        if let Ok(val) = text.parse::<usize>() {
                            let _ = update_tx.send(val).await;
                        }
                    }
                }
                else => break,
            }
        }
    } else if is_json {
        loop {
            tokio::select! {
                Ok(buf) = json_rx.recv() => {
                    if ws_stream.send(WsMessage::Binary(buf)).await.is_err() {
                        break;
                    }
                }
                Some(Ok(msg)) = ws_stream.next() => {
                    if let Ok(text) = msg.into_text() {
                        if let Ok(val) = text.parse::<usize>() {
                            let _ = update_tx.send(val).await;
                        }
                    }
                }
                else => break,
            }
        }
    }
}
