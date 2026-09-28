import test from 'node:test';
import assert from 'node:assert/strict';
import { CartPole, DEFAULTS, TARGET_LIMIT, computeLQR, derivatives, wrapAngle } from '../src/physics.js';

function runUntil(simulator, predicate, maxSeconds = 15) {
  for (let i = 0; i < maxSeconds * 120; i++) {
    simulator.step(1 / 120);
    if (predicate(simulator)) return simulator.time;
  }
  return null;
}

test('starts at rest hanging down and stays there until started', () => {
  const sim = new CartPole();
  sim.step(1);
  assert.deepEqual(sim.state, [0, 0, Math.PI, 0]);
  assert.equal(sim.mode, 'idle');
  assert.equal(sim.time, 0);
  assert.ok(Math.abs(wrapAngle(Math.PI) - Math.PI) < 1e-12);
});

test('upright has zero open-loop acceleration at zero force', () => {
  assert.deepEqual(derivatives([0, 0, 0, 0], 0, DEFAULTS), [0, 0, 0, 0]);
});

test('LQR gains remain finite and change when cost and plant change', () => {
  const base = computeLQR(DEFAULTS);
  for (const params of [
    { ...DEFAULTS, angleWeight: 300, effortWeight: .1 },
    { ...DEFAULTS, gravity: 1.62, cartMass: 3, poleMass: .7, length: 1.8 },
  ]) {
    const gain = computeLQR(params);
    assert.equal(gain.length, 4);
    assert.ok(gain.every(Number.isFinite));
    assert.notDeepEqual(gain, base);
  }
});

test('each state priority recalculates the LQR gain', () => {
  const base = computeLQR(DEFAULTS);
  for (const [key, value] of [
    ['positionWeight', 40],
    ['cartVelocityWeight', 6],
    ['angleWeight', 250],
    ['angularVelocityWeight', 22],
  ]) {
    const sim = new CartPole();
    sim.setParams({ [key]: value });
    assert.ok(sim.gain.every(Number.isFinite), key);
    assert.notDeepEqual(sim.gain, base, key);
  }
});

test('higher position priority tracks a new target faster', () => {
  const positionAfter = weight => {
    const sim = new CartPole({ ...DEFAULTS, positionWeight: weight });
    sim.start();
    runUntil(sim, s => s.time > 10);
    sim.setTarget(2);
    runUntil(sim, s => s.time > 13);
    assert.equal(sim.mode, 'balance');
    return sim.state[0];
  };
  assert.ok(positionAfter(40) > positionAfter(1) + .3);
});

test('swing-up hands off to LQR and balances the default system', () => {
  const sim = new CartPole();
  sim.start();
  const handoff = runUntil(sim, s => s.mode === 'balance');
  assert.ok(handoff !== null && handoff > .5 && handoff < 10, `handoff at ${handoff}`);
  runUntil(sim, s => s.time > handoff + 8);
  assert.equal(sim.mode, 'balance');
  assert.ok(Math.abs(wrapAngle(sim.state[2])) < .03);
  assert.ok(Math.abs(sim.state[0]) < .1);
});

test('heavy and low-gravity presets can also reach balance', () => {
  const configurations = [
    { ...DEFAULTS, cartMass: 1.6, poleMass: .48, length: 1.4, maxForce: 26, angleWeight: 190 },
    { ...DEFAULTS, gravity: 1.62, maxForce: 12, angleWeight: 85 },
  ];
  for (const params of configurations) {
    const sim = new CartPole(params);
    sim.start();
    assert.notEqual(runUntil(sim, s => s.mode === 'balance', 18), null);
  }
});

test('clicked rail target is clamped and the balanced cart tracks it', () => {
  const sim = new CartPole();
  sim.start();
  runUntil(sim, s => s.mode === 'balance');
  runUntil(sim, s => s.time > 10);
  sim.setTarget(100);
  assert.equal(sim.target, TARGET_LIMIT);
  const [x, velocity, theta, omega] = sim.state;
  const expectedForce = -sim.gain.reduce((sum, gain, i) =>
    sum + gain * [x - sim.target, velocity, wrapAngle(theta), omega][i], 0);
  assert.ok(Math.abs(expectedForce) < sim.params.maxForce);
  sim.step(1 / 120);
  assert.ok(Math.abs(sim.force - expectedForce) < 1e-9, 'controller must use the new target immediately');
  runUntil(sim, s => s.time > 20);
  assert.equal(sim.mode, 'balance');
  assert.ok(Math.abs(sim.state[0] - TARGET_LIMIT) < .05);
  assert.ok(Math.abs(wrapAngle(sim.state[2])) < .03);
});

test('a target set before startup is reached after swing-up', () => {
  const sim = new CartPole();
  sim.setTarget(-1.8);
  assert.equal(sim.state[0], 0, 'idle cart must remain at rest');
  sim.start();
  runUntil(sim, s => s.time > 20, 22);
  assert.equal(sim.mode, 'balance');
  assert.ok(Math.abs(sim.state[0] + 1.8) < .05);
});

test('reset restores the initial condition after a run', () => {
  const sim = new CartPole();
  sim.start();
  runUntil(sim, s => s.mode === 'balance');
  sim.push(1);
  sim.setTarget(2);
  sim.step(1 / 120);
  sim.reset();
  assert.equal(sim.force, 0);
  assert.equal(sim.time, 0);
  assert.equal(sim.mode, 'idle');
  assert.equal(sim.target, 0);
  assert.deepEqual(sim.state, [0, 0, Math.PI, 0]);
});
