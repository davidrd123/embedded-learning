#!/usr/bin/env bash
# Uniform wrapper for the Rust splitter implementation.
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BIN="${TMPDIR:-/tmp}/split-datasheet-rs"
SRC="${SCRIPT_DIR}/split-datasheet.rs"

command -v rustc >/dev/null 2>&1 || {
    echo "error: rustc is required but not installed" >&2
    exit 1
}

if [[ ! -x "$BIN" || "$SRC" -nt "$BIN" ]]; then
    rustc -O -o "$BIN" "$SRC"
fi

exec "$BIN" "$@"
