#!/usr/bin/env bash
# Inject input into the already-running GestureTestAbility and assert that the
# same ArkUI-native XComponent node reports both raw touch types and system
# gesture callbacks.
set -euo pipefail

BUNDLE="com.richerfu.ohos_example"
DIAGNOSTICS_DIR="${E2E_DIAGNOSTICS_DIR:-$(cd "$(dirname "$0")/.." && pwd)/.tools/e2e-diagnostics}"
LAYOUT_PATH="/data/local/tmp/xcomponent-gesture-layout.json"
HDC=(hdc)
if [ -n "${HDC_TARGET:-}" ]; then
  HDC=(hdc -t "${HDC_TARGET}")
fi
mkdir -p "$DIAGNOSTICS_DIR"

fail() {
  echo "::error::XComponent gesture injection failed: $*"
  echo "gesture injection failed: $*" >&2
  exit 1
}

hdc_run() {
  "${HDC[@]}" "$@"
}

dump_layout() {
  hdc_run shell uitest dumpLayout -b "$BUNDLE" -p "$LAYOUT_PATH" >/dev/null
  hdc_run shell cat "$LAYOUT_PATH" | tr -d '\r'
}

layout_attribute() {
  local layout="$1"
  local node_id="$2"
  local attribute="$3"
  jq -r --arg id "$node_id" --arg attribute "$attribute" \
    '.. | objects | select(.attributes?.id == $id) | .attributes[$attribute]' \
    <<<"$layout" | head -n 1
}

wait_until_ready() {
  local attempt layout readiness
  for attempt in $(seq 1 30); do
    layout="$(dump_layout 2>/dev/null || true)"
    readiness="$(layout_attribute "$layout" gesture-test-readiness text 2>/dev/null || true)"
    if [ "$readiness" = "gesture-test-ready" ]; then
      INITIAL_LAYOUT="$layout"
      printf '%s\n' "$layout" >"$DIAGNOSTICS_DIR/gesture-layout-ready.json"
      return 0
    fi
    case "$readiness" in
      gesture-test-error:*) fail "application reported $readiness" ;;
    esac
    sleep 1
  done
  printf '%s\n' "$layout" >"$DIAGNOSTICS_DIR/gesture-layout-last.json"
  hdc_run shell \
    'hilog -x | grep -iE "GestureTestAbility|XComponentGestureHost|ohos_example|arkui_test|render_service" | tail -400' \
    >"$DIAGNOSTICS_DIR/gesture-host-hilog.log" 2>&1 || true
  fail "GestureTestAbility did not become ready"
}

surface_points() {
  local layout="$1"
  local node_id="$2"
  local bounds
  bounds="$(layout_attribute "$layout" "$node_id" bounds)"
  if [[ ! "$bounds" =~ ^\[([0-9]+),([0-9]+)\]\[([0-9]+),([0-9]+)\]$ ]]; then
    fail "cannot parse $node_id bounds: $bounds"
  fi
  local left="${BASH_REMATCH[1]}"
  local top="${BASH_REMATCH[2]}"
  local right="${BASH_REMATCH[3]}"
  local bottom="${BASH_REMATCH[4]}"
  local width=$((right - left))
  local height=$((bottom - top))
  POINT_X1=$((left + width / 4))
  POINT_X2=$((left + width * 3 / 4))
  POINT_Y=$((top + height / 2))
  POINT_Y2=$((top + height * 2 / 3))
}

exercise_surface() {
  local layout="$1"
  local node_id="$2"
  surface_points "$layout" "$node_id"
  echo "    injecting tap / slow pan / fast swipe / multi-touch into $node_id"
  hdc_run shell uitest uiInput click "$POINT_X1" "$POINT_Y" >/dev/null
  hdc_run shell uinput -T -m "$POINT_X1" "$POINT_Y" "$POINT_X2" "$POINT_Y" 1000 >/dev/null
  hdc_run shell uitest uiInput fling "$POINT_X1" "$POINT_Y" "$POINT_X2" "$POINT_Y" 4000 >/dev/null
  hdc_run shell uinput -T -m \
    "$POINT_X1" "$POINT_Y" "$POINT_X2" "$POINT_Y" \
    "$POINT_X2" "$POINT_Y2" "$POINT_X1" "$POINT_Y2" 500 >/dev/null
}

assert_counter_at_least() {
  local result="$1"
  local name="$2"
  local minimum="$3"
  local value
  value="$(sed -n "s/.*${name}=\([0-9][0-9]*\).*/\1/p" <<<"$result")"
  [ -n "$value" ] || fail "counter $name missing from: $result"
  ((value >= minimum)) || fail "expected $name >= $minimum, got $value: $result"
}

assert_results() {
  sleep 1
  local layout input_result
  layout="$(dump_layout)"
  printf '%s\n' "$layout" >"$DIAGNOSTICS_DIR/gesture-layout-result.json"
  input_result="$(layout_attribute "$layout" xcomponent-input-result text)"

  assert_counter_at_least "$input_result" rawDown 3
  assert_counter_at_least "$input_result" rawMove 2
  assert_counter_at_least "$input_result" rawUp 3
  assert_counter_at_least "$input_result" tap 1
  assert_counter_at_least "$input_result" panAccept 2
  assert_counter_at_least "$input_result" panEnd 2
  assert_counter_at_least "$input_result" swipe 1

  echo "    raw + system: $input_result"
}

command -v jq >/dev/null 2>&1 || fail "jq is required"
wait_until_ready
exercise_surface "$INITIAL_LAYOUT" xcomponent-input-surface
assert_results
