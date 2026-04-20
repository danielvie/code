# Backend — Rust Simulation & Control Engine

The backend is a Rust binary that runs the physics simulation and LQR controller,
streaming state to connected frontend clients over WebSocket at 60 Hz using FlatBuffers.

---

## Architecture

Each browser tab that connects gets its own independent simulation instance.
Two concurrent async tasks run per connection:

```
┌──────────────────────────────────────────────────────────────┐
│  SEND LOOP  (60 Hz, drift-correcting)                        │
│  Simulation::step() → encode FlatBuffer → send binary frame  │
└──────────────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────────────┐
│  RECEIVE LOOP                                                │
│  receive binary frame → decode FlatBuffer → update sim state │
└──────────────────────────────────────────────────────────────┘
```

Both tasks share the simulation via `Arc<Mutex<Simulation>>`.

---

## 1. Physics Engine

### State vector

$$z = [x,\ \dot{x},\ \theta,\ \dot{\theta}]^T$$

| Symbol | Meaning | Unit |
|---|---|---|
| $x$ | Cart position | m |
| $\dot{x}$ | Cart velocity | m/s |
| $\theta$ | Pole angle from upright | rad |
| $\dot{\theta}$ | Pole angular velocity | rad/s |

### Equations of motion

The non-linear cart-pole ODE is integrated each frame:

$$\ddot{\theta} = \frac{g \sin\theta - \cos\theta \cdot \text{temp}}{L\left(\frac{4}{3} - \frac{m\cos^2\theta}{M+m}\right)}, \quad \text{temp} = \frac{F + mL\dot{\theta}^2\sin\theta}{M+m}$$

### Integration methods (user-selectable)

| Method | Order | Notes |
|---|---|---|
| Euler | 1st | Fastest, least accurate |
| ODE3 (Bogacki-Shampine) | 3rd | Good balance |
| RK4 | 4th | Most accurate |

All methods run at a fixed **60 Hz** timestep (`dt = 1/60 s`).

### Stability monitoring

Every step checks all state values for `NaN`, `±Inf`, or magnitude above `1e30`.
On overflow, `is_valid` is set to `false` and the simulation halts.
The frontend displays a "STABILITY FAILURE" overlay when it receives `is_valid = false`.

---

## 2. LQR Controller

The LQR computes a force `u = −K (state − target)` each frame.

### Gain computation steps

1. **Linearise** the cart-pole around the upright equilibrium → matrices **A** (4×4) and **B** (4×1).
2. **Solve CARE** — Continuous Algebraic Riccati Equation:
   $$A^T P + PA - PBR^{-1}B^TP + Q = 0$$
3. **Compute K**: $K = R^{-1} B^T P$

The solver uses the **matrix sign function on the Hamiltonian** (Kenney & Laub, 1989),
which converges quadratically without requiring a stabilising initial guess.

Gains are **recomputed every time the frontend sends updated parameters** (slider change).

### Tunable weights

| Parameter | Effect |
|---|---|
| Q (q_pos, q_vel, q_ang, q_omg) | Penalty for state errors |
| R (r_ctrl) | Penalty for control effort (force magnitude) |

---

## 3. Communication

### Message flow

```
Frontend                         Backend
────────                         ───────
Parameters / Command / Reset ──▶ decode FlatBuffer → update sim
                             ◀── encode FlatBuffer ← Simulation::step()
```

All messages use the two-layer FlatBuffer format defined in `schema/simulation.fbs`:
- **Outer**: `Message { type, data: [ubyte] }`
- **Inner**: the actual payload table (`PendulumState`, `Parameters`, `Command`)

### 60 Hz drift-correcting loop

The send loop uses **absolute deadline tracking** (`Instant`) rather than a relative sleep.
Any processing time spent on step + encode + send is automatically subtracted from
the next sleep interval, keeping a steady 60 Hz even under CPU load.

---

## Source files

| File | Purpose |
|---|---|
| `src/main.rs` | WebSocket server, encode/decode loop, per-client task spawning |
| `src/physics.rs` | Cart-pole ODE, integrators, LQR gain computation, CARE solver |
| `src/generated/` | Auto-generated Rust FlatBuffer types (do not edit) |
