//! Inverted-pendulum physics engine and LQR controller.
//!
//! This module owns everything that does not touch the network:
//!
//!   Params     — physical constants and LQR tuning weights
//!   Simulation — mutable runtime state (position, velocity, angle …)
//!   step()     — advance state by one 60 Hz timestep
//!
//! The controller is a Linear Quadratic Regulator (LQR).
//! It computes a single force applied to the cart each frame that keeps the
//! pole upright.  Tuning happens through the Q and R weight matrices:
//!   Q — how much to penalise deviations in the state (position, angle …)
//!   R — how much to penalise control effort (large forces)

use crate::generated::IntegrationMethod;
use nalgebra::{DMatrix, Matrix4, SVector, Vector4};

/// State magnitude above this value is treated as a numerical overflow.
const OVERFLOW_LIMIT: f64 = 1e30;

const GRAVITY: f64    = 9.81;       // m/s²
pub const DT_SECONDS: f64 = 1.0 / 60.0; // fixed 60 Hz timestep

/// Fallback LQR gains used only when the CARE solver fails.
/// These values are intentionally weak — the solver should always succeed
/// for reasonable physical parameters.
const K_FALLBACK: [f64; 4] = [-1.0, -1.0, 10.0, 1.0];

// ─── Parameters ──────────────────────────────────────────────────────────────

/// Everything the frontend can tune: physical constants + LQR weights.
/// Received as a FlatBuffer `Parameters` message and applied atomically.
#[derive(Clone)]
pub struct Params {
    pub mass_cart:          f64,  // cart mass M  (kg)
    pub mass_pole:          f64,  // pole mass m  (kg)
    pub length:             f64,  // full pole length (m)
    pub q_pos:              f64,  // LQR cost: cart position error
    pub q_vel:              f64,  // LQR cost: cart velocity error
    pub q_ang:              f64,  // LQR cost: pole angle error
    pub q_omg:              f64,  // LQR cost: pole angular velocity error
    pub r_ctrl:             f64,  // LQR cost: control effort (force)
    pub integration_method: IntegrationMethod,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            mass_cart: 1.0, mass_pole: 0.1, length: 1.0,
            q_pos: 10.0, q_vel: 1.0, q_ang: 100.0, q_omg: 10.0,
            r_ctrl: 0.01,
            integration_method: IntegrationMethod::Euler,
        }
    }
}

// ─── Simulation ───────────────────────────────────────────────────────────────

/// The complete mutable runtime state for one client session.
pub struct Simulation {
    pub params:   Params,
    pub gains_k:  Vector4<f64>, // LQR gain vector K  (recomputed when params change)
    pub target_x: f64,          // cart setpoint set by clicking the canvas

    // Physical state — the four values sent to the frontend each frame
    pub x:     f64,  // cart position       (m)
    pub v:     f64,  // cart velocity        (m/s)
    pub theta: f64,  // pole angle           (rad, 0 = upright)
    pub omega: f64,  // pole angular velocity (rad/s)

    pub is_valid: bool, // false after a numerical overflow; halts the loop
}

impl Simulation {
    pub fn new() -> Self {
        let params  = Params::default();
        let gains_k = compute_lqr_gains(&params);
        Self {
            gains_k, params,
            target_x: 0.0,
            x: 0.0, v: 0.0,
            theta: 0.1, // small nudge so the controller has something to correct
            omega: 0.0,
            is_valid: true,
        }
    }

    /// Reset position/velocity/angle back to the initial nudge.
    /// Called when the frontend sends a Reset message.
    /// Parameters and LQR gains are intentionally preserved.
    pub fn reset_state(&mut self) {
        self.x = 0.0; self.v = 0.0;
        self.theta = 0.1; self.omega = 0.0;
        self.is_valid = true;
    }

    /// Apply a new parameter set from the frontend.
    /// Recomputes the LQR gain matrix immediately so the next step()
    /// already uses the updated controller.
    pub fn apply_params(&mut self, p: Params) {
        self.params  = p;
        self.gains_k = compute_lqr_gains(&self.params);
    }

