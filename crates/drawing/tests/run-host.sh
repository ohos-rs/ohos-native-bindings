#!/usr/bin/env bash
set -euo pipefail
test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-drawing-host.XXXXXX")"
trap 'rm -rf "$test_tmp"' EXIT
rustc --edition 2021 --test "$test_dir/filter_matrix_host.rs" -o "$test_tmp/filter-matrix"
"$test_tmp/filter-matrix" "$@"
