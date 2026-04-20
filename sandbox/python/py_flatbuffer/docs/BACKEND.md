# Backend Specifications: Simulation & Control Engine

The backend is built as a high-performance Python application designed to execute non-linear physics simulations and solve optimal control problems with minimal overhead.

## 1. Physics Engine: Inverted Pendulum Dynamics

### State Representation
The system state is represented by the vector $z \in \mathbb{R}^4$:
$$z = [x, \dot{x}, \theta, \dot{\theta}]^T$$
- **$x$**: Horizontal position of the cart (meters).
- **$\dot{x}$**: Velocity of the cart ($m/s$).
- **$\theta$**: Angle of the pole from upright vertical (radians).
- **$\dot{\theta}$**: Angular velocity of the pole ($rad/s$).

### Dynamic Model
The system follows the non-linear equations of motion for a cart-pole system:
1. **Acceleration Calculations**: Utilizes the mass of the cart ($M$), mass of the pole ($m$), and the half-length of the pole ($L$).
2. **Numerical Integration**: Supports multiple fixed-step integration methods:
   - **Euler**: Standard first-order integration.
   - **RK4**: 4th-order Runge-Kutta for high precision.
   - **ODE3**: 3rd-order Bogacki-Shampine for a balance of speed and stability.
   - All methods run at a fixed frequency of 60Hz.

### Numerical Stability & Safety
- **Stability Monitoring**: Every step validates state finiteness using `math.isfinite`.
- **Magnitude Capping**: States are checked against a hardware-safe limit of $10^{30}$.
- **Crash Recovery**: If instability is detected, the simulation enters a `HALTED` state (`is_valid = false`) and zeros the state vectors to prevent serialization overflows.

---

## 2. Controller: Linear Quadratic Regulator (LQR)

### Continuous Linearization
The system is linearized around the unstable equilibrium (pole upright):
- **Matrix A**: The $4 \times 4$ system dynamics matrix.
- **Matrix B**: The $4 \times 1$ input distribution matrix.
- **Algebraic Riccati Equation (ARE)**: The system solves for the optimal cost-to-go matrix $P$:
  $$A^T P + P A - (P B R^{-1} B^T P) + Q = 0$$

### Weighting Matrices (User-Tunable)
- **Q Matrix**: Diagonal penalty for state errors: $diag(q_{pos}, q_{vel}, q_{ang}, q_{omg})$.
- **R Matrix**: Scalar penalty for control effort ($r_{ctrl}$).
- **Dynamic Solving**: The gain matrix $K$ is re-computed instantly whenever any of the $Q$ or $R$ weights are modified in the dashboard.

---

## 3. Communication & Infrastructure

### WebSocket Architecture
- **Per-Client Simulation**: Each connection spawns a dedicated `Simulation` instance, ensuring true "sandboxed" interaction.
- **Drift-Correcting Loop**: Uses absolute timestamps to calculate sleep intervals, preventing timing drift and compensating for processing time to maintain a strict 60Hz PPS.

### Serialization (FlatBuffers)
- **Schema**: `simulation.fbs`
- **Output**: Binary payloads are transmitted as `uint8` arrays via WebSocket. The backend explicitly manages the `flatbuffers.Builder` state to minimize memory allocation per frame.
