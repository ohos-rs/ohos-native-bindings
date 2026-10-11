# ohos-arkui-input-binding

This crate is a binding for the arkui input module in OpenHarmony.

## Install

```shell
cargo add ohos-arkui-input-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_arkui_input_binding as arkui_input;

// Use the safe Rust APIs exposed by `ohos-arkui-input-binding` from your native module.
```

### Owned pointer events (API 24+)

Enable `api-24` to use `ArkUIOwnedPointerEvent` for touch, mouse, and axis event
creation, independent cloning, editing, and posting to an ArkUI node. All 33
`OH_ArkUI_ClonedEvent_Set*` operations are wrapped, including pen pressure, tilt,
rotation, contact geometry, device/display IDs, timestamps, and pressed keys.
`UIPointerEventAction` keeps touch, mouse, and axis action codes distinct.

Before allocation, the owner initializes the NativeNode interface required by
ArkUI's clone and destroy functions; it returns an error if initialization fails.
The owner releases the event with the API 24 destructor. It is not `Send`, `Sync`,
or `Copy`; keep it on the ArkUI UI thread and use `try_clone` for an independent
copy. `clone_from_raw` and `post_to_node` are unsafe because the caller must
guarantee the validity of external event/node pointers. API 15's
`ArkUIInputClonedEvent` remains available with its original destructor.

Pressure values above 1 are preserved. Tilt is in degrees within `[-90, 90]`;
rotation is in degrees without normalization. Indexed setters accept contact
array indices, not pointer IDs. The native pressed-key/button setters reject
empty arrays. Event-category errors are returned from ArkUI without translation.

These APIs dispatch within an application. They do not provide system input
injection or add an ArkUI eraser tool enum. The separate XComponent API's Rubber
tool type remains available. Hardware hover/eraser delivery and application
routing still need their own integration tests.

### Validation

```sh
# Actual binding crate, with host native-function test doubles:
crates/arkui_input/tests/run-cloned-pointer.sh

# API 24+ native smoke-test example (requires the OHOS linker/SDK setup):
cargo build -p ohos-arkui-input-binding --features api-24 \
  --example pen_pointer --target aarch64-unknown-linux-ohos
```

The native example checks all setter calls, pressure readback, and
independent clone lifetime. On systems that restrict ArkUI dependencies to app
processes, run its body from an application's UI thread instead of `hdc shell`.
It does not claim hardware pen or node-dispatch coverage. Host tests also cover
allocation failure, invalid inputs, native errors, and slice ownership.

The current [ArkUI native implementation](https://github.com/openharmony/arkui_ace_engine/blob/master/interfaces/native/event/ui_input_event.cpp)
reads the final touch contact in the pressure/tilt getters, regardless of the
requested index. `SetTiltAngle` writes the changed contact instead. For newly
created events, the rotation getter can also read a different field than
`SetRollAngle` writes. A successful setter alone therefore does not prove tilt or
rotation readback. The binding preserves the native behavior; the smoke test
uses the final contact for pressure readback and reports tilt/rotation matches
separately. A `false` result is an unresolved native round-trip limitation.

An API 26 2in1 emulator (7.0.0.106) application run verified all 33 setters,
pressure readback, and independent clone/drop. It reported both
`tilt_readback_matches=false` and `rotation_readback_matches=false` for freshly
created events. Neither is counted as a successful nonzero round trip.

## License

MIT OR Apache-2.0
