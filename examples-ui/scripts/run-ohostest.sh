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
#   scripts/run-ohostest.sh                         # run every module for arm64
#   scripts/run-ohostest.sh --arch x64              # run every module for x64
#   scripts/run-ohostest.sh --arch x64 sensor vsync # run selected modules for x64
#   scripts/run-ohostest.sh --fail-fast sensor vsync # stop after the first failed module
set -euo pipefail

# pnpm forwards the conventional argument separator to nested workspace
# scripts; do not treat it as a test module name.
OHOS_ARCH="${OHOS_ARCH:-arm64}"
FAIL_FAST=0
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
    --fail-fast)
      FAIL_FAST=1
      shift
      ;;
    --*)
      echo "error: unsupported option '$1'" >&2
      exit 2
      ;;
    *)
      break
      ;;
  esac
done
case "$OHOS_ARCH" in
  arm64|aarch) OHOS_ARCH="arm64" ;;
  x86_64|x64) OHOS_ARCH="x64" ;;
  armv7a|arm) OHOS_ARCH="arm" ;;
  *) echo "error: unsupported architecture '$OHOS_ARCH' (expected arm64, armv7a, or x64)" >&2; exit 2 ;;
esac

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BUNDLE="com.richerfu.ohos_example"
DIAGNOSTICS_DIR="${E2E_DIAGNOSTICS_DIR:-$ROOT/.tools/e2e-diagnostics}"
MAIN_HAP="$ROOT/entry/build/default/outputs/default/entry-default-signed.hap"
TEST_HAP="$ROOT/entry/build/default/outputs/ohosTest/entry-ohosTest-signed.hap"
HDC=(hdc)
if [ -n "${HDC_TARGET:-}" ]; then
  HDC=(hdc -t "${HDC_TARGET}")
fi
# Debug profiles must authorize the device on which these HAPs will run.
if [ -z "${HAP_SIGN_DEVICE_ID:-}" ]; then
  HAP_SIGN_DEVICE_ID="$("${HDC[@]}" shell 'bm get --udid' | tr -d '\r' \
    | sed -nE 's/^[[:space:]]*([[:xdigit:]]{64})[[:space:]]*$/\1/p')"
fi
if [[ ! "$HAP_SIGN_DEVICE_ID" =~ ^[[:xdigit:]]{64}$ ]]; then
  echo "error: could not resolve the target device UDID for HAP signing" >&2
  exit 1
fi
export HAP_SIGN_DEVICE_ID
TESTDIR="$ROOT/entry/src/ohosTest/ets/test"
LIST="$TESTDIR/List.test.ets"
mkdir -p "$DIAGNOSTICS_DIR"
LIST_BACKUP="$(mktemp "$DIAGNOSTICS_DIR/List.test.ets.XXXXXX")"

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

run_ohostest() {
  local label="$1"
  local log="$2"
  local attempt_log
  local attempt
  local class_filter=""
  local status=1

  if [ "$label" = native_child_process ]; then
    class_filter="-s class native_child_process_extended"
  fi

  for attempt in 1 2 3; do
    attempt_log="$DIAGNOSTICS_DIR/ohostest-$label-attempt-$attempt.log"
    "${HDC[@]}" shell "power-shell wakeup" >/dev/null 2>&1 || true
    set +e
    "${HDC[@]}" shell "aa test -b $BUNDLE -m entry_test -s unittest OpenHarmonyTestRunner -s timeout 120000 $class_filter" \
      >"$attempt_log" 2>&1
    status=$?
    set -e
    cp "$attempt_log" "$log"
    if ! grep -Eqi 'screen is locked|unlock screen failed' "$attempt_log"; then
      return "$status"
    fi
    if [ "$attempt" -lt 3 ]; then
      echo "    screen is locked; waking, unlocking, and retrying aa test ($attempt/3)"
      "${HDC[@]}" shell "power-shell wakeup" >/dev/null 2>&1 || true
      "${HDC[@]}" shell "uitest uiInput swipe 400 450 400 100 600" >/dev/null 2>&1 || true
      sleep 2
    fi
  done
  return "$status"
}

