# Charts Guide

Use inline SVG inside `.chart` containers. Favor charts as the primary explanation tool.

## Chart Selection Rules

Choose the chart type before writing prose:

- **Overview architecture chart**: first chapter; shows major actors and data/control flow.
- **Timing chart**: latency, cadence, cycle, sampling, timeout, ordering, or before/after performance.
- **Sequence diagram**: request/response, message exchange, state transition, orchestration, retry, multi-actor logic.
- **Responsibility/data-flow chart**: ownership boundaries, pipelines, queue flow, module responsibilities.
- **Table only**: dense mappings after a chart, not as primary explanation when logic or flow exists.

If the topic has actors and ordered steps, use a sequence diagram. If it has durations or deadlines, use a timing chart. If both apply, use both.

## General Chart Rules

- Wrap every SVG in `<div class="chart" aria-label="...">`.
- Add `role="img"` to SVG.
- Use `viewBox` coordinates, not fixed pixel width/height.
- Use hierarchy-level fills: `.chart-level-1` (blue), `.chart-level-2` (green), `.chart-level-3` (purple).
- Use `.chart-muted` for idle/historical, `.chart-warn` for warnings.
- Use `.label` for titles, `.small` for captions and axis values.

## Arrow Marker

Define per SVG:

```html
<defs>
  <marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
    <path d="M0,0 L8,4 L0,8 Z" fill="#56666f"></path>
  </marker>
</defs>
```

Use unique marker ids when multiple charts exist on the same page.

## Coordinate Formula

For timing charts, map time to x deterministically:

```
axisStartX = 150, axisEndX = 930, tMin = 0, tMax = (total time)
plotWidth = axisEndX - axisStartX

x(t) = axisStartX + ((t - tMin) / (tMax - tMin)) * plotWidth
rect.x = x(t0), rect.width = x(t1) - x(t0)
```

Use rounded values in SVG coordinates, keep labels exact.

## Timing Chart Example

```html
<div class="chart" aria-label="Control cycle timing">
  <svg viewBox="0 0 1040 200" role="img">
    <defs>
      <marker id="tc-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
        <path d="M0,0 L8,4 L0,8 Z" fill="#56666f"></path>
      </marker>
    </defs>
    <text x="36" y="24" class="label">Control Cycle — 280 us</text>
    <line x1="150" y1="56" x2="930" y2="56" stroke="#56666f" stroke-width="1.4" style="marker-end:none"></line>
    <text x="150" y="48" class="small">0 us</text>
    <text x="920" y="48" class="small">280 us</text>

    <text x="36" y="92" class="small">Core 0</text>
    <rect x="150" y="76" width="260" height="24" rx="4" class="chart-level-1"></rect>

    <text x="36" y="132" class="small">Core 1</text>
    <rect x="150" y="116" width="620" height="24" rx="4" class="chart-level-2"></rect>

    <text x="150" y="178" class="small">Core 0 handles acquisition; Core 1 runs the control loop concurrently.</text>
  </svg>
</div>
```

## Sequence Diagram Example

```html
<div class="chart" aria-label="Sequence: command flow">
  <svg viewBox="0 0 1040 280" role="img">
    <defs>
      <marker id="seq-arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
        <path d="M0,0 L8,4 L0,8 Z" fill="#56666f"></path>
      </marker>
    </defs>

    <text x="150" y="32" class="label">Client</text>
    <text x="500" y="32" class="label">Server</text>
    <text x="820" y="32" class="label">Database</text>

    <line x1="180" y1="50" x2="180" y2="240" class="dash" stroke="#9aaab2"></line>
    <line x1="530" y1="50" x2="530" y2="240" class="dash" stroke="#9aaab2"></line>
    <line x1="850" y1="50" x2="850" y2="240" class="dash" stroke="#9aaab2"></line>

    <path d="M 180 80 L 530 80" class="arrow" style="marker-end:url(#seq-arrow)"></path>
    <text x="300" y="72" class="small">1. request</text>

    <path d="M 530 120 L 850 120" class="arrow" style="marker-end:url(#seq-arrow)"></path>
    <text x="640" y="112" class="small">2. query</text>

    <path d="M 850 160 L 530 160" class="arrow dash" style="marker-end:url(#seq-arrow)"></path>
    <text x="640" y="152" class="small">3. result</text>

    <path d="M 530 200 L 180 200" class="arrow dash" style="marker-end:url(#seq-arrow)"></path>
    <text x="300" y="192" class="small">4. response</text>
  </svg>
</div>
```

## Validation Checklist

Before finalizing a chart:

- [ ] Durations match numeric labels.
- [ ] Every lane is labeled.
- [ ] Colors follow hierarchy: level-1 blue, level-2 green, level-3 purple.
- [ ] Critical boundaries are marked.
- [ ] Chart has an implication caption.
- [ ] No overlapping labels at desktop width.
