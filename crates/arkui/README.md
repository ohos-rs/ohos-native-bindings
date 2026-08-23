# ohos-arkui-binding

This crate is a binding for the arkui module in OpenHarmony.

## Install

```shell
cargo add ohos-arkui-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_arkui_binding as arkui;

// Use the safe Rust APIs exposed by `ohos-arkui-binding` from your native module.
```

### XComponent gestures

An XComponent created as an ArkUI native node can use the system gesture
recognizers directly. Convenience methods create, register, and attach the
recognizer while preserving access to the native surface:

```rust,no_run
use ohos_arkui_binding::{
    component::built_in_component::XComponent,
    gesture::gesture_data::GestureEventData,
    types::gesture_direction::GestureDirection,
};

# fn configure(xcomponent: &XComponent) -> ohos_arkui_binding::common::error::ArkUIResult<()> {
let pan = xcomponent.on_pan_gesture(
    1,
    GestureDirection::All,
    8.0,
    |_event: GestureEventData| {
        // Handle accept/update/end/cancel.
    },
)?;

let native = xcomponent.native_xcomponent();

// Keep `pan` if the recognizer must later be removed or explicitly disposed.
let _ = (pan, native);
# Ok(())
# }
```

Use `on_tap_gesture` for taps, `on_pan_gesture` for continuous dragging, and
`on_swipe_gesture` for a fast directional swipe.

## License

MIT OR Apache-2.0
