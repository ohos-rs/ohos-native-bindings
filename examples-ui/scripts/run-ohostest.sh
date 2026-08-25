#!/usr/bin/env bash
# Run the ohosTest Hypium suite one binding module at a time.
#
# Why: every napi .so statically links napi-ohos whose lazy thread_locals
# consume pthread TLS keys at module registration (~30 keys per .so). With
# all 33 demo .so imported in one process the 1024-key pool is exhausted
# (measured: 10 keys left) and any later lazy TLS init aborts the app.
# Running each module in its own `aa test` process keeps per-process key
# usage tiny while still covering every binding.
#
# Usage:
#   scripts/run-ohostest.sh            # run every module
#   scripts/run-ohostest.sh sensor vsync  # run only the given modules
set -euo pipefail

# pnpm forwards the conventional argument separator to nested workspace
# scripts; do not treat it as a test module name.
if [ "${1:-}" = "--" ]; then
  shift
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BUNDLE="com.richerfu.h_openconnect"
DIAGNOSTICS_DIR="${E2E_DIAGNOSTICS_DIR:-$ROOT/.tools/e2e-diagnostics}"
MAIN_HAP="$ROOT/entry/build/default/outputs/default/entry-default-signed.hap"
TEST_HAP="$ROOT/entry/build/default/outputs/ohosTest/entry-ohosTest-signed.hap"
HDC=(hdc)
if [ -n "${HDC_TARGET:-}" ]; then
  HDC=(hdc -t "${HDC_TARGET}")
fi
TESTDIR="$ROOT/entry/src/ohosTest/ets/test"
LIST="$TESTDIR/List.test.ets"
LIST_BACKUP="$TESTDIR/.List.full.bak"
mkdir -p "$DIAGNOSTICS_DIR"

install_haps() {
  local label="$1"
  local install_log="$DIAGNOSTICS_DIR/install-$label.log"
  local summary
  local status

  set +e
  "${HDC[@]}" install -r "$MAIN_HAP" "$TEST_HAP" >"$install_log" 2>&1
  status=$?
  set -e
  cat "$install_log"
  if [ "$status" -ne 0 ] \
    || grep -Eqi '\[Fail\]|(^|[[:space:]])error:|failed to install' "$install_log" \
    || ! grep -Eqi 'success|successfully' "$install_log"; then
    summary="$(grep -Ei '\[Fail\]|(^|[[:space:]])error:|failed to install' "$install_log" | tail -1 || true)"
    echo "::error::HAP installation failed during $label: ${summary:-no success marker returned by hdc}"
    echo "HAP installation failed during $label" >&2
    return 1
  fi
}

start_gesture_host() {
  local label="$1"
  local start_log="$DIAGNOSTICS_DIR/start-$label.log"
  local status

  set +e
  "${HDC[@]}" shell "aa start -a GestureTestAbility -b $BUNDLE" >"$start_log" 2>&1
  status=$?
  set -e
  cat "$start_log"
  if [ "$status" -ne 0 ] \
    || grep -Eqi '(^|[[:space:]])error:|failed to start|does not exist|not installed' "$start_log"; then
    echo "::error::GestureTestAbility failed to start during $label"
    echo "GestureTestAbility failed to start during $label" >&2
    return 1
  fi
}

# module dir name -> test file stem (AbilityAccessControl.test.ets etc.)
declare -a MODULES=(
  ability_access_control:AbilityAccessControl
  ark_web:ArkWeb
  arkui:ArkUI
  arkui_input:ArkUIInput
  ashmem:Ashmem
  asset:Asset
  bundle:Bundle
  camera:Camera
  display:Display
  display_soloist:DisplaySoloist
  drawing:Drawing
  fileshare:FileShare
  fileuri:FileUri
  hilog:Hilog
  huks:Huks
  image:Image
  image_native:ImageNative
  ime:Ime
  init:Init
  jsvm:Jsvm
  native_buffer:NativeBuffer
  native_window:NativeWindow
  net_connection:NetConnection
  net_stack:NetStack
  pasteboard:Pasteboard
  qos:Qos
  raw:Raw
  sensor:Sensor
  udmf:Udmf
  vibrator:Vibrator
  vsync:Vsync
  xcomponent:XComponent
)

# Keep a pristine copy of the full List once.
if [ ! -f "$LIST_BACKUP" ]; then
  cp "$LIST" "$LIST_BACKUP"
fi

restore_list() {
  cp "$LIST_BACKUP" "$LIST"
}
trap restore_list EXIT

