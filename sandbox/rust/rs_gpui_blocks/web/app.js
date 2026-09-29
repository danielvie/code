const svg = document.querySelector('#diagram');
const wires = document.querySelector('#wires');
const blocks = document.querySelector('#blocks');
const preview = document.querySelector('#preview');
const status = document.querySelector('#status');
const zoomLevel = document.querySelector('#zoom-level');
const counts = document.querySelector('#counts');
const selection = document.querySelector('#selection');
const editor = document.querySelector('#editor');
const editorInput = document.querySelector('#editor-input');
const colors = ['#315bd6', '#128275', '#c97b19', '#a34bae', '#d14b53', '#62768c'];
const key = 'gpui-diagram-browser-v1';
const ns = 'http://www.w3.org/2000/svg';
let Diagram;
let app;
let state;
let gesture = null;
let hover = null;
let pending = null;
let mousePoint = { x: 0, y: 0 };
let lastClick = null;
let zoom = 1;
let viewX = 0;
let viewY = 0;

function setView() {
  svg.setAttribute('viewBox', `${viewX} ${viewY} ${1280 / zoom} ${840 / zoom}`);
  zoomLevel.textContent = `${Math.round(zoom * 100)}%`;
}

function zoomAt(factor, anchor) {
  const next = Math.max(0.4, Math.min(1.8, zoom * factor));
  if (next === zoom) return;
  const before = world(anchor);
  zoom = next;
  setView();
  const after = world(anchor);
  viewX += before.x - after.x;
  viewY += before.y - after.y;
  setView();
  if (app) { draw(); save(); }
}

function zoomCenter(factor) {
  if (!editor.hidden) return;
  const bounds = svg.getBoundingClientRect();
  zoomAt(factor, { clientX: bounds.left + bounds.width / 2, clientY: bounds.top + bounds.height / 2 });
}

function fitView() {
  if (!app || !editor.hidden) return;
  const blocks = state.diagram.blocks;
  if (!blocks.length) {
    zoom = 1;
    viewX = -36;
    viewY = -24;
  } else {
    const minX = Math.min(...blocks.map(b => b.pos.x)) - 60;
    const minY = Math.min(...blocks.map(b => b.pos.y)) - 60;
    const maxX = Math.max(...blocks.map(b => b.pos.x + 192)) + 60;
    const maxY = Math.max(...blocks.map(b => b.pos.y + 72 + Math.max(0, Math.max(b.inputs.length, b.outputs.length) - 1) * 24)) + 60;
    zoom = Math.max(0.4, Math.min(1.4, 1280 / (maxX - minX), 840 / (maxY - minY)));
    viewX = (minX + maxX - 1280 / zoom) / 2;
    viewY = (minY + maxY - 840 / zoom) / 2;
  }
  setView();
  save();
}

function element(tag, attributes = {}, text) {
  const node = document.createElementNS(ns, tag);
  for (const [name, value] of Object.entries(attributes)) node.setAttribute(name, value);
  if (text !== undefined) node.textContent = text;
  return node;
}

function world(event) {
  return new DOMPoint(event.clientX, event.clientY).matrixTransform(svg.getScreenCTM().inverse());
}

function portPosition(block, output, index) {
  return { x: block.pos.x + (output ? 192 : 0), y: block.pos.y + 60 + index * 24 };
}

function hitPort(point) {
  for (const block of [...state.diagram.blocks].reverse()) {
    for (const output of [false, true]) {
      const count = output ? block.outputs.length : block.inputs.length;
      for (let index = 0; index < count; index++) {
        const pos = portPosition(block, output, index);
        if (Math.abs(point.x - pos.x) <= 12 / zoom && Math.abs(point.y - pos.y) <= 12 / zoom) {
          return { block: block.id, index, output };
        }
      }
    }
  }
  return null;
}

function samePort(a, b) {
  return a?.block === b?.block && a?.index === b?.index && a?.output === b?.output;
}

function secondClick(kind, id, point) {
  const now = performance.now();
  const repeated = lastClick?.kind === kind && lastClick.id === id && now - lastClick.time < 450 &&
    Math.hypot(point.x - lastClick.point.x, point.y - lastClick.point.y) < 10 / zoom;
  lastClick = { kind, id, point, time: now };
  return repeated;
}

function validTarget(port) {
  return pending && port && !port.output &&
    app.can_connect(pending.block, pending.index, port.block, port.index);
}

