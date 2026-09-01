#!/usr/bin/env bash
# Exercise AccessKit through ArkUI CustomNode, XComponent, and the API 15
# multi-instance callback contract on an OpenHarmony device or QEMU guest.
#
# A screen reader must already be enabled; CI enables the system screen reader
# in its QEMU guest before invoking this script. Single-instance surfaces run
# in fresh app processes because their callback table is process-wide. The
# multi-instance surface deliberately keeps two providers in one process.
set -euo pipefail

OHOS_ARCH="${OHOS_ARCH:-arm64}"
if [ "${1:-}" = "--" ]; then
  shift
fi
case "${1:-}" in
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
esac
case "$OHOS_ARCH" in
  arm64|aarch) OHOS_ARCH="arm64" ;;
  x86_64|x64) OHOS_ARCH="x64" ;;
  *) echo "error: unsupported architecture '$OHOS_ARCH' (expected arm64 or x64)" >&2; exit 2 ;;
esac
if [ $# -gt 0 ]; then
  echo "error: unexpected argument '$1'" >&2
  exit 2
fi

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUNDLE="${ACCESSIBILITY_E2E_BUNDLE:-com.richerfu.ohos_example}"
DEFAULT_HAP="$ROOT/entry/build/default/outputs/default/entry-default-unsigned.hap"
HAP="${ACCESSIBILITY_E2E_HAP:-$DEFAULT_HAP}"
DIAGNOSTICS_DIR="${E2E_DIAGNOSTICS_DIR:-$ROOT/.tools/e2e-diagnostics/accessibility}"
DEVICE_LAYOUT="/data/local/tmp/accessibility-e2e-layout.json"
HOST_LAYOUT="$DIAGNOSTICS_DIR/current-layout.json"
HDC=(hdc)
if [ -n "${HDC_TARGET:-}" ]; then
  HDC=(hdc -t "$HDC_TARGET")
fi

mkdir -p "$DIAGNOSTICS_DIR"

fail() {
  echo "::error::Accessibility E2E failed: $*"
  echo "accessibility E2E failed: $*" >&2
  exit 1
}

hdc_run() {
  "${HDC[@]}" "$@"
}

ensure_device() {
  local attempt hdc_status
  local connect_log="$DIAGNOSTICS_DIR/hdc-connect.log"
  for attempt in 1 2 3; do
    set +e
    "${HDC[@]}" shell param get const.product.cpu.abilist >"$connect_log" 2>&1
    hdc_status=$?
    set -e
    if [ "$hdc_status" -eq 0 ] \
      && ! grep -Eqi 'Connect server failed|\[Fail\]|failed to connect' "$connect_log" \
      && grep -Eq 'arm64|aarch64|x86_64' "$connect_log"; then
      return 0
    fi
    sleep 2
  done
  cat "$connect_log" >&2 || true
  fail "hdc target is not connected: ${HDC_TARGET:-default target}"
}

dump_layout() {
  : >"$HOST_LAYOUT"
  hdc_run shell uitest dumpLayout -b "$BUNDLE" -p "$DEVICE_LAYOUT" >/dev/null
  hdc_run file recv "$DEVICE_LAYOUT" "$HOST_LAYOUT" >/dev/null
  jq -e . "$HOST_LAYOUT" >/dev/null
}

layout_attribute() {
  local layout_file="$1"
  local node_id="$2"
  local attribute="$3"
  jq -r --arg id "$node_id" --arg attribute "$attribute" \
    '.. | objects | select(.attributes?.id == $id) | .attributes[$attribute]' \
    "$layout_file" | head -n 1
}

counter() {
  local status="$1"
  local name="$2"
  local value
  value="$(sed -n "s/.*${name}=\([0-9][0-9]*\).*/\1/p" <<<"$status")"
  [ -n "$value" ] || fail "counter $name missing from: $status"
  printf '%s\n' "$value"
}

virtual_button_count() {
  local layout_file="$1"
  local host_id="$2"
  jq -r --arg id "$host_id" '
    .. | objects | select(.attributes?.id == $id) |
    [.. | objects |
      select(.attributes?.type == "button" and
             .attributes?.clickable == "true" and
             .attributes?.visible == "true" and
             .attributes?.enabled == "true")] | length
  ' "$layout_file" | head -n 1
}

click_layout_node() {
  local node_id="$1"
  local bounds left top right bottom x y
  bounds="$(layout_attribute "$HOST_LAYOUT" "$node_id" bounds)"
  read -r left top right bottom < <(
    sed -n 's/^\[\([0-9][0-9]*\),\([0-9][0-9]*\)\]\[\([0-9][0-9]*\),\([0-9][0-9]*\)\]$/\1 \2 \3 \4/p' \
      <<<"$bounds"
  )
  [ -n "${bottom:-}" ] || fail "invalid bounds for $node_id: $bounds"
  x=$(((left + right) / 2))
  y=$(((top + bottom) / 2))
  hdc_run shell uinput -T -c "$x" "$y" 50 >/dev/null
}

screen_reader_enabled() {
  local state_log="$DIAGNOSTICS_DIR/accessibility-manager-state.log"
  hdc_run shell hidumper -s AccessibilityManagerService -a -u >"$state_log" 2>&1
  grep -Eq 'accessible:[[:space:]]+1' "$state_log"
}

screen_timeout_overridden=0
prepare_device() {
  if hdc_run shell "power-shell timeout -o 86400000" >/dev/null 2>&1; then
    screen_timeout_overridden=1
  fi
  hdc_run shell "power-shell wakeup" >/dev/null 2>&1 || true
}

cleanup() {
  if [ "$screen_timeout_overridden" -eq 1 ]; then
    hdc_run shell "power-shell timeout -r" >/dev/null 2>&1 || true
  fi
}

gesture_coordinates() {
  local bounds left top right bottom width height swipe_x1 swipe_x2 gesture_y tap_x
  bounds="$(jq -r 'first(.. | objects | select(.attributes?.bounds != null) | .attributes.bounds)' "$HOST_LAYOUT")"
  read -r left top right bottom < <(
    sed -n 's/^\[\([0-9][0-9]*\),\([0-9][0-9]*\)\]\[\([0-9][0-9]*\),\([0-9][0-9]*\)\]$/\1 \2 \3 \4/p' <<<"$bounds"
  )
  [ -n "${bottom:-}" ] || fail "invalid screen bounds: $bounds"
  width=$((right - left))
  height=$((bottom - top))
  [ "$width" -gt 0 ] && [ "$height" -gt 0 ] || fail "empty screen bounds: $bounds"
  swipe_x1=$((left + width / 6))
  swipe_x2=$((left + width * 5 / 6))
  gesture_y=$((top + height * 4 / 5))
  tap_x=$((left + width / 2))
  printf '%s %s %s %s %s %s\n' "$swipe_x1" "$gesture_y" "$swipe_x2" "$gesture_y" "$tap_x" "$gesture_y"
}

wait_for_tree() {
  local surface="$1"
  local ready_text="$2"
  local host_id="$3"
  local activation_name="$4"
  local attempt readiness status buttons activations

  for attempt in $(seq 1 30); do
    dump_layout 2>/dev/null || true
    readiness="$(layout_attribute "$HOST_LAYOUT" accessibility-test-readiness text 2>/dev/null || true)"
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text 2>/dev/null || true)"
    buttons="$(virtual_button_count "$HOST_LAYOUT" "$host_id" 2>/dev/null || true)"
    activations="$(counter "$status" "$activation_name" 2>/dev/null || true)"
    if [ "$readiness" = "$ready_text" ] && [ "${buttons:-0}" -ge 1 ] \
      && [ "${activations:-0}" -ge 1 ]; then
      READY_STATUS="$status"
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/${surface}-tree.json"
      return 0
    fi
    case "$readiness" in
      accessibility-*-error:*) fail "$surface host reported $readiness" ;;
    esac
    sleep 1
  done

  cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/${surface}-tree-last.json" 2>/dev/null || true
  fail "$surface did not expose an activated virtual button"
}

