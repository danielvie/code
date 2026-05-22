# HTML Doc

Use this skill when creating standalone HTML documentation that should reproduce the visual and structural style of `docs/project_architecture_guide.html`. The output should feel like a technical study artifact: readable, visual, self-contained, navigable, and suitable for explaining architecture, timing, firmware/web/protocol interactions, and implementation tradeoffs.

## Goal

Create a single-file HTML document with:

- a clean technical report layout;
- a light engineering-paper background;
- collapsible chapters with `show` / `hide` affordances;
- an index with chapter links and short explanations;
- inline SVG charts, especially timing charts;
- tables for responsibilities, files, interfaces, and risks;
- dark code blocks with syntax coloring;
- minimal JavaScript only for expand/collapse controls and code highlighting.

The document should be understandable as a study report, not just a code reference.

---

## Document Structure Rules

Use this high-level structure:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Project / Feature - Study Guide</title>
    <style>
      /* all CSS inline */
    </style>
  </head>
  <body>
    <main>
      <section class="hero">...</section>
      <div class="toolbar">...</div>
      <nav class="index">...</nav>
      <details class="chapter firmware" id="...">...</details>
      <details class="chapter web" id="...">...</details>
      <details class="chapter signal" id="...">...</details>
      <details class="chapter config" id="...">...</details>
      <details class="chapter proto" id="...">...</details>
    </main>
    <script>
      /* small inline JS only */
    </script>
  </body>
</html>
```

### Required Sections

1. **Hero section**
   - Use a large title and a short lede paragraph.
   - Add a metadata card on the right with scope, source commits, date, or target subsystem.
   - Add chips for major topics, for example `Firmware`, `BLE`, `Signal Loop`, `Web UI`, `Config`, `Timing`.

2. **Toolbar**
   - Include `Open all` and `Close all` buttons.
   - Buttons should toggle all `details.chapter` sections.

3. **Index**
   - Use a visible `Documentation Index` panel near the top.
   - Use a responsive grid of links.
   - Each link points to a chapter id.
   - Each link includes:
     - the chapter title;
     - a small circular `?` hint badge;
     - `data-tip="..."` with a one-sentence explanation.

4. **Chapters**
   - Every major section must be a `<details class="chapter ..." id="...">` block.
   - Use `<summary><h2>Section Title</h2></summary>`.
   - The visible body must be wrapped in `<div class="content">`.
   - Use semantic class tags to color the left border:
     - `firmware`
     - `web`
     - `signal`
     - `config`
     - `proto`
   - Important chapters may start open with the `open` attribute, but avoid opening everything by default.

5. **Study Content Pattern**
   - Start each chapter with a short explanation of what changed or what the subsystem does.
   - Prefer concrete evidence: file names, functions, commits, timing values, queues, state machines, message schemas.
   - Follow with one or more visuals.
   - End with a concise table, checklist, or callout explaining the design consequences.

---

## Visual Design Rules

### Layout

- Use a centered `<main>` with `max-width: 1180px`.
- Use comfortable page padding: `32px 20px 70px` desktop, smaller on mobile.
- Use responsive grids:
  - hero: two columns, main content wider than metadata;
  - index: three columns on desktop;
  - cards: three columns on desktop;
  - all collapse to one column below `900px`.

### Typography

Use system UI fonts with a technical but readable tone:

```css
font-family: "Segoe UI", Candara, Tahoma, sans-serif;
line-height: 1.55;
```

Heading rules:

- `h1`: large, responsive, `clamp(30px, 5vw, 54px)`.
- `h2`: around `24px`.
- `h3`: around `18px`, with top margin.
- `h4`: uppercase, muted, letter-spaced, useful for category labels.

### Background

Use a subtle grid background that evokes engineering paper:

```css
body {
  background:
    linear-gradient(90deg, rgba(24, 32, 37, 0.035) 1px, transparent 1px) 0 0 / 28px 28px,
    linear-gradient(rgba(24, 32, 37, 0.025) 1px, transparent 1px) 0 0 / 28px 28px,
    var(--paper);
}
```

Do not use heavy gradients, dark page backgrounds, or decorative illustrations. The style should be restrained and analytical.

---

## Color Schema

Use CSS variables at `:root`. Keep the palette muted and technical.

```css
:root {
  --ink: #182025;
  --muted: #60707a;
  --line: #d7e1e7;
  --paper: #fbfcfb;
  --panel: #f3f7f5;
  --firmware: #16645a;
  --web: #8a4f10;
  --proto: #9b2f35;
  --signal: #365da8;
  --config: #5b4c94;
  --code: #101820;
}
```

### Semantic Meaning

Use colors consistently:

| Token | Meaning | Typical Use |
| --- | --- | --- |
| `--ink` | primary text | headings, normal text |
| `--muted` | secondary text | captions, descriptions, axis labels |
| `--line` | borders | panels, cards, tables, chart containers |
| `--paper` | page base | body background |
| `--panel` | light panel fill | index links, cards |
| `--firmware` | firmware / embedded logic | firmware chapters, hardware/DMA chart blocks |
| `--web` | web UI / browser | web chapters, UI chart blocks |
| `--proto` | protocol / schema / message format | protocol chapters, BLE payload flow |
| `--signal` | signal loop / timing-critical path | signal chapters, warnings, timing-critical blocks |
| `--config` | configuration / persistence | config chapters, NVS/settings blocks |
| `--code` | code block background | `pre` blocks |

Important: the original file has some class/color crossovers in chart classes. When creating new docs, prefer semantic consistency: firmware = green, web = brown, protocol = red, signal = blue, config = purple. Use warning red/brown only for hazards or constraints.

---

## Index and Show/Hide Sections

### Index Pattern

Use this structure:

```html
<nav class="index" aria-label="Documentation index">
  <h2>Documentation Index</h2>
  <div class="index-grid">
    <a href="#architecture">
      Architecture Overview
      <span class="hint" tabindex="0" data-tip="Shows the main components and how data moves between them.">?</span>
    </a>
    <a href="#timing">
      Timing Model
      <span class="hint" tabindex="0" data-tip="Explains cycle time, lanes, and critical timing constraints.">?</span>
    </a>
  </div>
