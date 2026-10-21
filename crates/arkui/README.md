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

### Externally owned nodes

`NodeContent` wraps a content handle. It does not retain the ArkTS owner.
`unsafe ArkUINodeRaw::borrowed_node` borrows an externally retained node on
its owning UI thread. Keep its owner alive until the node is detached and the
wrapper discarded. `ArkUICommonAttribute::add_existing_node` attaches an
identity-stable `Rc<RefCell<ArkUINode>>` without replacing its event receiver.
The host layer owns N-API references, attachment IDs and retirement routing.

`ArkUINode::retain_native` returns a `common::node_retention::NodeRetention`
guard. While any guard is held, safe `dispose` returns an error without
freeing the node. The host detaches the external edge, drops the guard, then
retries disposal. Use `common::node_registry::{register,unregister}` and
`ArkUINode::{add_event_receiver,remove_event_receiver}` on the UI thread to
preserve an identity-stable node's event route across NodeContent moves.

## License

MIT OR Apache-2.0
