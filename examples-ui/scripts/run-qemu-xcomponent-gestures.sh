#!/usr/bin/env bash
# Build the two Rust demos and both HAPs, install them, start the dedicated
# gesture host, inject input, and run the matching Hypium assertions.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORKSPACE_ROOT="$(cd "$ROOT/.." && pwd)"
HDC_TARGET="${HDC_TARGET:-127.0.0.1:5555}"
export HDC_TARGET

fail() {
  echo "QEMU XComponent gesture E2E failed: $*" >&2
  exit 1
}

output="$(hdc tconn "$HDC_TARGET" 2>&1 || true)"
case "$output" in
  *"[Fail]"*) fail "cannot connect $HDC_TARGET: $output" ;;
esac
hdc -t "$HDC_TARGET" shell true >/dev/null 2>&1 || fail "$HDC_TARGET is not ready"

guest_abi="$(hdc -t "$HDC_TARGET" shell param get const.product.cpu.abilist | tr -d '\r')"
case "$guest_abi" in
  *x86_64*) export OHOS_ARCH=x86_64 ;;
  *arm64-v8a*|*aarch64*) export OHOS_ARCH=arm64 ;;
  *) fail "unsupported QEMU ABI: $guest_abi" ;;
esac

echo "==> QEMU target=$HDC_TARGET ABI=$guest_abi"
cd "$WORKSPACE_ROOT"
pnpm install --frozen-lockfile
pnpm --filter @ohos-rs/examples-ui run sync:rust -- arkui xcomponent xcomponent_multi
pnpm --filter @ohos-rs/examples-ui run test -- arkui xcomponent
