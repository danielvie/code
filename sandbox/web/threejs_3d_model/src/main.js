import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { CartPole, DEFAULTS, TARGET_LIMIT, wrapAngle } from './physics.js';
import './style.css';
import './method.css';
import './interaction.css';

const $ = selector => document.querySelector(selector);
const simulator = new CartPole();
const parameterDefs = [
  { key: 'cartMass', label: 'Cart mass', symbol: 'M', min: 0.5, max: 3, step: 0.05, unit: 'kg', digits: 2, group: 'physical' },
  { key: 'poleMass', label: 'Pole mass', symbol: 'm', min: 0.08, max: 0.7, step: 0.01, unit: 'kg', digits: 2, group: 'physical' },
  { key: 'length', label: 'Pole length', symbol: 'L', min: 0.7, max: 1.8, step: 0.05, unit: 'm', digits: 2, group: 'physical' },
  { key: 'gravity', label: 'Gravity', symbol: 'g', min: 1.6, max: 15, step: 0.01, unit: 'm/s²', digits: 2, group: 'physical' },
  { key: 'friction', label: 'Rail friction', symbol: 'b', min: 0, max: 0.8, step: 0.01, unit: 'N·s/m', digits: 2, group: 'physical' },
  { key: 'maxForce', label: 'Motor force limit', symbol: 'F', min: 6, max: 35, step: 1, unit: 'N', digits: 0, group: 'physical' },
  { key: 'positionWeight', label: 'Position priority', symbol: 'Qx', min: 1, max: 200, step: 1, unit: '', digits: 0, group: 'controller' },
  { key: 'cartVelocityWeight', label: 'Cart velocity priority', symbol: 'Qẋ', min: 0.1, max: 50, step: 0.1, unit: '', digits: 1, group: 'controller' },
  { key: 'angleWeight', label: 'Angle priority', symbol: 'Qθ', min: 20, max: 300, step: 5, unit: '', digits: 0, group: 'controller' },
  { key: 'angularVelocityWeight', label: 'Pole speed priority', symbol: 'Qθ̇', min: 1, max: 30, step: 1, unit: '', digits: 0, group: 'controller' },
  { key: 'effortWeight', label: 'Effort penalty', symbol: 'R', min: 0.1, max: 2, step: 0.05, unit: '', digits: 2, group: 'controller' },
];
const presets = {
  standard: { ...DEFAULTS },
  heavy: { ...DEFAULTS, cartMass: 1.6, poleMass: 0.48, length: 1.4, maxForce: 26, angleWeight: 190 },
  moon: { ...DEFAULTS, gravity: 1.62, maxForce: 12, angleWeight: 85 },
};

function renderControls() {
  for (const definition of parameterDefs) {
    const { key, label, symbol, min, max, step, unit, digits, group } = definition;
    const wrapper = document.createElement('div');
    wrapper.className = 'slider-control';
    wrapper.innerHTML = `<div class="slider-head"><label for="param-${key}"><span class="parameter-symbol">${symbol}</span>${label}</label><output id="value-${key}" for="param-${key}"></output></div><input id="param-${key}" type="range" min="${min}" max="${max}" step="${step}" aria-label="${label}"><div class="range-ends"><span>${min}</span><span>${max}</span></div>`;
    $(`#${group}-controls`).appendChild(wrapper);
    wrapper.querySelector('input').addEventListener('input', event => {
      simulator.setParams({ [key]: Number(event.target.value) });
      updateControlValues();
      updateGain();
      document.querySelectorAll('.preset').forEach(button => button.classList.remove('active'));
    });
  }
  updateControlValues();
  updateGain();
}
function updateControlValues() {
  for (const def of parameterDefs) {
    const value = simulator.params[def.key];
    const slider = $(`#param-${def.key}`);
    slider.value = value;
    slider.style.setProperty('--fill', `${(value - def.min) / (def.max - def.min) * 100}%`);
    $(`#value-${def.key}`).innerHTML = `${value.toFixed(def.digits)}${def.unit ? `<small>${def.unit}</small>` : ''}`;
  }
}
function updateGain() {
  $('#gain-values').textContent = `K = [ ${simulator.gain.map(v => (v < 0 ? '−' : '+') + Math.abs(v).toFixed(2)).join(', ')} ]`;
}
renderControls();
document.querySelectorAll('.preset').forEach(button => button.addEventListener('click', () => {
  simulator.setParams(presets[button.dataset.preset]);
  updateControlValues(); updateGain();
  document.querySelectorAll('.preset').forEach(other => other.classList.toggle('active', other === button));
}));

