#!/usr/bin/env bash
# Build with the workspace-pinned arkdown, then sign with the Rust hapsigner.
# Missing Rust tools are installed under .tools/ and never written globally.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUILD_KIND="${1:-main}"
BUNDLE="com.richerfu.h_openconnect"
HAP_SIGN_VERSION="0.2.0"
OHPM_RS_VERSION="0.2.0"

fail() {
  echo "HAP build failed: $*" >&2
  exit 1
}

find_sdk() {
  local candidate
  for candidate in \
    "${OHOS_SDK_HOME:-}" \
    "${OHOS_BASE_SDK_HOME:-}" \
    "${DEVECO_SDK_HOME:-}" \
    "${OHOS_NDK_HOME:-}" \
    "/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony"; do
    if [ -n "$candidate" ] && [ -d "$candidate/ets" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done
  return 1
}

SDK_ROOT="$(find_sdk)" || fail \
  "OpenHarmony SDK not found; set OHOS_SDK_HOME or OHOS_NDK_HOME"
export OHOS_SDK_HOME="$SDK_ROOT"

find_hap_sign() {
  local tool_root="$ROOT/.tools/hapsigner-$HAP_SIGN_VERSION"
  if [ -n "${HAP_SIGN:-}" ] && [ -x "$HAP_SIGN" ]; then
    printf '%s\n' "$HAP_SIGN"
  elif command -v hap-sign >/dev/null 2>&1; then
    command -v hap-sign
  elif [ -x "$tool_root/bin/hap-sign" ]; then
    printf '%s\n' "$tool_root/bin/hap-sign"
  elif command -v cargo >/dev/null 2>&1; then
    mkdir -p "$tool_root"
    cargo install hapsigner --version "$HAP_SIGN_VERSION" --locked --root "$tool_root" >&2
    printf '%s\n' "$tool_root/bin/hap-sign"
  else
    return 1
  fi
}

supports_install() {
  local binary="$1"
  local version
  version="$("$binary" --version 2>/dev/null | awk '{ print $2 }')"
  [[ "$version" =~ ^([0-9]+)\.([0-9]+) ]] || return 1
  ((BASH_REMATCH[1] > 0 || BASH_REMATCH[2] >= 2))
}

find_ohpm() {
  local tool_root="$ROOT/.tools/ohpm-rs-$OHPM_RS_VERSION"
  local candidate

  if [ -n "${OHPM_RS:-}" ]; then
    candidate="$OHPM_RS"
    supports_install "$candidate" || return 1
    printf '%s\n' "$candidate"
  elif command -v ohpm-rs >/dev/null 2>&1 && supports_install "$(command -v ohpm-rs)"; then
    command -v ohpm-rs
  elif [ -x "$tool_root/bin/ohpm-rs" ]; then
    printf '%s\n' "$tool_root/bin/ohpm-rs"
  elif command -v cargo >/dev/null 2>&1; then
    mkdir -p "$tool_root"
    cargo install ohpm-cli --version "$OHPM_RS_VERSION" --locked --root "$tool_root" >&2
    printf '%s\n' "$tool_root/bin/ohpm-rs"
  else
    return 1
  fi
}

cd "$ROOT"
case "$BUILD_KIND" in
  main)
    pnpm exec arkdown build --project . --target hap --mode debug
    OUTPUT_DIR="$ROOT/entry/build/default/outputs/default"
    UNSIGNED_HAP="$OUTPUT_DIR/entry-default-unsigned.hap"
    SIGNED_HAP="$OUTPUT_DIR/entry-default-signed.hap"
    ;;
  test)
    if [ ! -f "$ROOT/oh_modules/@ohos/hypium/oh-package.json5" ]; then
      OHPM_BIN="$(find_ohpm)" || fail "ohpm-rs 0.2+ is required to install @ohos/hypium"
      "$OHPM_BIN" install
    fi
    pnpm exec arkdown build --project . --target hap --mode debug \
      --module entry --build-target-name ohosTest
    OUTPUT_DIR="$ROOT/entry/build/default/outputs/ohosTest"
    UNSIGNED_HAP="$OUTPUT_DIR/entry-ohosTest-unsigned.hap"
    SIGNED_HAP="$OUTPUT_DIR/entry-ohosTest-signed.hap"
    ;;
  *)
    fail "unknown build kind '$BUILD_KIND' (expected main or test)"
    ;;
esac

[ -f "$UNSIGNED_HAP" ] || fail "unsigned HAP was not produced: $UNSIGNED_HAP"

HAP_SIGN_BIN="$(find_hap_sign)" || fail \
  "hap-sign not found and cargo is unavailable"
"$HAP_SIGN_BIN" sign "$UNSIGNED_HAP" \
  --bundle-name "$BUNDLE" \
  --compatible-version 17 \
  --output "$SIGNED_HAP" \
  --force

echo "signed HAP: ${SIGNED_HAP#$ROOT/}"
