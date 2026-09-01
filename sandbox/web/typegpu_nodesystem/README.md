# Signal Canvas

A small TypeGPU/WebGPU node-system study inspired by Simulink. It intentionally uses plain HTML, CSS, SVG, and TypeGPU rather than React Flow so the canvas interaction model stays under application control.

## Run

```bash
npm install
npm run dev
```

Open the Vite URL in a WebGPU-capable browser.

## Interactions

- Click a block in the library to add it, or drag it onto the canvas.
- Drag a node header to move a block.
- Edit values inline or in the inspector.
- Drag an orange/purple output port to an input port to connect blocks.
- Select a connection and drag its segment dots or corner handles to route the line. Moving a segment next to a port creates a compensating elbow instead of a diagonal, while preserving a minimum straight port lead. Moving a connected block re-anchors custom routes without breaking their orthogonal angles; connectors have a forgiving 20px hit area so they are easy to select. The router keeps paths orthogonal, avoids blocks, and tries to avoid crossing existing connections. Adjust horizontal or vertical routing in the inspector; connectors use consistent rounded elbows.
- Select a block or line and press `Delete` to remove it.
- Use the pan tool, middle-click and drag, zoom controls, or scroll over the canvas to navigate.
- `Save` stores the graph in local browser storage; `Export` downloads JSON.

The GPU layer uses `typegpu` to initialize WebGPU and draw the canvas background. A CSS fallback keeps the editor usable when WebGPU is unavailable.