// The track and cart live in world units: one Three.js unit is one meter.
const mount = $('#scene');
const scene = new THREE.Scene();
scene.fog = new THREE.FogExp2(0x0a2638, 0.052);
const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100);
camera.position.set(4.2, 3.65, 8.2);
const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, powerPreference: 'high-performance' });
renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
renderer.setClearColor(0x000000, 0);
renderer.outputColorSpace = THREE.SRGBColorSpace;
renderer.toneMapping = THREE.ACESFilmicToneMapping;
renderer.toneMappingExposure = 1.45;
mount.appendChild(renderer.domElement);
const orbit = new OrbitControls(camera, renderer.domElement);
orbit.target.set(0, 1.48, 0);
orbit.enableDamping = true;
orbit.dampingFactor = 0.07;
orbit.minDistance = 5;
orbit.maxDistance = 20;
orbit.maxPolarAngle = Math.PI * .49;
orbit.minPolarAngle = .22;
orbit.enablePan = false;
orbit.mouseButtons = { LEFT: THREE.MOUSE.ROTATE, MIDDLE: THREE.MOUSE.ROTATE, RIGHT: null };
renderer.domElement.addEventListener('auxclick', event => { if (event.button === 1) event.preventDefault(); });
orbit.update();
scene.add(new THREE.HemisphereLight(0x9ae7ed, 0x102c39, 2.25));
const keyLight = new THREE.DirectionalLight(0xe0fbf4, 2.3);
keyLight.position.set(-3, 7, 5);
scene.add(keyLight);
const rimLight = new THREE.DirectionalLight(0x4ba7c8, 1.8);
rimLight.position.set(2, 4, -5);
scene.add(rimLight);
const materials = {
  base: new THREE.MeshStandardMaterial({ color: 0x143849, metalness: .5, roughness: .58 }),
  edge: new THREE.MeshStandardMaterial({ color: 0x5297a9, metalness: .7, roughness: .32 }),
  body: new THREE.MeshStandardMaterial({ color: 0x68c0c2, metalness: .58, roughness: .28 }),
  upper: new THREE.MeshStandardMaterial({ color: 0x9ce3db, metalness: .48, roughness: .26 }),
  dark: new THREE.MeshStandardMaterial({ color: 0x112a37, metalness: .48, roughness: .52 }),
  wheel: new THREE.MeshStandardMaterial({ color: 0x8dbbc0, metalness: .77, roughness: .25 }),
  gold: new THREE.MeshStandardMaterial({ color: 0xf1bb72, metalness: .57, roughness: .27, emissive: 0x8a5123, emissiveIntensity: .26 }),
  rod: new THREE.MeshStandardMaterial({ color: 0xd9e8dc, metalness: .75, roughness: .19 }),
  tick: new THREE.MeshBasicMaterial({ color: 0x44778a }),
};
function box(parent, width, height, depth, x, y, z, material) {
  const mesh = new THREE.Mesh(new THREE.BoxGeometry(width, height, depth), material);
  mesh.position.set(x, y, z); parent.add(mesh); return mesh;
}
function cylinder(parent, radius, height, x, y, z, material, segments = 24) {
  const mesh = new THREE.Mesh(new THREE.CylinderGeometry(radius, radius, height, segments), material);
  mesh.position.set(x, y, z); parent.add(mesh); return mesh;
}
function line(parent, from, to, color, opacity = 1) {
  const geometry = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(...from), new THREE.Vector3(...to)]);
  const object = new THREE.Line(geometry, new THREE.LineBasicMaterial({ color, transparent: opacity < 1, opacity }));
  parent.add(object); return object;
}
const groundGrid = new THREE.GridHelper(15, 30, 0x386f7d, 0x254b5c);
groundGrid.position.y = -.085;
const gridMaterials = Array.isArray(groundGrid.material) ? groundGrid.material : [groundGrid.material];
gridMaterials.forEach(material => { material.transparent = true; material.opacity = .24; material.depthWrite = false; });
scene.add(groundGrid);
const rig = new THREE.Group();
rig.position.y = 1.2;
scene.add(rig);
// The raised test stand leaves room for the pendulum when it hangs down.
for (const x of [-3.5, 3.5]) for (const z of [-.31, .31]) {
  box(scene, .12, 1.16, .12, x, .53, z, materials.edge);
  box(scene, .32, .065, .32, x, -.018, z, materials.base);
  box(scene, .28, .07, .28, x, 1.13, z, materials.dark);
}
box(rig, 8.65, .14, .92, 0, -.015, 0, materials.base);
for (const z of [-.39, .39]) {
  box(rig, 8.6, .045, .09, 0, .072, z, materials.edge);
  box(rig, 8.55, .018, .016, 0, .104, z, materials.upper);
}
for (let i = -17; i <= 17; i++) {
  const x = i * .24;
  box(rig, i % 5 === 0 ? .018 : .012, .007, i % 5 === 0 ? .23 : .11, x, .064, .0, materials.tick);
  if (i % 5 === 0) box(rig, .015, .007, .07, x, .064, .55, materials.edge);
}
for (const end of [-1, 1]) {
  box(rig, .11, .30, 1.02, end * 4.34, .08, 0, materials.edge);
  box(rig, .035, .17, .82, end * 4.27, .08, 0, materials.gold);
}
// A fixed origin mark gives the cart's displacement a visible reference.
line(rig, [0, -.072, -.78], [0, -.072, .78], 0xe8b676, .55);
// The preview covers the vertical slice from the rail down to the stand's feet.
const hoverSlice = box(rig, .43, 1.46, .95, 0, -.52, 0,
  new THREE.MeshBasicMaterial({ color: 0x6de2de, transparent: true, opacity: .10, depthWrite: false, side: THREE.DoubleSide }));