selected=()
if [ $# -gt 0 ]; then
  for want in "$@"; do
    hit=""
    for m in "${MODULES[@]}"; do
      if [ "${m%%:*}" = "$want" ]; then hit="$m"; break; fi
    done
    if [ -z "$hit" ]; then
      echo "error: unknown module '$want'" >&2
      exit 1
    fi
    selected+=("$hit")
  done
else
  selected=("${MODULES[@]}")
fi

# Gesture modules need a live main-module surface in the same bundle process.
needs_gesture_host=0
for m in "${selected[@]}"; do
  case "${m%%:*}" in
    arkui|xcomponent) needs_gesture_host=1 ;;
  esac
done
if [ "$needs_gesture_host" -eq 1 ]; then
  echo "==> building main HAP for automatic gesture host"
  pnpm --silent run build:hap
  # Start from a clean signer/provision state. The SDK-supplied OpenHarmony
  # certificate cache may have been regenerated since an older test install.
  "${HDC[@]}" uninstall "$BUNDLE" >/dev/null 2>&1 || true
fi

# Make sure the latest ohosTest HAP (with the full List) is installed once.
echo "==> building ohosTest HAP (full List)"
pnpm --silent run build:test
install_haps full

total_pass=0
total_fail=0
failed_modules=()

for m in "${selected[@]}"; do
  name="${m%%:*}"
  stem="${m##*:}"

  echo "==> [$name] generating single-module List"
  fn="$(echo "$name" | tr -c 'a-z0-9_' '_' | sed 's/_\{2,\}/_/g')Test"
  cat > "$LIST" <<EOF
import $fn from "./modules/${stem}.test";

export default function testsuite(): void {
  $fn();
}
EOF

  # hvigor's incremental build does not notice List.test.ets being
  # replaced by the driver; drop the compile caches so the abc is
  # regenerated with the trimmed List. The abc statically contains every
  # module (ETS compiles the whole source set) but napi .so modules only
  # load+register when their import executes, so only the modules named in
  # the trimmed List consume pthread TLS keys.
  rm -rf entry/build/default/intermediates/loader \
         entry/build/default/intermediates/loader_out
  build_log="$DIAGNOSTICS_DIR/build-test-$name.log"
  if ! pnpm --silent run build:test >"$build_log" 2>&1; then
    tail -200 "$build_log" >&2 || true
    echo "::error::ohosTest HAP build failed for $name"
    echo "    BUILD FAILED for $name" >&2
    total_fail=$((total_fail + 1))
    failed_modules+=("$name(build)")
    continue
  fi
  if ! install_haps "$name"; then
    total_fail=$((total_fail + 1))
    failed_modules+=("$name(install)")
    continue
  fi
  "${HDC[@]}" shell "aa force-stop $BUNDLE" >/dev/null 2>&1 || true

  case "$name" in
    arkui|xcomponent)
      echo "==> [$name] starting automatic gesture host"
      if ! start_gesture_host "$name"; then
        total_fail=$((total_fail + 1))
        failed_modules+=("$name(start)")
        continue
      fi
      HDC_TARGET="${HDC_TARGET:-}" "$ROOT/scripts/inject-xcomponent-gestures.sh"
      ;;
  esac

  echo "==> [$name] aa test"
  log="$(mktemp)"
  if ! "${HDC[@]}" shell "aa test -b $BUNDLE -m entry_test -s unittest OpenHarmonyTestRunner -s timeout 120000" >"$log" 2>&1; then
    :
  fi
  cp "$log" "$DIAGNOSTICS_DIR/ohostest-$name.log"
  pass=$(grep -c 'OHOS_REPORT_STATUS_CODE: 0' "$log" || true)
  fail=$(grep -cE 'OHOS_REPORT_STATUS_CODE: (-1|-2)' "$log" || true)
  pass=${pass:-0}
  fail=${fail:-0}
  echo "    pass=$pass fail=$fail"
  total_pass=$((total_pass + pass))
  total_fail=$((total_fail + fail))
  if [ "$fail" -gt 0 ] || ! grep -q "TestFinished" "$log"; then
    failed_modules+=("$name")
    cp "$log" "$ROOT/ohostest-$name.log"
  fi
  rm -f "$log"
done

# Restore the full List so the repo stays pristine.
restore_list
trap - EXIT

echo
echo "==== ohosTest summary: pass=$total_pass fail=$total_fail modules=${#selected[@]} ===="
if [ ${#failed_modules[@]} -gt 0 ]; then
  echo "::error::Failed E2E modules: ${failed_modules[*]}"
  echo "FAILED modules: ${failed_modules[*]}" >&2
  exit 1
fi