function path(points) {
  return points.map((p, i) => `${i ? 'L' : 'M'}${p.x} ${p.y}`).join(' ');
}

function draw() {
  wires.replaceChildren();
  blocks.replaceChildren();
  preview.replaceChildren();
  for (const route of state.routes) {
    if (!route.points.length) continue;
    const d = path(route.points);
    const selected = state.selected_wire === route.wire;
    wires.append(element('path', { d, fill: 'none', stroke: selected ? '#fabf36' : 'white', 'stroke-width': selected ? 10 : 8, 'data-wire': route.wire }));
    wires.append(element('path', { d, fill: 'none', stroke: `#${route.color.toString(16).padStart(6, '0')}`, 'stroke-width': selected ? 5 : 3, 'data-wire': route.wire }));
  }
  for (const dot of state.junctions) {
    wires.append(element('circle', { cx: dot.position.x, cy: dot.position.y, r: 4, fill: `#${dot.color.toString(16).padStart(6, '0')}` }));
  }
  for (const route of state.routes.filter(r => !r.points.length)) {
    const wire = state.diagram.wires.find(w => w.id === route.wire);
    for (const port of [wire.from, wire.to]) {
      const block = state.diagram.blocks.find(b => b.id === port.block);
      const { x, y } = portPosition(block, port.output, port.index);
      for (const [dx, dy] of [[-7, -7], [-7, 7]]) {
        wires.append(element('line', { x1: x + dx, y1: y + dy, x2: x - dx, y2: y - dy, stroke: '#d32f2f', 'stroke-width': 3, 'pointer-events': 'none' }));
      }
    }
  }
  for (const block of state.diagram.blocks) {
    const group = element('g', { 'data-block': block.id, class: 'block' });
    const height = 72 + Math.max(0, Math.max(block.inputs.length, block.outputs.length) - 1) * 24;
    const selected = state.selected.includes(block.id) || state.selected_port?.block === block.id;
    group.append(element('rect', { x: block.pos.x, y: block.pos.y, width: 192, height, fill: selected ? '#fff9d9' : '#fffce9', stroke: selected ? '#2563eb' : '#aaa586', 'stroke-width': selected ? 3 : 1 }));
    group.append(element('text', { x: block.pos.x + 96, y: block.pos.y + 18, 'text-anchor': 'middle', fill: '#807956', 'font-size': 10, 'pointer-events': 'none' }, `«block»  ${block.kind}`));
    group.append(element('text', { x: block.pos.x + 96, y: block.pos.y + 39, 'text-anchor': 'middle', 'font-weight': 'bold', 'font-size': 13, 'pointer-events': 'none' }, block.name));
    group.append(element('line', { x1: block.pos.x, x2: block.pos.x + 192, y1: block.pos.y + 43, y2: block.pos.y + 43, stroke: '#c8c1a3', 'pointer-events': 'none' }));
    for (const output of [false, true]) {
      const names = output ? block.outputs : block.inputs;
      names.forEach((name, index) => {
        const port = { block: block.id, index, output };
        const pos = portPosition(block, output, index);
        const connected = state.diagram.wires.find(w => !output && samePort(w.to, port));
        const active = samePort(hover, port);
        const selectedPort = samePort(state.selected_port, port);
        const valid = active && validTarget(port);
        const fill = valid ? '#9ce5d3' : active || (selectedPort && output) ? '#fabf36' : output ? colors[block.id % colors.length] : connected ? `#${connected.color.toString(16).padStart(6, '0')}` : 'white';
        group.append(element('text', { x: pos.x + (output ? -11 : 11), y: pos.y + 4, 'text-anchor': output ? 'end' : 'start', 'font-size': 11, 'pointer-events': 'none' }, name));
        group.append(element('rect', { x: pos.x - (active ? 8 : 5), y: pos.y - (active ? 8 : 5), width: active ? 16 : 10, height: active ? 16 : 10, fill, stroke: valid ? '#128275' : active || selectedPort ? '#2563eb' : colors[block.id % colors.length], 'stroke-width': active || selectedPort ? 2 : 1, 'pointer-events': 'none' }));
        group.append(element('circle', { cx: pos.x, cy: pos.y, r: 12 / zoom, fill: 'transparent', 'data-port': `${block.id}:${index}:${output}` }));
      });
    }
    blocks.append(group);
  }
  if (gesture?.kind === 'marquee') {
    const x = Math.min(gesture.start.x, gesture.point.x);
    const y = Math.min(gesture.start.y, gesture.point.y);
    preview.append(element('rect', { x, y, width: Math.abs(gesture.point.x - gesture.start.x), height: Math.abs(gesture.point.y - gesture.start.y), fill: '#315bd62b', stroke: '#315bd6', 'pointer-events': 'none' }));
  }
  if (pending) {
    const block = state.diagram.blocks.find(b => b.id === pending.block);
    const from = portPosition(block, true, pending.index);
    const to = gesture?.kind === 'wire' ? gesture.point : mousePoint;
    preview.append(element('path', { d: path([from, { x: from.x + 24, y: from.y }, { x: from.x + 24, y: to.y }, to]), fill: 'none', stroke: '#8793a6', 'stroke-width': 2, 'pointer-events': 'none' }));
  }
}