hoverSlice.visible = false;
const hoverBand = box(rig, .54, .012, 1.02, 0, .128, 0,
  new THREE.MeshBasicMaterial({ color: 0x83e9df, transparent: true, opacity: .42, depthWrite: false }));
hoverBand.visible = false;
const targetBand = box(rig, .24, .011, .98, 0, .122, 0,
  new THREE.MeshBasicMaterial({ color: 0xeebc76, transparent: true, opacity: .57, depthWrite: false }));
const targetMarker = new THREE.Mesh(new THREE.RingGeometry(.105, .142, 32),
  new THREE.MeshBasicMaterial({ color: 0xffcc83, side: THREE.DoubleSide, depthWrite: false }));
targetMarker.rotation.x = -Math.PI / 2;
targetMarker.position.set(0, .145, .59);
rig.add(targetMarker);
const targetPost = box(rig, .015, .19, .015, 0, .23, .59, materials.gold);
const cart = new THREE.Group(); rig.add(cart);
box(cart, 1.05, .31, .72, 0, .36, 0, materials.body);
box(cart, .91, .042, .76, 0, .535, 0, materials.upper);
box(cart, .85, .075, .56, 0, .19, 0, materials.dark);
for (const x of [-.36, .36]) for (const z of [-.39, .39]) {
  const wheel = cylinder(cart, .115, .085, x, .16, z, materials.wheel);
  wheel.rotation.x = Math.PI / 2;
  const hub = cylinder(cart, .045, .09, x, .16, z + Math.sign(z) * .025, materials.dark);
  hub.rotation.x = Math.PI / 2;
}
for (const x of [-.43, .43]) box(cart, .032, .07, .65, x, .57, 0, materials.edge);
const pivot = new THREE.Group(); pivot.position.y = .69; cart.add(pivot);
const stem = cylinder(pivot, .09, .25, 0, 0, 0, materials.dark); stem.rotation.x = Math.PI / 2;
const pivotRing = new THREE.Mesh(new THREE.TorusGeometry(.14, .017, 8, 40), materials.gold);
pivotRing.position.z = .17; pivot.add(pivotRing);
const pivotHub = cylinder(pivot, .07, .31, 0, 0, 0, materials.gold); pivotHub.rotation.x = Math.PI / 2;
const pendulum = new THREE.Group(); pivot.add(pendulum);
const rod = cylinder(pendulum, .026, simulator.params.length, 0, simulator.params.length / 2, 0, materials.rod);
const bob = new THREE.Mesh(new THREE.SphereGeometry(.17, 32, 20), materials.gold);
bob.position.y = simulator.params.length; pendulum.add(bob);
const bobHighlight = new THREE.Mesh(new THREE.SphereGeometry(.068, 16, 12), materials.upper);
bobHighlight.position.set(-.047, simulator.params.length + .07, .105); pendulum.add(bobHighlight);
const guide = new THREE.Group(); pivot.add(guide);
for (let i = 0; i < 12; i++) {
  const mark = cylinder(guide, .006, .052, 0, .10 + i * .105, -.05, new THREE.MeshBasicMaterial({ color: 0x82bac1, transparent: true, opacity: .45 }), 8);
  mark.userData.relativeHeight = .10 + i * .105;
}
const topGuide = new THREE.Mesh(new THREE.TorusGeometry(.09, .006, 6, 32), new THREE.MeshBasicMaterial({ color: 0x85cbd0, transparent: true, opacity: .65 }));
topGuide.position.set(0, simulator.params.length, -.05); guide.add(topGuide);
let lastRodLength = simulator.params.length;
// Only rebuild the rod when its length changes.
function syncModel() {
  const length = simulator.params.length;
  if (lastRodLength !== length) {
    rod.geometry.dispose();
    rod.geometry = new THREE.CylinderGeometry(.026, .026, length, 20);
    rod.position.y = length / 2;
    bob.position.y = length;
    bobHighlight.position.y = length + .07;
    topGuide.position.y = length;
    guide.children.forEach(mark => { if (mark.userData.relativeHeight != null) mark.visible = mark.userData.relativeHeight < length - .06; });
    lastRodLength = length;
  }
  cart.position.x = simulator.state[0];
  pendulum.rotation.z = -simulator.state[2];
}
function resizeScene() {
  const { width, height } = mount.getBoundingClientRect();
  if (!width || !height) return;
  renderer.setSize(width, height, false);
  camera.aspect = width / height;
  camera.fov = width < 550 ? 47 : 38;
  camera.updateProjectionMatrix();
}
new ResizeObserver(resizeScene).observe(mount);
resizeScene();

