---
name: html-doc
description: Create readable, self-contained interactive HTML architecture/study documentation with collapsible chapters, inline SVG charts, timing diagrams, tables, callouts, and syntax-colored code blocks.
disable-model-invocation: true
---

# Skill: Interactive HTML Study Documentation

Create standalone HTML study documents using a template-based multi-step assembly that avoids output truncation.

## Core Idea

A reusable `template.html` contains all CSS, JS, and structural boilerplate. The model only generates content (text, charts, tables, code snippets) and injects it into placeholders. No single tool call produces more than one chapter.

## Reference Files

Read before generating:

- `references/template.html` — the reusable HTML shell (copy verbatim to target path).
- `references/charts-guide.md` — chart selection rules, SVG patterns, coordinate formulas, examples.
- `references/writing-guide.md` — content pattern, writing style, accessibility, quality checklist.
- `references/diff-guide.md` — side-by-side code diff panels: HTML structure, line types, rules.

## Generation Workflow

### Phase 1: Setup

1. Read `references/template.html`.
2. Copy it verbatim to the target output path using `fs_write`.
3. Replace `{{TITLE}}` with the document title.
4. Replace `{{LEDE}}` with a one-sentence description.
5. Replace `{{META_ITEMS}}` with `<li>` elements (scope, date, subsystem, etc.).
6. Replace `{{CHIPS}}` with `<span class="chip">` elements for major topics.

All hero replacements can be done in parallel (independent targets).

### Phase 2: Index

Replace `{{INDEX}}` with the full index grid content:

```html
<a href="#chapter-id">
  Chapter Title
  <span class="hint" tabindex="0" data-tip="One-sentence explanation.">?</span>
</a>
```

Generate all index links in one replacement. Plan all chapters before this step.

### Phase 3: Chapters

For each chapter, replace `<!-- END_CHAPTERS -->` with:

```html
<details class="chapter level-1" id="chapter-id" open>
  <summary><h2>Chapter Title</h2></summary>
  <div class="content">
    <!-- chapter content here -->
  </div>
</details>
<!-- END_CHAPTERS -->
```

Rules:
- One chapter per `str_replace` call.
- The first chapter must be an overview with one or more charts before detailed prose.
- If a chapter exceeds ~150 lines of HTML, split it into two injections using the same anchor pattern.
- Use `level-1` class on all top-level chapters.
- Use green `h3` for subsections, purple `h4` for sub-subsections.
- Follow the content pattern: framing sentence → chart → details/table → code → callout.
- Sections are open by default (include the `open` attribute).

### Phase 4: Done

The `<!-- END_CHAPTERS -->` anchor remains in the file (invisible HTML comment). No cleanup needed. The JS toolbar controls and syntax highlighter work automatically on the injected content.

## Key Constraints

- Never regenerate CSS or JS — they are baked into the template.
- Never produce the entire document in one tool call.
- Each `str_replace` must have a unique, reliable match target.
- Code snippets go inside `<pre><code>...</code></pre>` — the template JS highlights them automatically.
- Charts use inline SVG with the classes defined in the template CSS.
