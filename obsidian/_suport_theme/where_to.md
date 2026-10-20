# Apply the Pi-style Obsidian theme

## Goal and scope

Use two independent CSS snippets on Obsidian's built-in dark theme. `pi-rendered-text` provides compact note rendering in Reading View and Live Preview. `pi-agent-colors` restores the Pi panel's former charcoal, blue, peach, and lavender palette without Border. Neither changes note content or math rendering. Border and Style Settings are not required.

Source folder: `C:\SANDBOX\REPOS\code.git\obsidian\_suport_theme\`.

## Files

- `pi-rendered-text.css`: authoritative note typography and palette.
- `pi-agent-colors.css`: Pi panel colors only; no typography or layout overrides.
- `appearance.json`: reference settings for dark mode, default theme, JetBrainsMono Nerd Font, 17px base size, and snippet enablement. Merge its keys into the destination settings; do not blindly overwrite them.
- `where_to.md`: these agent setup instructions.

The previous Border exports, `data.json` and `C:\SANDBOX\REPOS\code.git\obsidian\theme_configuration.json`, were removed. Do not recreate them or restore the Border theme.

## Reference sources

These exports mirror the current source vault. When refreshing them, copy only these files; do not restore Border or the obsolete Style Settings snapshots.

| Source | Export filename |
| --- | --- |
| `C:\SANDBOX\NOTES\OBSIDIAN\main\.obsidian\snippets\pi-rendered-text.css` | `pi-rendered-text.css` |
| `C:\SANDBOX\NOTES\OBSIDIAN\main\.obsidian\snippets\pi-agent-colors.css` | `pi-agent-colors.css` |
| `C:\SANDBOX\NOTES\OBSIDIAN\main\.obsidian\appearance.json` | `appearance.json` |

After copying, verify that each export matches its source and update the color tables below if the palette changed.

## Installation in another vault

1. Confirm the destination vault's absolute path. Do not assume it is the source vault. Check whether Obsidian uses a custom configuration folder; the default is `.obsidian`.
2. Back up the destination's appearance settings and any existing snippets with the same names. Close Obsidian before editing its JSON files so the app cannot overwrite the changes.
3. Copy both `pi-rendered-text.css` and `pi-agent-colors.css` into the destination configuration folder's `snippets` directory. Create that directory if necessary. Keep the filenames unchanged.
4. Read the existing destination `appearance.json`, or start with an empty object if it is absent. Merge the reference settings while preserving unrelated keys. For `enabledCssSnippets`, preserve existing entries and append `pi-rendered-text` and `pi-agent-colors` only if absent. Entries have no `.css` extension.
5. Install JetBrainsMono Nerd Font if available and wanted. Otherwise select an installed monospace font for `textFontFamily` and `interfaceFontFamily`; exact glyph appearance will differ.
6. Reopen Obsidian. Under Settings > Appearance, confirm Dark mode, the Default theme, 17px text, and both enabled CSS snippets. If either snippet is not listed, use the refresh button beside CSS snippets.
7. If Border was installed in the destination, select Default first and then uninstall Border through Appearance > Themes > Manage. Do not delete other themes, uninstall Style Settings, or erase its plugin preferences; those may serve unrelated customizations.
8. Check for other enabled snippets that override headings or emphasis. Disable only confirmed conflicts with the user's approval. Do not replace the entire enabled-snippet list.

For an already running vault, use Obsidian's Appearance settings instead of editing its JSON on disk. In this environment, the CLI's `snippet:enable` command did not enable the snippet reliably; confirm the result in the UI rather than trusting an empty command response.

## Note appearance contract

All heading levels use body-sized bold text. H1 is underlined. Body line height is 1.35; paragraph and heading spacing is 1.35em. The filename title is compact and underlined. Existing blank lines in the editor are not removed.

| Element | Dark-mode color |
| --- | --- |
| Note/editor background | `#202329` |
| Body | `#DEE0E1` |
| Muted text and quotes | `#9DA5A9` |
| H1 and filename title | `#CD9A22` gold |
| H2 | `#DCA16D` peach |
| H3 | `#79B8D8` sky blue |
| H4 | `#73BFB0` teal |
| H5 | `#B3A2DF` lavender |
| H6 | `#AAB5C4` slate |
| Bold | `#68B78D` green |
| Italics | `#D49AA6` rose |
| Links | `#69ADD0` blue |
| Code and list markers | `#A798D7` purple |

The note snippet sets the note/editor pane background to `#202329`, but does not set sidebar backgrounds, pane layout, syntax-token colors, or terminal-style math rendering. Obsidian's default theme supplies those defaults. In light mode, compact typography remains but the custom dark colors do not apply.

## Pi panel colors

`pi-agent-colors.css` is scoped to the Pi Agent panel in dark mode. It restores the saved pre-removal palette without reinstalling Border. Colors are independent of the note palette, so changing Pi does not change notes or sidebars.

| Pi element | Color |
| --- | --- |
| Panel background | `#282C34` |
| Secondary surfaces and code background | `#23272F` |
| User and assistant message backgrounds | `#282C34` |
| Body text | `#DEE0E1` |
| Muted text | `#9DA5A9` |
| H1 | `#CD9A22` gold |
| H2 and links | `#69ADD0` blue |
| H3 through H6, italics, inline code, accent | `#A798D7` lavender |
| Bold | `#DE8D5A` peach |
| Error text | `#EA7F81` |

This intentionally preserves Pi's old emphasis colors, not the newer green bold and rose italics used in notes. Code-token colors also come from the saved palette. Filled-button labels use `#23272F` for readable contrast on lavender and red backgrounds. Layout, font sizes, spacing, and plugin files remain unchanged. Plugin menus or modals mounted outside the Pi panel retain Obsidian's default colors. In light mode, this snippet does not apply.

## Verification

Open a temporary note with the sample below, then inspect it in Reading View and Live Preview. Do not add test headings to an existing user note.

```markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Plain text, **bold**, *italic*, and `inline code`.

[Example link](https://example.com)

- List item

> Quoted text

Inline math: $x^2 + y^2$.
```

Confirm a `#202329` note/editor background, six distinct note heading colors at the same 17px size, green bold, rose italics, blue links, purple code, and intact math. Remove the temporary note after checking.

Open the Pi panel and inspect a rendered answer containing headings, bold, italics, links, and code. Compare its colors with the Pi table above. Toggle `pi-agent-colors` off and on to confirm that only Pi changes; note colors, sidebars, and panel geometry must stay unchanged. Confirm that the panel and both user and assistant message backgrounds are `#282C34`, while code backgrounds remain the darker `#23272F`. The original checks also verified heading colors, peach bold, lavender italics and inline code, blue links, and unchanged note text colors and Pi layout properties.

The original vault was verified with computed-style checks in both rendering modes. Future agents must repeat verification in their destination; installed themes and snippets can introduce different overrides.

## Rollback

Disable `pi-rendered-text` to revert note styling, or `pi-agent-colors` to revert only Pi panel colors, under Appearance > CSS snippets. Restore the destination's backed-up appearance settings if fonts or theme selection must also be reverted. Do not overwrite notes or unrelated plugin settings.

## Suggested skills

- `frontend-ui-engineering` for CSS changes and visual verification.
- `incremental-implementation` if installation requires changes across multiple files.
- `clear-explanations` when updating these instructions.
