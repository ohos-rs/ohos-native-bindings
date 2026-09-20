#!/usr/bin/env bash
# Run ArkWeb against a real Web component in the main application process.
set -euo pipefail

OHOS_ARCH="${OHOS_ARCH:-arm64}"
while [ $# -gt 0 ]; do
  case "$1" in
    --)
      shift
      ;;
    --arch)
      if [ $# -lt 2 ]; then
        echo "error: --arch requires a value" >&2
        exit 2
      fi
      OHOS_ARCH="$2"
      shift 2
      ;;
    --arch=*)
      OHOS_ARCH="${1#--arch=}"
      shift
      ;;
    *)
      echo "error: unsupported argument '$1'" >&2
      exit 2
      ;;
  esac
done
case "$OHOS_ARCH" in
  arm64|aarch) OHOS_ARCH="arm64" ;;
  x86_64|x64) OHOS_ARCH="x64" ;;
  armv7a|arm) OHOS_ARCH="arm" ;;
  *) echo "error: unsupported architecture '$OHOS_ARCH'" >&2; exit 2 ;;
esac

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUNDLE="com.richerfu.ohos_example"
ABILITY="ArkWebE2ETestAbility"
MAIN_HAP="$ROOT/entry/build/default/outputs/default/entry-default-signed.hap"
DIAGNOSTICS_DIR="${E2E_DIAGNOSTICS_DIR:-$ROOT/.tools/e2e-diagnostics}"
LAYOUT_PATH="/data/local/tmp/ark-web-e2e-layout.json"
HDC=("${HDC_BIN:-hdc}")
if [ -n "${HDC_TARGET:-}" ]; then
  HDC+=( -t "$HDC_TARGET" )
fi
mkdir -p "$DIAGNOSTICS_DIR"

fail() {
  collect_diagnostics
  echo "::error::ArkWeb E2E failed: $*"
  echo "ArkWeb E2E failed: $*" >&2
  exit 1
}

collect_diagnostics() {
  local fault_paths="$DIAGNOSTICS_DIR/ark-web-faultlogger-paths.log"
  local remote_path
  local output_name

  "${HDC[@]}" shell 'hilog -x | tail -2000' \
    >"$DIAGNOSTICS_DIR/ark-web-hilog.log" 2>&1 || true
  "${HDC[@]}" shell 'ls -laR /data/log/faultlog 2>/dev/null' \
    >"$DIAGNOSTICS_DIR/ark-web-faultlogger-list.log" 2>&1 || true
  "${HDC[@]}" shell \
    "grep -R -l '$BUNDLE' /data/log/faultlog/faultlogger 2>/dev/null || true" \
    | tr -d '\r' >"$fault_paths" || true
  while IFS= read -r remote_path; do
    [ -n "$remote_path" ] || continue
    output_name="$(basename "$remote_path")"
    "${HDC[@]}" file recv "$remote_path" \
      "$DIAGNOSTICS_DIR/ark-web-faultlogger-$output_name" >/dev/null 2>&1 || true
  done <"$fault_paths"
}

dump_layout() {
  "${HDC[@]}" shell uitest dumpLayout -b "$BUNDLE" -p "$LAYOUT_PATH" >/dev/null
  "${HDC[@]}" shell cat "$LAYOUT_PATH" | tr -d '\r'
}

layout_status() {
  local layout="$1"
  jq -r \
    'first(.. | objects | select(.attributes?.id == "ark-web-e2e-status") | .attributes.text) // empty' \
    <<<"$layout"
}

if [ -z "${HAP_SIGN_DEVICE_ID:-}" ]; then
  HAP_SIGN_DEVICE_ID="$("${HDC[@]}" shell 'bm get --udid' | tr -d '\r' \
    | sed -nE 's/^[[:space:]]*([[:xdigit:]]{64})[[:space:]]*$/\1/p')"
fi
if [[ ! "$HAP_SIGN_DEVICE_ID" =~ ^[[:xdigit:]]{64}$ ]]; then
  fail "could not resolve the target device UDID for HAP signing"
fi
export HAP_SIGN_DEVICE_ID

echo "==> [ark_web] building main-process UI host"
pnpm --dir "$ROOT" --silent run build:hap
"${HDC[@]}" uninstall "$BUNDLE" >/dev/null 2>&1 || true
install_log="$DIAGNOSTICS_DIR/install-ark-web.log"
if ! "${HDC[@]}" install -r "$MAIN_HAP" >"$install_log" 2>&1 \
  || grep -Eqi '\[Fail\]|(^|[[:space:]])error:|failed to install' "$install_log" \
  || ! grep -Eqi 'success|successfully' "$install_log"; then
  fail "main HAP installation failed"
fi

"${HDC[@]}" shell "aa force-stop $BUNDLE" >/dev/null 2>&1 || true
"${HDC[@]}" shell 'hilog -r' >/dev/null 2>&1 || true
"${HDC[@]}" shell 'power-shell wakeup' >/dev/null 2>&1 || true
start_log="$DIAGNOSTICS_DIR/start-ark-web.log"
if ! "${HDC[@]}" shell "aa start -a $ABILITY -b $BUNDLE" >"$start_log" 2>&1 \
  || grep -Eqi '(^|[[:space:]])error:|failed to start|does not exist|not installed' "$start_log"; then
  fail "$ABILITY failed to start"
fi

last_layout=""
last_status=""
for _attempt in $(seq 1 45); do
  last_layout="$(dump_layout 2>/dev/null || true)"
  last_status="$(layout_status "$last_layout" 2>/dev/null || true)"
  case "$last_status" in
    ark-web-e2e-pass)
      printf '%s\n' "$last_layout" >"$DIAGNOSTICS_DIR/ark-web-layout-pass.json"
      collect_diagnostics
      echo "    ArkWeb proxy registration, refresh, and JavaScript invocation passed"
      exit 0
      ;;
    ark-web-e2e-fail:*)
      printf '%s\n' "$last_layout" >"$DIAGNOSTICS_DIR/ark-web-layout-fail.json"
      fail "application reported $last_status"
      ;;
  esac
  sleep 1
done

printf '%s\n' "$last_layout" >"$DIAGNOSTICS_DIR/ark-web-layout-last.json"
fail "$ABILITY did not report success (last status: ${last_status:-missing})"
