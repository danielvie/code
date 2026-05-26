# TODO

- [x] Analyze `invalid_grant` error in `go run main.go`
- [x] Identify cause: Expired/invalid OAuth token in `token.json`
- [x] Remove `token.json` to force re-authentication
- [x] Verify fix by running `go run main.go` or `task main`
- [x] Initialize Electron + Svelte + Tailwind TS project in `web/`
- [x] Setup IPC to read `client_secret.json` and `token.json`
- [x] Fetch up to 400 inbox emails via Gmail API
- [x] Build distinctive frontend UI (Table, Sortable Columns, Search Filter)
- [x] Implement row selection (Left click single, Shift+Left multi-select)
- [x] Implement Archive and Delete actions on selected items
- [x] Create Taskfile.yml for project management
- [x] Add Agentation for visual feedback
- [x] Change selection: Left click toggle, Right click range
- [x] Implement marking behavior (Archive/Delete marks)
- [x] Add "Execute" button to perform marked actions
