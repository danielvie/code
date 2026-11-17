# Spec: Native Windows TLAPM integration

Status: blocked on a verified native Windows backend.
Module id: `tlapm-lsp`.
Parent scope: `CAPABILITIES.md`.

## Objective and hard constraint

Run TLAPM with native Windows Zed projects using Windows backend processes.
The user explicitly requires Windows only and rejected WSL. The previous
WSL-backed proposal is withdrawn. Do not implement its bridge or configuration.

No WSL, Linux path translation, Linux containers, remote backend, or remote
Zed workspace may be introduced as a substitute.

## Current evidence and blocker

Upstream's language-server executable is `tlapm_lsp`, launched with `--stdio`.
The separate `tlapm` proof manager and backend provers are needed for proof
checking. Upstream discovers the proof manager relative to the language-server
installation, so copying only the server is not a complete installation.

The inspected upstream setup targets Unix-like environments. No working,
upstream-supported native Windows server distribution or native Windows build
has been verified. This is a blocker, not proof that a future port is impossible.
A Zed launcher cannot turn a Unix executable into a native Windows executable.

There is currently no TLAPM runtime code in this extension. Do not add a dummy
server registration or claim successful integration based only on launcher or
mock tests.

## Requirements before implementation

1. Identify a real Windows-runnable TLAPM language server and document its source,
   version, build/install steps, and any runtime dependencies.
2. Verify stdio initialization, document opening, and diagnostics with Windows
   file URIs and native filesystem paths.
3. Verify the Windows proof-manager and prover dependencies separately before
   advertising proof checking.
4. Ask before installing a toolchain, prover stack, or compatibility runtime.
   A port or custom server is a separate scope decision, not a silent replacement.

Once these prerequisites are met, draft the concrete launcher specification
using the compatible Zed SDK. Honor explicit executable, argument, environment,
and initialization-option overrides. Register it for `TLA+`, not TLC `.cfg`.

## Expected capabilities and limits

Upstream advertises diagnostics, proof-related code actions, proof-step rename,
and execute-command support. It does not advertise completion, definition
navigation, or standard hover. Verify actual behavior in Zed before promising
these features on Windows.

Custom TLAPM proof-marker notifications do not themselves create Zed proof
gutters or panels. Such UI is outside this scope.

## Verification

There is no runnable native-Windows TLAPM check to prescribe until a real backend
is identified. Do not invent an executable path or an installation command.

Acceptance requires a real server round trip and a manual Windows Zed check.
Report unavailable prerequisites as blocked, not passed. Keep the existing
syntax checks working independently.

## Independent deliverable

`SPEC-tlc-config.md` remains implementable on native Windows without TLAPM,
WSL, or a language-server launcher. Do not block syntax support on this research.

## Boundaries

- Never launch or install WSL, a Linux bridge, containers, or a remote backend.
- Never substitute SANY or another tool while calling it TLAPM proof checking.
- Never auto-install backend dependencies or modify global Zed settings.
- Never modify user specifications to test a launcher.
- Never touch `prototypes/counterexample-viewer/`.

## Sources

- Server CLI: https://github.com/tlaplus/tlapm/blob/main/lsp/bin/tlapm_lsp.ml
- Prover discovery: https://github.com/tlaplus/tlapm/blob/main/lsp/lib/prover/prover.ml
- Server capabilities: https://github.com/tlaplus/tlapm/blob/main/lsp/lib/server/handlers.ml
- Build prerequisites: https://github.com/tlaplus/tlapm/blob/main/DEVELOPING.md

## Open question

Can a genuine native Windows TLAPM server and proof stack be obtained or built?
Until verified, defer this integration rather than reintroducing the rejected
WSL approach.
