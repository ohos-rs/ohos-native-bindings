#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_output="$test_dir/../../../target/arkui-host"
mkdir -p "$test_output"
rustc --edition 2021 --test "$test_dir/node_host.rs" -o "$test_output/node-tests"
"$test_output/node-tests" "$@"
rustc --edition 2021 --test "$test_dir/root_host.rs" -o "$test_output/root-tests"
"$test_output/root-tests" "$@"
