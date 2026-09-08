# Herdr Pi detection on Windows

## Update: persistent local workaround installed

The user subsequently requested implementation. See `fixes/herdr-pi-windows/README.md` for the confirmed source-level cause, implementation, tests, installation backup, limitations, and rollback. That document supersedes the earlier unresolved diagnosis below. A custom lifecycle reporter is installed globally; the official and diagnostic extensions were backed up outside discovery. Nine unit tests, installer/rollback tests, and a live lifecycle/report-release smoke test passed. Real Pi activation via `/reload` remains to be confirmed by the user. No Herdr binary modification or server restart occurred.

## Original investigation

Investigate why native Windows Herdr does not detect Pi agents, and preserve the manual registration instructions. The user reported that Pi became visible during troubleshooting, but a permanent fix has NOT been confirmed.

Workspace: the current `play` workspace. No project code was changed. No subagents were launched.

## Verified environment

- Herdr client and server: 0.8.2, stable, protocol 20, compatible.
- Pi CLI: 0.85.1, globally installed npm package `@earendil-works/pi-coding-agent`.
- Native Windows, Pi launched through PowerShell and Node.
- This conversation's Herdr pane was `w5:p2`. Recheck before targeting it; IDs and processes can change.
- Pi inherits `HERDR_ENV=1`, `HERDR_PANE_ID`, `HERDR_SOCKET_PATH`, and `HERDR_BIN_PATH`.
- Initially `herdr agent list` returned an empty list despite multiple panes with Pi terminal titles.
- `herdr pane process-info --pane w5:p2` showed only the PowerShell shell. Windows process inspection independently showed a Node Pi process directly parented by that shell. This demonstrates missed process identification, but its internal cause remains unknown.

## Changes made

1. With user approval, ran:
   ```powershell
   herdr integration install pi
   ```
   Herdr installed `%USERPROFILE%\.pi\agent\extensions\herdr-agent-state.ts` and reported it current, version 8.
2. Recommended `/reload` in each Pi session, or restarting Pi. The user reported restarting did not fix detection.
3. Added a temporary diagnostic extension:
   - `%USERPROFILE%\.pi\agent\extensions\herdr-diagnostic.ts`
   - Writes `%USERPROFILE%\.pi\agent\herdr-diagnostic.json` on `session_start`.
   - Registers `/herdr-diagnostic` to record/show runtime details.
   - Records mode, PID, idle/UI flags, and Herdr environment. It does NOT report or register agents.
   - Read the file for implementation; do not duplicate it or assume it fixes detection.
4. Sent temporary reports with source `diagnostic:pi` to `w5:p2`, followed by matching release commands. These made Pi appear immediately during tests. A later test included the current session ID and a `Date.now()*1000` sequence number.

No edits were made to the official managed integration. No Herdr update or server restart was performed by the assistant.

## Findings and uncertainties

- A read-only Node socket probe successfully connected using the exact endpoint scheme in the installed extension: on Windows, prefix `HERDR_SOCKET_PATH` with `\\.\pipe\`.
- A direct `agent.list` socket request succeeded. This is not a general connectivity failure.
- The installed extension was loaded in isolation through Jiti with a mock Pi API and a deliberately nonexistent pane. It registered `session_start`, `agent_start`, and `agent_settled` handlers, connected successfully, and received expected `pane_not_found` responses. No real Pi agent was launched in this test.
- The official extension gates initialization on `ctx.mode === "tui"`. The installed Pi implementation supports that property and uses `"tui"` for interactive mode.
- The official extension silently ignores socket/report errors. It treats receiving any response data as delivery, including API errors. Read its source before assuming a report was accepted.
- It only accepts session file paths starting with `/`, so native Windows session paths fall back to session ID reporting. This was noticed but not established as the detection cause.
- After the user said detection worked, `herdr agent list` showed Pi in `w5:p2`, status `working`, state-change sequence 6. That sequence matched the earlier diagnostic registration test, so do NOT assume the real integration caused the appearance.
- `herdr agent explain w5:p2 --json` then showed `screen_detection_skipped: false`, a remote Pi screen manifest, no matching working rule, and `default_known_agent_idle_fallback`. Thus there was no evidence of active authoritative lifecycle reporting. The list and explain outputs even differed on working versus idle at that point.
- The diagnostic JSON most recently inspected was written by a DIFFERENT pane, `w4:p6`, on startup, with mode `tui`, UI enabled, idle true, and valid Herdr environment. All Pi sessions write the same diagnostic file, so it can be overwritten by another pane. It does not prove the diagnostic or official extension loaded in `w5:p2`.
- The user was told honestly that temporary manual registration may have triggered recognition and that the diagnostic extension itself does not register agents.

## Manual commands requested by the user

From PowerShell inside the target Herdr pane, when at a shell prompt:

```powershell
herdr pane report-agent $env:HERDR_PANE_ID --source manual:pi --agent pi --state idle
```

From another terminal, using the explicit target pane ID:

```powershell
herdr pane report-agent w5:p2 --source manual:pi --agent pi --state idle
herdr agent list
```

Release with the SAME source and target:

```powershell
herdr pane release-agent w5:p2 --source manual:pi --agent pi
```

A manual report declares a fixed state; it does not automatically track activity. Supported report states in this build are `idle`, `working`, `blocked`, and `unknown`. Do not send these as ordinary Pi prompts; execute them in a shell or through an appropriate shell tool.

## Useful diagnostics

```powershell
herdr status
herdr integration status
herdr agent list
herdr api snapshot
herdr pane process-info --pane w5:p2
herdr agent explain w5:p2 --json
```

`herdr integration status` takes no `pi` positional argument. `pane process-info` requires `--pane`, not a positional ID. `agent explain` fails with `agent_not_found` when the target has not registered as an agent.

Local logs and rules:
- `%APPDATA%\herdr\herdr-server.log`
- `%APPDATA%\herdr\herdr-client.log`
- `%LOCALAPPDATA%\herdr\agent-detection\remote\pi.toml`

The inspected Pi screen rule matched literal `Working...`, whereas Pi's displayed working indicator can differ. This affects state classification after identity detection and is not itself proof of the original identity failure.

## Next steps if continuing the investigation

1. Recheck live pane IDs and authoritative state; do not report this as resolved solely because the list contains Pi.
2. Determine whether the actual official extension loads and reports successfully in the specific target Pi process. Avoid using shared diagnostic output from another pane as evidence.
3. If further instrumentation is needed, prefer per-pane diagnostic output and capture actual API replies without introducing competing lifecycle sources.
4. Remove the temporary diagnostic extension and JSON when finished, then reload affected sessions. Coordinate with the user before cleanup or disruptive operations.
5. Do not restart the Herdr server or upgrade it without discussing potential effects on live panes.

## References

- Herdr documentation index: https://herdr.dev/llms.txt
- Integrations: https://herdr.dev/docs/integrations/#pi
- The fetched documentation index advertised stable 0.9.0, newer than installed 0.8.2. Treat version differences carefully.
- Pi installed docs are under `%APPDATA%\npm\node_modules\@earendil-works\pi-coding-agent\docs`.
- Pi `docs/extensions.md` was read in full during this conversation. It documents `/reload`, `session_start`, `ctx.mode`, and lifecycle events.

## Suggested skills

- `unslop`: always apply for concise, accurate wording.
- `handoff`: maintain this continuation summary when requested.
- `how`: if the next task is explaining Herdr/Pi lifecycle ownership or runtime flow.
- `research` or `web-search-subagent`: if checking upstream Windows bugs or release fixes becomes necessary. No upstream fix has been verified in this conversation.
