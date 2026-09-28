// A cart-pole with a point mass at the end of a massless rod.
// theta = 0 is upright; theta = PI is hanging down.
export const DEFAULTS = Object.freeze({
  cartMass: 1.0,
  poleMass: 0.22,
  length: 1.2,
  gravity: 9.81,
  friction: 0.12,
  maxForce: 20,
  positionWeight: 6,
  cartVelocityWeight: 1.2,
  angleWeight: 140,
  angularVelocityWeight: 9,
  effortWeight: 0.5,
});

export const RAIL_LIMIT = 3.55;
export const TARGET_LIMIT = 2.6;
const DT_LQR = 1 / 60;
const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
export const wrapAngle = angle => Math.atan2(Math.sin(angle), Math.cos(angle));

export function derivatives(s, force, p) {
  const [x, velocity, angle, omega] = s;
  const { cartMass: M, poleMass: m, length: l, gravity: g, friction: b } = p;
  const sn = Math.sin(angle), cs = Math.cos(angle);
  const pivotDamping = 0.018;
  const denominator = M + m - m * cs * cs;
  const acceleration = (force - b * velocity + m * l * sn * omega * omega
    - m * g * sn * cs + pivotDamping * omega * cs / l) / denominator;
  const angularAcceleration = (g * sn - cs * acceleration - pivotDamping * omega / (m * l)) / l;
  return [velocity, acceleration, omega, angularAcceleration];
}

function rk4(state, force, dt, p) {
  const add = (s, k, h) => s.map((v, i) => v + h * k[i]);
  const a = derivatives(state, force, p);
  const b = derivatives(add(state, a, dt / 2), force, p);
  const c = derivatives(add(state, b, dt / 2), force, p);
  const d = derivatives(add(state, c, dt), force, p);
  return state.map((v, i) => v + dt * (a[i] + 2 * b[i] + 2 * c[i] + d[i]) / 6);
}

// Linearize the nonlinear dynamics at the upright equilibrium. A discrete
// Riccati iteration then computes the infinite-horizon feedback gain.
export function computeLQR(p) {
  const zero = [0, 0, 0, 0];
  const h = 1e-5;
  const A = Array.from({ length: 4 }, () => Array(4).fill(0));
  for (let j = 0; j < 4; j++) {
    const plus = [...zero], minus = [...zero];
    plus[j] = h; minus[j] = -h;
    const fp = derivatives(plus, 0, p), fm = derivatives(minus, 0, p);
    for (let i = 0; i < 4; i++) A[i][j] = (fp[i] - fm[i]) / (2 * h);
  }
  const fp = derivatives(zero, h, p), fm = derivatives(zero, -h, p);
  const B = fp.map((v, i) => (v - fm[i]) / (2 * h) * DT_LQR);
  const Ad = A.map((row, i) => row.map((v, j) => v * DT_LQR + (i === j ? 1 : 0)));
  const Q = [p.positionWeight, p.cartVelocityWeight, p.angleWeight, p.angularVelocityWeight]
    .map(v => v * DT_LQR);
  const R = p.effortWeight * DT_LQR;
  let P = Array.from({ length: 4 }, (_, i) => Array.from({ length: 4 }, (_, j) => i === j ? Q[i] * 30 : 0));
  let K = [0, 0, 0, 0];
  for (let iteration = 0; iteration < 12000; iteration++) {
    const PB = P.map(row => row.reduce((sum, v, j) => sum + v * B[j], 0));
    const denominator = R + B.reduce((sum, v, i) => sum + v * PB[i], 0);
    const BPA = Array.from({ length: 4 }, (_, j) => B.reduce((sum, v, i) => sum + v * P[i].reduce((a, pij, k) => a + pij * Ad[k][j], 0), 0));
    K = BPA.map(v => v / denominator);
    const next = Array.from({ length: 4 }, (_, i) => Array.from({ length: 4 }, (_, j) => {
      let value = i === j ? Q[i] : 0;
      for (let k = 0; k < 4; k++) for (let n = 0; n < 4; n++) value += Ad[k][i] * P[k][n] * Ad[n][j];
      return value - BPA[i] * BPA[j] / denominator;
    }));
    let difference = 0;
    // Symmetrize to prevent floating-point asymmetry from growing along the unstable mode.
    for (let i = 0; i < 4; i++) for (let j = i; j < 4; j++) {
      const value = (next[i][j] + next[j][i]) / 2;
      difference = Math.max(difference, Math.abs(value - P[i][j]));
      next[i][j] = next[j][i] = value;
    }
    P = next;
    if (difference < 1e-9) break;
  }
  return K;
}

export class CartPole {
  constructor(params = DEFAULTS) {
    this.params = { ...params };
    this.gain = computeLQR(this.params);
    this.reset();
  }

  reset() {
    this.state = [0, 0, Math.PI, 0];
    this.mode = 'idle';
    this.force = 0;
    this.time = 0;
    this.kickDirection = 1;
    this.disturbance = 0;
    this.target = 0;
  }

  setTarget(position) {
    this.target = clamp(position, -TARGET_LIMIT, TARGET_LIMIT);
  }

  setParams(patch) {
    this.params = { ...this.params, ...patch };
    this.gain = computeLQR(this.params);
  }

  start() {
    if (this.mode === 'idle') this.mode = 'swing-up';
  }

  push(direction) {
    if (this.mode !== 'idle') this.disturbance += direction * 8;
  }

  step(dt) {
    if (this.mode === 'idle') return;
    const p = this.params;
    const [x, velocity, rawAngle, omega] = this.state;
    const angle = wrapAngle(rawAngle);
    if (this.mode === 'swing-up' && Math.abs(angle) < 0.27 && Math.abs(omega) < 2.5 && Math.abs(x) < 2.85) this.mode = 'balance';
    if (this.mode === 'balance' && (Math.abs(angle) > 0.72 || Math.abs(x) > 3.35)) this.mode = 'swing-up';

    let requested;
    if (this.mode === 'balance') {
      requested = -this.gain.reduce((sum, gain, i) => sum + gain * [x - this.target, velocity, angle, omega][i], 0);
    } else {
      const energy = 0.5 * p.poleMass * p.length ** 2 * omega ** 2 + p.poleMass * p.gravity * p.length * Math.cos(angle);
      const target = p.poleMass * p.gravity * p.length;
      // Aim a little above the upright energy to overcome pivot damping.
      const error = (energy - 1.16 * target) / target;
      requested = 18 * error * omega * Math.cos(angle) - 1.8 * (x - this.target) - 2.5 * velocity;
      // Exactly at rest at the bottom, energy shaping has no direction to act in.
      if (Math.abs(omega) < 0.23 && Math.abs(Math.abs(angle) - Math.PI) < 0.35) requested += 3.5 * this.kickDirection;
      if (Math.abs(x) > 2.75) requested += -Math.sign(x) * (Math.abs(x) - 2.75) * 32;
    }
    this.force = clamp(requested, -p.maxForce, p.maxForce);
    const actualForce = this.force + this.disturbance;
    this.disturbance *= Math.exp(-dt * 7);
    this.state = rk4(this.state, actualForce, dt, p);
    if (Math.abs(this.state[0]) > RAIL_LIMIT) {
      this.state[0] = Math.sign(this.state[0]) * RAIL_LIMIT;
      this.state[1] *= -0.15;
      this.kickDirection *= -1;
    }
    this.time += dt;
  }
}
