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

### Custom layout

Custom-node measure and layout helpers use physical pixels and validate the
callback phase. Borrowed layout constraints remain valid for that callback;
copy their dimensions when storing layout inputs for later use. `ArkUIEvent`
and `ArkUIGesture` remain available after converting a component into its
identity-stable node. `off_event` retires one handler while preserving others.

### Native content ownership

With `napi`, `ContentRoot` owns a NodeContent object for a host ContentSlot.
Create, mount, unmount and destroy it on its creating UI thread.
`NativeAttachment` connects externally owned ArkTS content through opaque
target IDs. The external creator must retain its node until synchronous
`retire` succeeds. Failed detachment retains the target for a later retry;
`invalidate` rejects late construction results.

Active roots and attachment targets retain their native owners. Explicit node
disposal returns an error until these owners retire. Unmount retries continue
disposal after the native edge has been detached successfully.

### Retained accessibility providers

The `accessibility,api-23` features expose `NodeAccessibility` on API-23 and
newer runtimes. It retains the owned Custom node and its native-owner guard
until callbacks retire. `new` returns a registration or the native error.

### Owned dialog content

`Dialog::content` owns its transferred node; `content_node` retains a shared
owned node. Content remains protected from disposal until native detachment
succeeds. `remove_content` and `dispose` preserve failed cleanup for retry.
Dismiss callbacks run synchronously on the UI thread and retain their callback
data throughout reentrant calls.

## License

MIT OR Apache-2.0
