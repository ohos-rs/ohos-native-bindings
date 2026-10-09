#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-http-body-stream.XXXXXX")"
test_binary="$test_tmp/http-body-stream"
test_library="$test_tmp/libohos_web_binding.rlib"
trap 'rm -f "$test_binary" "$test_library"; rmdir "$test_tmp"' EXIT

# Compile the production reader against the mock directly, so a host linker does
# not inherit ohos-web-sys's unconditional dependency on the native ohweb library.
rustc --edition=2021 --test "$test_dir/host/http_body_stream.rs" -o "$test_binary"
"$test_binary" "$@"

# Check the actual consuming API: submitting the same handle twice must not compile.
rustc --edition=2021 --crate-type=rlib --crate-name ohos_web_binding \
  "$test_dir/host/http_body_stream.rs" -o "$test_library"
rustdoc --edition=2021 --test "$test_dir/host/http_body_stream.rs" \
  --extern "ohos_web_binding=$test_library"
