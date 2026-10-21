#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
workspace="$(cd -- "$test_dir/../../.." && pwd)"
test_output="$workspace/target/native-image-host"
mkdir -p "$test_output"
# Empty archives satisfy generated SDK link attributes; tests substitute only
# the native entry points reached by the production owner algorithms.
printf 'void ohos_host_anchor(void) {}\n' > "$test_output/anchor.c"
cc -c "$test_output/anchor.c" -o "$test_output/anchor.o"
for library in native_image native_window native_buffer; do
  ar rcs "$test_output/lib$library.a" "$test_output/anchor.o"
done
cd -- "$workspace"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_output" cargo test -p ohos-native-image-binding --no-default-features --lib "$@"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_output" cargo test -p ohos-native-image-binding --features api-13 --lib "$@"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_output" cargo test -p ohos-native-image-binding --no-default-features --doc "$@"
