# Plugins

- Auto Card Link
- Dataview
- Excalidraw
- GIT
- Iconize
- Omnisearch
- Paste Link
- PlanUML
- Recent Files
- Style Settings
- Tasks
- Vim Yank Highlight
- Vimrc Support

# Theme
- Border with a Pi-inspired dark palette and compact headings.
- Gold headings, blue links, peach bold text, and lavender italics.
- JetBrainsMono Nerd Font, base font size 17px.
- Saved settings and restore instructions: `_suport_theme/where_to.md`.
- `theme_configuration.json` contains the same Style Settings values as `_suport_theme/data.json`.
  
# Task queries

```tasks
# Only tasks that are not done, that is, which begin like this (but without the quotes):
#   '- [ ] ' or
#   '* [ ] ' or
#   '1. [ ] '
# Indented tasks are supported, but only single-line tasks.
not done

# Tasks due today or earlier:
due before tomorrow

# Restrict to at most 100 tasks.
# If you ask Tasks to display many hundreds or thousands of tasks,
# Obsidian's editing performance really slows down.
limit 100

# Group and sort the output:
group by filename
sort by due reverse
sort by description

# Optionally, ask Tasks to explain how it interpreted this query:
explain

# filter by function
filter by function task.file.folder.includes("LTP")

# filter by tag
tags includes #important

# filter by date
done 2025-01-01 2025-12-20
```

pi agent wrapper (pi.cmd)
```
@echo off
"C:\SANDBOX\APP\pi\pi.exe" %*
```
