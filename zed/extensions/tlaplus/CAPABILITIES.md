# TLA+ extension capability map

Status: Windows-only scope. TLC configuration implementation is pending;
TLAPM integration is blocked on a verified native Windows backend.

The requested additions are TLC configuration syntax and TLAPM integration.
Zed, project files, and backend processes must run on Windows. The user explicitly
rejected WSL. Do not substitute a Linux, container, or remote backend.

| Module id | Responsibility | Depends on |
|---|---|---|
| tlc-config | Recognize and highlight TLC model configuration files | Existing extension and a separate pinned Tree-sitter grammar |
| tlapm-lsp | Integrate TLAPM only after a native Windows backend is verified | A working Windows TLAPM language server and proof-manager installation; currently unverified |

The modules do not depend on one another. TLC configuration support can proceed
without a language server or runtime backend. Do not register or advertise TLAPM
as working until the native backend has been verified. Any eventual TLAPM server
applies to TLA+ modules, not TLC configuration files.

## Specifications

- `SPEC-tlc-config.md`
- `SPEC-tlapm-lsp.md`

## Scope boundaries

- Preserve existing `.tla` syntax support.
- Keep Windows Zed and local-drive project paths native.
- Do not use WSL, a Linux bridge, containers, or a remote server.
- Do not install TLAPS, OCaml packages, provers, or compatibility runtimes automatically.
- Do not promise custom proof gutters, a proof panel, or capabilities the server
  does not advertise.
- Do not modify unrelated TLA+ specifications in the parent repository.
