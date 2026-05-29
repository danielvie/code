# Visual Design Reference

## Layout

- Use a centered `<main>` with `max-width: 1180px`.
- Use comfortable page padding: `32px 20px 70px` desktop, smaller on mobile.
- Use responsive grids:
  - hero: two columns, main content wider than metadata;
  - index: three columns on desktop;
  - cards: three columns on desktop;
  - all collapse to one column below `900px`.

## Typography

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

## Background

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

## Color Schema

Use CSS variables at `:root`. Keep the palette muted and technical.

```css
:root {
  --ink: #182025;
  --muted: #60707a;
  --line: #d7e1e7;
  --paper: #fbfcfb;
  --panel: #f3f7f5;
  --level-1: #365da8;
  --level-2: #16645a;
  --level-3: #5b4c94;
  --code: #101820;
}
```

Hierarchy meaning:

| Token | Meaning | Typical Use |
| --- | --- | --- |
| `--ink` | primary text | headings, normal text |
| `--muted` | secondary text | captions, descriptions, axis labels |
| `--line` | borders | panels, cards, tables, chart containers |
| `--paper` | page base | body background |
| `--panel` | light panel fill | index links, cards |
| `--level-1` | top-level section | chapter border, section-level chart blocks |
| `--level-2` | subsection | `h3` accent, subsection chart blocks, callouts |
| `--level-3` | sub-subsection | `h4`, `.sub-subsection-label`, third-level chart blocks |
| `--code` | code block background | `pre` blocks |

Important: color communicates hierarchy, not subsystem type. Use the same color for the same nesting level across the whole document: sections blue, subsections green, sub-subsections purple.

Use this pattern for hierarchy accents:

```css
details.level-1 { border-left-color: var(--level-1); }

h3 {
  border-left: 4px solid var(--level-2);
  padding-left: 10px;
}

h4,
.sub-subsection-label {
  color: var(--level-3);
  font-weight: 700;
}
```

## Cards

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

## Tables

Use tables for detailed mappings:

- file / responsibility / reason;
- component / input / output / owner;
- command / payload / response;
- risk / cause / mitigation;
- timing event / source / deadline.

Keep table headings practical and scannable. Use inline `<code>` for filenames, functions, constants, commands, and payload fields.

## Callouts

Use callouts for constraints, hazards, and design rules.

```html
<div class="callout">
  <strong>Timing rule:</strong>
  Core 1 must not call blocking drivers during signal playback.
</div>
```

Do not overuse callouts. Reserve them for statements that the reader should remember.
