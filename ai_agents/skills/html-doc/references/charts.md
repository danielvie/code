# Charts Reference

Use inline SVG inside `.chart` containers. Do not rely on external image files unless the chart is a screenshot or hardware pinout that cannot be represented clearly as SVG.

Favor charts as the primary explanation tool. Use prose to introduce and interpret charts, not to replace them. The first chapter should include overview charts before the document moves into detailed sections.

```html
<div class="chart" aria-label="Architecture data flow chart">
  <svg viewBox="0 0 1040 360" role="img">
    ...
  </svg>
</div>
```

## General Chart Style

- Wrap every SVG in `<div class="chart">`.
- Add a meaningful `aria-label` to the chart container.
- Add `role="img"` to SVG.
- Use `viewBox` coordinates, not fixed pixel `width` / `height`.
- Use `max-width: 100%; height: auto;` in CSS.
- Use `overflow-x: auto` on `.chart` so dense diagrams remain readable on small screens.
- Prefer boxes, arrows, lanes, and labels over decorative shapes.
- Use `rx="4"` or `rx="7"` for rounded rectangles.
- Use muted strokes and hierarchy-level fills.

## Standard SVG Classes

```css
.box {
  fill: #fff;
  stroke: #b8c6cd;
  stroke-width: 1.3;
  rx: 7;
}
.box-level-1,
.chart-level-1 { fill: rgba(54, 93, 168, 0.1); stroke: var(--level-1); }
.box-level-2,
.chart-level-2 { fill: rgba(22, 100, 90, 0.1); stroke: var(--level-2); }
.box-level-3,
.chart-level-3 { fill: rgba(91, 76, 148, 0.1); stroke: var(--level-3); }
.chart-muted { fill: rgba(96, 112, 122, 0.08); stroke: #9aaab2; }
.chart-warn { fill: #fff4df; stroke: #b57418; }
.arrow { stroke: #56666f; stroke-width: 1.5; fill: none; marker-end: url(#arrow); }
.dash { stroke-dasharray: 6 5; }
.label { fill: var(--ink); font-size: 13px; font-weight: 650; }
.small { fill: var(--muted); font-size: 12px; }
.lane { fill: rgba(96, 112, 122, 0.07); stroke: #d7e1e7; }
```

When in doubt:

- use `.label` for chart titles and lane names;
- use `.small` for captions, axis values, notes, and durations;
- use `.chart-level-1` for top-level section spans;
- use `.chart-level-2` for subsection spans;
- use `.chart-level-3` for sub-subsection spans;
- use `.chart-warn` for warnings;
- use `.chart-muted` for idle, calibration, unknown, or historical behavior.

## Arrow Marker Pattern

Define a marker inside each SVG if arrows are used. Use a unique id per SVG if multiple charts are present, or use a shared id only when it will not collide.

```html
<defs>
  <marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
    <path d="M0,0 L8,4 L0,8 Z" fill="#56666f"></path>
  </marker>
</defs>
```

Then:

```html
<path d="M 180 100 L 300 100" class="arrow"></path>
```

For non-arrow timelines, use the same stroke color but disable marker end:

```html
<line x1="120" y1="56" x2="980" y2="56" class="arrow" style="marker-end: none"></line>
```

## Chart Selection Rules

Choose the chart type before writing detailed prose:

- **Overview architecture chart**: first chapter; shows major actors/components and data/control flow.
- **Timing chart**: any latency, cadence, cycle, sampling, timeout, ordering, or before/after performance topic.
- **Sequence diagram**: request/response flow, message exchange, state transition, orchestration, retry path, or multi-actor logic.
- **Responsibility/data-flow chart**: ownership boundaries, pipelines, queue flow, file/module responsibilities.
- **Table only**: dense mappings after a chart, not as the primary explanation when logic or flow exists.

If the topic has actors and ordered steps, create a sequence diagram. If it has durations or deadlines, create a timing chart. If both apply, use both.

## Time Chart Rules

Time charts are one of the signature elements of this style. They should explain timing behavior visually, especially before/after performance changes, CPU core usage, signal generation, DMA sampling, BLE frame processing, or UI update cadence.

Always prefer a time chart when explaining:

- control-cycle execution;
- DMA vs blocking I/O;
- ISR/task interactions;
- Core 0 vs Core 1 responsibilities;
- BLE packet receive/decode/respond latency;
- generated digital signal patterns;
- sampling windows;
- dead-time, pulse width, or duty-cycle constraints;
- before/after timing comparisons.