screen_reader_swipe_forward() {
  local swipe_x1 swipe_y1 swipe_x2 swipe_y2 tap_x tap_y
  read -r swipe_x1 swipe_y1 swipe_x2 swipe_y2 tap_x tap_y < <(gesture_coordinates)
  hdc_run shell uitest uiInput swipe "$swipe_x1" "$swipe_y1" "$swipe_x2" "$swipe_y2" 3000 >/dev/null
}

screen_reader_double_tap() {
  local swipe_x1 swipe_y1 swipe_x2 swipe_y2 tap_x tap_y
  read -r swipe_x1 swipe_y1 swipe_x2 swipe_y2 tap_x tap_y < <(gesture_coordinates)
  # Two separate low-level touch injections are recognized by the system screen
  # reader as one double-tap gesture. uitest's high-level doubleClick is not.
  hdc_run shell \
    "uinput -T -c $tap_x $tap_y 50 >/dev/null; uinput -T -c $tap_x $tap_y 50 >/dev/null"
}

exercise_action() {
  local surface="$1"
  local action_name="$2"
  local update_name="$3"
  local attempt status actions updates

  for attempt in $(seq 1 8); do
    screen_reader_swipe_forward
    # Let the screen reader finish focus movement and speech before activation.
    sleep 4
    screen_reader_double_tap
    sleep 2
    dump_layout
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text)"
    actions="$(counter "$status" "$action_name")"
    updates="$(counter "$status" "$update_name")"
    if [ "$actions" -ge 1 ] && [ "$updates" -ge "$actions" ]; then
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/${surface}-action.json"
      hdc_run shell uitest screenCap \
        -p "/data/local/tmp/accessibility-${surface}-action.png" >/dev/null || true
      echo "    $status"
      return 0
    fi
  done

  cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/${surface}-action-last.json"
  fail "$surface virtual button did not handle Click and publish its tree update"
}

