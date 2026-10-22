#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_output="$test_dir/../../../target/arkui-host"
mkdir -p "$test_output"
rustc --edition 2021 --test "$test_dir/host.rs" -o "$test_output/tests"
"$test_output/tests" "$@"
rustc --edition 2021 --cfg 'feature="api-13"' --test "$test_dir/host.rs" -o "$test_output/tests-api-13"
"$test_output/tests-api-13" "$@"
rustc --edition 2021 --test "$test_dir/node_host.rs" -o "$test_output/node-tests"
"$test_output/node-tests" "$@"
rustc --edition 2021 --test "$test_dir/root_host.rs" -o "$test_output/root-tests"
"$test_output/root-tests" "$@"
rustc --edition 2021 --test "$test_dir/dialog_host.rs" -o "$test_output/dialog-tests"
"$test_output/dialog-tests" "$@"
