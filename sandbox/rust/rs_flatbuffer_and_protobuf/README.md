# Streaming Vectors Benchmark: Protobuf vs FlatBuffers

A performance comparison proof-of-concept between **Protocol Buffers (Protobuf)** and **Google FlatBuffers** focusing on deserialization overhead during high-frequency telemetry.

The project simulates an aggressive 60Hz vector coordinate stream from a backend orchestrator. A sleek dashboard maps identical payloads across the two binary wire formats to showcase the architectural advantages of FlatBuffers' zero-copy pointer traversal against standard Protobuf deserialization mechanisms.

## Tech Stack
* **Rust**: Headless `tokio::mpsc` WebSocket orchestrator allocating dynamic 60Hz simulations.
* **React**: Cyberpunk/Technical HUD layout (Vite).
* **Code Gen**: Uses schema compilation dynamically orchestrated by the Taskfile.

---

## 🚀 Setup & Execution

You will need **Node.js**, **Cargo (Rust)**, and the **[Go Task](https://taskfile.dev/)** orchestrator accessible securely in your path.

### 1. Initialize System & Install Dependencies
This configures `npm`, fetches the native `flatc` `.exe` Google FlatBuffers compiler binary to `/bin/`, and configures core files:
```powershell
task install
```

### 2. Formulate Binaries
Cross-compile `.proto` and `.fbs` layouts to dynamic TS + Rust modules.
```powershell
task build-schemas
```

### 3. Run Environment
Spawns both the Rust binary simulator (`:8080`) and Vite frontend local networking concurrently in parallel:
```powershell
task dev
```

---

## ⚙ Controls & Features
* **Matrix Reallocation:** Tweak real-time array sizes (`1k` through `50k+`) dynamically on a live connection.
* **HUD Signal Filters:** Smooth aggressive jitter locally using a live math-driven interactive Low-Pass temporal filtering slider.

## Available Manual Tasks
* `task front:run` - Start client isolated.
* `task back:run` - Start matrix host isolated.
* `task front:build` - Output Vite Production Bundle
* `task back:build` - Strip Rust `cargo release` Bundle