    /// Advance the simulation by one fixed timestep (1/60 s).
    pub fn step(&mut self) {
        if !self.is_valid { return; }

        let state  = Vector4::new(self.x, self.v, self.theta, self.omega);
        let target = Vector4::new(self.target_x, 0.0, 0.0, 0.0);

        // LQR control law: u = -K (x - x_target)
        // K was computed offline from the linearised system matrices.
        // A positive force pushes the cart right.
        let force = -self.gains_k.dot(&(state - target));

        // Integrate the non-linear equations of motion.
        let next = match self.params.integration_method {
            IntegrationMethod::RK4  => rk4(state, force, DT_SECONDS, &self.params),
            IntegrationMethod::ODE3 => ode3(state, force, DT_SECONDS, &self.params),
            _                       => euler(state, force, DT_SECONDS, &self.params),
        };

        self.x     = next[0];
        self.v     = next[1] * 0.999; // tiny damping keeps the cart from drifting
        self.theta = next[2];
        self.omega = next[3] * 0.999;

        // Keep theta in (−π, π] so the angle display makes sense.
        self.theta = (self.theta + std::f64::consts::PI) % (2.0 * std::f64::consts::PI)
            - std::f64::consts::PI;

        // If any value is NaN, ±Inf, or astronomically large the simulation
        // has diverged.  Zero out the state and set is_valid = false so the
        // frontend can show the "STABILITY FAILURE" overlay.
        let ok = [self.x, self.v, self.theta, self.omega]
            .iter()
            .all(|v| v.is_finite() && v.abs() < OVERFLOW_LIMIT);

        if !ok {
            self.is_valid = false;
            self.x = 0.0; self.v = 0.0; self.theta = 0.0; self.omega = 0.0;
            log::warn!("Simulation halted: numerical overflow detected.");
        }
    }
}

// ─── Equations of motion ──────────────────────────────────────────────────────
//
// The cart-pole is a classic non-linear system.  Given the current state and
// the applied force, these equations give the time-derivatives of the state:
//
//   d/dt [x, ẋ, θ, θ̇] = [ẋ, ẍ, θ̇, θ̈]
//
// ẍ  and  θ̈  are coupled through the constraint between cart and pole.

#[allow(non_snake_case)]
fn cart_pole_ode(state: Vector4<f64>, force: f64, p: &Params) -> Vector4<f64> {
    let (M, m, l, g) = (p.mass_cart, p.mass_pole, p.length, GRAVITY);
    let (_x, x_dot, theta, theta_dot) = (state[0], state[1], state[2], state[3]);

    let L     = l / 2.0;          // distance from pivot to pole centre of mass
    let sin_t = theta.sin();
    let cos_t = theta.cos();

    // Intermediate quantity shared by both acceleration formulas.
    let temp      = (force + m * L * theta_dot * theta_dot * sin_t) / (M + m);
    let theta_acc = (g * sin_t - cos_t * temp)
                    / (L * (4.0 / 3.0 - m * cos_t * cos_t / (M + m)));
    let x_acc     = temp - m * L * theta_acc * cos_t / (M + m);

    Vector4::new(x_dot, x_acc, theta_dot, theta_acc)
}

// ─── Numerical integrators ────────────────────────────────────────────────────
//
// All three run at the same fixed step (dt = 1/60 s).
// Higher-order methods are more accurate but cost more per frame.

/// Euler — fastest, least accurate.  Good enough for demo purposes.
fn euler(state: Vector4<f64>, force: f64, dt: f64, p: &Params) -> Vector4<f64> {
    state + cart_pole_ode(state, force, p) * dt
}

