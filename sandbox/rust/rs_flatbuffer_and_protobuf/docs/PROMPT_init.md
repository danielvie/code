Build a performance comparison POC between Protobuf and FlatBuffers for high-frequency streaming.

* **Environment:** Windows 11. Orchestrate all commands using a `Taskfile.yml`.

* **Backend (Rust):**

  * Use `cargo` to initialize the project.

  * Create a simulation loop generating an array of 5,000 `Sensor` structs (id: u32, value: f32, x: f32, y: f32, z: f32) at 60Hz.

  * Implement a `tokio-tungstenite` WebSocket server.

  * Serialize the data into Protobuf (using `prost`) on `/proto` and FlatBuffers (using `flatbuffers`) on `/flat`. Use a pre-allocated `FlatBufferBuilder` to avoid memory reallocation.

* **Frontend (React + Vite):**

  * Use React with TypeScript.

  * Client A: Connect to `/proto`, decode via `protobufjs`.

  * Client B: Connect to `/flat`, read via generated FlatBuffers JS classes.

  * Build a UI to display: Payload Size (bytes) and Deserialization/Access Time (ms).

* **Tooling:** Provide the exact `.proto` and `.fbs` schema files. The `Taskfile.yml` must include tasks for schema compilation (`protoc` and `flatc`), installing dependencies, and running both applications concurrently.