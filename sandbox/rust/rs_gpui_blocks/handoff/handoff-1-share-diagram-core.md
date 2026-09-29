# Handoff: share diagram behavior between native and web

## Next-session goal

Refactor the diagram project so native GPUI and browser/Wasm use the same diagram behavior without changing existing interactions or saved data. The user asked for this handoff after discussing code distribution; no refactor has been started in this turn.

## Current structure

- Native executable: `src/main.rs`, `src/diagram_prototype.rs` (events, rendering, selection state, routing cache), `src/label_editor.rs`, `src/diagram_store.rs` (atomic file replacement and single-writer lock).
- Browser: `web/src/lib.rs` (Wasm API and browser editor state), `web/app.js` (DOM/SVG events, rendering, viewport, `localStorage`). The web crate includes shared source files through `#[path = "../../src/..."]` rather than a Cargo dependency.
- Shared source in practice: `src/diagram_model.rs`, `src/diagram_router.rs`, `src/diagram_selection.rs`, `src/diagram_validate.rs`. See `Cargo.toml` and `web/Cargo.toml`; these are currently separate packages with separate lockfiles.
- `README.md` documents the implemented native/browser interactions and save formats. `Taskfile.yml` has `run`, `test`, `web-build`, and `web-run`; `task web-run` serves http://localhost:6300/.

The working tree has substantial **uncommitted, intentional changes** from prior sessions. `web/` and `src/diagram_validate.rs` are untracked; the old counter example and its helpers have been deleted. Do not reset or discard these changes. Inspect `git status` before editing.

## Observed duplication and recommended owners

| Behavior | Today | Recommended owner |
| --- | --- | --- |
| Find a nearby free position, snap/clamp, then add a block | Native `DiagramPrototype::add_at` and Wasm `BrowserDiagram::add_block` each have the same bounded search | Shared `Diagram::add_near(Pos) -> id`, called by both |
| Add/remove ports, reindex attached wire endpoints, rename labels | Native methods in `src/diagram_prototype.rs` and browser methods in `web/src/lib.rs` | Shared model mutations with domain validation; UI retains selection/status/rename input |
| Block height, port coordinates, diagram bounds/fit | `Block` geometry exists in Rust, but `web/app.js` repeats `192`, `72`, `24` and has its own fit calculation | Shared geometry and pure fit-bounds calculation; Wasm view exposes coordinates needed by SVG |
| Marquee and group-drag geometry, routing, connection constraints, diagram validation | Shared source via `#[path]` | A dedicated, platform-free Rust core crate |
| Mouse/keyboard handling, drawing, edit widget, autosave | Separate GPUI and browser implementations | Keep separate; they adapt different platform events and storage |

**Suggested layout:** `diagram-core/src/{lib,model,router,selection,validate}.rs` with only portable dependencies (e.g. Serde). The root native crate and `web` depend on it by path; remove the `#[path]` includes and duplicate `mod` declarations. A Cargo workspace is reasonable if it simplifies lockfiles, but not a prerequisite. Keep GPUI, `tempfile`, `wasm-bindgen`, DOM code, and platform storage out of the core crate.

Do **not** unify native `Snapshot` and browser `localStorage` envelopes as part of a source-code move: the native view saves pixel pan plus zoom; the web saves world-space SVG view origin plus zoom. Preserve existing saves unless a deliberate migration and tests are added. Native file locking and browser storage cannot be the same implementation.

## Incremental refactor and checks

1. Extract the existing four portable modules to the core crate, re-export a small API, and switch both packages to path dependencies. No behavior change. Run `task test`, `task web-build`, and the existing store round-trip tests.
2. Move block placement into `Diagram::add_near`, retaining the 300-attempt search, snapping, bounds, and default block data. Delete both copies of the placement loop. Add one core test for occupied positions and workspace edges.
3. Move port add/delete and rename rules into core methods. Preserve wire cleanup and index shifting, six-port limit, trimmed 1–64-character labels, and existing UI feedback. Test port deletion with remaining wires and block deletion with attached wires.
4. Remove browser copies of model geometry by exposing computed dimensions/port positions through the Wasm view. Move fit bounds to a pure function only if both adapters can call it without coupling to GPUI pixels or SVG DOM.
5. Leave interaction state (selection/pending gesture) in each adapter initially; both already use pure `Marquee` and `GroupDrag` geometry. Extract a common editor state machine only if concrete duplicated transitions still drift after steps 1–4. Avoid a speculative framework.

Check both browser and native flows after migration: add near an occupied block, add/rename/remove ports, delete a connected port, connect, route, group drag, reload persisted data, and fit. `task test` currently runs native and web core tests; `task web-build` compiles the Wasm target. Browser behavior was previously smoke-tested in headless Chrome against the static server, but there is no committed browser automation harness.

## Suggested skills

- `how`: trace ownership and call paths before moving files.
- `domain-modeling`: only if naming shared mutations or recording a domain decision becomes necessary.
- `clear-explanations` and `unslop`: for the final explanation or documentation. Apply `unslop` to all writing.
