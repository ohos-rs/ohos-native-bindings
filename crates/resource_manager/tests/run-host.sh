#!/usr/bin/env bash
set -euo pipefail
test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_output="$test_dir/../../../target/resource-host"
mkdir -p "$test_output"
rustc --edition 2021 --test "$test_dir/host.rs" -o "$test_output/tests"
"$test_output/tests" "$@"
