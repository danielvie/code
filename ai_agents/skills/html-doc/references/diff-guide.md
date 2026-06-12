# Code Diff Guide

Side-by-side before/after code comparison panels with line numbers and syntax highlighting. Styled to match VS Code's diff view.

## When to Use

- Showing code changes from a commit or refactor
- Before/after comparisons in tutorials
- Highlighting what was added, removed, or unchanged

## HTML Structure

```html
<div class="diff-container">
  <div class="diff-panel diff-panel-before">
    <span class="diff-panel-label">Before</span>
    <pre><code><span class="diff-line-removed">old code line</span>
<span class="diff-line-context">unchanged line</span>
<span class="diff-line-context">unchanged line</span></code></pre>
  </div>
  <div class="diff-panel diff-panel-after">
    <span class="diff-panel-label">After</span>
    <pre><code><span class="diff-line-added">new code line</span>
<span class="diff-line-context">unchanged line</span>
<span class="diff-line-added">another new line</span></code></pre>
  </div>
</div>
```

## Line Types

| Class | Meaning | Visual |
|-------|---------|--------|
| `diff-line-removed` | Deleted line (before panel) | Red background + red left border |
| `diff-line-added` | Added line (after panel) | Green background + green left border |
| `diff-line-context` | Unchanged context | No background, transparent border |

## Rules

1. Each line of code is one `<span>` element. No `<br>` tags -- use newlines between spans.
2. Spans must be direct children of `<code>` inside `<pre>`.
3. The template JS automatically:
   - Adds line numbers (ascending from 1)
   - Applies syntax highlighting to each line's text content
   - Converts spans into `div.diff-line` rows with `.diff-ln` + `.diff-content`
4. Do NOT pre-highlight code. Write plain text inside spans -- the JS handles coloring.
5. The `diff-panel-label` text can be customized (e.g., "Before (v1.2)", "After (fixed)").
6. When a commit hash is available, add it to the Before label: `<span class="diff-panel-label">Before <span class="diff-hash">abc1234</span></span>`. Use the short hash (7-10 chars).
7. Empty lines: use `<span class="diff-line-context"></span>` (renders as a blank numbered row).

## Layout Notes

- Side-by-side on screens > 900px, stacked on mobile.
- Panels auto-scroll horizontally for long lines.
- Line height is fixed at 20px for compact display.
- Background: `#1e1e1e` (VS Code dark theme).
- No border-radius on inner `<pre>` -- the container handles rounding.
- Wrap diff blocks inside a `<section>` with class `code-details` or `visuals` to get the white card background that separates content from the page grid.

## Example: Minimal Diff

```html
<div class="diff-container">
  <div class="diff-panel diff-panel-before">
    <span class="diff-panel-label">Before</span>
    <pre><code><span class="diff-line-removed">const x = getValue();</span>
<span class="diff-line-context">console.log(x);</span></code></pre>
  </div>
  <div class="diff-panel diff-panel-after">
    <span class="diff-panel-label">After</span>
    <pre><code><span class="diff-line-added">const x = await getValue();</span>
<span class="diff-line-context">console.log(x);</span></code></pre>
  </div>
</div>
```

## Combining with Explanations

Place a `<p>` after the `</div>` (closing diff-container) to explain the change. Use `<div class="callout">` for critical notes about the diff.

## Live Example

See `references/diff-example.html` for a working standalone example with two diff blocks.
