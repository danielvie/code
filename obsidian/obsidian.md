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
- Obsidian's default dark theme with `pi-rendered-text` for notes and `pi-agent-colors` for the Pi panel; Border and Style Settings are not required.
- Body-sized headings: gold H1, peach H2, sky-blue H3, teal H4, lavender H5, slate H6.
- Backgrounds: note/editor `#202329`, Pi panel and messages `#282C34`, Pi code `#23272F`.
- Notes use blue links, green bold text, rose italics, and purple code.
- Pi retains its previous charcoal surfaces, blue links, peach bold text, and lavender italics and inline code, without changing panel layout.
- JetBrainsMono Nerd Font, base font size 17px, line height 1.35.
- Theme files and agent setup instructions: `C:\SANDBOX\REPOS\code.git\obsidian\_suport_theme\where_to.md`.
  
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
