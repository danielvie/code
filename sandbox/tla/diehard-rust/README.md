# DieHard with a Rust TLA+ checker

Runs the existing DieHard TLA+ example through `tla-checker` 0.24.9 as a Rust library. No Java, TLC installation, or separate `tla` executable is required.

The runner uses local copies at `model/DieHard.tla` and `model/DieHard.cfg`, initially copied from the original TLA+ tools distribution. The model is not rewritten in Rust. Paths are resolved from the Cargo project directory, not the shell's working directory. The model and configuration are read at runtime.

## Run

Requires Rust with edition 2024 support and Cargo. The first build downloads dependencies. Task is optional.

```sh
cargo run --locked
cargo test --locked
```

Or use `task run`, `task test`, `task build`, `task check`, and `task clean`. Run `task` to list commands.

## What the result means

The model describes a five-gallon jug, a three-gallon jug, and six actions for filling, emptying, and pouring water.

The original configuration checks two invariants:

- `TypeOK`: the jug volumes stay within capacity.
- `NotSolved`: the big jug never contains four gallons.

`NotSolved` is intentionally false for a reachable state. Its counterexample is the puzzle solution, not a checker failure. The runner treats this specific violation as success and prints the trace. An unexpected result is an error.

The tests also run with only `TypeOK`, so the checker can explore the entire reachable state space without stopping when the puzzle is solved.

## Checker documentation

- Library API: https://docs.rs/tla-checker/0.24.9/tla_checker/
- Source: https://github.com/fabracht/tla-rs
- Supported TLA+ subset: https://github.com/fabracht/tla-rs/blob/main/TLA_SUPPORT.md

`tla-checker` is not fully TLC-compatible. This project exercises the supplied DieHard example; it does not establish compatibility with arbitrary TLA+ specifications.