function refresh() {
  state = JSON.parse(app.view());
  const failures = state.routes.filter(route => !route.points.length).length;
  counts.textContent = `${state.diagram.blocks.length} blocks · ${state.diagram.wires.length} wires · ${failures} unroutable`;
  counts.classList.toggle('error', failures > 0);
  const name = port => {
    const block = state.diagram.blocks.find(b => b.id === port.block);
    return (port.output ? block.outputs : block.inputs)[port.index];
  };
  if (state.selected.length > 1) selection.textContent = `${state.selected.length} blocks selected`;
  else if (state.selected.length) selection.textContent = `Block: ${state.diagram.blocks.find(b => b.id === state.selected[0]).name}`;
  else if (state.selected_port) selection.textContent = `${state.selected_port.output ? 'Output' : 'Input'}: ${name(state.selected_port)}`;
  else if (state.selected_wire !== null) {
    const wire = state.diagram.wires.find(w => w.id === state.selected_wire);
    selection.textContent = `${name(wire.from)} → ${name(wire.to)}`;
  } else selection.textContent = 'Nothing selected';
  draw();
}

function save() {
  try {
    localStorage.setItem(key, JSON.stringify({ version: 1, diagram: JSON.parse(app.save()), view: { x: viewX, y: viewY, zoom } }));
    status.classList.remove('error');
    return true;
  } catch {
    status.classList.add('error');
    status.textContent = 'Changes are not saved: browser storage is unavailable.';
    return false;
  }
}

function addBlock(x, y) {
  if (!app || !editor.hidden || !app.add_block(x, y)) return;
  pending = null;
  const saved = save();
  refresh();
  if (saved) status.textContent = 'Block added. Double-click it or press F2 to rename.';
}

function addPort(output) {
  if (!app || !editor.hidden) return;
  if (!app.add_port(output)) {
    status.textContent = 'Select one block (up to six ports per side).';
    return;
  }
  pending = null;
  save();
  refresh();
  startEdit();
}

function deleteSelection() {
  if (!app || !editor.hidden) return;
  if (!app.delete_selected()) {
    status.textContent = 'Select a block, port, or wire to delete.';
    return;
  }
  pending = null;
  const saved = save();
  refresh();
  if (saved) status.textContent = 'Deleted. This prototype has no undo.';
}

function startEdit() {
  if (!app || !editor.hidden) return;
  const port = state.selected_port;
  const ids = state.selected;
  if (!port && ids.length !== 1) {
    status.textContent = 'Select exactly one block or port to rename.';
    return;
  }
  const block = state.diagram.blocks.find(b => b.id === (port ? port.block : ids[0]));
  const value = port ? (port.output ? block.outputs : block.inputs)[port.index] : block.name;
  const at = port ? portPosition(block, port.output, port.index) : block.pos;
  const screen = new DOMPoint(at.x, at.y).matrixTransform(svg.getScreenCTM());
  const bounds = document.querySelector('main').getBoundingClientRect();
  editor.style.left = `${Math.max(8, Math.min(screen.x - bounds.left, bounds.width - 308))}px`;
  editor.style.top = `${Math.max(8, Math.min(screen.y - bounds.top - 110, bounds.height - 110))}px`;
  document.querySelector('#editor-title').textContent = port ? `${port.output ? 'OUTPUT' : 'INPUT'} · ${block.name} / ${value}` : `BLOCK · ${block.name}`;
  pending = null;
  lastClick = null;
  editorInput.value = value;
  editor.hidden = false;
  editorInput.focus();
  editorInput.select();
}