run_surface() {
  local surface="$1"
  local ability="$2"
  local ready_text="$3"
  local host_id="$4"
  local activation_name="$5"
  local action_name="$6"
  local update_name="$7"

  echo "==> [$surface] starting $ability"
  hdc_run shell "power-shell wakeup" >/dev/null 2>&1 || true
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
  hdc_run shell aa start -a "$ability" -b "$BUNDLE" >/dev/null
  wait_for_tree "$surface" "$ready_text" "$host_id" "$activation_name"
  echo "    virtual tree: $READY_STATUS"
  exercise_action "$surface" "$action_name" "$update_name"
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
}

wait_for_multi_tree() {
  local attempt readiness status buttons_a buttons_b activations_a activations_b registered_a

  for attempt in $(seq 1 30); do
    dump_layout 2>/dev/null || true
    readiness="$(layout_attribute "$HOST_LAYOUT" accessibility-test-readiness text 2>/dev/null || true)"
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text 2>/dev/null || true)"
    buttons_a="$(virtual_button_count "$HOST_LAYOUT" accesskit-multi-a-host 2>/dev/null || true)"
    buttons_b="$(virtual_button_count "$HOST_LAYOUT" accesskit-multi-b-host 2>/dev/null || true)"
    activations_a="$(counter "$status" multiAActivated 2>/dev/null || true)"
    activations_b="$(counter "$status" multiBActivated 2>/dev/null || true)"
    registered_a="$(counter "$status" multiARegistered 2>/dev/null || true)"
    if [ "$readiness" = "accessibility-multi-ready" ] \
      && [ "${buttons_a:-0}" -eq 1 ] && [ "${buttons_b:-0}" -eq 1 ] \
      && [ "${activations_a:-0}" -ge 1 ] && [ "${activations_b:-0}" -ge 1 ] \
      && [ "${registered_a:-0}" -eq 1 ]; then
      READY_STATUS="$status"
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-tree.json"
      return 0
    fi
    case "$readiness" in
      accessibility-*-error:*) fail "multi-instance host reported $readiness" ;;
    esac
    sleep 1
  done

  cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-tree-last.json" 2>/dev/null || true
  fail "multi-instance providers did not expose two independently activated trees"
}

exercise_multi_actions() {
  local attempt status actions_a actions_b updates_a updates_b

  for attempt in $(seq 1 12); do
    screen_reader_swipe_forward
    sleep 4
    screen_reader_double_tap
    sleep 2
    dump_layout
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text)"
    actions_a="$(counter "$status" multiAActions)"
    actions_b="$(counter "$status" multiBActions)"
    updates_a="$(counter "$status" multiAUpdates)"
    updates_b="$(counter "$status" multiBUpdates)"
    if [ "$actions_a" -ge 1 ] && [ "$actions_b" -ge 1 ] \
      && [ "$updates_a" -ge "$actions_a" ] && [ "$updates_b" -ge "$actions_b" ]; then
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-actions.json"
      echo "    $status"
      return 0
    fi
  done

  cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-actions-last.json"
  fail "multi-instance buttons did not route Click and tree updates to both instance IDs"
}

