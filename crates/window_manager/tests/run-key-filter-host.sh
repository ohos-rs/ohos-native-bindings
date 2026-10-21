#!/usr/bin/env bash
set -euo pipefail
test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-window-filter-host.XXXXXX")"
trap 'rm -rf "$test_tmp"' EXIT
rustc --edition 2021 --test "$test_dir/key_filter_host.rs" -o "$test_tmp/tests"
"$test_tmp/tests" "$@"