const raycaster = new THREE.Raycaster();
// A generous volume around the stand covers the track and the space down to
// its feet. A ray outside that volume leaves the left button free to orbit.
const targetVolume = new THREE.Box3(
  new THREE.Vector3(-4.2, -.2, -.85),
  new THREE.Vector3(4.2, 1.95, .85),
);
const intersection = new THREE.Vector3();
const tooltip = $('#target-tooltip');
let selectingPointerId = null;
function railPoint(event) {
  const bounds = renderer.domElement.getBoundingClientRect();
  raycaster.setFromCamera(new THREE.Vector2(
    (event.clientX - bounds.left) / bounds.width * 2 - 1,
    -(event.clientY - bounds.top) / bounds.height * 2 + 1,
  ), camera);
  if (!raycaster.ray.intersectBox(targetVolume, intersection)) return null;
  return Math.max(-TARGET_LIMIT, Math.min(TARGET_LIMIT, intersection.x));
}
function clearHover(dragging = false) {
  hoverSlice.visible = false;
  hoverBand.visible = false;
  tooltip.classList.remove('visible');
  renderer.domElement.style.cursor = dragging ? 'grabbing' : 'grab';
}
function showPreview(position, event) {
  hoverSlice.position.x = hoverBand.position.x = position;
  hoverSlice.visible = hoverBand.visible = true;
  $('#preview-target').textContent = `${position >= 0 ? '+' : '−'}${Math.abs(position).toFixed(2)} m`;
  const rect = mount.getBoundingClientRect();
  tooltip.style.left = `${Math.max(90, Math.min(rect.width - 90, event.clientX - rect.left))}px`;
  tooltip.style.top = `${Math.max(65, event.clientY - rect.top - 43)}px`;
  tooltip.classList.add('visible');
  renderer.domElement.style.cursor = 'crosshair';
}
function chooseTarget(position) {
  simulator.setTarget(position);
  targetBand.position.x = targetMarker.position.x = targetPost.position.x = simulator.target;
  updateUI();
}
renderer.domElement.style.cursor = 'grab';
renderer.domElement.addEventListener('pointermove', event => {
  if (selectingPointerId !== null && event.pointerId === selectingPointerId) {
    const position = railPoint(event);
    if (position === null) { clearHover(); return; }
    showPreview(position, event);
    chooseTarget(position);
    return;
  }
  if (event.buttons !== 0) { clearHover(true); return; }
  const position = railPoint(event);
  if (position === null) { clearHover(); return; }
  showPreview(position, event);
});
renderer.domElement.addEventListener('pointerleave', () => { if (selectingPointerId === null) clearHover(); });
// Capture first so OrbitControls never starts rotating for a target selection.
renderer.domElement.addEventListener('pointerdown', event => {
  if (event.button !== 0) { clearHover(true); return; }
  const position = railPoint(event);
  if (position === null) { clearHover(true); return; }
  event.preventDefault();
  event.stopImmediatePropagation();
  selectingPointerId = event.pointerId;
  try { renderer.domElement.setPointerCapture(event.pointerId); } catch { /* Synthetic pointer events cannot be captured. */ }
  showPreview(position, event);
  chooseTarget(position);
}, { capture: true });
function finishSelection(event) {
  if (event.pointerId !== selectingPointerId) { clearHover(); return; }
  event.stopImmediatePropagation();
  if (renderer.domElement.hasPointerCapture(event.pointerId)) renderer.domElement.releasePointerCapture(event.pointerId);
  selectingPointerId = null;
  const position = railPoint(event);
  if (position === null) clearHover();
  else showPreview(position, event);
}
renderer.domElement.addEventListener('pointerup', finishSelection, { capture: true });
renderer.domElement.addEventListener('pointercancel', finishSelection, { capture: true });

