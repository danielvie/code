#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
# Keep generated Cargo files inside the exercise.
export CARGO_HOME="$PWD/tools/cargo-home"
export CARGO_TARGET_DIR="$PWD/target"
mkdir -p "$CARGO_HOME" artifacts
python3 scripts/check_model.py
cargo fmt --all -- --check
cargo clippy --offline --all-targets -- -D warnings
cargo test --offline -- --nocapture
