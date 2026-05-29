# Writing and Quality Reference

## Writing Style Rules

Use precise engineering language.

Prefer:

- “Core 1 reads a pre-built snapshot; it does not call the ADC driver.”
- “DMA produces triples continuously; Core 0 parses and publishes snapshots.”
- “This removes blocking driver calls from the control-cycle path.”

Avoid:

- vague claims like “this is better”;
- marketing language;
- long paragraphs before diagrams;
- prose-only explanations of timing, ordering, message flow, or multi-actor logic;
- unexplained acronyms;
- diagrams without numeric labels when timing matters.

Each major design claim should be backed by one of:

- an overview, timing, sequence, or data-flow chart;
- a file/function reference;
- a table entry;
- a code snippet;
- a commit or historical note;
- a callout explaining the rule.

## Accessibility and Responsiveness Rules

- Use semantic HTML where possible.
- Give charts `aria-label` and SVG `role="img"`.
- Ensure hint badges can receive keyboard focus.
- Do not encode important information only in color; also use labels and captions.
- Keep text contrast high.
- Make wide charts horizontally scrollable.
- Collapse multi-column grids to one column under `900px`.

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
- hierarchy classes and accents: `.level-1`, `h3`, `h4`, `.sub-subsection-label`;
- `.content` reveal animation;
- `.twocol`, `.cards`, `.card`;
- `.chart`, `svg`, chart shape classes;
- `pre`, `pre code`, syntax token classes;
- `table`, `th`, `td`;
- `.callout`;
- mobile `@media (max-width: 900px)`.

## Generation Workflow

When creating a new document in this style:

1. Identify the study subject and scope.
2. List the major chapters before writing HTML.
3. Use hierarchy-based colors: blue for chapters, green for subsections, purple for sub-subsections.
4. Create the hero, toolbar, index, and overview chapter first.
5. Put overview charts in the first chapter before detailed topic sections.
6. Write each chapter as:
   - short framing sentence;
   - chart first when possible;
   - table or details;
   - code snippet if useful;
   - design implication or callout.
7. For every timing-sensitive topic, create a time chart with real units.
8. For every multi-actor or ordered logic flow, create a sequence diagram.
9. Add syntax-colored code block support.
10. Check responsiveness by ensuring grids collapse and charts scroll horizontally.
11. Check that all index links match chapter ids.
12. Check that `Open all` / `Close all` controls work.

## Final Quality Checklist

Before considering the document complete, verify:

- [ ] The file is standalone HTML with inline CSS and small inline JS.
- [ ] The hero explains what the study is about.
- [ ] The first chapter gives a visual overview before detailed prose.
- [ ] The index covers all major chapters.
- [ ] Every chapter uses `details.chapter.level-1`, has a stable `id`, and starts open.
- [ ] Show/hide labels work via CSS.
- [ ] Open/close-all buttons work via JavaScript.
- [ ] Colors follow the hierarchy palette: level 1 blue, level 2 green, level 3 purple.
- [ ] Charts use inline SVG and have accessible labels.
- [ ] Logic and flow are shown with charts before detailed text when possible.
- [ ] Timing charts use calculated coordinates and real units.
- [ ] Sequence diagrams are used for multi-actor or ordered message flows.
- [ ] Code snippets are syntax-colored.
- [ ] Tables are used for dense mappings instead of long prose.
- [ ] Critical constraints are shown as callouts.
- [ ] The document remains readable on narrow screens.
- [ ] The content explains design consequences, not only implementation details.
