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

### Callback string errors

`Event::string_value(index)` returns `Result<String, event::StringValueError>`.
It distinguishes an invalid index, native failure, malformed UTF-8, missing
termination and allocation/capacity failure. Buffers grow without a fixed
policy size cap. The legacy strcpy status workaround is limited to API-12
builds; API-13+ builds require the native success status.

Typed text insertion/deletion handlers log decode failures and leave the
native return value unchanged instead of passing fabricated empty text.
NodeContent callback panics are logged and contained at the FFI boundary.

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

`ArkUINode::into_rc` creates an identity-stable `Rc<RefCell<ArkUINode>>`.
Use `ArkUINode::bind_rc` for an existing caller-created `Rc`. Native owners
retain a clone of that `Rc`; safe `dispose` returns an error while another
strong owner is active. Detach the native edge and drop its `Rc` before
retrying disposal. Use `common::node_registry::{register,unregister}` and
`ArkUINode::{add_event_receiver,remove_event_receiver}` on the UI thread to
preserve an identity-stable node's event route across NodeContent moves.
`ArkUINode::{parent,remove_from_parent}` query and detach the native parent
edge without changing those routes or the host's attachment bookkeeping.

### Retained accessibility providers

The `accessibility,api-23` features expose `NodeAccessibility` on API-23 and
newer runtimes. It retains a strong `Rc` to the owned Custom node
until callbacks retire. `new` returns a registration or the native error.

### Accessibility API floor

The `accessibility` capability enables `api-23`, so `NodeAccessibility` and its
native provider are selected together. Target an API-23+ runtime when enabling
this feature.

## License

MIT OR Apache-2.0
