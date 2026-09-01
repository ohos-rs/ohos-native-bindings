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

if [ "${1:-}" = "--" ]; then
  shift
fi
OHOS_ARCH="${OHOS_ARCH:-arm64}"
case "${1:-}" in
  --arch)
    if [ $# -lt 2 ]; then
      fail "--arch requires a value"
    fi
    OHOS_ARCH="$2"
    shift 2
    ;;
  --arch=*)
    OHOS_ARCH="${1#--arch=}"
    shift
    ;;
esac
case "$OHOS_ARCH" in
  arm64|aarch) OHOS_ARCH="arm64" ;;
  x86_64|x64) OHOS_ARCH="x64" ;;
  *) fail "unsupported architecture '$OHOS_ARCH' (expected arm64 or x64)" ;;
esac
if [ $# -gt 0 ]; then
  fail "unexpected argument: $1"
fi

output="$(hdc tconn "$HDC_TARGET" 2>&1 || true)"
case "$output" in
  *"[Fail]"*) fail "cannot connect $HDC_TARGET: $output" ;;
esac
hdc -t "$HDC_TARGET" shell true >/dev/null 2>&1 || fail "$HDC_TARGET is not ready"

echo "==> QEMU target=$HDC_TARGET arch=$OHOS_ARCH"
cd "$WORKSPACE_ROOT"
pnpm install --frozen-lockfile
pnpm --filter @ohos-rs/examples-ui run sync:rust -- --arch "$OHOS_ARCH" --fail-fast arkui xcomponent xcomponent_multi
pnpm --filter @ohos-rs/examples-ui run test -- --arch "$OHOS_ARCH" --fail-fast arkui xcomponent
