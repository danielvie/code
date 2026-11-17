# Spec: TLC configuration syntax

Status: proposed, awaiting approval.
Module id: `tlc-config`.
Parent scope: `CAPABILITIES.md`.

## Objective

Recognize TLC model configuration files as a separate Zed language named
`TLC configuration`, without changing the existing TLA+ module grammar.
Provide syntax highlighting, line comments, set/string bracket matching, and
basic indentation. This works in native Windows Zed without WSL or a language
server. This is editor syntax support, not TLC validation.

## Stack and dependency

Use `Akanoa/tree-sitter-tlaplus-cfg`, grammar name `tlaplus_cfg`, pinned to
`afc588ce622ad71ab0bd08f6dd4e0b80bc8beb9c`. Its generated parser uses ABI 14,
has no external scanner, and is BSD-3-Clause licensed. Preserve attribution.

The initial release covers the upstream grammar's common directives,
multiline name lists, assignments, operator substitutions, model values,
unsigned integers, strings, booleans, nested sets, and `\*` line comments.

The upstream grammar does not cover all TLC syntax. Known gaps include block
comments, module-qualified substitutions/assignments, qualified identifiers,
operator-table entries, and plural `POSTCONDITIONS`. It also accepts some
constructs that TLC rejects. Document these limits; do not present its parse
results as semantic validation or silently patch a downloaded grammar.
Sequences and general TLA+ expressions are not TLC configuration literals.

## Project structure

- `extension.toml`: register the second grammar.
- `languages/tlc_config/`: configuration and highlighting, brackets, indentation,
  and scope queries, limited to constructs exposed by the grammar.
- `examples/Counter.cfg`: a model configuration paired with `Counter.tla`.
- `tests/test_tlc_config.py`: focused parser/query/configuration checks.
- `Taskfile.yml`: fetch/build both pinned grammars and expose syntax checks.
- `README.md` and `THIRD_PARTY_NOTICES.md`: usage, limitations, attribution.

## Commands

The following are target commands to add or update during implementation:

```sh
task setup
task build
task test:syntax
task test
task run
```

Keep the existing default task listing and `clean` task. These commands are
scoped to this extension, not other projects in the parent repository.

## Code style

Use the existing TOML and Tree-sitter query conventions. Capture names must
match Zed's documented categories, for example:

```scheme
(keyword) @keyword
(boolean) @boolean
```

Avoid new runtime code for declarative syntax features. Keep test dependencies
in Python's standard library and the existing Tree-sitter CLI.

## Verification and success criteria

- `.cfg` files select `TLC configuration`; document the generic suffix collision
  and how users can override associations for unrelated configuration formats.
- `.tla` files continue selecting `TLA+` with the current grammar and queries.
- Parse fixtures for supported directives, multiline constant/list sections,
  nested sets, strings, substitutions, empty files, and final line comments
  without a newline.
- Compile every new query and assert representative highlight/bracket captures.
- Check a configuration matching the existing `DieHard.cfg` syntax without
  modifying that project.
- Document unsupported grammar forms rather than hiding failing fixtures.
- TLAPM is not registered for `TLC configuration`.
- Manually verify highlighting and editing behavior after installing the dev
  extension in Zed; distinguish this from native parser tests.

## Boundaries

- Always preserve existing TLA+ support and pin the grammar revision.
- Always retain third-party licensing and document parser limitations.
- Ask before publishing a fork or changing the scope to a new custom grammar.
- Never modify downloaded parsers under `grammars/` as maintained source.
- Never add TLC execution, model checking, or configuration validation here.
- Never touch `prototypes/counterexample-viewer/` or unrelated model files.

## Sources

- Grammar: https://github.com/Akanoa/tree-sitter-tlaplus-cfg/blob/afc588ce622ad71ab0bd08f6dd4e0b80bc8beb9c/grammar.js
- Actual TLC configuration parser: https://github.com/tlaplus/tlaplus/blob/master/tlatools/org.lamport.tlatools/src/tlc2/tool/impl/ModelConfig.java
- Zed language format: https://zed.dev/docs/extensions/languages

## Open questions

No additional product choice is needed for this initial, explicitly limited
syntax release. Full TLC grammar coverage would be a separate upstream/fork
change requiring approval.
