# Handoff: Rust/Wasm Canvas UI Experiment

## GOAL

Determine whether a highly interactive browser UI can use Rust/Wasm for visible rendering, layout, state, and interaction while keeping the DOM as a small browser-services and accessibility adapter.

The experiment must validate four realistic workflows:

1. Document editor
2. Large data list
3. Code/diff viewer
4. Transcript editor

Canvas 2D remains the renderer until input, accessibility, workflow, and performance risks are understood. WebGPU is deferred unless measurements show that Canvas 2D is insufficient.

## Project

- Prototype: `C:/SANDBOX/play/wasm-ui-lab`
- Main implementation: `C:/SANDBOX/play/wasm-ui-lab/src/lib.rs`
- Research: `C:/SANDBOX/play/research/rust-wasm-custom-ui-assessment.md`
- Run: `task run`
- Test: `task test`
- Build: `task build`
- Clean: `task clean`

This is throwaway validation code, not production architecture.

## Current state

The repository is clean on `master`.

Relevant commits:

- `4f5b31e` — Document workflow and accessible canvas controls
- `548ddc2` — Virtualized data workflow and flexible filtering
- `1704d59` — Fuzzy ordering and filter clear control

Read these commits and the research report for implementation detail instead of reconstructing the conversation.

## Validated foundation

- One visible canvas owns all visible UI.
- One hidden textarea supplies browser text, clipboard, undo, IME, and selection services.
- Rust owns layout, rendering, hit testing, focus state, scrolling, virtualization, and selection.
- Canvas editor supports Unicode/grapheme navigation, multiline movement, selection direction, composition display, and caret-following textarea placement.
- Hidden accessibility mirror exposes the action button, switch, editor, multi-select listbox, active option, and live status.
- Keyboard list selection supports arrows, Home/End, Shift extension, Ctrl/Cmd-click toggles, and Vim `h/j/k/l` motions.
- `Alt+H/J/K/L` moves focus between UI panes. It is disabled during IME composition and overlays.
- `?` shows shortcut help outside the document editor.

Real browser/IME and accessibility behavior was manually accepted by the user.

## Completed workflow 1: Document

- Multiline canvas editor
- `Ctrl/Cmd+F` search with match highlighting and next/previous match
- `Ctrl/Cmd+P` command palette
- Commands: select all, clear search, toggle rendering
- Accessible labels and live result announcements

## Completed workflow 2: Data

- 10,000 generated rows with virtualization
- Multi-selection and batch “mark reviewed”
- Numeric ascending/descending sorting
- `/` opens the centered quick-filter overlay
- Clicking the filter field edits in place
- `X` clears the filter and restores all rows
- Filter-field Home/End and Shift+Home/End work and render selection highlights
- Explicit filter modes cycle with `m` or the mode control:
  1. Fuzzy — default
  2. Regex
  3. Literal
- Fuzzy matching controls inclusion but preserves the selected numeric sort order.
- Regex uses Rust’s linear-time `regex` crate, is debounced by 120 ms, and preserves the last valid result for invalid patterns.
- UI and accessibility state show mode, row count, elapsed filter time, and regex errors.

Measured release filtering over 10,000 short generated rows:

- Literal: approximately 1.4 ms
- Fuzzy: approximately 3.3 ms
- Regex: approximately 20–25 ms after debounce

Current release Wasm is approximately 657 KB raw. The regex dependency caused most of the increase; measure compressed size again during the performance phase.

## Current validation

At the last commit:

- 15 Rust tests passed
- Wasm Clippy passed with warnings denied
- Optimized Trunk build passed
- Chromium checks passed for the Data workflow, filtering modes, invalid regex, inline editing, selection rendering, clear control, numeric fuzzy ordering, and pane navigation
- Development server was stopped

## Next phase: Code/Diff viewer

Build a thin generated-fixture workflow. Do not add Git integration yet.

Minimum slice:

1. Add a clear workflow/mode switch without breaking Document or Data.
2. Render a file list and a side-by-side old/new diff.
3. Represent added, removed, modified, and unchanged lines.
4. Virtualize a large generated diff.
5. Add next/previous change navigation.
6. Collapse and expand unchanged sections.
7. Support keyboard selection and `Alt+H/J/K/L` pane movement.
8. Extend the accessibility mirror with file and change descriptions.
9. Surface relevant Rust state and timing in the canvas.
10. Verify with pure Rust tests, Clippy, release build, and Chromium interaction checks.

Keep this phase minimal. The question is whether the Canvas/Rust state model handles a dense review workflow, not whether it can become a Git client.

## Remaining experiment

After Diff:

1. Build the Transcript workflow:
   - virtualized timestamped segments
   - edit one segment
   - multi-select segments
   - search
   - simulated playback cursor and follow mode
2. Run a combined stress pass:
   - frame-time percentiles
   - input latency
   - startup time
   - memory growth
   - long scroll sessions
   - resize and DPR changes
   - filter and diff scaling
3. Re-measure raw, gzip, and Brotli Wasm sizes.
4. Test target browsers and assistive technology again after all workflows exist.
5. Write the final verdict:
   - whether minimal-DOM Rust/Wasm is viable
   - which browser adapter responsibilities remain
   - accessibility limitations
   - Canvas 2D limits
   - whether WebGPU is justified

## Important constraints

- Keep visible UI canvas-rendered.
- Keep DOM usage small and semantic.
- Use raw `wasm-bindgen` and `web-sys`; do not add a frontend framework.
- Avoid new dependencies unless they answer an experiment question.
- Do not add WebGPU without measured Canvas 2D failure.
- Preserve native textarea behavior during IME composition.
- Browser-hosted `Ctrl+H/J/K/L` is not reliable; retain `Alt+H/J/K/L`.
- Keep `Taskfile.yml` as the one-command run/test/build interface.
- Make surgical changes and commit each validated workflow separately.

## Suggested skills

- `prototype` — continue the throwaway experiment and keep each workflow focused on a validation question.
- `domain-modeling` — use only if workflow terminology becomes ambiguous and a `CONTEXT.md` or domain vocabulary is needed.
- `handoff` — update this handoff after Diff or before another session boundary.
