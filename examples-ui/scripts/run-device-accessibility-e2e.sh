#!/usr/bin/env bash
# Exercise AccessKit through ArkUI CustomNode and XComponent on a real device.
#
# A screen reader must already be enabled. Each surface runs in a fresh app
# process because the official single-instance callback table is process-wide.
set -euo pipefail

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

screen_reader_enabled() {
  local state_log="$DIAGNOSTICS_DIR/accessibility-manager-state.log"
  hdc_run shell hidumper -s AccessibilityManagerService -a -u >"$state_log" 2>&1
  grep -Eq 'accessible:[[:space:]]+1' "$state_log"
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
  hdc_run shell uitest uiInput swipe 200 1800 1000 1800 3000 >/dev/null
}

screen_reader_double_tap() {
  # Two separate low-level touch injections are recognized by the real screen
  # reader as one double-tap gesture. uitest's high-level doubleClick is not.
  hdc_run shell \
    "uinput -T -c 600 1800 50 >/dev/null; uinput -T -c 600 1800 50 >/dev/null"
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
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
  hdc_run shell aa start -a "$ability" -b "$BUNDLE" >/dev/null
  wait_for_tree "$surface" "$ready_text" "$host_id" "$activation_name"
  echo "    virtual tree: $READY_STATUS"
  exercise_action "$surface" "$action_name" "$update_name"
  hdc_run shell aa force-stop "$BUNDLE" >/dev/null 2>&1 || true
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
command -v hdc >/dev/null 2>&1 || fail "hdc is required"
ensure_device

if [ "${ACCESSIBILITY_E2E_SKIP_INSTALL:-0}" != "1" ]; then
  if [ ! -f "$HAP" ] && [ -z "${ACCESSIBILITY_E2E_HAP:-}" ]; then
    echo "==> building unsigned CI HAP"
    (cd "$ROOT" && pnpm exec arkdown build --project . --target hap --mode debug)
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

echo "==== Accessibility E2E passed: ArkUI CustomNode + XComponent ===="
