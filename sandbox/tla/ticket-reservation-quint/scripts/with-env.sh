#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

# Export in the actual subprocess. Task's own executable lookup and inherited
# environment can otherwise take precedence over a Taskfile-level PATH.
export PATH="$PWD/node_modules/.bin:$PATH"
export QUINT_HOME="$PWD/tools/quint"
export CARGO_HOME="$PWD/tools/cargo-home"
export CARGO_TARGET_DIR="$PWD/target"
export npm_config_cache="$PWD/tools/npm-cache"
export TMPDIR="$PWD/artifacts/tmp"
export JAVA_TOOL_OPTIONS="-Djava.io.tmpdir=$TMPDIR"
export QUINT_SEED="${QUINT_SEED:-42}"
mkdir -p "$CARGO_HOME" "$TMPDIR"
exec "$@"