editorInput.addEventListener('keydown', event => {
  if (event.key !== 'Enter' && event.key !== 'Escape') return;
  event.preventDefault();
  event.stopPropagation();
  if (event.key === 'Escape') {
    editor.hidden = true;
    editorInput.blur();
    return;
  }
  if (!app.rename_selected(editorInput.value)) {
    status.textContent = 'Use 1–64 non-control characters for a label.';
    return;
  }
  editor.hidden = true;
  editorInput.blur();
  const saved = save();
  refresh();
  if (saved) status.textContent = 'Label updated.';
});

svg.addEventListener('dblclick', event => {
  if (!app || !editor.hidden) return;
  const point = world(event);
  const port = hitPort(point);
  if (port) {
    app.select_port(port.block, port.index, port.output);
    refresh();
    startEdit();
  } else if (event.target.closest('[data-block]')) {
    const id = Number(event.target.closest('[data-block]').dataset.block);
    app.select_block(id, false, point.x, point.y);
    app.end_drag();
    refresh();
    startEdit();
  } else if (!event.target.closest('[data-wire]')) {
    addBlock(point.x - 96, point.y - 48);
  }
});

svg.addEventListener('pointerdown', event => {
  if (!app || !editor.hidden) return;
  if (event.button === 1) {
    lastClick = null;
    gesture = { kind: 'pan', anchor: world(event) };
    svg.classList.add('panning');
    svg.setPointerCapture(event.pointerId);
    event.preventDefault();
    return;
  }
  if (event.button !== 0) return;
  const point = world(event);
  mousePoint = point;
  const hit = hitPort(point);
  if (hit) {
    app.select_port(hit.block, hit.index, hit.output);
    if (!event.shiftKey && secondClick('port', `${hit.block}:${hit.index}:${hit.output}`, point)) {
      refresh();
      startEdit();
      event.preventDefault();
      return;
    }
    if (!hit.output && pending) {
      if (app.connect(pending.block, pending.index, hit.block, hit.index)) {
        pending = null;
        const saved = save();
        if (saved) status.textContent = 'Connected.';
      } else {
        status.textContent = 'Input occupied or on the same block.';
      }
      refresh();
      return;
    }
    if (hit.output) {
      pending = hit;
      gesture = { kind: 'wire', from: hit, point };
      status.textContent = 'Click an input or drag to one; Escape cancels.';
    }
  } else {
    pending = null;
    const block = event.target.closest('[data-block]');
    if (block) {
      const id = Number(block.dataset.block);
      app.select_block(id, event.shiftKey, point.x, point.y);
      if (!event.shiftKey && secondClick('block', id, point)) {
        app.end_drag();
        refresh();
        startEdit();
        event.preventDefault();
        return;
      }
      gesture = { kind: 'block', moved: false };
    } else if (app.select_wire(point.x, point.y, 7 / zoom)) {
      lastClick = null;
      status.textContent = 'Wire selected.';
    } else {
      lastClick = null;
      app.start_marquee(point.x, point.y, event.shiftKey);
      gesture = { kind: 'marquee', start: point, point };
    }
  }
  if (gesture) svg.setPointerCapture(event.pointerId);
  event.preventDefault();
  refresh();
});

svg.addEventListener('pointermove', event => {
  if (!app || !editor.hidden) return;
  const point = world(event);
  mousePoint = point;
  if (gesture?.kind === 'pan') {
    viewX += gesture.anchor.x - point.x;
    viewY += gesture.anchor.y - point.y;
    setView();
    save();
    return;
  }
  if (gesture?.kind === 'block') {
    const changed = app.drag_to(point.x, point.y);
    gesture.moved = changed || gesture.moved;
    if (changed) { lastClick = null; refresh(); save(); }
    return;
  }
  if (gesture?.kind === 'marquee') {
    gesture.point = point;
    app.marquee_to(point.x, point.y);
    refresh();
    return;
  }
  const next = hitPort(point);
  if (gesture?.kind === 'wire') gesture.point = point;
  if (!samePort(next, hover) || pending) {
    hover = next;
    draw();
  }
});

