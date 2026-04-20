# Frontend — React Telemetry Dashboard

A React + TypeScript single-page application that connects to the Rust backend over WebSocket,
renders the cart-pole simulation in real time, and exposes controls for parameter tuning.

---

## 1. Rendering

### Simulation viewport (main canvas)

- HTML5 Canvas 2D, updated via `requestAnimationFrame`.
- Coordinate mapping: simulation metres → pixels at **220 px/m**, pivot-centred.
- `ResizeObserver` keeps the canvas drawing buffer in sync with the DOM size.
- Visual states:
  - **Normal**: blue cart/bob, grey pole.
  - **Stability failure**: viewport darkens, "STABILITY FAILURE" overlay, objects turn red.
- Click anywhere on the canvas to set the cart's target position (`target_x`).

### Position chart (sidebar)

- Sliding window of the last **300 samples** (~5 seconds at 60 Hz).
- Managed via `useRef` to avoid React re-renders on every 60 Hz update.
- Plots cart position (blue) vs. target setpoint (red dashed).

---

## 2. Controls

| Control | What it does |
|---|---|
| Pole Length slider | Updates `length` in `Parameters` → backend recomputes LQR |
| Pole Mass slider | Updates `mass_pole` → backend recomputes LQR |
| Q sliders (q_pos, q_vel, q_ang, q_omg) | LQR state-error weights |
| R slider (r_ctrl) | LQR control-effort weight |
| Solver dropdown | Switches between Euler / ODE3 / RK4 |
| Canvas click | Sets target cart position (sends `Command` message) |
| Reset button (↺) | Sends `Reset` message — restores physical state, keeps params |
| Restore defaults button (↩) | Reverts all sliders to `DEFAULT_PARAMS` locally |

Parameter changes are sent to the backend automatically via a `useEffect` that fires
whenever the `params` state changes. The backend recomputes LQR gains on every `Parameters`
message it receives.

---

## 3. Communication

### Incoming — `State` frames (60 Hz)

```ts
// Decode the outer Message envelope
const msg   = Message.getRootAsMessage(new ByteBuffer(new Uint8Array(event.data)));

// Decode the inner PendulumState from the nested byte vector
const inner = new ByteBuffer(msg.dataArray()!);
const s     = PendulumState.getRootAsPendulumState(inner);

s.cartPosition();             // float → canvas x coordinate
s.pendulumAngle();            // float → pole rotation
s.isValid();                  // bool  → show/hide stability overlay
```

### Outgoing — `Parameters` / `Command` / `Reset`

All outgoing messages use the same two-layer pattern:

```ts
// 1. Build the inner payload
const inner  = new flatbuffers.Builder(256);
const offset = Parameters.endParameters(inner); // after adding fields
inner.finish(offset);

// 2. Wrap in a Message envelope
const outer  = new flatbuffers.Builder(512);
const data   = Message.createDataVector(outer, inner.asUint8Array());
Message.startMessage(outer);
Message.addType(outer, MessageType.Params);
Message.addData(outer, data);
outer.finish(Message.endMessage(outer));

ws.send(outer.asUint8Array());
```

### Connection indicator

The status LED in the sidebar header reflects the WebSocket state:

| LED | State |
|---|---|
| 🟢 pulsing green | Connected, simulation running |
| 🔴 blinking red | Connected, stability failure |
| ⚫ dim grey | Disconnected from backend |

---

## 4. Performance

- The 60 Hz `PendulumState` stream updates physics state via `setState` but the canvas
  runs its own `requestAnimationFrame` loop — React re-renders and canvas redraws are
  decoupled.
- History and packet counter are stored in `useRef` (no re-render on write).
- The FlatBuffers `ByteBuffer` is allocated once per incoming frame from the raw
  `ArrayBuffer` — no JSON parsing, no intermediate objects.

---

## Source files

| File | Purpose |
|---|---|
| `src/App.tsx` | Main component: WebSocket setup, state, renders, controls |
| `src/index.css` | Design system tokens, layout, LED animations |
| `src/generated/` | Auto-generated TypeScript FlatBuffer types (do not edit) |
