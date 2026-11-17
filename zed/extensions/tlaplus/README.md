# TLA+ for Zed

A syntax-only Zed extension for `.tla` files. It provides syntax highlighting
for ASCII and Unicode TLA+, line and block comments, bracket matching and
autoclosing, and basic indentation for definitions and bracketed expressions.

There is no language server, completion engine, formatter, or TLC integration.
TLC `.cfg` files and PlusCal-specific highlighting are not included.

## Install locally

1. Open Zed's command palette and run `zed: install dev extension`.
2. Select this directory, `zed/extensions/tlaplus`, containing `extension.toml`.
3. Open `examples/Counter.tla`. The language indicator should say `TLA+`.

Zed fetches the pinned grammar and compiles it for the extension. The first
installation needs network access. Zed downloads the WASI SDK automatically
if needed. See the [official development guide](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)
for toolchain requirements and troubleshooting through `zed: open log`.

`task run` opens this directory and the counter example in Zed. It does not
install the extension; complete the dev-extension installation first.

## Development commands

Local checks require [Task](https://taskfile.dev/), Git, Python 3.11 or newer,
the [Tree-sitter CLI](https://tree-sitter.github.io/tree-sitter/cli/installation.html),
and a native C compiler. The checks were verified with Tree-sitter CLI 0.26.3.
Run commands from this directory:

```sh
task          # List available tasks
task setup    # Fetch the grammar revision from extension.toml
task build    # Compile a native parser for local checks
task test     # Build and run the focused syntax checks
task run      # Open the project and example in Zed
task clean    # Remove downloaded grammar and generated builds
```

The native test library uses the filename `.cache/tlaplus.so` on all platforms,
including Windows. It is only a test artifact, not the grammar Zed installs.

## Verification

The tests parse the ASCII and Unicode examples, compile every query, and check
highlight captures, bracket pairs, indentation captures, comment scopes, and
language configuration. They do not launch Zed or validate its rendering and
editing behavior.

After installation, check these behaviors in Zed:

- `.tla` files select `TLA+`, and keywords, operators, strings, and comments have
  highlighting.
- Toggle line comments to insert and remove `\* `.
- Match `<< >>`, `⟨ ⟩`, `〈 〉`, ordinary brackets, and action brackets such as
  `[Next]_vars` and `<<Next>>_vars`.
- Insert a newline after `Init ==` to check basic indentation. Conjuncts at the
  same level should remain aligned.
- Type `count'`. The prime must not insert a closing single quote.

## Implementation

`extension.toml` pins the upstream grammar. `languages/tlaplus/config.toml`
defines file associations and editing settings. The `.scm` files define
highlighting, matching brackets, indentation, and string/comment scopes using
[Zed's language-extension format](https://zed.dev/docs/extensions/languages).
No Rust crate is needed for this syntax-only extension.

The grammar is [tree-sitter-tlaplus](https://github.com/tlaplus-community/tree-sitter-tlaplus).
Highlights adapt its query patterns to Zed's capture names and ordering.
See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for the upstream license.
