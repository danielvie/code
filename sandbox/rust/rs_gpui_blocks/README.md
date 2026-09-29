# GPUI diagram PoC

Prototype 1 explores direct manipulation of Cameo-style block diagrams in native GPUI. It opens a small climate-control diagram. There are no menus or model browser, just a canvas and a control strip.

This is a capability experiment, not a SysML editor. Diagram changes autosave to `.local/diagram.json`, which is ignored by Git.

## Run

From this directory:

```sh
task run
# or
cargo run
```

`task check` type-checks the project and `task fmt` formats it.

## Browser prototype

Install the `wasm32-unknown-unknown` target and `wasm-pack`, then run `task web-run`. Open http://localhost:6300/ in a browser. The command builds the WebAssembly module and serves the static files; no backend is needed. Run `task web-test` for browser-core tests and a WebAssembly build.

The browser version shares the Rust model, routing, selection geometry, and save validation with the desktop app. Use **+ Block**, `N`, or double-click empty space to add a block. Click to select a block, port, or wire; Shift-click blocks to select more than one, or drag empty space to box-select fully enclosed blocks. Drag a selected block to move the group. **+ Input** and **+ Output** add ports to the selected block. Double-click a block or port, or press `F2`, to rename it; Enter saves and Escape cancels. Delete/Backspace or **Delete** removes the selection and its attached wires. Click an output then an input, or drag between them, to connect. Escape cancels an unfinished connection.

The mouse wheel pans; Ctrl+wheel zooms around the pointer. Middle-button drag also pans; the +/− buttons zoom and **Fit** shows all blocks. Unroutable wires show red crosses at their endpoints and a count in the footer. The 3600-pixel workspace and connection rules match the native app. Browser saves live in `localStorage`, separately from the desktop `.local/diagram.json` file. The diagram, pan, and zoom survive reloads; selection and unfinished edits do not. The browser uses its native text input rather than GPUI's custom label editor. Reset diagram replaces the browser save with the sample diagram. Invalid saved data is not overwritten automatically. The browser cannot silently write the desktop save file or take its file lock; avoid editing the same browser diagram in multiple tabs.

## Development loop

Close any separately launched diagram window, then run:

```sh
task dev
```

This requires Python 3 on PATH. The watcher builds the app and runs a copy of the executable. Saving a change in `src/`, `Cargo.toml`, `Cargo.lock`, `.cargo/`, `build.rs`, or a Rust toolchain file triggers a rebuild. After a successful build it stops its previous app and launches the new one. Failed builds leave the previous app running. Ctrl+C stops the watcher and its app.

The executable copy avoids Windows file locking during subsequent builds. A separately launched `task run` instance still needs to be closed before starting `task dev`. The watcher never closes unrelated app instances. Source edits made during a build trigger another build before restarting.

This is automatic rebuild/restart, not Rust hot reload. Restarts restore blocks, labels, ports, connections, IDs, pan, and zoom. Temporary selection, an unfinished connection, and label text not yet confirmed with Enter are not restored. Changes in a window launched before autosave was implemented cannot be recovered by the new version.

## Autosave

Changes save immediately, including drag updates. Unchanged state does not rewrite the file. Routes and junction dots are recalculated after loading. A missing save starts the sample diagram; an empty diagram you deliberately saved stays empty.

Saves use a temporary file followed by atomic replacement. A separate file lock prevents two app instances from writing the same save. Invalid or unsupported save files are left untouched, and startup stops with an error rather than overwriting them. Save failures during use appear in red in the bottom strip. Resolve those errors before relying on a development restart to preserve your changes.

To reset, close the app and watcher, then move `.local/diagram.json` aside. Set `GPUI_DIAGRAM_STATE` to another file path for a separate diagram or an isolated experiment. The default path is relative to the project, regardless of the launch directory. The watcher ignores saves and build outputs, so autosave does not trigger restart loops.

Requires stable Rust and a graphical desktop session. On Windows, use the MSVC toolchain with the Visual Studio C++ build tools and Windows SDK. The first build compiles GPUI's dependencies and takes longer. Other platforms may need GPUI's native system dependencies; this PoC was built and launched on Windows.

## Work with the diagram