start_gesture_host() {
  local label="$1"
  local start_log="$DIAGNOSTICS_DIR/start-$label.log"
  local attempt_log
  local attempt
  local status

  # Wake immediately before each launch as a second guard after the timeout
  # override applied before the potentially long native and HAP builds.
  : >"$start_log"

  for attempt in 1 2 3; do
    attempt_log="$DIAGNOSTICS_DIR/start-$label-attempt-$attempt.log"
    "${HDC[@]}" shell "power-shell wakeup" >/dev/null 2>&1 || true
    if [ "$attempt" -gt 1 ]; then
      sleep 5
    fi

    set +e
    "${HDC[@]}" shell "aa start -a GestureTestAbility -b $BUNDLE" >"$attempt_log" 2>&1
    status=$?
    set -e
    cat "$attempt_log" | tee -a "$start_log"
    if [ "$status" -eq 0 ] \
      && ! grep -Eqi '(^|[[:space:]])error:|failed to start|does not exist|not installed' "$attempt_log"; then
      return 0
    fi
    if ! grep -Eqi 'screen is locked|unlock screen failed' "$attempt_log"; then
      break
    fi
    if [ "$attempt" -lt 3 ]; then
      echo "    screen is still locked; retrying ability start ($attempt/3)"
    fi
  done

  echo "::error::GestureTestAbility failed to start during $label"
  echo "GestureTestAbility failed to start during $label" >&2
  return 1
}

# module dir name -> test file stem (AbilityAccessControl.test.ets etc.)
declare -a MODULES=(
  ability_access_control:AbilityAccessControl
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
  native_child_process:NativeChildProcess
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
  window_manager:WindowManager
  xcomponent:XComponent
)

# Keep a per-run pristine copy so an old local backup cannot restore a stale
# module list after the selected-module runner exits.
cp "$LIST" "$LIST_BACKUP"

restore_list() {
  cp "$LIST_BACKUP" "$LIST"
}

screen_timeout_overridden=0
prepare_gesture_device() {
  if "${HDC[@]}" shell "power-shell timeout -o 86400000" >/dev/null 2>&1; then
    screen_timeout_overridden=1
  fi
  "${HDC[@]}" shell "power-shell wakeup" >/dev/null 2>&1 || true
}

verify_gesture_libraries() {
  local hap="$1"
  local abi_dir
  local entries
  local library

  case "$OHOS_ARCH" in
    arm64|aarch) abi_dir="arm64-v8a" ;;
    x86_64|x64) abi_dir="x86_64" ;;
    arm|armv7a) abi_dir="armeabi-v7a" ;;
    *) echo "error: unsupported architecture '$OHOS_ARCH'" >&2; return 1 ;;
  esac
  entries="$(unzip -Z1 "$hap")"
  for library in libarkui_test.so libxcomponent_test.so libxcomponent_multi_test.so; do
    if ! grep -Fxq "libs/$abi_dir/$library" <<<"$entries"; then
      echo "::error::$hap does not contain libs/$abi_dir/$library"
      echo "missing libs/$abi_dir/$library in $hap" >&2
      return 1
    fi
  done
}

cleanup() {
  restore_list
  rm -f "$LIST_BACKUP"
  if [ "$screen_timeout_overridden" -eq 1 ]; then
    "${HDC[@]}" shell "power-shell timeout -r" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

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
needs_awake_device=0
for m in "${selected[@]}"; do
  case "${m%%:*}" in
    arkui|xcomponent) needs_gesture_host=1; needs_awake_device=1 ;;
    window_manager) needs_awake_device=1 ;;
  esac
done
if [ "$needs_awake_device" -eq 1 ]; then
  # Building and reinstalling the per-module HAPs can outlive the default
  # screen timeout. Apply the override before any build begins, while a fresh
  # QEMU guest is still unlocked.
  prepare_gesture_device
fi

# Every ohosTest HAP is associated with the entry module, so a fresh QEMU
# runner must build and install the main HAP even when no gesture host is used.
echo "==> building main HAP"
pnpm --silent run build:hap
# Start from a clean signer/provision state. The SDK-supplied OpenHarmony
# certificate cache may have been regenerated since an older test install.
"${HDC[@]}" uninstall "$BUNDLE" >/dev/null 2>&1 || true

# Make sure the latest ohosTest HAP (with the full List) is installed once.
echo "==> building ohosTest HAP (full List)"
pnpm --silent run build:test
if [ "$needs_gesture_host" -eq 1 ]; then
  verify_gesture_libraries "$MAIN_HAP"
  verify_gesture_libraries "$TEST_HAP"
fi
install_haps full

