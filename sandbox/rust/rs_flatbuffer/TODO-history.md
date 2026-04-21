# Header Refactor Tasks

## Components
- [x] Create `frontend/src/components/Header.tsx`
- [x] Move telemetry grid (Target, Pos.X, Deg.θ) from sidebar into `Header`
- [x] Remove old telemetry grid markup from `App.tsx` sidebar

## Layout & Styling
- [x] Place `Header` inside `.viewport` at top
- [x] Style header to match design spec: position (344, 11), size 816×78px
- [x] Update `index.css` with `.viewport-header` styles (absolute positioning, flex row, background, border)
- [x] Ensure header overlays viewport content without shifting canvas

## Integration
- [x] Import and render `Header` in `App.tsx` inside `<main className="viewport">`
- [x] Pass `state` and `targetX` props to `Header`
- [x] Verify chart remains in sidebar footer
- [x] Verify simulation canvas fills remaining viewport space