/// RK4 — four ODE evaluations per step.  High accuracy, standard choice.
fn rk4(state: Vector4<f64>, force: f64, dt: f64, p: &Params) -> Vector4<f64> {
    let k1 = cart_pole_ode(state,                  force, p);
    let k2 = cart_pole_ode(state + 0.5 * dt * k1, force, p);
    let k3 = cart_pole_ode(state + 0.5 * dt * k2, force, p);
    let k4 = cart_pole_ode(state + dt * k3,        force, p);
    state + (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}

/// ODE3 — Bogacki-Shampine third-order.  Good balance between RK4 and Euler.
fn ode3(state: Vector4<f64>, force: f64, dt: f64, p: &Params) -> Vector4<f64> {
    let k1 = cart_pole_ode(state,                          force, p);
    let k2 = cart_pole_ode(state + (1.0 / 2.0) * dt * k1, force, p);
    let k3 = cart_pole_ode(state + (3.0 / 4.0) * dt * k2, force, p);
    state + (dt / 9.0) * (2.0 * k1 + 3.0 * k2 + 4.0 * k3)
}

// ─── LQR gain computation ─────────────────────────────────────────────────────
//
// The LQR for a non-linear system is derived by linearising around the
// operating point (here: pole upright, everything at rest).
//
// Step 1 — Linearise: compute A (system dynamics) and B (input effect) matrices.
// Step 2 — Solve the Continuous Algebraic Riccati Equation (CARE) for P.
// Step 3 — Compute K = R⁻¹ Bᵀ P.
//
// K is then used every frame: force = -K (state - target).

#[allow(non_snake_case)]
pub fn compute_lqr_gains(p: &Params) -> Vector4<f64> {
    let (M, m, l, g) = (p.mass_cart, p.mass_pole, p.length, GRAVITY);

    let L       = l / 2.0;
    let inertia = (1.0 / 12.0) * m * l * l;

    // Denominator shared by several linearised matrix entries.
    let denom = (M + m) * (inertia + m * L * L) - (m * L).powi(2);

    if denom.abs() < 1e-12 {
        log::warn!("LQR: degenerate system — using fallback gains.");
        return Vector4::from(K_FALLBACK);
    }

    // A — how the state evolves on its own (at the linearisation point).
    #[rustfmt::skip]
    let a = Matrix4::new(
        0.0, 1.0,  0.0,                                   0.0,
        0.0, 0.0, -(m * m * L * L * g) / denom,           0.0,
        0.0, 0.0,  0.0,                                   1.0,
        0.0, 0.0,  (m * g * L * (M + m)) / denom,         0.0,
    );

    // B — how the force input affects each state derivative.
    let b = SVector::<f64, 4>::new(
        0.0,
        (inertia + m * L * L) / denom,
        0.0,
        -(m * L) / denom,
    );

    // Q penalises state errors; R penalises control effort.
    let q = Matrix4::from_diagonal(&Vector4::new(p.q_pos, p.q_vel, p.q_ang, p.q_omg));

    assert!(p.r_ctrl > 0.0, "R weight must be > 0");

    match solve_care(&a, &b, &q, p.r_ctrl) {
        Some(p_mat) => {
            let k = (1.0 / p.r_ctrl) * (b.transpose() * p_mat);
            log::info!("LQR K = [{:.2}, {:.2}, {:.2}, {:.2}]", k[0], k[1], k[2], k[3]);
            Vector4::new(k[0], k[1], k[2], k[3])
        }
        None => {
            log::warn!("CARE solver failed — using fallback gains.");
            Vector4::from(K_FALLBACK)
        }
    }
}

// ─── CARE solver (matrix sign function) ──────────────────────────────────────
//
// The Continuous Algebraic Riccati Equation:
//   Aᵀ P + P A − P B R⁻¹ Bᵀ P + Q = 0
//
// We solve it by iterating the matrix sign function on the 8×8 Hamiltonian:
//   H = [[ A,  −S ],    where S = B R⁻¹ Bᵀ
//        [ −Q, −Aᵀ ]]
//
// The iteration  H ← (H + H⁻¹) / 2  converges quadratically.
// Eigenvalues with Re < 0 → −1;  Re > 0 → +1.
//
// Once converged, P is extracted from the stable invariant subspace:
//   W = (I − sign(H)) / 2,   P = W₂₁ · W₁₁⁻¹

fn solve_care(
    a: &Matrix4<f64>,
    b: &SVector<f64, 4>,
    q: &Matrix4<f64>,
    r: f64,
) -> Option<Matrix4<f64>> {
    assert!(r > 0.0);

    let s = b * b.transpose() * (1.0 / r); // S = B R⁻¹ Bᵀ

    // Assemble the 8×8 Hamiltonian.
    let mut h = DMatrix::<f64>::zeros(8, 8);
    for i in 0..4 {
        for j in 0..4 {
            h[(i,     j    )] =  a[(i, j)];
            h[(i,     j + 4)] = -s[(i, j)];
            h[(i + 4, j    )] = -q[(i, j)];
            h[(i + 4, j + 4)] = -a[(j, i)]; // -Aᵀ: swap indices
        }
    }

    // Matrix sign function iteration.
    for iter in 0..200 {
        let h_inv = match h.clone().try_inverse() {
            Some(inv) => inv,
            None => { log::warn!("CARE: Hamiltonian singular at iter {iter}"); return None; }
        };
        let h_new = (&h + &h_inv) * 0.5;
        let delta = (&h_new - &h).norm();
        h = h_new;
        if delta < 1e-12 { break; }
    }

    // Extract P from the converged sign matrix.
    let w   = (DMatrix::<f64>::identity(8, 8) - &h) * 0.5;
    let w11 = Matrix4::from_iterator(w.view((0, 0), (4, 4)).iter().copied());
    let w21 = Matrix4::from_iterator(w.view((4, 0), (4, 4)).iter().copied());

    let p_raw = w21 * w11.try_inverse()?;
    let p_sym = (p_raw + p_raw.transpose()) * 0.5; // enforce exact symmetry

    if p_sym.iter().all(|v| v.is_finite()) { Some(p_sym) }
    else { log::warn!("CARE: non-finite P after extraction."); None }
}
