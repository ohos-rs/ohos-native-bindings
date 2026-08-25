#!/usr/bin/env bash
# Build the Rust examples of ohos-native-bindings and copy the produced
# .so / index.d.ts artifacts into this ArkTS host project.
#
# Usage:
#   scripts/sync-rust.sh              # build every demo
#   scripts/sync-rust.sh arkui vsync  # build only the given example dirs
#
# The bindings repo is the parent of examples-ui in the monorepo. Set
# $OHOS_NATIVE_BINDINGS only when intentionally testing another checkout.
set -euo pipefail

# pnpm forwards the conventional argument separator to nested workspace
# scripts; do not treat it as an example name.
if [ "${1:-}" = "--" ]; then
  shift
fi

DEMO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OHOS_ARCH="${OHOS_ARCH:-arm64}"
case "$OHOS_ARCH" in
  arm64|aarch) ABI_DIR="arm64-v8a" ;;
  x86_64|x64) ABI_DIR="x86_64" ;;
  *) echo "error: unsupported OHOS_ARCH=$OHOS_ARCH" >&2; exit 2 ;;
esac
LIBS_DIR="$DEMO_ROOT/entry/libs/$ABI_DIR"
TYPES_DIR="$DEMO_ROOT/entry/src/main/ets/types"
TEST_TYPES_DIR="$DEMO_ROOT/entry/src/ohosTest/ets/types"

# --- locate the bindings repo -------------------------------------------------
find_bindings() {
  local candidate
  if [ -n "${OHOS_NATIVE_BINDINGS:-}" ]; then
    candidate="$OHOS_NATIVE_BINDINGS"
    if [ -f "$candidate/Cargo.toml" ] && [ -d "$candidate/examples" ]; then
      echo "$candidate"
      return 0
    fi
    echo "error: OHOS_NATIVE_BINDINGS=$candidate does not look like the bindings repo" >&2
    return 1
  fi
  candidate="$(cd "$DEMO_ROOT/.." && pwd)"
  if [ -f "$candidate/Cargo.toml" ] && [ -d "$candidate/examples" ]; then
    echo "$candidate"
    return 0
  fi
  echo "error: $candidate is not an ohos-native-bindings workspace" >&2
  echo "  Set OHOS_NATIVE_BINDINGS=<path> to override the monorepo root." >&2
  return 1
}

BINDINGS="$(find_bindings)"
echo "bindings repo: $BINDINGS"

normalize_dts_eof() {
  local file="$1"
  local normalized="${file}.normalized"
  LC_ALL=C awk '
    NF { while (blank > 0) { print ""; blank-- } print; next }
    { blank++ }
  ' "$file" > "$normalized"
  mv "$normalized" "$file"
}

# --- pick demos ----------------------------------------------------------------
ALL_DEMOS=(
  ability_access_control ark_web arkui arkui_input ashmem asset bundle camera
  display display_soloist drawing fileshare fileuri hilog huks ime image
  image_native init jsvm native_buffer native_window net_connection net_stack
  pasteboard qos raw sensor udmf vibrator vsync xcomponent xcomponent_multi
)
# HMS OpenGTX needs the real libopengtx from the HMS SDK; it is not in the
# OpenHarmony NDK. Build it separately: scripts/sync-rust.sh opengtx
if [ $# -gt 0 ]; then
  DEMOS=("$@")
else
  DEMOS=("${ALL_DEMOS[@]}")
fi

mkdir -p "$LIBS_DIR" "$TYPES_DIR" "$TEST_TYPES_DIR"

# Full sync starts from a clean libs dir so renamed/removed demos do not
# leave stale .so files behind.
if [ ${#DEMOS[@]} -eq ${#ALL_DEMOS[@]} ]; then
  rm -f "$LIBS_DIR"/lib*.so
fi

failed=()
for demo in "${DEMOS[@]}"; do
  example_dir="$BINDINGS/examples/$demo"
  if [ ! -d "$example_dir" ]; then
    echo "error: no such example: $demo ($example_dir missing)" >&2
    failed+=("$demo")
    continue
  fi

  echo "==> [$demo] ohrs build"
  if ! (cd "$example_dir" && ohrs build -a "$OHOS_ARCH" >/dev/null); then
    echo "error: ohrs build failed for $demo" >&2
    failed+=("$demo")
    continue
  fi

  # Every example crate is named <demo>_test, so the .so is always
  # lib<demo>_test.so — the *_test suffix keeps ESM module names from
  # colliding with system modules (qos, ability_access_control, ...).
  # ohrs may also copy dependency .so files into dist/; pick this crate's own.
  pkg_name="$(awk -F '"' '/^name[[:space:]]*=/ { print $2; exit }' "$example_dir/Cargo.toml")"
  so_name="lib${pkg_name//-/_}.so"
  so_file="$example_dir/dist/$ABI_DIR/$so_name"
  if [ ! -f "$so_file" ]; then
    echo "error: $so_name not produced for $demo in $example_dir/dist/$ABI_DIR" >&2
    failed+=("$demo")
    continue
  fi

  cp "$so_file" "$LIBS_DIR/"
  cp "$example_dir/dist/index.d.ts" "$TYPES_DIR/$demo.d.ts"
  cp "$example_dir/dist/index.d.ts" "$TEST_TYPES_DIR/$demo.d.ts"
  # ohrs currently emits an extra blank line at EOF; normalize generated
  # declarations so running the sync does not fail git diff --check.
  normalize_dts_eof "$TYPES_DIR/$demo.d.ts"
  normalize_dts_eof "$TEST_TYPES_DIR/$demo.d.ts"
  echo "    $(basename "$so_file") -> entry/libs/$ABI_DIR/"
done

if [ ${#failed[@]} -gt 0 ]; then
  echo "FAILED demos: ${failed[*]}" >&2
  exit 1
fi

echo "done: ${#DEMOS[@]} demo(s) synced."