release_multi_a() {
  local attempt readiness status registered_a buttons_b

  dump_layout
  click_layout_node accessibility-multi-release-a
  sleep 2
  screen_reader_double_tap
  for attempt in $(seq 1 15); do
    sleep 1
    dump_layout
    readiness="$(layout_attribute "$HOST_LAYOUT" accessibility-test-readiness text)"
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text)"
    registered_a="$(counter "$status" multiARegistered)"
    buttons_b="$(virtual_button_count "$HOST_LAYOUT" accesskit-multi-b-host)"
    if [ "$readiness" = "accessibility-multi-a-released" ] \
      && [ "$registered_a" -eq 0 ] && [ "$buttons_b" -eq 1 ]; then
      RELEASE_STATUS="$status"
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-release-a.json"
      return 0
    fi
    case "$readiness" in
      accessibility-multi-release-error:*|accessibility-multi-a-release-missed)
        fail "multi-instance release reported $readiness"
        ;;
    esac
  done
  fail "instance A registration was not released while instance B stayed available"
}

exercise_multi_b_after_release() {
  local baseline_b attempt status actions_b updates_b registered_a
  baseline_b="$(counter "$RELEASE_STATUS" multiBActions)"

  for attempt in $(seq 1 12); do
    screen_reader_swipe_forward
    sleep 4
    screen_reader_double_tap
    sleep 2
    dump_layout
    status="$(layout_attribute "$HOST_LAYOUT" accessibility-test-status text)"
    actions_b="$(counter "$status" multiBActions)"
    updates_b="$(counter "$status" multiBUpdates)"
    registered_a="$(counter "$status" multiARegistered)"
    if [ "$actions_b" -gt "$baseline_b" ] && [ "$updates_b" -ge "$actions_b" ] \
      && [ "$registered_a" -eq 0 ]; then
      cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-after-release.json"
      echo "    $status"
      return 0
    fi
  done

  cp "$HOST_LAYOUT" "$DIAGNOSTICS_DIR/multi-after-release-last.json"
  fail "instance B stopped routing actions after instance A was released"
}

run_multi_surface() {
  echo "==> [multi] starting AccessibilityMultiInstanceTestAbility"
  hdc_run shell "power-shell wakeup" >/dev/null 2>&1 || true
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
  hdc_run shell aa start -a AccessibilityMultiInstanceTestAbility -b "$BUNDLE" >/dev/null
  wait_for_multi_tree
  echo "    virtual trees: $READY_STATUS"
  exercise_multi_actions
  release_multi_a
  echo "    released A: $RELEASE_STATUS"
  exercise_multi_b_after_release
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
command -v hdc >/dev/null 2>&1 || fail "hdc is required"
ensure_device
prepare_device
trap cleanup EXIT

if [ "${ACCESSIBILITY_E2E_SKIP_INSTALL:-0}" != "1" ]; then
  if [ ! -f "$HAP" ] && [ -z "${ACCESSIBILITY_E2E_HAP:-}" ]; then
    echo "==> building accessibility example and unsigned CI HAP"
    (
      cd "$ROOT"
      ./scripts/sync-rust.sh --arch "$OHOS_ARCH" --fail-fast accessibility
      pnpm exec arkdown build --project . --target hap --mode debug --no-cache
    )
  fi
  [ -f "$HAP" ] || fail "HAP not found: $HAP"
  echo "==> installing $HAP"
  install_log="$DIAGNOSTICS_DIR/install.log"
  set +e
  "${HDC[@]}" install -r "$HAP" >"$install_log" 2>&1
  install_status=$?
  set -e
  cat "$install_log"
  if [ "$install_status" -ne 0 ] \
    || grep -Eqi 'Connect server failed|\[Fail\]|failed to install' "$install_log" \
    || ! grep -Eqi 'success|successfully' "$install_log"; then
    fail "HAP installation failed"
  fi
fi

screen_reader_enabled || fail \
  "enable a screen reader before running this test (AccessibilityManagerService accessible=1)"

run_surface \
  arkui \
  AccessibilityArkUiTestAbility \
  accessibility-arkui-ready \
  accesskit-arkui-host \
  arkuiActivated \
  arkuiActions \
  arkuiUpdates

run_surface \
  xcomponent \
  AccessibilityXComponentTestAbility \
  accessibility-xcomponent-ready \
  accesskit-xcomponent-host \
  xcomponentActivated \
  xcomponentActions \
  xcomponentUpdates

run_multi_surface

echo "==== Accessibility E2E passed: ArkUI CustomNode + XComponent + multi-instance ===="
