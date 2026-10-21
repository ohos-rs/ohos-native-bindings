#!/usr/bin/env bash
set -euo pipefail
test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_output="$test_dir/../../../target/pasteboard-host"
mkdir -p "$test_output"
rustc --edition 2021 --test "$test_dir/native_data_host.rs" -o "$test_output/api-13"
"$test_output/api-13" "$@"
rustc --edition 2021 --test --cfg 'feature="api-14"' "$test_dir/native_data_host.rs" -o "$test_output/api-14"
"$test_output/api-14" "$@"
rustc --edition 2021 --test --cfg 'feature="api-14"' --cfg 'feature="api-18"' "$test_dir/native_data_host.rs" -o "$test_output/api-18"
"$test_output/api-18" "$@"
