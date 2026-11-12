#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-run-javascript.XXXXXX")"
test_binary="$test_tmp/run-javascript"
trap 'rm -f "$test_binary"; rmdir "$test_tmp"' EXIT

# Compile the production controller wrapper against the mock directly, so a host
# linker does not inherit ohos-web-sys's unconditional dependency on libohweb.
rustc --edition=2021 --test "$test_dir/host/run_javascript.rs" -o "$test_binary"
"$test_binary" "$@"
