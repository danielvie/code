---
name: html-concept
description: Create a standalone HTML page that explains a single concept using timing charts and sequence diagrams, with code references. No chapters or sections - just description, visuals, and annotated code.
disable-model-invocation: true
---

# Skill: HTML Concept Explainer

Generate a single-page HTML document that explains one concept visually. The output favors timing charts and sequence diagrams over prose, making the concept easy to reflect on at a glance.

## Document Structure

The output has exactly three zones (no collapsible sections, no index):

1. **Header** - title + 1-3 sentence description of the concept.
2. **Visuals** - one or more inline SVG charts (timing charts, sequence diagrams, or flow diagrams).
3. **Code Details** - annotated code snippets with short explanations pointing back to the visuals.

## Reference Files

Read before generating:

- `references/template.html` - the HTML shell (copy verbatim, then strip unused parts).
- `references/charts-guide.md` - chart selection rules, SVG patterns, coordinate formulas.

## Generation Workflow

### Phase 1: Setup

1. Read `references/template.html`.
2. Copy it to the target output path using `fs_write`.
3. Replace `{{TITLE}}` with the concept name.
4. Replace `{{LEDE}}` with a 1-3 sentence plain-language description of the concept.
5. Replace `{{META_ITEMS}}` with `<li>` elements (e.g., language, subsystem, date).
6. Replace `{{CHIPS}}` with `<span class="chip">` elements for related keywords.
7. Remove the toolbar buttons block (the `<div class="toolbar">...</div>` section).
8. Remove the index nav block (the `<nav class="index" ...>...</nav>` section).

Hero replacements can be done in parallel.

### Phase 2: Visuals

Replace `<!-- END_CHAPTERS -->` with one or more chart blocks followed by the anchor:

```html
<section class="visuals" aria-label="Concept diagrams">
  <!-- charts go here -->
</section>
<!-- END_CODE -->
```

Chart rules:
- Prefer **timing charts** when the concept involves duration, ordering, latency, or lifecycle.
- Prefer **sequence diagrams** when the concept involves actors exchanging messages or state transitions.
- Use both if the concept has timing AND multi-actor interaction.
- Each chart is wrapped in `<div class="chart" aria-label="...">`.
- Follow coordinate formulas and marker definitions from `references/charts-guide.md`.
- Add a one-line caption below each chart as `<p style="margin-top:6px;font-size:13px;color:var(--muted)">...</p>`.

### Phase 3: Code Details

Replace `<!-- END_CODE -->` with annotated code blocks followed by the anchor:

```html
<section class="code-details" aria-label="Code references">
  <h3>Code References</h3>
  <!-- code blocks here -->
</section>
<!-- END_CODE -->
```

Code detail rules:
- Each code block has a short heading (`<h4>`) naming the file or function.
- Use `<pre><code>...</code></pre>` for snippets (template JS highlights automatically).
- After each snippet, add 1-2 sentences explaining how it relates to the diagram above.
- Use callouts (`<div class="callout">`) for critical constraints or gotchas.
- Keep snippets 5-25 lines.

### Phase 4: Done

The `<!-- END_CODE -->` anchor remains. No cleanup needed.

## Key Constraints

- Never regenerate CSS or JS - they live in the template.
- Never produce the entire document in one tool call.
- No collapsible `<details>` sections - the page is flat and scannable.
- Charts are the primary explanation; prose is secondary.
- Every code snippet must reference something visible in a chart.
- Use `str_replace` with unique anchors for each injection step.
