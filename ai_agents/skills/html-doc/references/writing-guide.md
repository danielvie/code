# Writing Guide

## Content Pattern

Each chapter follows this structure:

1. Short framing sentence (what this section explains and why it matters).
2. Chart first — when logic, flow, or timing exists, show it visually before prose.
3. Details/table — expand on the chart with specifics.
4. Code snippet — if a contract, type, or critical function is relevant.
5. Design implication — a callout or closing sentence explaining the consequence.

## Writing Style

Use precise engineering language.

Prefer:
- "Core 1 reads a pre-built snapshot; it does not call the ADC driver."
- "DMA produces triples continuously; Core 0 parses and publishes snapshots."

Avoid:
- Vague claims like "this is better."
- Marketing language.
- Long paragraphs before diagrams.
- Prose-only explanations of timing, ordering, or multi-actor logic.
- Unexplained acronyms.

Every major design claim should be backed by: a chart, a file/function reference, a table entry, a code snippet, or a callout.

## Accessibility

- Use semantic HTML.
- Give charts `aria-label` and SVG `role="img"`.
- Ensure hint badges are keyboard-focusable (`tabindex="0"`).
- Do not encode information only in color — also use labels and captions.
- Keep text contrast high.
- Make wide charts horizontally scrollable.

## Tables

Use tables for dense mappings:
- file / responsibility / reason
- component / input / output / owner
- risk / cause / mitigation
- timing event / source / deadline

Keep headings practical and scannable. Use `<code>` for filenames, functions, constants.

## Callouts

Reserve callouts for constraints, hazards, and design rules the reader must remember.

```html
<div class="callout">
  <strong>Rule:</strong> Each step must complete within the cycle budget.
</div>
```

## Code Snippets

- Keep snippets 5–25 lines.
- Use snippets to illustrate contracts, types, state transitions, or timing-critical logic.
- Use inline `<code>` for filenames, fields, and function names in prose and tables.

## Quality Checklist

Before considering the document complete:

- [ ] Hero explains what the study is about.
- [ ] First chapter gives a visual overview before detailed prose.
- [ ] Index covers all chapters.
- [ ] Every chapter uses `<details class="chapter level-1" id="..." open>`.
- [ ] Colors follow hierarchy: level-1 blue, level-2 green, level-3 purple.
- [ ] Charts use inline SVG with accessible labels.
- [ ] Timing charts use calculated coordinates and real units.
- [ ] Sequence diagrams are used for multi-actor flows.
- [ ] Code snippets are inside `<pre><code>` blocks (highlighter runs automatically).
- [ ] Critical constraints shown as callouts.
- [ ] Document readable on narrow screens.
- [ ] Content explains design consequences, not only implementation details.