const chart = $('#response-chart');
const ctx = chart.getContext('2d');
let samples = [{ time: 0, angle: 180 }];
function drawChart() {
  const rect = chart.getBoundingClientRect();
  if (!rect.width || !rect.height) return;
  const dpr = Math.min(window.devicePixelRatio || 1, 2);
  if (chart.width !== Math.round(rect.width * dpr) || chart.height !== Math.round(rect.height * dpr)) {
    chart.width = Math.round(rect.width * dpr); chart.height = Math.round(rect.height * dpr);
  }
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  const w = rect.width, h = rect.height;
  ctx.clearRect(0, 0, w, h);
  ctx.strokeStyle = '#264c5c'; ctx.lineWidth = 1;
  for (let i = 0; i <= 4; i++) {
    const y = .5 + i * (h - 1) / 4;
    ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke();
  }
  for (let i = 0; i <= 6; i++) {
    const x = .5 + i * (w - 1) / 6;
    ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke();
  }
  ctx.strokeStyle = '#5e9ca1'; ctx.setLineDash([4, 5]);
  ctx.beginPath(); ctx.moveTo(0, h - 1); ctx.lineTo(w, h - 1); ctx.stroke(); ctx.setLineDash([]);
  const endTime = Math.max(12, simulator.time);
  const startTime = endTime - 12;
  const visible = samples.filter(sample => sample.time >= startTime - .1);
  if (!visible.length) return;
  const px = sample => (sample.time - startTime) / 12 * w;
  const py = sample => (1 - sample.angle / 180) * (h - 2) + 1;
  const gradient = ctx.createLinearGradient(0, 0, 0, h);
  gradient.addColorStop(0, '#e7b6783c'); gradient.addColorStop(1, '#e7b67800');
  ctx.beginPath();
  ctx.moveTo(px(visible[0]), h);
  for (const sample of visible) ctx.lineTo(px(sample), py(sample));
  ctx.lineTo(px(visible[visible.length - 1]), h);
  ctx.closePath(); ctx.fillStyle = gradient; ctx.fill();
  ctx.beginPath();
  visible.forEach((sample, i) => i ? ctx.lineTo(px(sample), py(sample)) : ctx.moveTo(px(sample), py(sample)));
  ctx.strokeStyle = '#f0bf7e'; ctx.lineWidth = 2; ctx.lineJoin = 'round'; ctx.stroke();
  const last = visible[visible.length - 1];
  ctx.beginPath(); ctx.arc(px(last), py(last), 3.2, 0, Math.PI * 2); ctx.fillStyle = '#f3d29e'; ctx.fill();
}
const signed = (value, digits) => `${value >= 0 ? '+' : '−'}${Math.abs(value).toFixed(digits)}`;
let paused = false;
let lastMode = '';
function updateUI() {
  const [x, , theta] = simulator.state;
  const angle = wrapAngle(theta) * 180 / Math.PI;
  $('#coordinate-x').textContent = signed(x, 2);
  $('#coordinate-theta').textContent = Math.abs(angle).toFixed(1);
  $('#readout-angle').innerHTML = `${signed(angle, 1)}<span>°</span>`;
  $('#readout-position').innerHTML = `${signed(x, 2)}<span> m</span>`;
  $('#readout-target').innerHTML = `${signed(simulator.target, 2)}<span> m</span>`;
  $('#readout-force').innerHTML = `${signed(simulator.force, 1)}<span> N</span>`;
  $('#chart-current').textContent = `${Math.abs(angle).toFixed(1)}°`; 
  const minutes = String(Math.floor(simulator.time / 60)).padStart(2, '0');
  const seconds = (simulator.time % 60).toFixed(1).padStart(4, '0');
  $('#time-value').textContent = `${minutes}:${seconds}`;
  const mode = paused ? 'paused' : simulator.mode;
  if (mode !== lastMode) {
    lastMode = mode;
    const labels = { idle: 'AWAITING START', 'swing-up': 'SWING-UP ACTIVE', balance: 'LQR ENGAGED', paused: 'SIMULATION PAUSED' };
    $('#mode-label').textContent = labels[mode];
    $('#status-dot').className = `mode-dot ${mode}`;
    $('#start-text').textContent = simulator.mode === 'idle' ? 'BEGIN SWING-UP' : simulator.mode === 'balance' ? 'LQR ENGAGED' : 'SWING-UP ACTIVE';
    $('#start-button').disabled = simulator.mode !== 'idle';
    $('#pause-button').disabled = simulator.mode === 'idle';
    $('#pause-icon').textContent = paused ? '▶' : 'Ⅱ';
    $('#pause-text').textContent = paused ? 'RESUME' : 'PAUSE';
    $('#readout-note').textContent = simulator.mode === 'idle' ? 'SYSTEM AT REST' : paused ? 'SIMULATION HELD' : simulator.mode === 'balance' ? 'FEEDBACK LOOP CLOSED' : 'BUILDING PENDULUM ENERGY';
  }
}
$('#start-button').addEventListener('click', () => { simulator.start(); paused = false; updateUI(); });
$('#pause-button').addEventListener('click', () => { paused = !paused; updateUI(); });
$('#reset-button').addEventListener('click', () => {
  simulator.reset(); paused = false; samples = [{ time: 0, angle: 180 }]; accumulator = 0; sampleClock = 0;
  targetBand.position.x = targetMarker.position.x = targetPost.position.x = 0;
  clearHover(); updateUI(); drawChart();
});
$('#push-left').addEventListener('click', () => simulator.push(-1));
$('#push-right').addEventListener('click', () => simulator.push(1));
updateUI();
let previous = performance.now();
let accumulator = 0;
let sampleClock = 0;
let uiClock = 0;
function frame(now) {
  requestAnimationFrame(frame);
  const elapsed = Math.min((now - previous) / 1000, .05);
  previous = now;
  if (!paused && simulator.mode !== 'idle') {
    accumulator += elapsed;
    while (accumulator >= 1 / 120) {
      simulator.step(1 / 120);
      accumulator -= 1 / 120;
      sampleClock += 1 / 120;
      if (sampleClock >= 1 / 30) {
        sampleClock = 0;
        samples.push({ time: simulator.time, angle: Math.abs(wrapAngle(simulator.state[2])) * 180 / Math.PI });
        if (samples.length > 500) samples.shift();
      }
    }
  }
  syncModel();
  orbit.update();
  renderer.render(scene, camera);
  uiClock += elapsed;
  if (uiClock > 1 / 24) { updateUI(); drawChart(); uiClock = 0; }
}
requestAnimationFrame(frame);
