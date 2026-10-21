#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
workspace="$(cd -- "$test_dir/../../.." && pwd)"
sdk_native="${OHOS_NDK_HOME:?Set OHOS_NDK_HOME to the SDK root or native directory}"
if [[ -d "$sdk_native/native/sysroot" ]]; then
  sdk_native="$sdk_native/native"
fi
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-native-image-egl.XXXXXX")"
trap 'rm -rf "$test_tmp"' EXIT
# Use the official header to type-check the statically linked EGL substitutes.
cc -idirafter "$sdk_native/sysroot/usr/include" -c "$test_dir/egl_host.c" -o "$test_tmp/egl.o"
ar rcs "$test_tmp/libegl_host.a" "$test_tmp/egl.o"
for library in native_image native_window native_buffer; do
  ar rcs "$test_tmp/lib$library.a" "$test_tmp/egl.o"
done
cd -- "$workspace"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_tmp -l static=egl_host" cargo test -p ohos-native-image-binding --features texture-readback --lib "$@"
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_tmp -l static=egl_host" cargo test -p ohos-native-image-binding --features texture-readback --doc "$@"
