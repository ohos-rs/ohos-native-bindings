#!/usr/bin/env bash
set -euo pipefail

test_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_tmp="$(mktemp -d "${TMPDIR:-/tmp}/ohos-cloned-pointer.XXXXXX")"
trap 'rm -rf "$test_tmp"' EXIT

# The integration test supplies native symbols. An empty host archive satisfies
# the sys crate's unconditional link attribute without linking an OHOS library.
printf 'void ohos_input_test_link_anchor(void) {}\n' > "$test_tmp/anchor.c"
"${CC:-cc}" -c "$test_tmp/anchor.c" -o "$test_tmp/anchor.o"
"${AR:-ar}" rcs "$test_tmp/libace_ndk.z.a" "$test_tmp/anchor.o"
host_target="$(rustc -vV | sed -n 's/^host: //p')"

cd "$test_dir/../../.."
RUSTFLAGS="${RUSTFLAGS:-} -L native=$test_tmp" cargo test \
  -p ohos-arkui-input-binding --features api-24 --target "$host_target" \
  --test cloned_pointer "$@"
