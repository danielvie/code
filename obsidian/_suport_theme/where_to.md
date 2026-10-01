# Restore the current Obsidian layout

Requires the Border theme, Style Settings plugin, and JetBrainsMono Nerd Font. Use dark mode. Body and interface text use this font; note text has a base size of 17px.

## Palette and headings

| Element | Color |
| --- | --- |
| Background | `#282C34` charcoal |
| Body text | `#DEE0E1` soft gray |
| Headings | `#CD9A22` gold |
| Links | `#69ADD0` blue |
| Bold | `#DE8D5A` warm peach |
| Italics, inline code, page title | `#A798D7` lavender |

Heading sizes H1 through H6 are `1.45em`, `1.3em`, `1.18em`, `1.08em`, `1em`, and `0.95em`. The page title uses the H1 size. Syntax highlighting, tags, tasks, and callouts use the matching Pi-inspired palette.

## Saved files

- `_suport_theme/data.json`: Style Settings snapshot.
- `_suport_theme/appearance.json`: theme selection, fonts, and base font size.
- `theme_configuration.json`: duplicate Style Settings snapshot with the same settings as `_suport_theme/data.json`.

## Restore in a vault

1. Install Border, Style Settings, and the font if needed.
2. Back up the destination vault's existing settings, then close Obsidian.
3. Copy `_suport_theme/data.json` to `<vault>\.obsidian\plugins\obsidian-style-settings\data.json`.
4. Copy `_suport_theme/appearance.json` to `<vault>\.obsidian\appearance.json`.
5. Reopen Obsidian and select dark mode under Settings > Appearance.

Copying these files replaces the destination vault's existing appearance and Style Settings preferences. No CSS snippet is required.

## Refresh from the current vault

Palette and heading settings:

```text
C:\SANDBOX\NOTES\OBSIDIAN\main\.obsidian\plugins\obsidian-style-settings\data.json
```

Copy this file into both `_suport_theme/data.json` and `theme_configuration.json` after future styling changes.

Theme selection and fonts:

```text
C:\SANDBOX\NOTES\OBSIDIAN\main\.obsidian\appearance.json
```

Copy this file into `_suport_theme/appearance.json`. Close Obsidian first so the saved files reflect its latest settings.
