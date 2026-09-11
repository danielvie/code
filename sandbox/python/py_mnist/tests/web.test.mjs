// Run with Node 18 or newer: node --test tests/web.test.mjs
// These checks need neither a browser nor downloaded model/runtime files.
import test from "node:test";
import assert from "node:assert/strict";
import { findInkBounds, centerByMass, softmax } from "../web/preprocessing.mjs";

test("a blank canvas has no ink bounds", () => {
  assert.equal(findInkBounds(new Uint8ClampedArray(4 * 10 * 10), 10, 10), null);
});

test("bounds include ink at the canvas edges and ignore near-black noise", () => {
  const rgba = new Uint8ClampedArray(4 * 10 * 10);
  rgba[(2 * 10 + 4) * 4] = 255;
  rgba[(9 * 10 + 9) * 4] = 128;
  rgba[0] = 5;
  assert.deepEqual(findInkBounds(rgba, 10, 10), { left: 4, top: 2, width: 6, height: 8 });
});

test("centering shifts ink to the middle and preserves intensity", () => {
  const pixels = new Float32Array(784);
  pixels[3 * 28 + 5] = 0.5;
  const centered = centerByMass(pixels);
  assert.equal(centered[14 * 28 + 14], 0.5);
  assert.equal(centered.reduce((sum, value) => sum + value, 0), 0.5);
  assert.equal(pixels[3 * 28 + 5], 0.5, "source pixels must not change");
});

test("centering a blank image returns finite zeros", () => {
  assert.ok(centerByMass(new Float32Array(784)).every(value => value === 0));
});

test("softmax is stable for large logits and preserves the winner", () => {
  const probabilities = softmax([1000, 1002, 1001]);
  assert.ok(probabilities.every(value => Number.isFinite(value) && value > 0));
  assert.ok(Math.abs(probabilities.reduce((a, b) => a + b) - 1) < 1e-12);
  assert.equal(probabilities.indexOf(Math.max(...probabilities)), 1);
  assert.deepEqual(softmax([2, 2]), [0.5, 0.5]);
});
