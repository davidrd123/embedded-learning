#!/usr/bin/env bash
# Uniform wrapper for the Clojure splitter implementation.
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

command -v clojure >/dev/null 2>&1 || {
    echo "error: clojure is required but not installed" >&2
    exit 1
}

exec clojure "${SCRIPT_DIR}/split-datasheet.clj" "$@"