</nav>
```

Rules:

- Keep index labels short.
- Use hints to explain why a section matters, not to repeat the title.
- Make hints keyboard-focusable with `tabindex="0"`.
- Use smooth scrolling via `html { scroll-behavior: smooth; }`.

### Collapsible Chapter Pattern

Use native HTML `details` / `summary`, not custom accordions.

```html
<details class="chapter signal" id="timing" open>
  <summary><h2>Timing Model</h2></summary>
  <div class="content">
    <p>...</p>
  </div>
</details>
```

CSS behavior:

```css
details.chapter > summary::after {
  content: "show";
  border: 1px solid var(--line);
  color: var(--muted);
  border-radius: 999px;
  padding: 3px 10px;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

details.chapter[open] > summary::after {
  content: "hide";
}
```

JavaScript for global controls:

```js
const chapters = Array.from(document.querySelectorAll("details.chapter"));

document.querySelector('[data-action="open-all"]').addEventListener("click", () => {
  chapters.forEach((chapter) => {
    chapter.open = true;
  });
});

document.querySelector('[data-action="close-all"]').addEventListener("click", () => {
  chapters.forEach((chapter) => {
    chapter.open = false;
  });
});
```

Toolbar HTML:

```html
<div class="toolbar">
  <button type="button" data-action="open-all">Open all sections</button>
  <button type="button" data-action="close-all">Close all sections</button>
</div>
```

---

## Chart Rules

Use inline SVG inside `.chart` containers. Do not rely on external image files unless the chart is a screenshot or hardware pinout that cannot be represented clearly as SVG.

```html
<div class="chart" aria-label="Architecture data flow chart">
  <svg viewBox="0 0 1040 360" role="img">
    ...
  </svg>
</div>
```

### General Chart Style

- Wrap every SVG in `<div class="chart">`.
- Add a meaningful `aria-label` to the chart container.
- Add `role="img"` to SVG.
- Use `viewBox` coordinates, not fixed pixel `width` / `height`.
- Use `max-width: 100%; height: auto;` in CSS.
- Use `overflow-x: auto` on `.chart` so dense diagrams remain readable on small screens.
- Prefer boxes, arrows, lanes, and labels over decorative shapes.
- Use `rx="4"` or `rx="7"` for rounded rectangles.
- Use muted strokes and semantic fills.

### Standard SVG Classes

Use these reusable classes:

```css
.box {
  fill: #fff;
  stroke: #b8c6cd;
  stroke-width: 1.3;
  rx: 7;
}
.box-fw { fill: rgba(22, 100, 90, 0.09); stroke: var(--firmware); }
.box-web { fill: rgba(138, 79, 16, 0.09); stroke: var(--web); }
.box-proto { fill: rgba(155, 47, 53, 0.09); stroke: var(--proto); }
.box-signal { fill: rgba(54, 93, 168, 0.09); stroke: var(--signal); }
.box-config { fill: rgba(91, 76, 148, 0.09); stroke: var(--config); }
.chart-level { fill: rgba(22, 100, 90, 0.1); stroke: var(--firmware); }
.chart-sub { fill: rgba(54, 93, 168, 0.1); stroke: var(--signal); }
.chart-highlight { fill: rgba(155, 47, 53, 0.1); stroke: var(--proto); }
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
- use `.chart-highlight` for critical timing or risky execution spans;
- use `.chart-warn` for warnings;
- use `.chart-muted` for idle, calibration, unknown, or historical behavior.

### Arrow Marker Pattern

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

---

## Time Chart Rules

Time charts are one of the signature elements of this style. They should explain timing behavior visually, especially before/after performance changes, CPU core usage, signal generation, DMA sampling, BLE frame processing, or UI update cadence.

### When to Use a Time Chart

Use a time chart when explaining:

- control-cycle execution;
- DMA vs blocking I/O;
- ISR/task interactions;
- Core 0 vs Core 1 responsibilities;
- BLE packet receive/decode/respond latency;
- generated digital signal patterns;
- sampling windows;
- dead-time, pulse width, or duty-cycle constraints;
- before/after timing comparisons.

### Basic Time Chart Layout

Use this structure:

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

### Coordinate Formula

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

### Before/After Timing Chart Pattern

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
    <rect x="120" y="74" width="200" height="24" rx="4" class="chart-level"></rect>

    <text x="36" y="128" class="small">Core 0</text>
    <rect x="310" y="110" width="36" height="24" rx="4" class="chart-sub"></rect>

    <text x="36" y="168" class="small">Core 1</text>
    <rect x="120" y="156" width="620" height="24" rx="4" class="chart-highlight"></rect>
    <line x1="740" y1="148" x2="740" y2="188" stroke="#9b2f35" stroke-width="2" stroke-dasharray="6 5"></line>
    <text x="648" y="146" class="small">control point</text>

    <text x="120" y="218" class="small" fill="#1b4a32" font-weight="700">
      ✓ Continuous background work keeps fresh data available during the cycle.
    </text>
  </svg>
</div>
```

### Digital Signal Timing Chart Pattern

Use this chart type for generated GPIO or mode-bit patterns.

Rules:

- Use vertical shaded bands for each signal step or mode interval.
- Label each band with mode number and binary bits, for example `4 / 100`.
- Use one pair of y-levels per phase/channel:
  - `U1 HIGH`, `U1 LOW`
  - `U2 HIGH`, `U2 LOW`
  - `U3 HIGH`, `U3 LOW`
- Draw each signal with a thick `polyline`.
- Use different semantic colors per channel.
- Add a time axis below the waveforms.
- Label both absolute timestamps and interval durations.

Example skeleton:

```html
<div class="chart" aria-label="Generated digital signal chart">
  <svg viewBox="0 0 1040 450" role="img">
    <text x="22" y="30" class="label">alpha 0.4 generated pattern, T = 282 us</text>
    <text x="22" y="52" class="small">Mode bits are d6 d5 d4.</text>

    <!-- shaded mode intervals -->
    <rect x="150" y="74" width="207" height="310" fill="rgba(22,100,90,.07)" stroke="#d7e1e7"></rect>
    <rect x="357" y="74" width="53" height="310" fill="rgba(138,79,16,.08)" stroke="#d7e1e7"></rect>

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
      stroke="#16645a"
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

### Time Chart Validation Checklist

Before finalizing a timing chart, verify:

- [ ] every duration shown in the visual matches the numeric labels;
- [ ] every lane is labeled;
- [ ] colors communicate meaning consistently;
- [ ] critical boundaries are visibly marked;
- [ ] the chart has a clear implication caption;
- [ ] long labels do not overlap at desktop width;
- [ ] the chart remains horizontally scrollable on small screens;
- [ ] the chart can be understood without reading the entire chapter.

---

## Tables, Cards, and Callouts

### Cards

Use cards for short conceptual summaries, architecture responsibilities, or design principles.

```html
<div class="cards">
  <div class="card">
    <strong>Single responsibility</strong>
    <p>Each task owns one timing or communication concern.</p>
  </div>
</div>
```

Cards should be short. If a card needs many sentences, use a table or subsection instead.

### Tables

Use tables for detailed mappings:

- file / responsibility / reason;
- component / input / output / owner;
- command / payload / response;
- risk / cause / mitigation;
- timing event / source / deadline.

Keep table headings practical and scannable. Use inline `<code>` for filenames, functions, constants, commands, and payload fields.

### Callouts

Use callouts for constraints, hazards, and design rules.

```html
<div class="callout">
  <strong>Timing rule:</strong>
  Core 1 must not call blocking drivers during signal playback.
</div>
```

Do not overuse callouts. Reserve them for statements that the reader should remember.

---

## Code Snippet Rules

Code snippets must be readable and syntax-colored. Use dark `pre` blocks with inline JavaScript highlighting.

### Code Block Style

```css
pre {
  margin: 12px 0;
  padding: 14px;
  overflow-x: auto;
  border-radius: 8px;
  border: 1px solid #293640;
  background: var(--code);
  color: #dce8e5;
  font-size: 13px;
  line-height: 1.45;
}

pre code {
  padding: 0;
  border: 0;
  background: transparent;
  color: inherit;
  font-size: inherit;
}
```

### Syntax Colors

Use token classes:

```css
.tok-comment { color: #91a8a2; font-style: italic; }
.tok-keyword { color: #7dd3fc; font-weight: 700; }
.tok-type { color: #a7f3d0; }
.tok-fn { color: #fde68a; }
.tok-string { color: #fda4af; }
.tok-number { color: #c4b5fd; }
.tok-literal { color: #f9a8d4; font-weight: 700; }
.tok-operator { color: #93c5fd; }
```

### Syntax Highlighter Rules

Use a small inline highlighter if no external libraries are allowed. It should:

- escape `&`, `<`, and `>` before injecting HTML;
- highlight `//` comments;
- highlight line-start `#` comments for config snippets;
- highlight quoted strings;
- highlight numbers, hex values, floats, and simple units such as `20K`;
- highlight known keywords;
- highlight known literals;
- highlight known project types;
- treat capitalized identifiers as types;
- treat words followed by `(` as function names;
- highlight common operators and punctuation.

Use this pattern:

```js
const syntaxKeywords = new Set([
  "async", "await", "const", "float", "if", "message", "oneof", "return",
  "static", "string", "struct", "uint32", "uint32_t", "void",
]);

const syntaxLiterals = new Set(["false", "true", "y", "n"]);

const syntaxTypes = new Set([
  "BlePacket", "DataSet", "MatrixData", "SignalStep", "Telemetry",
  "UiCommand", "UiCommandResult", "Uint8Array",
]);

const escapeHtml = (text) =>
  text.replace(/[&<>]/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[char]);

const colorToken = (className, text) =>
  `<span class="${className}">${escapeHtml(text)}</span>`;

const highlightCodeBlock = (code) => {
  const source = code.textContent;
  let html = "";
  let index = 0;

  while (index < source.length) {
    const rest = source.slice(index);

    if (rest.startsWith("//")) {
      const end = source.indexOf("\n", index);
      const next = end === -1 ? source.length : end;
      html += colorToken("tok-comment", source.slice(index, next));
      index = next;
      continue;
    }

    if (source[index] === "#" && (index === 0 || source[index - 1] === "\n")) {
      const end = source.indexOf("\n", index);
      const next = end === -1 ? source.length : end;
      html += colorToken("tok-comment", source.slice(index, next));
      index = next;
      continue;
    }

    const stringMatch = rest.match(/^"([^"\\]|\\.)*"|^'([^'\\]|\\.)*'/);
    if (stringMatch) {
      html += colorToken("tok-string", stringMatch[0]);
      index += stringMatch[0].length;
      continue;
    }

    const numberMatch = rest.match(/^(0x[\da-fA-F]+|\d+(?:\.\d+)?f?|\d+K)\b/);
    if (numberMatch) {
      html += colorToken("tok-number", numberMatch[0]);
      index += numberMatch[0].length;
      continue;
    }

    const wordMatch = rest.match(/^[A-Za-z_][\w:]*/);
    if (wordMatch) {
      const word = wordMatch[0];
      const afterWord = source.slice(index + word.length);
      if (syntaxKeywords.has(word)) html += colorToken("tok-keyword", word);
      else if (syntaxLiterals.has(word)) html += colorToken("tok-literal", word);
      else if (syntaxTypes.has(word) || /^[A-Z][\w:]*$/.test(word)) html += colorToken("tok-type", word);
      else if (/^\s*\(/.test(afterWord)) html += colorToken("tok-fn", word);
      else html += escapeHtml(word);
      index += word.length;
      continue;
    }

    if (/^[{}()[\];,.=:+\-*/&|!<>]/.test(rest)) {
      html += colorToken("tok-operator", source[index]);
      index += 1;
      continue;
    }

    html += escapeHtml(source[index]);
    index += 1;
  }

  code.innerHTML = html;
};

document.querySelectorAll("pre code").forEach(highlightCodeBlock);
```

Adapt `syntaxKeywords`, `syntaxTypes`, and `syntaxLiterals` to the document topic. For example, protocol documents should include `message`, `oneof`, `uint32`; firmware documents should include `static`, `volatile`, `IRAM_ATTR`, `uint32_t`; TypeScript/UI documents should include `type`, `interface`, `async`, `await`, `Uint8Array`.

### Code Snippet Content Rules

- Keep snippets short: ideally 5 to 25 lines.
- Use snippets to illustrate contracts, types, state transitions, or timing-critical logic.
- Do not paste huge functions unless the whole function is the subject of the study.
- Use inline `<code>` for filenames, fields, constants, and function names in paragraphs and tables.
- Escape `<`, `>`, and `&` in raw HTML snippets unless the highlighter reads from `textContent` safely.

---

## Writing Style Rules

Use precise engineering language.

Prefer:

- “Core 1 reads a pre-built snapshot; it does not call the ADC driver.”
- “DMA produces triples continuously; Core 0 parses and publishes snapshots.”
- “This removes blocking driver calls from the control-cycle path.”

Avoid:

- vague claims like “this is better”;
- marketing language;
- long paragraphs without diagrams or tables;
- unexplained acronyms;
- diagrams without numeric labels when timing matters.

Each major design claim should be backed by one of:

- a file/function reference;
- a timing chart;
- a table entry;
- a code snippet;
- a commit or historical note;
- a callout explaining the rule.

---

## Accessibility and Responsiveness Rules

- Use semantic HTML where possible.
- Give charts `aria-label` and SVG `role="img"`.
- Ensure hint badges can receive keyboard focus.
- Do not encode important information only in color; also use labels and captions.
- Keep text contrast high.
- Make wide charts horizontally scrollable.
- Collapse multi-column grids to one column under `900px`.

---

## Minimal CSS Checklist

A document in this style should define styles for:

- `:root` color variables;
- `body` grid background;
- `main` width and padding;
- headings and paragraphs;
- inline `code`;
- `.hero`, `.lede`, `.meta`, `.chips`, `.chip`;
- `.toolbar` and `button`;
- `.index`, `.index-grid`, `.index a`, `.hint`, `.hint::after`;
- `details.chapter`, `summary`, show/hide pseudo-labels;
- semantic chapter classes: `.firmware`, `.web`, `.signal`, `.config`, `.proto`;
- `.content` reveal animation;
- `.twocol`, `.cards`, `.card`;
- `.chart`, `svg`, chart shape classes;
- `pre`, `pre code`, syntax token classes;
- `table`, `th`, `td`;
- `.callout`;
- mobile `@media (max-width: 900px)`.

---

## Generation Workflow

When creating a new document in this style:

1. Identify the study subject and scope.
2. List the major chapters before writing HTML.
3. Assign each chapter a semantic category: firmware, web, signal, config, or proto.
4. Create the hero, toolbar, and index first.
5. Write each chapter as:
   - short explanation;
   - chart or table;
   - code snippet if useful;
   - design implication or callout.
6. For every timing-sensitive topic, create a time chart with real units.
7. Add syntax-colored code block support.
8. Check responsiveness by ensuring grids collapse and charts scroll horizontally.
9. Check that all index links match chapter ids.
10. Check that `Open all` / `Close all` controls work.

---

## Final Quality Checklist

Before considering the document complete, verify:

- [ ] The file is standalone HTML with inline CSS and small inline JS.
- [ ] The hero explains what the study is about.
- [ ] The index covers all major chapters.
- [ ] Every chapter uses `details.chapter` and has a stable `id`.
- [ ] Show/hide labels work via CSS.
- [ ] Open/close-all buttons work via JavaScript.
- [ ] Colors follow the semantic palette.
- [ ] Charts use inline SVG and have accessible labels.
- [ ] Timing charts use calculated coordinates and real units.
- [ ] Code snippets are syntax-colored.
- [ ] Tables are used for dense mappings instead of long prose.
- [ ] Critical constraints are shown as callouts.
- [ ] The document remains readable on narrow screens.
- [ ] The content explains design consequences, not only implementation details.