function end(event, cancelled = false) {
  if (!gesture) return;
  if (gesture.kind === 'wire' && !cancelled) {
    const target = hitPort(world(event));
    if (validTarget(target) && app.connect(gesture.from.block, gesture.from.index, target.block, target.index)) {
      pending = null;
      lastClick = null;
      if (save()) status.textContent = 'Connected.';
    } else if (!samePort(target, gesture.from)) {
      status.textContent = 'No connection made. Drop on an unoccupied input of another block.';
    }
  } else if (gesture.kind === 'block') {
    if (!cancelled) {
      const point = world(event);
      gesture.moved = app.drag_to(point.x, point.y) || gesture.moved;
    }
    app.end_drag();
    if (gesture.moved && save()) status.textContent = 'Block moved.';
    else if (!gesture.moved) status.textContent = 'Block selected.';
  } else if (gesture.kind === 'marquee') {
    if (!cancelled) app.marquee_to(world(event).x, world(event).y);
    app.end_marquee();
    status.textContent = 'Selection updated.';
  } else if (gesture.kind === 'pan') {
    if (save()) status.textContent = 'View moved.';
  }
  gesture = null;
  svg.classList.remove('panning');
  hover = hitPort(world(event));
  refresh();
}
svg.addEventListener('pointerup', event => end(event));
svg.addEventListener('pointercancel', event => end(event, true));
svg.addEventListener('pointerleave', () => { if (!gesture) { hover = null; if (app) draw(); } });
svg.addEventListener('wheel', event => {
  if (!app || gesture || !editor.hidden) return;
  event.preventDefault();
  if (event.ctrlKey) {
    zoomAt(event.deltaY < 0 ? 1.1 : 1 / 1.1, event);
  } else {
    const step = event.deltaMode === 1 ? 24 : event.deltaMode === 2 ? 840 : 1;
    const transform = svg.getScreenCTM().inverse();
    viewX += event.deltaX * step * transform.a;
    viewY += event.deltaY * step * transform.d;
    setView();
    save();
  }
}, { passive: false });
document.querySelector('#zoom-in').addEventListener('click', () => zoomCenter(1.2));
document.querySelector('#zoom-out').addEventListener('click', () => zoomCenter(1 / 1.2));
document.querySelector('#fit').addEventListener('click', fitView);
document.querySelector('#add-block').addEventListener('click', () => addBlock(viewX + 640 / zoom - 96, viewY + 420 / zoom - 48));
document.querySelector('#add-input').addEventListener('click', () => addPort(false));
document.querySelector('#add-output').addEventListener('click', () => addPort(true));
document.querySelector('#rename').addEventListener('click', startEdit);
document.querySelector('#delete').addEventListener('click', deleteSelection);
window.addEventListener('keydown', event => {
  if (!app || event.target.closest('input, textarea, [contenteditable]')) return;
  if (event.key === 'Delete' || event.key === 'Backspace') {
    event.preventDefault();
    deleteSelection();
  } else if (event.key === 'Escape') {
    gesture = null;
    pending = null;
    lastClick = null;
    svg.classList.remove('panning');
    app.clear_selection();
    refresh();
    status.textContent = 'Selection cleared / connection cancelled.';
  } else if (event.key === 'F2') {
    event.preventDefault();
    startEdit();
  } else if (event.key.toLowerCase() === 'n' && !event.ctrlKey && !event.altKey && !event.metaKey) {
    addBlock(viewX + 640 / zoom - 96, viewY + 420 / zoom - 48);
  }
});

document.querySelector('#reset').addEventListener('click', () => {
  if (!Diagram || !confirm('Replace the saved browser diagram with the demo?')) return;
  app = new Diagram('');
  gesture = null;
  svg.classList.remove('panning');
  hover = null;
  pending = null;
  editor.hidden = true;
  zoom = 1;
  viewX = 0;
  viewY = 0;
  setView();
  const saved = save();
  refresh();
  if (saved) status.textContent = 'Demo diagram restored.';
});

try {
  const module = await import('./pkg/diagram_web.js');
  await module.default();
  Diagram = module.BrowserDiagram;
  const saved = localStorage.getItem(key);
  const data = saved ? JSON.parse(saved) : null;
  if (data?.diagram && data.version !== 1) throw new Error('Unsupported browser save version');
  app = new Diagram(data ? JSON.stringify(data.diagram ?? data) : '');
  if (data?.view) {
    const view = data.view;
    if (![view.x, view.y, view.zoom].every(Number.isFinite) ||
        view.zoom < 0.4 || view.zoom > 1.8) throw new Error('Invalid saved viewport');
    viewX = view.x;
    viewY = view.y;
    zoom = view.zoom;
    setView();
  }
  refresh();
  status.textContent = 'Ready.';
} catch (error) {
  status.textContent = `Cannot load diagram: ${error}. Reset to replace saved data.`;
}
