# Document Structure Reference

## HTML Shell

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
      <details class="chapter level-1" id="overview" open>...</details>
      <details class="chapter level-1" id="..." open>...</details>
      <details class="chapter level-1" id="..." open>...</details>
    </main>
    <script>
      /* small inline JS only */
    </script>
  </body>
</html>
```

## Required Sections

1. **Hero section**
   - Use a large title and a short lede paragraph.
   - Add a metadata card on the right with scope, source commits, date, or target subsystem.
   - Add chips for major topics or hierarchy cues, for example `Level 1: Sections`, `Level 2: Subsections`, `Level 3: Sub-subsections`, `Timing`, `Tables`.

2. **Toolbar**
   - Include `Open all` and `Close all` buttons.
   - Buttons should toggle all `details.chapter` sections.

3. **Index**
   - Use a visible `Documentation Index` panel near the top.
   - Put the overview chapter first in the index.
   - Use a responsive grid of links.
   - Each link points to a chapter id.
   - Each link includes:
     - the chapter title;
     - a small circular `?` hint badge;
     - `data-tip="..."` with a one-sentence explanation.

4. **Chapters**
   - The first chapter must be an overview chapter with one or more charts before detailed sections.
   - Every major section must be a `<details class="chapter ..." id="...">` block.
   - Use `<summary><h2>Section Title</h2></summary>`.
   - The visible body must be wrapped in `<div class="content">`.
   - Use `level-1` on chapter blocks so all top-level sections share the same blue border.
   - Add the `open` attribute to every chapter by default.
   - Use green styling for `h3` subsections and purple styling for `h4` or `.sub-subsection-label` sub-subsections.

5. **Study Content Pattern**
   - Start the document with overview charts that show the whole system, flow, or timing model before detailed prose.
   - Start each chapter with a short framing sentence, then show a chart when possible.
   - Prefer concrete evidence: file names, functions, commits, timing values, queues, state machines, message schemas.
   - Use timing charts for time-sensitive logic and sequence diagrams for multi-actor logic or message flow.
   - Put details, tables, and code snippets after the chart.
   - End with a concise table, checklist, or callout explaining the design consequences.

## Index Pattern

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

## Collapsible Chapter Pattern

Use native HTML `details` / `summary`, not custom accordions.

```html
<details class="chapter level-1" id="timing" open>
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

Toolbar HTML:

```html
<div class="toolbar">
  <button type="button" data-action="open-all">Open all sections</button>
  <button type="button" data-action="close-all">Close all sections</button>
</div>
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
