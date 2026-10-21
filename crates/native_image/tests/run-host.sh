#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
workspace="$(cd -- "$test_dir/../../.." && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-native-image-host.XXXXXX")"
trap 'rm -rf "$test_tmp"' EXIT
# Empty archives satisfy generated SDK link attributes; tests substitute only
# the native entry points reached by the production owner algorithms.
printf 'void ohos_host_anchor(void) {}\n' > "$test_tmp/anchor.c"
cc -c "$test_tmp/anchor.c" -o "$test_tmp/anchor.o"
for library in native_image native_window native_buffer; do
  ar rcs "$test_tmp/lib$library.a" "$test_tmp/anchor.o"
done
cd -- "$workspace"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_tmp" cargo test -p ohos-native-image-binding --no-default-features --lib "$@"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_tmp" cargo test -p ohos-native-image-binding --no-default-features --doc "$@"
