import './styles.css';
import tgpu, { common, d } from 'typegpu';

const app = document.querySelector('#app');

const icons = {
  blocks: '<svg viewBox="0 0 24 24"><rect x="4" y="4" width="6" height="6" rx="1"/><rect x="14" y="4" width="6" height="6" rx="1"/><rect x="4" y="14" width="6" height="6" rx="1"/><rect x="14" y="14" width="6" height="6" rx="1"/></svg>',
  sliders: '<svg viewBox="0 0 24 24"><path d="M4 7h5M15 7h5M4 17h9M19 17h1M9 4v6M15 14v6"/><circle cx="12" cy="7" r="3"/><circle cx="16" cy="17" r="3"/></svg>',
  search: '<svg viewBox="0 0 24 24"><circle cx="10.8" cy="10.8" r="6.2"/><path d="m16 16 4.5 4.5"/></svg>',
  plus: '<svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"/></svg>',
  minus: '<svg viewBox="0 0 24 24"><path d="M5 12h14"/></svg>',
  maximize: '<svg viewBox="0 0 24 24"><path d="M8 4H4v4M16 4h4v4M20 16v4h-4M4 16v4h4"/></svg>',
  save: '<svg viewBox="0 0 24 24"><path d="M5 4h12l2 2v14H5zM8 4v6h8V4M8 20v-6h8v6"/></svg>',
  download: '<svg viewBox="0 0 24 24"><path d="M12 4v11M8 11l4 4 4-4M5 20h14"/></svg>',
  play: '<svg viewBox="0 0 24 24"><path d="m8 5 11 7-11 7z" fill="currentColor" stroke="none"/></svg>',
  stop: '<svg viewBox="0 0 24 24"><rect x="7" y="7" width="10" height="10" rx="1" fill="currentColor" stroke="none"/></svg>',
  select: '<svg viewBox="0 0 24 24"><path d="m6 3 12 11-6 .8 3 5-2.4 1.2-3-5-3.6 3.5z"/></svg>',
  pan: '<svg viewBox="0 0 24 24"><path d="M8 11V6a1.5 1.5 0 0 1 3 0v4-6a1.5 1.5 0 0 1 3 0v6-4a1.5 1.5 0 0 1 3 0v6-2a1.5 1.5 0 0 1 3 0v5c0 4-2 6-6 6h-2c-3 0-4-2-6-5l-2-3a1.5 1.5 0 0 1 2.5-1.5z"/></svg>',
  trash: '<svg viewBox="0 0 24 24"><path d="M5 7h14M10 11v5M14 11v5M8 7l1 13h6l1-13M9 7V4h6v3"/></svg>',
  info: '<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="8.5"/><path d="M12 11v5M12 8h.01"/></svg>',
  route: '<svg viewBox="0 0 24 24"><path d="M5 6h5v12h9M5 6v12M19 18l-3-3m3 3-3 3"/></svg>',
  cpu: '<svg viewBox="0 0 24 24"><rect x="7" y="7" width="10" height="10" rx="1"/><path d="M9 2v3M15 2v3M9 19v3M15 19v3M2 9h3M2 15h3M19 9h3M19 15h3"/></svg>',
  waveform: '<svg viewBox="0 0 24 24"><path d="M3 13h3l2-7 4 13 3-9 2 3h4"/></svg>',
  filter: '<svg viewBox="0 0 24 24"><path d="M4 5h16M7 12h10M10 19h4"/></svg>',
  gain: '<svg viewBox="0 0 24 24"><path d="M4 16 9 11l3 3 7-8"/><path d="M15 6h4v4"/></svg>',
  scope: '<svg viewBox="0 0 24 24"><rect x="4" y="5" width="16" height="14" rx="2"/><path d="M7 14h2l2-5 2 7 2-4h2"/></svg>',
  reset: '<svg viewBox="0 0 24 24"><path d="M5 8a8 8 0 1 1 1 9"/><path d="M5 4v4h4"/></svg>',
};

const blockDefinitions = {
  source: {
    label: 'Signal Source',
    shortLabel: 'Source',
    group: 'Sources',
    description: 'generates a signal',
    icon: icons.waveform,
    fields: [
      { key: 'waveform', label: 'waveform', type: 'select', options: ['sine', 'square', 'triangle'], value: 'sine' },
      { key: 'frequency', label: 'frequency', type: 'number', suffix: 'Hz', value: 2.5, step: 0.1 },
      { key: 'amplitude', label: 'amplitude', type: 'number', suffix: 'V', value: 1, step: 0.1 },
    ],
    inputs: [],
    outputs: [{ id: 'out', label: 'signal', kind: 'f32' }],
  },
  filter: {
    label: 'Low-pass Filter',
    shortLabel: 'Filter',
    group: 'Transforms',
    description: 'smooths a signal',
    icon: icons.filter,
    fields: [
      { key: 'cutoff', label: 'cutoff', type: 'number', suffix: 'Hz', value: 12, step: 1 },
      { key: 'resonance', label: 'resonance', type: 'number', suffix: 'Q', value: 0.7, step: 0.1 },
    ],
    inputs: [{ id: 'in', label: 'input', kind: 'f32' }],
    outputs: [{ id: 'out', label: 'filtered', kind: 'f32' }],
  },
  gain: {
    label: 'Gain',
    shortLabel: 'Gain',
    group: 'Transforms',
    description: 'scales a signal',
    icon: icons.gain,
    fields: [
      { key: 'gain', label: 'gain', type: 'number', suffix: '×', value: 1.25, step: 0.05 },
      { key: 'clamp', label: 'clamp output', type: 'select', options: ['off', 'on'], value: 'off' },
    ],
    inputs: [{ id: 'in', label: 'input', kind: 'f32' }],
    outputs: [{ id: 'out', label: 'scaled', kind: 'f32' }],
  },
  scope: {
    label: 'Signal Scope',
    shortLabel: 'Scope',
    group: 'Outputs',
    description: 'plots a signal',
    icon: icons.scope,
    fields: [
      { key: 'window', label: 'time window', type: 'select', options: ['1 s', '2 s', '5 s', '10 s'], value: '5 s' },
      { key: 'refresh', label: 'refresh rate', type: 'number', suffix: 'fps', value: 30, step: 1 },
    ],
    inputs: [{ id: 'in', label: 'signal', kind: 'f32' }],
    outputs: [],
  },
};

const cloneFields = (type) => blockDefinitions[type].fields.map((field) => ({ ...field, options: field.options ? [...field.options] : undefined }));
let nodeSequence = 5;
const createNode = (type, x, y) => ({
  id: `${type}-${nodeSequence++}`,
  type,
  title: blockDefinitions[type].label,
  x,
  y,
  fields: cloneFields(type),
});

const initialNodes = [
  { ...createNode('source', 78, 195), id: 'source-1' },
  { ...createNode('filter', 350, 215), id: 'filter-1' },
  { ...createNode('gain', 350, 462), id: 'gain-1' },
  { ...createNode('scope', 635, 245), id: 'scope-1' },
];
const initialEdges = [
  { id: 'edge-1', source: 'source-1', sourcePort: 'out', target: 'filter-1', targetPort: 'in', bend: 0 },
  { id: 'edge-2', source: 'filter-1', sourcePort: 'out', target: 'gain-1', targetPort: 'in', bend: 0 },
  { id: 'edge-3', source: 'gain-1', sourcePort: 'out', target: 'scope-1', targetPort: 'in', bend: -40 },
];

const initialState = () => ({
  nodes: initialNodes.map((node) => ({ ...node, fields: node.fields.map((field) => ({ ...field, options: field.options ? [...field.options] : undefined })) })),
  edges: initialEdges.map((edge) => ({ ...edge })),
  selection: { kind: 'node', id: 'filter-1' },
  zoom: 1,
  pan: { x: 0, y: 0 },
  interactionMode: 'select',
  running: false,
});

let state = initialState();
let dragState = null;
let connectState = null;
let toastTimer;
const routeCache = new Map();
let gpuRenderer = null;

