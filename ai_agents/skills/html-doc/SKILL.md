---
name: html-doc
description: Create readable, self-contained interactive HTML architecture/study documentation with collapsible chapters, inline SVG charts, timing diagrams, tables, callouts, and syntax-colored code blocks.
disable-model-invocation: true
---

# Skill: Interactive HTML Study Documentation

Use this skill when creating standalone HTML documentation that should feel like a technical study artifact: readable, visual, self-contained, navigable, and suitable for explaining architecture, timing, firmware/web/protocol interactions, and implementation tradeoffs.

## Core Idea

Create a single-file HTML document with:

- a clean technical report layout;
- a light engineering-paper background;
- collapsible chapters with `show` / `hide` affordances, open by default;
- an index with chapter links and short explanations;
- hierarchy-based colors: sections blue, subsections green, sub-subsections purple;
- visual-first explanations: overview charts before detailed prose;
- inline SVG charts, especially timing charts and sequence diagrams;
- tables for responsibilities, files, interfaces, and risks;
- dark code blocks with syntax coloring;
- minimal JavaScript only for expand/collapse controls and code highlighting.

The document should explain design consequences, not only implementation details. Favor charts over prose whenever charts can make the logic, timing, ownership, or message flow explicit.

## When Generating a Document

1. Identify the study subject and scope.
2. List the major chapters before writing HTML.
3. Use hierarchy-based classes and colors: `level-1` for chapters, `level-2` for subsections, `level-3` for sub-subsections.
4. Build the hero, toolbar, index, and first overview chapter with one or more charts before detailed sections.
5. Write each chapter as: short framing sentence → chart first when possible → details/table → code snippet if useful → design implication/callout.
6. Use timing charts with real units for timing-sensitive topics and sequence diagrams for logic, message exchange, request/response flows, state transitions, or multi-actor interactions.
7. Keep the result standalone: inline CSS, inline SVG, and small inline JS only.
8. Verify responsiveness, accessibility, index links, and expand/collapse controls.

## Reference Files

Read the relevant reference files before producing the final HTML:

- `references/document-structure.md` — required HTML shell, hero, toolbar, index, and chapter patterns.
- `references/visual-design.md` — layout, typography, background, colors, cards, tables, and callouts.
- `references/charts.md` — visual-first chart rules, reusable SVG classes, arrows, timing charts, and sequence diagrams.
- `references/code-snippets.md` — dark code block styling and inline syntax highlighter pattern.
- `references/writing-quality.md` — writing style, accessibility, CSS checklist, workflow, and final quality checklist.
