# Frontend Specifications: Telemetry Dashboard

The frontend is a React-based real-time dashboard designed to visualize high-frequency binary telemetry and provide a low-latency interface for physical parameter injection.

## 1. Visualization Architecture

### Viewport Rendering (Main Canvas)
- **Engine**: HTML5 Canvas 2D API.
- **Coordinate System**: Pivot-centered. Simulation coordinates ($m$) are scaled to pixels using a calibration constant ($220 px/m$).
- **Adaptive Resolution**: Utilizes `ResizeObserver` to synchronize the internal canvas drawing buffer with the physical DOM dimensions, ensuring high-fidelity rendering on high-DPI displays.
- **Visual Logic**:
  - **Blue Traces**: Active system state (Bob/Cart).
  - **Red Dotted Trace**: Navigation Setpoint (Target).
  - **Post-Crash State**: Entire viewport darkens with a "STABILITY FAILURE" overlay and red object coloration.

### Performance Tracking (Telemetry Chart)
- **Data Buffering**: Maintains a sliding window of the last 300 samples ($ \sim 5$ seconds of data).
- **Optimization**: State history is managed via `useRef` to bypass the React re-render reconciliation bottleneck for 60Hz updates.
- **Visuals**: Plots current Position ($x$) against Target Setpoint ($x_{target}$).
- **Layout**: The telemetry grid displays **Target**, **Pos.X**, and **Deg.θ** in a single horizontal row for maximum density.

---

## 2. Interaction Design

### Direct Physical Control
- **Click-to-Set**: Implements screen-to-simulation coordinate transformation.
  - $x_{target} = (pixelX - centerX) / scale$
- **Cursor Feedback**: Context-aware cursor changes to `crosshair` when hovering over the interactive viewport.
- **Simplified Controls**: Manual setpoint sliders have been replaced by this direct interaction model.

### Parameter Injection Logic
- **LQR Tuning**: Exposes full granular control over the $Q$ and $R$ diagonal elements.
- **Physical Modification**: Real-time adjustment of Pole Length and Pole Mass.
- **Solver Selection**: Integrated dropdown to switch between **Euler**, **RK4**, and **ODE3** numerical integrators.
- **State Restoration**:
  - **Undo (Restore Defaults)**: Reverts local component state to `DEFAULT_PARAMS`.
  - **Rotate (Reset Simulation)**: Dispatches a discrete `Reset` signal to restart the physical state while preserving current parameters.

---

## 3. Communication Layer

### Binary Streaming (FlatBuffers)
- **Decoder**: Processes incoming `ArrayBuffer` payloads into generated `PendulumState` objects.
- **Encoder**: Serializes user updates into binary `Parameters` or `Command` messages.
- **Multiplexing**: Uses the `Message` wrapper table to handle multiple data types over a single WebSocket channel.

### System Diagnostics
- **PPS Monitor**: Samples packet arrival frequency.
  - Calculated as: $COUNT(packets) \Delta t$ where $\Delta t = 1.0s$.
- **Latency Indicator**: Visual LED pulses linked to WebSocket `onopen` and `onclose` events.
