#!/usr/bin/env bash
set -euo pipefail
ime_test_dir="$(cd "$(dirname "$0")" && pwd)"
ime_mock_dir="$(mktemp -d "${TMPDIR:-/tmp}/ime-native-mock.XXXXXX")"
trap 'rm -rf "$ime_mock_dir"' EXIT
cc -std=c11 -Wall -Wextra -Werror -c "$ime_test_dir/native/ime.c" -o "$ime_mock_dir/ime.o"
ar rcs "$ime_mock_dir/libohinputmethod.a" "$ime_mock_dir/ime.o"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$ime_mock_dir" cargo test \
  --manifest-path "$ime_test_dir/../Cargo.toml" --lib --test lifecycle "$@"