collect_failure_diagnostics() {
  local label="$1"
  local fault_paths="$DIAGNOSTICS_DIR/faultlogger-$label-paths.log"
  local remote_path
  local output_name

  "${HDC[@]}" shell 'hilog -x | tail -2000' \
    >"$DIAGNOSTICS_DIR/hilog-$label.log" 2>&1 || true
  "${HDC[@]}" shell 'ls -laR /data/log/faultlog 2>/dev/null' \
    >"$DIAGNOSTICS_DIR/faultlogger-$label-list.log" 2>&1 || true
  "${HDC[@]}" shell \
    "grep -R -l '$BUNDLE' /data/log/faultlog/faultlogger 2>/dev/null || true" \
    | tr -d '\r' >"$fault_paths" || true

  while IFS= read -r remote_path; do
    [ -n "$remote_path" ] || continue
    output_name="$(basename "$remote_path")"
    "${HDC[@]}" file recv "$remote_path" \
      "$DIAGNOSTICS_DIR/faultlogger-$label-$output_name" >/dev/null 2>&1 || true
  done <"$fault_paths"
}

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
    if [ "$FAIL_FAST" -eq 1 ]; then
      break
    fi
    continue
  fi
  if ! install_haps "$name"; then
    total_fail=$((total_fail + 1))
    failed_modules+=("$name(install)")
    if [ "$FAIL_FAST" -eq 1 ]; then
      break
    fi
    continue
  fi
  if [ "$name" = window_manager ] && [ "${WINDOW_MANAGER_GRANT_SCREEN_CAPTURE:-0}" = 1 ]; then
    token_dump="$DIAGNOSTICS_DIR/window-manager-access-tokens.log"
    "${HDC[@]}" shell 'atm dump -t' | tr -d '\r' > "$token_dump"
    token_id="$(awk -F: '/com\.richerfu\.ohos_example/ && !found { gsub(/[[:space:]]/, "", $1); print $1; found=1 }' "$token_dump")"
    if ! [[ "$token_id" =~ ^[0-9]+$ ]]; then
      echo "::error::Could not find the WindowManager test app access token" >&2
      exit 1
    fi
    "${HDC[@]}" shell "atm perm -g -i $token_id -p ohos.permission.CUSTOM_SCREEN_CAPTURE"
  fi
  "${HDC[@]}" shell "aa force-stop $BUNDLE" >/dev/null 2>&1 || true

  case "$name" in
    arkui|xcomponent)
      echo "==> [$name] starting automatic gesture host"
      if ! start_gesture_host "$name"; then
        total_fail=$((total_fail + 1))
        failed_modules+=("$name(start)")
        if [ "$FAIL_FAST" -eq 1 ]; then
          break
        fi
        continue
      fi
      HDC_TARGET="${HDC_TARGET:-}" "$ROOT/scripts/inject-xcomponent-gestures.sh"
      ;;
  esac

  echo "==> [$name] aa test"
  log="$(mktemp)"
  if ! run_ohostest "$name" "$log"; then
    :
  fi
  cp "$log" "$DIAGNOSTICS_DIR/ohostest-$name.log"
  pass=$(grep -c 'OHOS_REPORT_STATUS_CODE: 0' "$log" || true)
  fail=$(grep -cE 'OHOS_REPORT_STATUS_CODE: (-1|-2)' "$log" || true)
  pass=${pass:-0}
  fail=${fail:-0}
  if [ "$name" = native_child_process ]; then
    echo "==> [$name] parent-exit E2E (host observes all four creation APIs)"
    if python3 "$ROOT/scripts/native-child-process-parent-exit.py" \
      --target "${HDC_TARGET:-}" --bundle "$BUNDLE" --diagnostics-dir "$DIAGNOSTICS_DIR"; then
      pass=$((pass + 1))
    else
      fail=$((fail + 1))
    fi
  fi
  echo "    pass=$pass fail=$fail"
  total_pass=$((total_pass + pass))
  total_fail=$((total_fail + fail))
  if [ "$pass" -eq 0 ] || [ "$fail" -gt 0 ] \
    || ! grep -q "TestFinished" "$log" \
    || grep -Eq 'TestFinished-ResultCode:[[:space:]]*-' "$log"; then
    failed_modules+=("$name")
    cp "$log" "$ROOT/ohostest-$name.log"
    collect_failure_diagnostics "$name"
    if [ "$FAIL_FAST" -eq 1 ]; then
      rm -f "$log"
      break
    fi
  fi
  rm -f "$log"
done

# Restore the full List so the repo stays pristine.
cleanup
trap - EXIT

echo
echo "==== ohosTest summary: pass=$total_pass fail=$total_fail modules=${#selected[@]} ===="
if [ ${#failed_modules[@]} -gt 0 ]; then
  echo "::error::Failed E2E modules: ${failed_modules[*]}"
  echo "FAILED modules: ${failed_modules[*]}" >&2
  exit 1
fi