| Operation | Control |
| --- | --- |
| Add a block | **+ Block**, `N`, or double-click empty canvas |
| Select multiple blocks | Shift-click blocks to add or remove them from the selection |
| Box-select blocks | Drag empty canvas; only fully enclosed blocks are selected. Hold Shift to add to the selection |
| Move blocks | Drag the body of any selected block to move the whole group; positions snap to a 12-pixel grid |
| Clear selection | Click empty canvas or press Escape |
| Rename a block | Select a single block and double-click its body or press `F2` |
| Add a port | Select a block, then **+ Input** or **+ Output** |
| Rename a port | Click its square and press `F2`, or double-click the square |
| Finish editing | `Enter` saves, `Escape` cancels |
| Move the text caret | Left/Right, Home/End, or click in the text |
| Select text | Shift+Arrow, Shift+Home/End, mouse drag, or Ctrl+A; double-click selects a word |
| Move/select by word | Ctrl+Left/Right; add Shift to select |
| Edit text | Type, Backspace/Delete, Ctrl+Backspace/Delete, Ctrl+C/X/V |
| Connect | Click a filled output square on the right, then an empty input square on another block's left; or drag from output to input |
| Cancel a connection | `Escape` or click empty canvas |
| Delete | Select blocks, a port, or a wire, then **Delete** or the Delete key |
| Pan | Middle-button drag or scroll |
| Zoom | **+** / **−**, or Ctrl+scroll around the pointer |
| Show all blocks | **Fit** |

The selection box is translucent blue, and enclosed blocks highlight as you drag in any direction. Blocks that only intersect the box are excluded. Clicking an already selected block keeps the group selected for dragging; clicking an unselected block without Shift selects only that block. The group preserves its spacing at the workspace edges. Wires reroute and positions autosave together after each movement. Renaming and adding ports require a single block selection.

New labels start fully selected, so typing replaces them. The editor identifies the block or port being renamed. Selected characters appear white on blue, with a selected-character count underneath. A separate caret blinks every half-second and never changes text spacing. Long labels scroll horizontally to keep the caret visible. Arrow movement and deletion respect Unicode graphemes, including combined emoji.

Port names and block names can contain up to 64 characters. IME composition is not implemented yet.

Inputs accept one connection. Outputs can feed several inputs. Deleting a block or port also deletes its attached wires. This PoC allows six ports per side and does not support self-connections.

The bottom strip shows the selection, block/wire counts, routing failures, and feedback from your last action. Colors identify the source block, not a SysML signal type. Connected inputs fill with their source output's color; unconnected inputs stay hollow. Selecting an input changes its border without hiding the connection color. Connected wires terminate at the filled port without an arrowhead. Click a wire to highlight it.

## What to try

1. Drag the control block across the existing wires. They should reroute around its new position.
2. Add a block and rename it. Connect the power supply's unused `status` output to its input.
3. Add another input to that block, rename it, and connect a different output.
4. Move a connected block to the left of its source. The router should find a path around the blocks.
5. Overlap connected blocks. If a port is trapped, the status strip reports an unroutable wire. Move the blocks apart to recover.
6. Delete an occupied port. Its wire should disappear without disturbing the remaining connections.

## Routing and limits

`src/diagram_router.rs` runs A* on a 12-pixel grid. Block rectangles have a clearance margin; short horizontal segments let wires leave and enter ports. Bends, shared segments, and crossings have extra costs. Routes recompute when blocks move or the diagram changes, not when you pan or zoom.

Filled dots mark where wires from the same output split from a shared path. White gaps mark ordinary crossings, which do not create connections. Dots follow the routes when blocks move. Shared output branches can still overlap, and the router does not guarantee the fewest crossings or globally optimal paths. Unroutable wires are omitted and marked with red endpoint crosses, rather than drawn through blocks. Crosses may be hidden by overlapping blocks; the failure count remains visible. The temporary connection preview does not avoid obstacles.

No undo, block resizing, nested blocks, SysML model validation, typed signals, or manual wire waypoints yet. Block positions are bounded to a 3600-pixel workspace in each direction. The router is synchronous and intended for small diagrams; large-diagram performance is not validated.

## Code map

- `src/main.rs`: window and application lifecycle.
- `src/diagram_prototype.rs`: rendering, pointer/keyboard handling, label editing, pan/zoom.
- `src/diagram_model.rs`: blocks, ports, connections, and sample data.
- `src/diagram_router.rs`: obstacle-aware orthogonal routing.
- `src/diagram_selection.rs`: Shift-selection, full-containment box selection, and group movement.
- `src/label_editor.rs`: text selection, word navigation, shaped-text painting, and caret feedback.
- `src/diagram_store.rs`: versioned saves, atomic replacement, and single-writer locking.
- `src/diagram_validate.rs`: save validation shared by desktop and browser.
- `web/src/lib.rs`: Wasm interface to the shared diagram model and router.
- `web/app.js`: browser drawing, pointer events, and local storage.
- `scripts/dev.py`: build watcher and restart lifecycle.
- `scripts/test_dev.py`: watcher checks without launching a GUI.

Run `cargo test` and `python -m unittest discover -s scripts -p 'test_*.py'` for the routing, selection, text editing, persistence, and watcher checks.

The project pins Zed's published `gpui` 0.2.2. It uses `Application::new()`; current Zed `main` has different platform startup code. Compare with the [GPUI 0.2.2 source](https://github.com/zed-industries/zed/tree/gpui-v0.2.2/crates/gpui).