Do not describe timing-sensitive behavior only in prose. Show the timeline visually with real units.

## Basic Time Chart Layout

1. Title at the top-left.
2. Horizontal time axis near the top or bottom.
3. Tick labels with real units, usually `us`, `ms`, or `s`.
4. One horizontal lane per actor:
   - `ADC HW`
   - `Core 0`
   - `Core 1`
   - `BLE`
   - `Web UI`
   - `Signal Output`
   - `NVS`
5. Rectangles represent execution intervals.
6. Narrow rectangles represent short parse/copy/publish work.
7. Dashed vertical lines represent control points, boundaries, interrupts, or snapshot moments.
8. Captions below the chart explain the design implication.

## Sequence Diagram Rules

Use sequence diagrams to make logic explicit across actors. Draw them as inline SVG with vertical lifelines and horizontal message arrows.

Use a sequence diagram when explaining:

- UI → API → service → database flows;
- browser ↔ firmware / BLE / protocol exchanges;
- task-to-task messages;
- request/response handling;
- retries, errors, acknowledgements, or timeouts;
- state transitions involving more than one actor.

Sequence diagram layout:

1. Actor names at the top.
2. One vertical dashed lifeline per actor.
3. Horizontal arrows for messages, commands, events, or callbacks.
4. Return/ack messages as dashed arrows.
5. Number steps when ordering matters.
6. Add a short implication caption below the diagram.

Example skeleton:

```html
<div class="chart" aria-label="Sequence diagram: command acknowledgement flow">
  <svg viewBox="0 0 1040 360" role="img">
    <defs>
      <marker id="seq-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
        <path d="M0,0 L8,4 L0,8 Z" fill="#56666f"></path>
      </marker>
    </defs>

    <text x="120" y="32" class="label">Web UI</text>
    <text x="440" y="32" class="label">Protocol</text>
    <text x="760" y="32" class="label">Firmware</text>

    <line x1="150" y1="54" x2="150" y2="300" class="dash" stroke="#9aaab2"></line>
    <line x1="470" y1="54" x2="470" y2="300" class="dash" stroke="#9aaab2"></line>
    <line x1="800" y1="54" x2="800" y2="300" class="dash" stroke="#9aaab2"></line>

    <path d="M 150 92 L 470 92" class="arrow" style="marker-end: url(#seq-arrow)"></path>
    <text x="236" y="84" class="small">1. encode command</text>

    <path d="M 470 150 L 800 150" class="arrow" style="marker-end: url(#seq-arrow)"></path>
    <text x="560" y="142" class="small">2. send packet</text>

    <path d="M 800 214 L 470 214" class="arrow dash" style="marker-end: url(#seq-arrow)"></path>
    <text x="568" y="206" class="small">3. acknowledgement</text>

    <path d="M 470 266 L 150 266" class="arrow dash" style="marker-end: url(#seq-arrow)"></path>
    <text x="236" y="258" class="small">4. update UI state</text>
  </svg>
</div>
```

## Coordinate Formula

Use deterministic coordinate mapping. Do not eyeball timing charts when the numbers matter.

Define:

- `axisStartX`: left coordinate of time axis, for example `120` or `150`.
- `axisEndX`: right coordinate of time axis, for example `980` or `930`.
- `tMin`: start time, usually `0`.
- `tMax`: end time, for example `280` for a `280 us` cycle.
- `plotWidth = axisEndX - axisStartX`.

Map time to x coordinate:

```text
x(t) = axisStartX + ((t - tMin) / (tMax - tMin)) * plotWidth
```

Map an interval `[t0, t1]` to a rectangle:

```text
rect.x = x(t0)
rect.width = x(t1) - x(t0)
```

For example, with `axisStartX = 150`, `axisEndX = 930`, `tMax = 282`:

```text
x(0)   = 150
x(75)  = 150 + (75 / 282) * 780 ≈ 357
x(94)  = 150 + (94 / 282) * 780 ≈ 410
x(169) = 150 + (169 / 282) * 780 ≈ 617
x(188) = 150 + (188 / 282) * 780 ≈ 670
x(263) = 150 + (263 / 282) * 780 ≈ 877
x(282) = 930
```

Use rounded values in SVG coordinates, but keep labels exact.

## Before/After Timing Chart Pattern

Use two stacked charts when comparing an old and new implementation.

Before chart:

- title: `Before — <old design>`;
- show blocking work as long sequential rectangles;
- show affected lanes, stale data, or delay as highlighted spans;
- include a warning caption beginning with `⚠`.

After chart:

- title: `After — <new design>`;
- show hardware/background activity on its own lane;
- show CPU work as short intervals;
- show snapshot/control boundary as a dashed line;
- include a success caption beginning with `✓`.

Example skeleton:

```html
<div class="chart" aria-label="After timeline: DMA acquisition">
  <svg viewBox="0 0 1040 240" role="img">
    <text x="36" y="24" class="label">After — DMA continuous</text>
    <line x1="120" y1="56" x2="980" y2="56" class="arrow" style="marker-end: none"></line>
    <text x="120" y="48" class="small">0 us</text>
    <text x="540" y="48" class="small">140</text>
    <text x="950" y="48" class="small">280 us</text>

    <text x="36" y="92" class="small">ADC HW</text>
    <rect x="120" y="74" width="200" height="24" rx="4" class="chart-level-1"></rect>

    <text x="36" y="128" class="small">Core 0</text>
    <rect x="310" y="110" width="36" height="24" rx="4" class="chart-level-2"></rect>

    <text x="36" y="168" class="small">Core 1</text>
    <rect x="120" y="156" width="620" height="24" rx="4" class="chart-level-3"></rect>
    <line x1="740" y1="148" x2="740" y2="188" stroke="#5b4c94" stroke-width="2" stroke-dasharray="6 5"></line>
    <text x="648" y="146" class="small">control point</text>

    <text x="120" y="218" class="small" fill="#1b4a32" font-weight="700">
      ✓ Continuous background work keeps fresh data available during the cycle.
    </text>
  </svg>
</div>
```

## Digital Signal Timing Chart Pattern

Use this chart type for generated GPIO or mode-bit patterns.

Rules:

- Use vertical shaded bands for each signal step or mode interval.
- Label each band with mode number and binary bits, for example `4 / 100`.
- Use one pair of y-levels per phase/channel:
  - `U1 HIGH`, `U1 LOW`
  - `U2 HIGH`, `U2 LOW`
  - `U3 HIGH`, `U3 LOW`
- Draw each signal with a thick `polyline`.
- Use hierarchy-level colors consistently when grouping or nesting signal details.
- Add a time axis below the waveforms.
- Label both absolute timestamps and interval durations.

Example skeleton:

```html
<div class="chart" aria-label="Generated digital signal chart">
  <svg viewBox="0 0 1040 450" role="img">
    <text x="22" y="30" class="label">alpha 0.4 generated pattern, T = 282 us</text>
    <text x="22" y="52" class="small">Mode bits are d6 d5 d4.</text>

    <!-- shaded mode intervals -->
    <rect x="150" y="74" width="207" height="310" class="chart-level-2"></rect>
    <rect x="357" y="74" width="53" height="310" class="chart-level-3"></rect>

    <!-- mode labels -->
    <text x="230" y="96" class="small">4 / 100</text>
    <text x="368" y="96" class="small">6 / 110</text>

    <!-- signal grid lines -->
    <line x1="150" y1="116" x2="930" y2="116" stroke="#b8c6cd"></line>
    <line x1="150" y1="156" x2="930" y2="156" stroke="#b8c6cd"></line>

    <!-- waveform lane -->
    <text x="54" y="122" class="label">U1 HIGH</text>
    <text x="68" y="160" class="small">U1 LOW</text>
    <polyline
      points="150,116 410,116 410,156 877,156 877,116 930,116"
      fill="none"
      stroke="#365da8"
      stroke-width="4"
      stroke-linejoin="round">
    </polyline>

    <!-- time axis -->
    <line x1="150" y1="386" x2="930" y2="386" stroke="#56666f" stroke-width="1.4"></line>
    <text x="146" y="414" class="small">0</text>
    <text x="908" y="414" class="small">282 us</text>
  </svg>
</div>
```

## Time Chart Validation Checklist

Before finalizing a timing chart, verify:

- [ ] every duration shown in the visual matches the numeric labels;
- [ ] every lane is labeled;
- [ ] colors communicate hierarchy consistently: level 1 blue, level 2 green, level 3 purple;
- [ ] critical boundaries are visibly marked;
- [ ] the chart has a clear implication caption;
- [ ] long labels do not overlap at desktop width;
- [ ] the chart remains horizontally scrollable on small screens;
- [ ] the chart can be understood without reading the entire chapter.
