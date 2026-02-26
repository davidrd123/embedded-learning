#!/usr/bin/env bash
# Uniform wrapper for the Go splitter implementation.
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
GO_CACHE_DIR="${GOCACHE:-${TMPDIR:-/tmp}/go-build-cache}"

command -v go >/dev/null 2>&1 || {
    echo "error: go is required but not installed" >&2
    exit 1
}

mkdir -p "$GO_CACHE_DIR"
export GOCACHE="$GO_CACHE_DIR"

exec go run "${SCRIPT_DIR}/split-datasheet.go" "$@"
