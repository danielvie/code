---
name: html-concept
description: Create a standalone HTML page that explains a single concept using timing charts and sequence diagrams, with code references. No chapters or sections - just description, visuals, and annotated code.
disable-model-invocation: true
---

# Skill: HTML Concept Explainer

Generate a single-page HTML document that explains a concept (or a set of related changes) visually. The output favors timing charts, sequence diagrams, and side-by-side code diffs over prose.

## Document Structure

The output has these zones:

1. **Hero** - title, 1-3 sentence description, meta info, keyword chips.
2. **Overview** - general description of the problem + inline SVG charts (timing, sequence, flow).
3. **Issue Chapters** - one `<section class="code-details">` per isolated responsibility/fix, each containing:
   - `<h3>` naming the issue
   - One or more diff blocks showing the code change
   - A **Problem** paragraph explaining what was broken
   - A **Fix** paragraph explaining how the change solves it

When the concept is a single change (not multiple issues), use one chapter after the overview.

## Reference Files

Read before generating:

- `references/template.html` - the HTML shell (copy verbatim, then strip unused parts).
- `references/charts-guide.md` - chart selection rules, SVG patterns, coordinate formulas.
- `references/diff-guide.md` - side-by-side code diff panels: HTML structure, line types, commit hash.

## Generation Workflow

### Phase 1: Setup

1. Read `references/template.html`.
2. Copy it to the target output path using `fs_write`.
3. Replace `{{TITLE}}` with the concept name.
4. Replace `{{LEDE}}` with a 1-3 sentence plain-language description.
5. Replace `{{META_ITEMS}}` with `<li>` elements (commit, area, files changed).
6. Replace `{{CHIPS}}` with `<span class="chip">` elements for related keywords.
7. Remove the toolbar buttons block (the `<div class="toolbar">...</div>` section).
8. Remove the index nav block (the `<nav class="index" ...>...</nav>` section).

Hero replacements can be done in parallel.

### Phase 2: Overview

Replace `<!-- END_CHAPTERS -->` with:

```html
<section class="visuals" aria-label="Concept diagrams">
  <h3>Overview: [Problem Name]</h3>
  <p>[General description covering all issues and the approach taken]</p>
  <!-- charts -->
</section>
<!-- END_CHAPTERS -->
```

Chart rules:
- Prefer **timing charts** for duration, ordering, latency, or lifecycle.
- Prefer **sequence diagrams** for actors exchanging messages or state transitions.
- Each chart is wrapped in `<div class="chart">`.
- Follow coordinate formulas from `references/charts-guide.md`.

### Phase 3: Issue Chapters

For each isolated issue/responsibility, replace `<!-- END_CHAPTERS -->` with:

```html
<section class="code-details">
  <h3>Issue N: [Title]</h3>

  <h4>[filename] — [brief context]</h4>
  <div class="diff-container">
    <div class="diff-panel diff-panel-before">
      <span class="diff-panel-label">Before <span class="diff-hash">[parent-hash]</span></span>
      <pre><code><!-- diff lines --></code></pre>
    </div>
    <div class="diff-panel diff-panel-after">
      <span class="diff-panel-label">After <span class="diff-hash">[commit-hash]</span></span>
      <pre><code><!-- diff lines --></code></pre>
    </div>
  </div>

  <p><strong>Problem:</strong> [what was broken and why]</p>
  <p><strong>Fix:</strong> [how the change solves it]</p>
</section>
<!-- END_CHAPTERS -->
```

Rules:
- One chapter per `str_replace` call.
- Use `diff-line-removed`, `diff-line-added`, `diff-line-context` spans (see `references/diff-guide.md`).
- Include the commit hash in both Before and After labels when available.
- Multiple diffs per chapter are fine if they belong to the same responsibility.
- Problem/Fix paragraphs come AFTER all diffs in the chapter.
- Use `<div class="callout">` for critical gotchas.

### Phase 4: Done

The `<!-- END_CHAPTERS -->` anchor remains. No cleanup needed.

## Key Constraints

- Never regenerate CSS or JS - they live in the template.
- Never produce the entire document in one tool call.
- No collapsible `<details>` sections - the page is flat and scannable.
- Charts are the primary explanation in the overview; diffs are primary in issue chapters.
- Use `str_replace` with unique anchors for each injection step.
- Group related file changes into one issue chapter; separate unrelated changes into their own chapter.