const escapeHtml = (value) => String(value).replace(/[&<>"']/g, (character) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#039;' }[character]));
const findNode = (id) => state.nodes.find((node) => node.id === id);
const findEdge = (id) => state.edges.find((edge) => edge.id === id);
const selectedNode = () => state.selection?.kind === 'node' ? findNode(state.selection.id) : null;
const selectedEdge = () => state.selection?.kind === 'edge' ? findEdge(state.selection.id) : null;

app.innerHTML = `
  <div class="app-shell">
    <header class="topbar">
      <div class="brand">
        <div class="brand-mark" aria-hidden="true"></div>
        <div class="brand-copy">
          <div class="brand-name">signal<span>canvas</span></div>
          <div class="brand-meta">typegpu node system study</div>
        </div>
      </div>
      <div class="topbar-center">
        <div class="workspace-name">Audio dynamics</div>
        <div class="status-pill"><span class="status-dot"></span><span id="renderer-status">initializing GPU</span></div>
      </div>
      <div class="top-actions">
        <button class="icon-button" id="reset-button" title="Reset example graph">${icons.reset}</button>
        <button class="icon-button" id="export-button" title="Export graph">${icons.download}</button>
        <button class="ghost-button" id="save-button">${icons.save}<span>Save</span></button>
        <button class="primary-button" id="run-button">${icons.play}<span>Run graph</span></button>
      </div>
    </header>

    <main class="workspace">
      <aside class="sidebar left-sidebar">
        <div class="panel-heading"><h2>Block library</h2><span class="count" id="library-count">04</span></div>
        <div class="search-box">${icons.search}<input id="block-search" type="search" aria-label="Search blocks" placeholder="Search blocks" autocomplete="off" /></div>
        <div class="library-group">
          <div class="group-label">Sources</div>
          <div class="library-list">
            <button class="library-card" data-type="source" draggable="true"><span class="library-icon">${icons.waveform}</span><span class="library-copy"><span class="library-title">Signal Source</span><span class="library-desc">generates a signal</span></span><span class="library-add">+</span></button>
          </div>
        </div>
        <div class="library-group">
          <div class="group-label">Transforms</div>
          <div class="library-list">
            <button class="library-card" data-type="filter" draggable="true"><span class="library-icon">${icons.filter}</span><span class="library-copy"><span class="library-title">Low-pass Filter</span><span class="library-desc">smooths a signal</span></span><span class="library-add">+</span></button>
            <button class="library-card" data-type="gain" draggable="true"><span class="library-icon">${icons.gain}</span><span class="library-copy"><span class="library-title">Gain</span><span class="library-desc">scales a signal</span></span><span class="library-add">+</span></button>
          </div>
        </div>
        <div class="library-group">
          <div class="group-label">Outputs</div>
          <div class="library-list">
            <button class="library-card" data-type="scope" draggable="true"><span class="library-icon">${icons.scope}</span><span class="library-copy"><span class="library-title">Signal Scope</span><span class="library-desc">plots a signal</span></span><span class="library-add">+</span></button>
          </div>
        </div>
        <div class="sidebar-tip">
          <div class="tip-kicker">${icons.info}<span>Quick start</span></div>
          <p>Drag a block onto the canvas, then pull from an output port to an input. Select a connection to route it.</p>
          <p><kbd>Del</kbd> removes the selected item.</p>
        </div>
      </aside>

      <section class="canvas-panel">
        <div class="canvas-header">
          <div class="canvas-tabs"><div class="canvas-tab active">${icons.blocks}<span>Graph editor</span></div><div class="canvas-tab">${icons.sliders}<span>Parameters</span></div></div>
          <div class="canvas-tools">
            <button class="icon-button active" id="select-mode" title="Select and move nodes">${icons.select}</button>
            <button class="icon-button" id="pan-mode" title="Pan canvas">${icons.pan}</button>
            <span class="tool-divider"></span>
            <button class="icon-button" id="zoom-out" title="Zoom out">${icons.minus}</button><span class="zoom-readout" id="zoom-readout">100%</span><button class="icon-button" id="zoom-in" title="Zoom in">${icons.plus}</button>
            <button class="icon-button" id="zoom-reset" title="Reset zoom">${icons.maximize}</button>
          </div>
        </div>
        <div class="canvas-viewport" id="canvas-viewport">
          <canvas id="gpu-canvas" aria-hidden="true"></canvas>
          <div class="grid-overlay" aria-hidden="true"></div>
          <div class="canvas-hud"><span class="hud-badge"><span class="mini-dot"></span><strong id="node-count">04</strong> blocks</span><span class="hud-badge">${icons.route}<strong id="edge-count">03</strong> connections</span></div>
          <div class="graph-layer" id="graph-layer"><svg class="connections" id="connections" aria-label="Graph connections"></svg><div id="node-layer"></div></div>
          <div class="drop-hint" id="drop-hint"><div class="drop-hint-icon">${icons.plus}</div><p>Drop a block here</p><small>or choose one from the library</small></div>
          <div class="canvas-help"><span><kbd>Port</kbd> + drag to connect</span><span><kbd>Dots</kbd> route line</span><span><kbd>2×</kbd> add bend</span><span><kbd>Scroll</kbd> zoom</span></div>
        </div>
        <div class="canvas-footer"><div class="footer-left"><span class="footer-item">${icons.cpu}<span id="render-engine">TypeGPU / WebGPU</span></span><span class="footer-item">${icons.route}<strong id="footer-selection">Node selected</strong></span></div><div class="footer-right"><span class="footer-item">x <strong id="cursor-x">0</strong></span><span class="footer-item">y <strong id="cursor-y">0</strong></span><span class="footer-item" id="save-state">All changes saved</span></div></div>
      </section>

      <aside class="sidebar right-sidebar"><div id="inspector"></div></aside>
    </main>
    <div class="toast" id="toast"></div>
  </div>
`;

const viewport = document.querySelector('#canvas-viewport');
const graphLayer = document.querySelector('#graph-layer');
const nodeLayer = document.querySelector('#node-layer');
const connections = document.querySelector('#connections');
const inspector = document.querySelector('#inspector');
const dropHint = document.querySelector('#drop-hint');
const toast = document.querySelector('#toast');

function fieldControl(field, context = 'node') {
  const fieldId = `${context}-field-${field.key}`;
  const dataAttr = `data-field="${escapeHtml(field.key)}"`;
  if (field.type === 'select') {
    return `<select id="${fieldId}" ${dataAttr}>${field.options.map((option) => `<option value="${escapeHtml(option)}" ${option === field.value ? 'selected' : ''}>${escapeHtml(option)}</option>`).join('')}</select>`;
  }
  return `<input id="${fieldId}" ${dataAttr} type="number" value="${escapeHtml(field.value)}" step="${field.step ?? 1}" />`;
}

function renderNode(node) {
  const definition = blockDefinitions[node.type];
  const portRows = [
    ...definition.inputs.map((port) => `<div class="port-row input"><span class="port" data-port-direction="input" data-node-id="${node.id}" data-port-id="${port.id}" title="Connect to ${port.label}"></span><span class="port-label">${escapeHtml(port.label)} <em>${escapeHtml(port.kind)}</em></span></div>`),
    ...definition.outputs.map((port) => `<div class="port-row output"><span class="port-label">${escapeHtml(port.label)} <em>${escapeHtml(port.kind)}</em></span><span class="port" data-port-direction="output" data-node-id="${node.id}" data-port-id="${port.id}" title="Drag to connect"></span></div>`),
  ].join('');
  const params = node.fields.map((field) => `<div class="node-param"><label for="node-${escapeHtml(node.id)}-field-${escapeHtml(field.key)}">${escapeHtml(field.label)}</label>${fieldControl(field, `node-${node.id}`)}</div>`).join('');
  const stateLabel = state.running ? 'streaming' : 'ready';
  return `<article class="node ${node.type} ${state.selection?.kind === 'node' && state.selection.id === node.id ? 'selected' : ''}" data-node-id="${node.id}" style="left:${node.x}px;top:${node.y}px">
    <div class="node-header"><div class="node-symbol">${definition.icon}</div><div class="node-title-wrap"><div class="node-type">${escapeHtml(definition.shortLabel)}</div><div class="node-title">${escapeHtml(node.title)}</div></div><div class="node-actions"><button class="node-action node-delete" title="Remove block">${icons.trash}</button></div></div>
    <div class="node-body"><div class="node-section-label">ports</div>${portRows}<div class="node-param-list"><div class="node-section-label">parameters</div>${params}</div><div class="node-footer"><span class="node-state"><i></i>${stateLabel}</span><span class="node-id">${escapeHtml(node.id)}</span></div></div>
  </article>`;
}

function getPortPosition(node, portId, direction) {
  const definition = blockDefinitions[node.type];
  const ports = direction === 'input' ? definition.inputs : definition.outputs;
  const index = Math.max(0, ports.findIndex((port) => port.id === portId));
  const rowY = node.y + 49 + 8 + 14 + index * 27 + 13;
  return { x: node.x + (direction === 'input' ? 0 : 220), y: rowY };
}

const ROUTE_MARGIN = 18;
const PORT_STUB = 36;
const ROUTE_RADIUS = 12;

function routeNodeRect(node) {
  const element = nodeLayer.querySelector(`[data-node-id="${node.id}"]`);
  const height = element?.offsetHeight || 220;
  return { id: node.id, left: node.x, top: node.y, right: node.x + 220, bottom: node.y + height };
}

function expandedRouteRect(rect) {
  return { left: rect.left - ROUTE_MARGIN, top: rect.top - ROUTE_MARGIN, right: rect.right + ROUTE_MARGIN, bottom: rect.bottom + ROUTE_MARGIN };
}

function pointIsInside(point, rect) {
  return point.x > rect.left + 0.1 && point.x < rect.right - 0.1 && point.y > rect.top + 0.1 && point.y < rect.bottom - 0.1;
}

function segmentBlocked(start, end, obstacles) {
  const horizontal = Math.abs(start.y - end.y) < 0.1;
  const vertical = Math.abs(start.x - end.x) < 0.1;
  for (const obstacle of obstacles) {
    if (horizontal && start.y > obstacle.top + 0.1 && start.y < obstacle.bottom - 0.1) {
      const left = Math.min(start.x, end.x);
      const right = Math.max(start.x, end.x);
      if (right > obstacle.left + 0.1 && left < obstacle.right - 0.1) return true;
    }
    if (vertical && start.x > obstacle.left + 0.1 && start.x < obstacle.right - 0.1) {
      const top = Math.min(start.y, end.y);
      const bottom = Math.max(start.y, end.y);
      if (bottom > obstacle.top + 0.1 && top < obstacle.bottom - 0.1) return true;
    }
  }
  return false;
}

function samePoint(a, b) {
  return Math.abs(a.x - b.x) < 0.1 && Math.abs(a.y - b.y) < 0.1;
}

function segmentsCross(a, b, c, d) {
  const firstHorizontal = Math.abs(a.y - b.y) < 0.1;
  const secondHorizontal = Math.abs(c.y - d.y) < 0.1;
  if (firstHorizontal && secondHorizontal) {
    if (Math.abs(a.y - c.y) > 0.1) return false;
    return Math.min(Math.max(a.x, b.x), Math.max(c.x, d.x)) - Math.max(Math.min(a.x, b.x), Math.min(c.x, d.x)) > 2;
  }
  if (!firstHorizontal && !secondHorizontal) {
    if (Math.abs(a.x - c.x) > 0.1) return false;
    return Math.min(Math.max(a.y, b.y), Math.max(c.y, d.y)) - Math.max(Math.min(a.y, b.y), Math.min(c.y, d.y)) > 2;
  }
  const horizontalStart = firstHorizontal ? a : c;
  const horizontalEnd = firstHorizontal ? b : d;
  const verticalStart = firstHorizontal ? c : a;
  const verticalEnd = firstHorizontal ? d : b;
  const crossingX = verticalStart.x;
  const crossingY = horizontalStart.y;
  const withinHorizontal = crossingX > Math.min(horizontalStart.x, horizontalEnd.x) + 1 && crossingX < Math.max(horizontalStart.x, horizontalEnd.x) - 1;
  const withinVertical = crossingY > Math.min(verticalStart.y, verticalEnd.y) + 1 && crossingY < Math.max(verticalStart.y, verticalEnd.y) - 1;
  return withinHorizontal && withinVertical;
}

function routeCrossesOccupied(start, end, occupiedRoutes) {
  return occupiedRoutes.some((route) => route.some((point, index) => index > 0 && segmentsCross(start, end, route[index - 1], point)));
}

function simplifyRoute(points) {
  const result = [];
  for (const point of points) {
    const previous = result[result.length - 1];
    if (previous && samePoint(previous, point)) continue;
    const beforePrevious = result[result.length - 2];
    if (beforePrevious && previous && ((beforePrevious.x === previous.x && previous.x === point.x) || (beforePrevious.y === previous.y && previous.y === point.y))) {
      result[result.length - 1] = point;
    } else {
      result.push(point);
    }
  }
  return result;
}

function routeToSvgPath(points, radius = ROUTE_RADIUS) {
  if (points.length < 2) return '';
  let path = `M ${points[0].x} ${points[0].y}`;
  for (let index = 1; index < points.length; index += 1) {
    const point = points[index];
    const previous = points[index - 1];
    if (index === points.length - 1 || radius <= 0) {
      path += ` L ${point.x} ${point.y}`;
      continue;
    }
    const next = points[index + 1];
    const incomingLength = Math.abs(point.x - previous.x) + Math.abs(point.y - previous.y);
    const outgoingLength = Math.abs(next.x - point.x) + Math.abs(next.y - point.y);
    const cornerRadius = Math.min(radius, incomingLength / 2, outgoingLength / 2);
    const before = point.x === previous.x
      ? { x: point.x, y: point.y + (previous.y < point.y ? -cornerRadius : cornerRadius) }
      : { x: point.x + (previous.x < point.x ? -cornerRadius : cornerRadius), y: point.y };
    const after = next.x === point.x
      ? { x: point.x, y: point.y + (next.y < point.y ? -cornerRadius : cornerRadius) }
      : { x: point.x + (next.x < point.x ? -cornerRadius : cornerRadius), y: point.y };
    path += ` L ${before.x} ${before.y} Q ${point.x} ${point.y} ${after.x} ${after.y}`;
  }
  return path;
}

function routeHandle(points, preferredX = null) {
  const verticalSegments = [];
  for (let index = 1; index < points.length; index += 1) {
    const start = points[index - 1];
    const end = points[index];
    if (start.x === end.x && Math.abs(start.y - end.y) > 20) {
      verticalSegments.push({ x: start.x, y: (start.y + end.y) / 2, length: Math.abs(start.y - end.y) });
    }
  }
  const sortedSegments = preferredX === null
    ? verticalSegments.sort((a, b) => b.length - a.length)
    : verticalSegments.sort((a, b) => Math.abs(a.x - preferredX) - Math.abs(b.x - preferredX) || b.length - a.length);
  return sortedSegments[0] || { x: (points[0].x + points[points.length - 1].x) / 2, y: (points[0].y + points[points.length - 1].y) / 2 };
}

function routeControlMarkup(edge, points) {
  const segmentControls = points.slice(0, -1).map((point, index) => {
    const next = points[index + 1];
    const length = Math.abs(next.x - point.x) + Math.abs(next.y - point.y);
    if (length < 18) return '';
    const movesOn = point.y === next.y ? 'vertical' : 'horizontal';
    return `<circle class="connection-control" data-segment-handle="${edge.id}:${index}" data-axis="${movesOn}" cx="${(point.x + next.x) / 2}" cy="${(point.y + next.y) / 2}" r="3.5"></circle>`;
  }).join('');
  const bendControls = points.slice(1, -1).map((point, index) => {
    const pointIndex = index + 1;
    const previous = points[pointIndex - 1];
    const next = points[pointIndex + 1];
    const movesOn = previous.y === point.y ? 'horizontal' : 'vertical';
    return `<circle class="connection-bend" data-bend-handle="${edge.id}:${pointIndex}" data-axis="${movesOn}" cx="${point.x}" cy="${point.y}" r="5"></circle>`;
  }).join('');
  return `${segmentControls}${bendControls}`;
}

function fallbackOrthogonalRoute(start, end, bend = 0, manualX = null) {
  const horizontalGap = end.x - start.x;
  const laneX = manualX ?? (horizontalGap >= 40
    ? Math.max(start.x + PORT_STUB, Math.min(end.x - PORT_STUB, (start.x + end.x) / 2 + bend * 0.6))
    : Math.max(start.x, end.x) + 60 + bend * 0.4);
  const points = simplifyRoute([start, { x: laneX, y: start.y }, { x: laneX, y: end.y }, end]);
  return { points, path: routeToSvgPath(points), route: routeHandle(points, laneX) };
}

function preferredRouteX(start, end, bend) {
  return end.x - start.x >= 40
    ? (start.x + end.x) / 2 + bend * 0.6
    : Math.max(start.x, end.x) + 60 + bend * 0.4;
}

function preferredRouteY(start, end, bend) {
  return (start.y + end.y) / 2 + bend;
}

function findOrthogonalRoute(start, end, bend, verticalBend, sourceId, targetId, occupiedRoutes, manualX = null) {
  const obstacles = state.nodes.map(routeNodeRect).map(expandedRouteRect);
  const preferredX = manualX ?? preferredRouteX(start, end, bend);
  const preferredY = preferredRouteY(start, end, verticalBend);
  const startStub = { x: start.x + PORT_STUB, y: start.y };
  const endStub = { x: end.x - PORT_STUB, y: end.y };
  const xValues = [startStub.x, endStub.x, preferredX, start.x, end.x];
  if (manualX !== null) xValues.push(manualX);
  const yValues = [startStub.y, endStub.y, start.y, end.y, preferredY];
  state.nodes.forEach((node) => {
    const rect = routeNodeRect(node);
    const expanded = expandedRouteRect(rect);
    xValues.push(expanded.left, expanded.right, rect.left, rect.right);
    yValues.push(expanded.top, expanded.bottom, rect.top, rect.bottom);
  });
  occupiedRoutes.forEach((route) => route.forEach((point) => { xValues.push(point.x); yValues.push(point.y); }));
  const uniqueSorted = (values) => [...new Set(values.map((value) => Math.round(value * 10) / 10))].sort((a, b) => a - b);
  const xs = uniqueSorted(xValues);
  const ys = uniqueSorted(yValues);
  const startX = xs.findIndex((value) => Math.abs(value - startStub.x) < 0.1);
  const startY = ys.findIndex((value) => Math.abs(value - startStub.y) < 0.1);
  const endX = xs.findIndex((value) => Math.abs(value - endStub.x) < 0.1);
  const endY = ys.findIndex((value) => Math.abs(value - endStub.y) < 0.1);
  if (startX < 0 || startY < 0 || endX < 0 || endY < 0) return null;

  const keyFor = (x, y) => `${x}:${y}`;
  const startKey = keyFor(startX, startY);
  const endKey = keyFor(endX, endY);
  const open = [{ x: startX, y: startY, key: startKey, g: 0, f: 0, direction: null }];
  const best = new Map([[startKey, 0]]);
  const cameFrom = new Map();
  const directions = [[1, 0, 'H'], [-1, 0, 'H'], [0, 1, 'V'], [0, -1, 'V']];
  const pointAt = (x, y) => ({ x: xs[x], y: ys[y] });
  const pointBlocked = (point) => obstacles.some((obstacle) => pointIsInside(point, obstacle));
  let iterations = 0;
  while (open.length && iterations < 5000) {
    iterations += 1;
    open.sort((a, b) => a.f - b.f);
    const current = open.shift();
    if (!current || current.g !== best.get(current.key)) continue;
    if (current.key === endKey) {
      const indices = [];
      let cursor = current.key;
      while (cursor) {
        const [x, y] = cursor.split(':').map(Number);
        indices.unshift(pointAt(x, y));
        cursor = cameFrom.get(cursor)?.key;
      }
      const points = simplifyRoute([start, ...indices, end]);
      return { points, path: routeToSvgPath(points), route: routeHandle(points, manualX ?? preferredX) };
    }
    for (const [dx, dy, direction] of directions) {
      const nextX = current.x + dx;
      const nextY = current.y + dy;
      if (nextX < 0 || nextX >= xs.length || nextY < 0 || nextY >= ys.length) continue;
      const from = pointAt(current.x, current.y);
      const to = pointAt(nextX, nextY);
      if (pointBlocked(to) || segmentBlocked(from, to, obstacles)) continue;
      const distance = Math.abs(to.x - from.x) + Math.abs(to.y - from.y);
      const turnPenalty = current.direction && current.direction !== direction ? 18 : 0;
      const crossingPenalty = routeCrossesOccupied(from, to, occupiedRoutes) ? 1200 : 0;
      // Backward/feedback routes should stay on the outer lane rather than
      // hugging the source port, leaving a clean channel for downstream lines.
      const laneWeight = manualX !== null ? 10 : (end.x - start.x >= 40 ? 0.025 : 1.5);
      const lanePenalty = Math.abs(to.x - preferredX) * laneWeight;
      const heightPenalty = Math.abs(to.y - preferredY) * 0.025;
      const score = current.g + distance + turnPenalty + crossingPenalty + lanePenalty + heightPenalty;
      const nextKey = keyFor(nextX, nextY);
      if (score < (best.get(nextKey) ?? Infinity)) {
        best.set(nextKey, score);
        cameFrom.set(nextKey, { key: current.key });
        const heuristic = Math.abs(to.x - endStub.x) + Math.abs(to.y - endStub.y);
        open.push({ x: nextX, y: nextY, key: nextKey, g: score, f: score + heuristic, direction });
      }
    }
  }
  return null;
}

function orthogonalRoute(start, end, bend = 0, verticalBend = 0, sourceId = null, targetId = null, occupiedRoutes = [], manualX = null) {
  if (sourceId && targetId) {
    const routed = findOrthogonalRoute(start, end, bend, verticalBend, sourceId, targetId, occupiedRoutes, manualX);
    if (routed) return routed;
    // If a manually chosen lane becomes blocked after moving a node, recover
    // with the automatic obstacle-aware route instead of drawing through it.
    if (manualX !== null) {
      const recovered = findOrthogonalRoute(start, end, bend, verticalBend, sourceId, targetId, occupiedRoutes);
      if (recovered) return recovered;
    }
  }
  return fallbackOrthogonalRoute(start, end, bend, manualX);
}

function enforcePortStubs(points, start, end) {
  const nextPoints = points.map((point) => ({ ...point }));
  if (nextPoints.length < 2) return nextPoints;
  const first = nextPoints[1];
  if (first.y === start.y && first.x < start.x + PORT_STUB) {
    const previousX = first.x;
    first.x = start.x + PORT_STUB;
    for (let index = 2; index < nextPoints.length - 1 && nextPoints[index].x === previousX; index += 1) nextPoints[index].x = first.x;
  }
  const lastIndex = nextPoints.length - 2;
  const last = nextPoints[lastIndex];
  if (last.y === end.y && last.x > end.x - PORT_STUB) {
    const previousX = last.x;
    last.x = end.x - PORT_STUB;
    for (let index = lastIndex - 1; index > 0 && nextPoints[index].x === previousX; index -= 1) nextPoints[index].x = last.x;
  }
  return simplifyRoute(nextPoints);
}

function edgeGeometry(edge, occupiedRoutes = []) {
  const source = findNode(edge.source);
  const target = findNode(edge.target);
  if (!source || !target) return null;
  const start = getPortPosition(source, edge.sourcePort, 'output');
  const end = getPortPosition(target, edge.targetPort, 'input');
  const customPoints = Array.isArray(edge.routePoints) && edge.routePoints.length > 0
    ? enforcePortStubs(simplifyRoute([start, ...edge.routePoints, end]), start, end)
    : null;
  const route = customPoints
    ? { points: customPoints, route: routeHandle(customPoints, edge.routeX ?? null) }
    : orthogonalRoute(start, end, edge.bend || 0, edge.verticalBend || 0, source.id, target.id, occupiedRoutes, edge.routeX ?? null);
  const points = customPoints ? route.points : enforcePortStubs(route.points, start, end);
  return { start, end, route: routeHandle(points, edge.routeX ?? null), points, path: routeToSvgPath(points) };
}

function renderConnections() {
  const bounds = viewport.getBoundingClientRect();
  connections.setAttribute('viewBox', `0 0 ${Math.max(1, bounds.width)} ${Math.max(1, bounds.height)}`);
  const occupiedRoutes = [];
  routeCache.clear();
  const edgeMarkup = state.edges.map((edge) => {
    const geometry = edgeGeometry(edge, occupiedRoutes);
    if (!geometry) return '';
    occupiedRoutes.push(geometry.points);
    routeCache.set(edge.id, geometry.points.map((point) => ({ ...point })));
    const selected = state.selection?.kind === 'edge' && state.selection.id === edge.id;
    return `<path class="connection-hitbox" data-edge-id="${edge.id}" d="${geometry.path}"></path><path class="connection-shadow" d="${geometry.path}"></path><path class="connection-path ${selected ? 'selected' : ''} ${state.running ? 'live' : ''}" data-edge-id="${edge.id}" d="${geometry.path}" marker-end="url(#edge-arrow)"></path>${selected ? routeControlMarkup(edge, geometry.points) : ''}`;
  }).join('');
  const tempRoute = connectState ? orthogonalRoute(connectState.start, connectState.point) : null;
  const tempMarkup = connectState ? `<path class="temp-connection" d="${tempRoute.path}"></path><circle class="connection-node" cx="${tempRoute.route.x}" cy="${tempRoute.route.y}" r="5"></circle>` : '';
  connections.innerHTML = `<defs><marker id="edge-arrow" markerWidth="7" markerHeight="7" refX="5" refY="3.5" orient="auto"><path class="connection-arrow" d="M0,0 L7,3.5 L0,7 Z"></path></marker></defs>${edgeMarkup}${tempMarkup}`;
}

function applyGraphTransform() {
  graphLayer.style.transform = `translate(${state.pan.x}px, ${state.pan.y}px) scale(${state.zoom})`;
  document.querySelector('#zoom-readout').textContent = `${Math.round(state.zoom * 100)}%`;
}

function renderNodes() {
  nodeLayer.innerHTML = state.nodes.map(renderNode).join('');
  document.querySelector('#node-count').textContent = String(state.nodes.length).padStart(2, '0');
  document.querySelector('#edge-count').textContent = String(state.edges.length).padStart(2, '0');
  dropHint.classList.toggle('visible', state.nodes.length === 0);
}

function inspectorField(field) {
  const control = fieldControl(field, 'inspector');
  return `<div class="field"><label for="inspector-field-${escapeHtml(field.key)}"><span>${escapeHtml(field.label)}</span><small>${field.suffix ? escapeHtml(field.suffix) : field.type === 'number' ? 'numeric' : 'option'}</small></label>${control}</div>`;
}

function renderNodeInspector(node) {
  const definition = blockDefinitions[node.type];
  const portList = [...definition.inputs.map((port) => ({ ...port, direction: 'in' })), ...definition.outputs.map((port) => ({ ...port, direction: 'out' }))];
  return `<div class="inspector-kicker"><span>Node inspector</span><span>active</span></div><div class="inspector-card">
    <div class="inspector-node-head"><div class="inspector-symbol">${definition.icon}</div><div><div class="node-type">${escapeHtml(definition.shortLabel)} block</div><div class="node-title">${escapeHtml(node.title)}</div></div></div>
    <div class="inspector-fields"><div class="field"><label for="node-title-input"><span>label</span><small>display name</small></label><input id="node-title-input" data-node-title type="text" value="${escapeHtml(node.title)}" /></div>${node.fields.map(inspectorField).join('')}</div>
    <div class="inspector-section"><div class="inspector-section-title"><span>Signal interface</span><span>${portList.length} ports</span></div><div class="port-list">${portList.map((port) => `<div class="port-list-row"><span class="port" style="--node-accent:${node.type === 'scope' ? 'var(--cyan)' : 'var(--blue)'}"></span><span>${escapeHtml(port.label)}</span><span class="direction">${port.direction} · ${escapeHtml(port.kind)}</span></div>`).join('')}</div></div>
    <button class="inspector-delete" id="delete-selected">${icons.trash}<span>Remove block</span></button>
  </div>`;
}

function renderEdgeInspector(edge) {
  const source = findNode(edge.source);
  const target = findNode(edge.target);
  if (!source || !target) return '';
  return `<div class="inspector-kicker"><span>Connection inspector</span><span>active</span></div><div class="inspector-card edge-inspector">
    <div class="inspector-node-head"><div class="edge-route-icon">${icons.route}</div><div><div class="node-type">Signal connection</div><div class="node-title">${escapeHtml(source.title)} → ${escapeHtml(target.title)}</div></div></div>
    <div class="edge-summary"><div class="edge-summary-row"><span>source</span><strong>${escapeHtml(source.id)} / ${escapeHtml(edge.sourcePort)}</strong></div><div class="edge-summary-row"><span>target</span><strong>${escapeHtml(target.id)} / ${escapeHtml(edge.targetPort)}</strong></div></div>
    <div class="route-control"><div class="field"><label for="route-input"><span>horizontal route</span><small>drag the purple dot</small></label><div class="range-wrap"><input id="route-input" data-route-field type="range" min="-150" max="150" step="1" value="${edge.bend || 0}" /><span class="range-value" id="route-value">${edge.bend || 0}px</span></div></div><div class="field"><label for="vertical-route-input"><span>vertical route</span><small>drag the purple dot</small></label><div class="range-wrap"><input id="vertical-route-input" data-vertical-route-field type="range" min="-150" max="150" step="1" value="${edge.verticalBend || 0}" /><span class="range-value" id="vertical-route-value">${edge.verticalBend || 0}px</span></div></div></div>
    <button class="inspector-delete" id="delete-selected">${icons.trash}<span>Disconnect line</span></button>
  </div>`;
}

function renderEmptyInspector() {
  return `<div class="inspector-kicker"><span>Inspector</span><span>idle</span></div><div class="empty-inspector"><div class="empty-icon">${icons.select}</div><h3>Select something to edit</h3><p>Choose a block to tune its parameters, or select a line to change its route.</p></div>`;
}

function renderInspector() {
  const node = selectedNode();
  const edge = selectedEdge();
  inspector.innerHTML = node ? renderNodeInspector(node) : edge ? renderEdgeInspector(edge) : renderEmptyInspector();
  const footerSelection = document.querySelector('#footer-selection');
  footerSelection.textContent = node ? `${node.title} selected` : edge ? 'Connection selected' : 'Nothing selected';
}

function renderAll() {
  renderNodes();
  applyGraphTransform();
  renderConnections();
  renderInspector();
}

function showToast(message) {
  toast.textContent = message;
  toast.classList.add('visible');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => toast.classList.remove('visible'), 2100);
}

function markChanged(label = 'Unsaved changes') {
  const saveState = document.querySelector('#save-state');
  saveState.textContent = label;
  saveState.classList.remove('saved');
}

function persistState() {
  const serializable = { nodes: state.nodes, edges: state.edges };
  localStorage.setItem('signal-canvas-graph', JSON.stringify(serializable));
  const saveState = document.querySelector('#save-state');
  saveState.textContent = 'All changes saved';
  saveState.classList.add('saved');
}

function saveGraph() {
  persistState();
  showToast('Graph saved locally');
}

function canvasPoint(event) {
  const rect = viewport.getBoundingClientRect();
  return { x: (event.clientX - rect.left - state.pan.x) / state.zoom, y: (event.clientY - rect.top - state.pan.y) / state.zoom };
}

function setSelection(kind, id) {
  state.selection = id ? { kind, id } : null;
  renderAll();
}

function addBlock(type, point) {
  const fallback = { x: 150 + (state.nodes.length % 3) * 38, y: 125 + (state.nodes.length % 4) * 34 };
  const position = point || fallback;
  const node = createNode(type, Math.max(20, Math.round(position.x - 110)), Math.max(70, Math.round(position.y - 60)));
  state.nodes.push(node);
  state.selection = { kind: 'node', id: node.id };
  markChanged();
  renderAll();
  showToast(`${blockDefinitions[type].label} added`);
}

function removeSelection() {
  if (state.selection?.kind === 'node') {
    const node = findNode(state.selection.id);
    if (!node) return;
    state.nodes = state.nodes.filter((item) => item.id !== node.id);
    state.edges = state.edges.filter((edge) => edge.source !== node.id && edge.target !== node.id);
    state.selection = null;
    markChanged();
    renderAll();
    showToast(`${node.title} removed`);
  } else if (state.selection?.kind === 'edge') {
    const edge = findEdge(state.selection.id);
    state.edges = state.edges.filter((item) => item.id !== state.selection.id);
    state.selection = null;
    markChanged();
    renderAll();
    if (edge) showToast('Connection disconnected');
  }
}

function updateNodeField(node, key, rawValue) {
  const field = node.fields.find((item) => item.key === key);
  if (!field) return;
  field.value = field.type === 'number' ? Number(rawValue) : rawValue;
  markChanged();
}

function setMode(mode) {
  state.interactionMode = mode;
  document.querySelector('#select-mode').classList.toggle('active', mode === 'select');
  document.querySelector('#pan-mode').classList.toggle('active', mode === 'pan');
  viewport.style.cursor = mode === 'pan' ? 'grab' : 'default';
}

function startNodeDrag(event, nodeElement) {
  const node = findNode(nodeElement.dataset.nodeId);
  if (!node || state.interactionMode !== 'select') return;
  let activeElement = nodeElement;
  if (state.selection?.kind !== 'node' || state.selection.id !== node.id) {
    state.selection = { kind: 'node', id: node.id };
    renderAll();
    activeElement = document.querySelector(`[data-node-id="${node.id}"]`);
  }
  dragState = { type: 'node', node, startX: event.clientX, startY: event.clientY, originX: node.x, originY: node.y, element: activeElement };
  activeElement?.classList.add('dragging');
  event.preventDefault();
}

function startPan(event, force = false) {
  if (!force && state.interactionMode !== 'pan') return;
  dragState = { type: 'pan', startX: event.clientX, startY: event.clientY, originX: state.pan.x, originY: state.pan.y };
  viewport.style.cursor = 'grabbing';
  event.preventDefault();
}

function targetPortAt(clientX, clientY) {
  const element = document.elementFromPoint(clientX, clientY);
  return element?.closest('.port[data-port-direction="input"]') || null;
}

function updateConnectTarget(clientX, clientY) {
  document.querySelectorAll('.connect-target').forEach((element) => element.classList.remove('connect-target'));
  const port = targetPortAt(clientX, clientY);
  if (port && connectState && port.dataset.nodeId !== connectState.sourceNodeId) {
    port.closest('.node')?.classList.add('connect-target');
  }
  return port;
}

function beginConnection(event, port) {
  const node = findNode(port.dataset.nodeId);
  if (!node) return;
  const portPoint = getPortPosition(node, port.dataset.portId, 'output');
  connectState = { sourceNodeId: node.id, sourcePort: port.dataset.portId, start: portPoint, point: portPoint };
  setSelection('node', node.id);
  document.body.style.cursor = 'crosshair';
  event.preventDefault();
  event.stopPropagation();
}

function finishConnection(event) {
  if (!connectState) return;
  const targetPort = targetPortAt(event.clientX, event.clientY);
  const sourceId = connectState.sourceNodeId;
  if (targetPort && targetPort.dataset.nodeId !== sourceId) {
    state.edges = state.edges.filter((edge) => !(edge.target === targetPort.dataset.nodeId && edge.targetPort === targetPort.dataset.portId));
    const edge = { id: `edge-${Date.now()}`, source: sourceId, sourcePort: connectState.sourcePort, target: targetPort.dataset.nodeId, targetPort: targetPort.dataset.portId, bend: 0 };
    state.edges.push(edge);
    state.selection = { kind: 'edge', id: edge.id };
    markChanged();
    showToast('Signal connected');
  }
  connectState = null;
  document.body.style.cursor = '';
  document.querySelectorAll('.connect-target').forEach((element) => element.classList.remove('connect-target'));
  renderAll();
}

function prepareCustomRoute(edge) {
  const points = routeCache.get(edge.id);
  if (!points || points.length < 2) return null;
  edge.routePoints = points.slice(1, -1).map((point) => ({ ...point }));
  return points.map((point) => ({ ...point }));
}

function reanchorCustomRoute(edge) {
  if (!Array.isArray(edge.routePoints) || edge.routePoints.length === 0) return;
  const source = findNode(edge.source);
  const target = findNode(edge.target);
  if (!source || !target) return;
  const start = getPortPosition(source, edge.sourcePort, 'output');
  const end = getPortPosition(target, edge.targetPort, 'input');
  const points = simplifyRoute([start, ...edge.routePoints, end]);
  if (points.length < 3) return;

  const first = points[1];
  const previousFirstX = first.x;
  first.y = start.y;
  if (first.x < start.x + PORT_STUB) {
    first.x = start.x + PORT_STUB;
    for (let index = 2; index < points.length - 1 && points[index].x === previousFirstX; index += 1) points[index].x = first.x;
  }

  const lastIndex = points.length - 2;
  const last = points[lastIndex];
  const previousLastX = last.x;
  last.y = end.y;
  if (last.x > end.x - PORT_STUB) {
    last.x = end.x - PORT_STUB;
    for (let index = lastIndex - 1; index > 0 && points[index].x === previousLastX; index -= 1) points[index].x = last.x;
  }
  edge.routePoints = simplifyRoute(points).slice(1, -1).map((point) => ({ ...point }));
}

function reanchorConnectedRoutes(nodeId) {
  state.edges.filter((edge) => edge.source === nodeId || edge.target === nodeId).forEach(reanchorCustomRoute);
}

function shiftLongestRouteSegment(edge, axis, value) {
  const points = prepareCustomRoute(edge);
  if (!points) return;
  const currentValue = axis === 'x' ? edge.bend || 0 : edge.verticalBend || 0;
  const delta = Number(value) - currentValue;
  if (!delta) return;
  const desiredOrientation = axis === 'x' ? 'vertical' : 'horizontal';
  let selectedSegment = null;
  for (let index = 0; index < points.length - 1; index += 1) {
    const start = points[index];
    const end = points[index + 1];
    const orientation = start.y === end.y ? 'horizontal' : 'vertical';
    const length = Math.abs(start.x - end.x) + Math.abs(start.y - end.y);
    if (orientation === desiredOrientation && (!selectedSegment || length > selectedSegment.length)) selectedSegment = { index, length };
  }
  if (selectedSegment) {
    const nextPoints = points.map((point) => ({ ...point }));
    const startIndex = selectedSegment.index;
    const endIndex = startIndex + 1;
    if (startIndex > 0) nextPoints[startIndex][axis] += delta;
    if (endIndex < nextPoints.length - 1) nextPoints[endIndex][axis] += delta;
    edge.routePoints = nextPoints.slice(1, -1).map((point) => ({ ...point }));
  }
  if (axis === 'x') edge.bend = Number(value);
  else edge.verticalBend = Number(value);
}

function startSegmentDrag(event, edgeId, segmentIndex) {
  const edge = findEdge(edgeId);
  const points = edge && prepareCustomRoute(edge);
  const index = Number(segmentIndex);
  if (!edge || !points || !points[index] || !points[index + 1]) return;
  state.selection = { kind: 'edge', id: edge.id };
  const start = points[index];
  const end = points[index + 1];
  const axis = start.y === end.y ? 'y' : 'x';
  const point = canvasPoint(event);
  dragState = { type: 'segment', edge, points, segmentIndex: index, axis, startCoordinate: point[axis], originBend: edge.bend || 0, originVerticalBend: edge.verticalBend || 0 };
  event.preventDefault();
  event.stopPropagation();
  renderInspector();
}

function startBendDrag(event, edgeId, pointIndex) {
  const edge = findEdge(edgeId);
  const points = edge && prepareCustomRoute(edge);
  const index = Number(pointIndex);
  if (!edge || !points || !points[index] || index <= 0 || index >= points.length - 1) return;
  state.selection = { kind: 'edge', id: edge.id };
  const point = canvasPoint(event);
  const previous = points[index - 1];
  const current = points[index];
  const axis = previous.y === current.y ? 'x' : 'y';
  dragState = { type: 'bend', edge, points, pointIndex: index, axis, startCoordinate: point[axis], originCoordinate: current[axis], originBend: edge.bend || 0, originVerticalBend: edge.verticalBend || 0 };
  event.preventDefault();
  event.stopPropagation();
  renderInspector();
}

function moveSegmentPoints(points, segmentIndex, axis, delta) {
  const startIndex = segmentIndex;
  const endIndex = segmentIndex + 1;
  const start = points[startIndex];
  const end = points[endIndex];
  const nextValue = start[axis] + delta;
  const startsAtPort = startIndex === 0;
  const endsAtPort = endIndex === points.length - 1;
  const before = points.slice(0, startIndex);
  const after = points.slice(endIndex + 1);
  let replacement;
  if (startsAtPort && endsAtPort) {
    const startStub = { ...start, x: start.x + PORT_STUB };
    const startBridge = { ...startStub, [axis]: nextValue };
    const endStub = { ...end, x: end.x - PORT_STUB };
    const endBridge = { ...endStub, [axis]: nextValue };
    replacement = [start, startStub, startBridge, endBridge, endStub, end];
  } else if (startsAtPort) {
    const startStub = { ...start, x: start.x + PORT_STUB };
    const startBridge = { ...startStub, [axis]: nextValue };
    const movedEnd = { ...end, [axis]: nextValue };
    if (axis === 'y' && delta !== 0 && movedEnd.x === startStub.x) {
      const detourX = startStub.x + PORT_STUB;
      replacement = [start, startStub, { x: detourX, y: start.y }, { x: detourX, y: nextValue }, movedEnd];
    } else {
      replacement = [start, startStub, startBridge, movedEnd];
    }
  } else if (endsAtPort) {
    const endStub = { ...end, x: end.x - PORT_STUB };
    const endBridge = { ...endStub, [axis]: nextValue };
    const movedStart = { ...start, [axis]: nextValue };
    if (axis === 'y' && delta !== 0 && movedStart.x === endStub.x) {
      const detourX = endStub.x - PORT_STUB;
      replacement = [movedStart, { x: detourX, y: nextValue }, { x: detourX, y: end.y }, endStub, end];
    } else {
      replacement = [movedStart, endBridge, endStub, end];
    }
  } else {
    replacement = [{ ...start, [axis]: nextValue }, { ...end, [axis]: nextValue }];
  }
  return simplifyRoute([...before, ...replacement, ...after]);
}

function moveBendPoint(points, pointIndex, axis, delta) {
  const nextPoints = points.map((point) => ({ ...point }));
  const current = nextPoints[pointIndex];
  current[axis] += delta;
  if (pointIndex + 1 === nextPoints.length - 1) {
    // A bend moved next to a fixed port needs a new elbow, not a diagonal.
    const end = nextPoints[pointIndex + 1];
    const bridge = { ...end, [axis]: current[axis] };
    nextPoints.splice(pointIndex + 1, 0, bridge);
  } else {
    nextPoints[pointIndex + 1][axis] = current[axis];
  }
  return simplifyRoute(nextPoints);
}

function insertRouteBend(event, edgeId) {
  const edge = findEdge(edgeId);
  const points = edge && routeCache.get(edge.id);
  if (!edge || !points || points.length < 2) return;
  const point = canvasPoint(event);
  let closest = { index: 0, distance: Infinity };
  for (let index = 0; index < points.length - 1; index += 1) {
    const start = points[index];
    const end = points[index + 1];
    const horizontal = start.y === end.y;
    const projected = horizontal
      ? { x: Math.max(Math.min(point.x, Math.max(start.x, end.x)), Math.min(start.x, end.x)), y: start.y }
      : { x: start.x, y: Math.max(Math.min(point.y, Math.max(start.y, end.y)), Math.min(start.y, end.y)) };
    const distance = Math.hypot(point.x - projected.x, point.y - projected.y);
    if (distance < closest.distance) closest = { index, distance };
  }
  const index = closest.index;
  const start = points[index];
  const end = points[index + 1];
  const horizontal = start.y === end.y;
  const padding = 20;
  if (horizontal) {
    const x = Math.max(Math.min(point.x, Math.max(start.x, end.x) - padding), Math.min(start.x, end.x) + padding);
    const y = Math.abs(point.y - start.y) < 12 ? point.y + 64 : point.y;
    const replacement = [start, { x, y: start.y }, { x, y }, { x: end.x, y }, end];
    edge.routePoints = [...points.slice(0, index), ...replacement.slice(0, -1), ...points.slice(index + 1)].slice(1, -1).map((item) => ({ ...item }));
  } else {
    const y = Math.max(Math.min(point.y, Math.max(start.y, end.y) - padding), Math.min(start.y, end.y) + padding);
    const x = Math.abs(point.x - start.x) < 12 ? point.x + 64 : point.x;
    const replacement = [start, { x: start.x, y }, { x, y }, { x, y: end.y }, end];
    edge.routePoints = [...points.slice(0, index), ...replacement.slice(0, -1), ...points.slice(index + 1)].slice(1, -1).map((item) => ({ ...item }));
  }
  state.selection = { kind: 'edge', id: edge.id };
  markChanged();
  renderAll();
  showToast('Connector bend added');
}

function handlePointerMove(event) {
  const point = canvasPoint(event);
  document.querySelector('#cursor-x').textContent = Math.round(point.x);
  document.querySelector('#cursor-y').textContent = Math.round(point.y);
  if (connectState) {
    connectState.point = point;
    updateConnectTarget(event.clientX, event.clientY);
    renderConnections();
    return;
  }
  if (!dragState) return;
  if (dragState.type === 'node') {
    dragState.node.x = Math.max(12, Math.round(dragState.originX + (event.clientX - dragState.startX) / state.zoom));
    dragState.node.y = Math.max(70, Math.round(dragState.originY + (event.clientY - dragState.startY) / state.zoom));
    dragState.element.style.left = `${dragState.node.x}px`;
    dragState.element.style.top = `${dragState.node.y}px`;
    reanchorConnectedRoutes(dragState.node.id);
    renderConnections();
    markChanged();
  } else if (dragState.type === 'segment') {
    const delta = point[dragState.axis] - dragState.startCoordinate;
    const nextPoints = moveSegmentPoints(dragState.points, dragState.segmentIndex, dragState.axis, delta);
    dragState.edge.routePoints = nextPoints.slice(1, -1).map((item) => ({ ...item }));
    if (dragState.axis === 'x') {
      dragState.edge.bend = Math.round(dragState.originBend + delta);
      const routeValue = document.querySelector('#route-value');
      if (routeValue) routeValue.textContent = `${dragState.edge.bend}px`;
      const routeInput = document.querySelector('#route-input');
      if (routeInput) routeInput.value = dragState.edge.bend;
    } else {
      dragState.edge.verticalBend = Math.round(dragState.originVerticalBend + delta);
      const verticalRouteValue = document.querySelector('#vertical-route-value');
      if (verticalRouteValue) verticalRouteValue.textContent = `${dragState.edge.verticalBend}px`;
      const verticalRouteInput = document.querySelector('#vertical-route-input');
      if (verticalRouteInput) verticalRouteInput.value = dragState.edge.verticalBend;
    }
    renderConnections();
    markChanged();
  } else if (dragState.type === 'bend') {
    const delta = point[dragState.axis] - dragState.startCoordinate;
    const nextPoints = moveBendPoint(dragState.points, dragState.pointIndex, dragState.axis, delta);
    dragState.edge.routePoints = nextPoints.slice(1, -1).map((item) => ({ ...item }));
    if (dragState.axis === 'x') {
      dragState.edge.bend = Math.round(dragState.originBend + delta);
      const routeValue = document.querySelector('#route-value');
      if (routeValue) routeValue.textContent = `${dragState.edge.bend}px`;
      const routeInput = document.querySelector('#route-input');
      if (routeInput) routeInput.value = dragState.edge.bend;
    } else {
      dragState.edge.verticalBend = Math.round(dragState.originVerticalBend + delta);
      const verticalRouteValue = document.querySelector('#vertical-route-value');
      if (verticalRouteValue) verticalRouteValue.textContent = `${dragState.edge.verticalBend}px`;
      const verticalRouteInput = document.querySelector('#vertical-route-input');
      if (verticalRouteInput) verticalRouteInput.value = dragState.edge.verticalBend;
    }
    renderConnections();
    markChanged();
  } else if (dragState.type === 'pan') {
    state.pan.x = dragState.originX + event.clientX - dragState.startX;
    state.pan.y = dragState.originY + event.clientY - dragState.startY;
    applyGraphTransform();
  }
}

function handlePointerUp(event) {
  if (connectState) {
    finishConnection(event);
    return;
  }
  if (!dragState) return;
  if (dragState.type === 'node') {
    dragState.element?.classList.remove('dragging');
  }
  if (dragState.type === 'pan') viewport.style.cursor = 'grab';
  dragState = null;
}

// Canvas and graph interaction.
nodeLayer.addEventListener('pointerdown', (event) => {
  const port = event.target.closest('.port');
  if (port?.dataset.portDirection === 'output') {
    beginConnection(event, port);
    renderConnections();
    return;
  }
  const deleteButton = event.target.closest('.node-delete');
  if (deleteButton) {
    const node = deleteButton.closest('.node');
    if (node) { setSelection('node', node.dataset.nodeId); removeSelection(); }
    event.stopPropagation();
    return;
  }
  const header = event.target.closest('.node-header');
  if (header) {
    startNodeDrag(event, header.closest('.node'));
    return;
  }
  const nodeElement = event.target.closest('.node');
  if (nodeElement) setSelection('node', nodeElement.dataset.nodeId);
});

connections.addEventListener('pointerdown', (event) => {
  const segment = event.target.closest('[data-segment-handle]');
  if (segment) {
    const [edgeId, segmentIndex] = segment.dataset.segmentHandle.split(':');
    startSegmentDrag(event, edgeId, segmentIndex);
    return;
  }
  const bend = event.target.closest('[data-bend-handle]');
  if (bend) {
    const [edgeId, pointIndex] = bend.dataset.bendHandle.split(':');
    startBendDrag(event, edgeId, pointIndex);
    return;
  }
  const path = event.target.closest('[data-edge-id]');
  if (path) {
    setSelection('edge', path.dataset.edgeId);
    event.stopPropagation();
  }
});
connections.addEventListener('dblclick', (event) => {
  const path = event.target.closest('[data-edge-id]');
  if (path) {
    insertRouteBend(event, path.dataset.edgeId);
    event.preventDefault();
  }
});

viewport.addEventListener('pointerdown', (event) => {
  // Middle mouse is a temporary hand tool, independent of the selected mode.
  if (event.button === 1) {
    startPan(event, true);
    event.stopPropagation();
    return;
  }
  const clickedGraphItem = event.target.closest('.node, [data-edge-id], .port');
  if (!clickedGraphItem) {
    if (state.interactionMode === 'pan') startPan(event);
    else setSelection(null, null);
  }
}, true);
viewport.addEventListener('wheel', (event) => {
  event.preventDefault();
  const nextZoom = Math.max(.65, Math.min(1.45, state.zoom + (event.deltaY > 0 ? -.06 : .06)));
  state.zoom = Math.round(nextZoom * 100) / 100;
  applyGraphTransform();
  renderConnections();
}, { passive: false });
window.addEventListener('pointermove', handlePointerMove);
window.addEventListener('pointerup', handlePointerUp);

// Inputs in nodes and the inspector.
nodeLayer.addEventListener('change', (event) => {
  const input = event.target.closest('[data-field]');
  if (!input) return;
  const node = findNode(input.closest('.node')?.dataset.nodeId);
  if (!node) return;
  updateNodeField(node, input.dataset.field, input.value);
  const selected = selectedNode();
  if (selected?.id === node.id) renderInspector();
});

inspector.addEventListener('change', (event) => {
  const node = selectedNode();
  const fieldInput = event.target.closest('[data-field]');
  if (node && fieldInput) {
    updateNodeField(node, fieldInput.dataset.field, fieldInput.value);
    renderNodes();
    return;
  }
  if (node && event.target.matches('[data-node-title]')) {
    node.title = event.target.value.trim() || blockDefinitions[node.type].label;
    markChanged();
    renderNodes();
    return;
  }
  const routeInput = event.target.closest('[data-route-field]');
  if (routeInput) {
    const edge = selectedEdge();
    if (edge) { shiftLongestRouteSegment(edge, 'x', Number(routeInput.value)); markChanged(); renderConnections(); renderInspector(); }
    return;
  }
  const verticalRouteInput = event.target.closest('[data-vertical-route-field]');
  if (verticalRouteInput) {
    const edge = selectedEdge();
    if (edge) { shiftLongestRouteSegment(edge, 'y', Number(verticalRouteInput.value)); markChanged(); renderConnections(); renderInspector(); }
  }
});
inspector.addEventListener('input', (event) => {
  const node = selectedNode();
  const fieldInput = event.target.closest('[data-field]');
  if (node && fieldInput) {
    updateNodeField(node, fieldInput.dataset.field, fieldInput.value);
    renderNodes();
    return;
  }
  const routeInput = event.target.closest('[data-route-field]');
  if (routeInput) {
    const edge = selectedEdge();
    if (edge) { shiftLongestRouteSegment(edge, 'x', Number(routeInput.value)); document.querySelector('#route-value').textContent = `${edge.bend}px`; renderConnections(); markChanged(); }
    return;
  }
  const verticalRouteInput = event.target.closest('[data-vertical-route-field]');
  if (verticalRouteInput) {
    const edge = selectedEdge();
    if (edge) { shiftLongestRouteSegment(edge, 'y', Number(verticalRouteInput.value)); document.querySelector('#vertical-route-value').textContent = `${edge.verticalBend}px`; renderConnections(); markChanged(); }
  }
});
inspector.addEventListener('click', (event) => {
  if (event.target.closest('#delete-selected')) removeSelection();
});

// Library actions and drag/drop.
document.querySelectorAll('.library-card').forEach((card) => {
  card.addEventListener('click', () => addBlock(card.dataset.type));
  card.addEventListener('dragstart', (event) => {
    event.dataTransfer.setData('application/x-signal-block', card.dataset.type);
    event.dataTransfer.effectAllowed = 'copy';
  });
});
viewport.addEventListener('dragover', (event) => { event.preventDefault(); event.dataTransfer.dropEffect = 'copy'; });
viewport.addEventListener('drop', (event) => {
  event.preventDefault();
  const type = event.dataTransfer.getData('application/x-signal-block');
  if (type && blockDefinitions[type]) addBlock(type, canvasPoint(event));
});
document.querySelector('#block-search').addEventListener('input', (event) => {
  const query = event.target.value.toLowerCase().trim();
  let visible = 0;
  document.querySelectorAll('.library-card').forEach((card) => {
    const match = card.textContent.toLowerCase().includes(query);
    card.style.display = match ? '' : 'none';
    if (match) visible += 1;
  });
  document.querySelector('#library-count').textContent = String(visible).padStart(2, '0');
});

// Toolbar actions.
document.querySelector('#zoom-in').addEventListener('click', () => { state.zoom = Math.min(1.45, Math.round((state.zoom + .1) * 100) / 100); applyGraphTransform(); renderConnections(); });
document.querySelector('#zoom-out').addEventListener('click', () => { state.zoom = Math.max(.65, Math.round((state.zoom - .1) * 100) / 100); applyGraphTransform(); renderConnections(); });
document.querySelector('#zoom-reset').addEventListener('click', () => { state.zoom = 1; state.pan = { x: 0, y: 0 }; applyGraphTransform(); renderConnections(); });
document.querySelector('#select-mode').addEventListener('click', () => setMode('select'));
document.querySelector('#pan-mode').addEventListener('click', () => setMode('pan'));
document.querySelector('#run-button').addEventListener('click', () => {
  state.running = !state.running;
  const button = document.querySelector('#run-button');
  button.classList.toggle('is-running', state.running);
  button.innerHTML = state.running ? `${icons.stop}<span>Stop graph</span>` : `${icons.play}<span>Run graph</span>`;
  renderNodes();
  renderConnections();
  showToast(state.running ? 'Graph is running' : 'Graph stopped');
});
document.querySelector('#save-button').addEventListener('click', saveGraph);
document.querySelector('#export-button').addEventListener('click', () => {
  const file = new Blob([JSON.stringify({ nodes: state.nodes, edges: state.edges }, null, 2)], { type: 'application/json' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(file);
  link.download = 'signal-canvas-graph.json';
  link.click();
  URL.revokeObjectURL(link.href);
  showToast('Graph exported');
});
document.querySelector('#reset-button').addEventListener('click', () => {
  state = initialState();
  localStorage.removeItem('signal-canvas-graph');
  renderAll();
  showToast('Example graph reset');
});
window.addEventListener('keydown', (event) => {
  if ((event.key === 'Delete' || event.key === 'Backspace') && !['INPUT', 'SELECT', 'TEXTAREA'].includes(document.activeElement?.tagName)) {
    event.preventDefault();
    removeSelection();
  }
  if (event.key === 'Escape') {
    connectState = null;
    dragState = null;
    document.body.style.cursor = '';
    renderConnections();
  }
});
window.addEventListener('resize', () => { renderConnections(); resizeGpuCanvas(); });

// Keep the graph in the browser between visits when possible.
function restoreGraph() {
  try {
    const saved = JSON.parse(localStorage.getItem('signal-canvas-graph'));
    if (saved?.nodes?.length && Array.isArray(saved.edges)) {
      state.nodes = saved.nodes;
      state.edges = saved.edges;
      state.selection = { kind: 'node', id: saved.nodes[0].id };
    }
  } catch { /* start with the example graph */ }
}

function resizeGpuCanvas() {
  if (!gpuRenderer) return;
  const rect = viewport.getBoundingClientRect();
  const ratio = Math.min(window.devicePixelRatio || 1, 2);
  const width = Math.max(1, Math.floor(rect.width * ratio));
  const height = Math.max(1, Math.floor(rect.height * ratio));
  if (gpuRenderer.canvas.width !== width || gpuRenderer.canvas.height !== height) {
    gpuRenderer.canvas.width = width;
    gpuRenderer.canvas.height = height;
    drawGpuBackground();
  }
}

function drawGpuBackground() {
  if (!gpuRenderer) return;
  try {
    gpuRenderer.pipeline.withColorAttachment({ view: gpuRenderer.context, clearValue: [0.025, 0.047, 0.08, 1], loadOp: 'clear', storeOp: 'store' }).draw(3);
  } catch (error) {
    console.warn('TypeGPU draw skipped:', error);
  }
}

async function initGpuBackground() {
  const canvas = document.querySelector('#gpu-canvas');
  if (!navigator.gpu) {
    document.querySelector('#renderer-status').textContent = 'css fallback';
    document.querySelector('#render-engine').textContent = 'CSS fallback / TypeGPU ready';
    return;
  }
  try {
    const root = await tgpu.init({ unstable_names: 'strict' });
    const context = root.configureContext({ canvas, alphaMode: 'opaque' });
    const fragment = tgpu.fragmentFn({ in: { position: d.builtin.position }, out: d.vec4f })`{
      let p = position.xy;
      let normalized = p / vec2f(960.0, 720.0);
      let fine = step(0.965, fract(p.x / 24.0)) + step(0.965, fract(p.y / 24.0));
      let major = step(0.978, fract(p.x / 120.0)) + step(0.978, fract(p.y / 120.0));
      let glow = max(0.0, 1.0 - distance(normalized, vec2f(0.72, 0.35)) * 1.1);
      let base = vec3f(0.025, 0.047, 0.08) + vec3f(0.005, 0.012, 0.022) * glow;
      let color = base + vec3f(0.018, 0.035, 0.055) * fine + vec3f(0.022, 0.045, 0.07) * major;
      return vec4f(color, 1.0);
    }`;
    const pipeline = root.createRenderPipeline({ vertex: common.fullScreenTriangle, fragment });
    gpuRenderer = { root, context, pipeline, canvas };
    document.querySelector('#renderer-status').textContent = 'webgpu active';
    document.querySelector('#render-engine').textContent = 'TypeGPU / WebGPU';
    resizeGpuCanvas();
    drawGpuBackground();
  } catch (error) {
    console.warn('TypeGPU unavailable, using CSS canvas:', error);
    document.querySelector('#renderer-status').textContent = 'css fallback';
    document.querySelector('#render-engine').textContent = 'CSS fallback / TypeGPU ready';
  }
}

restoreGraph();
renderAll();
initGpuBackground();
