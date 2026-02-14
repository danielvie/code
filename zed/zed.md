# installed extensions

- HTML
- TOML
- One Dark Pro
- Emmet
- LOG
- HTML Snippets


# extension path

$env:LOCALAPPDATA\Zed\extensions

# run in terminal

in **$env:APPDATA/Zed/tasks.json**
```json
[
  {
    "label": "run in terminal",
    "command": "task",
  },
]
```

in **keymap.json**
```json
{
  "context": "Editor && vim_mode == normal",
  "bindings": {
    "space v": ["task::Spawn", { "task_name": "run in terminal" }],
  },
}
```
