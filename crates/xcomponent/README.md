# ohos-xcomponent-binding

This crate is a binding for the xcomponent module in OpenHarmony.

## Install

```shell
cargo add ohos-xcomponent-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_xcomponent_binding as xcomponent;

// Use the safe Rust APIs exposed by `ohos-xcomponent-binding` from your native module.
```

`ohos-xcomponent-binding` exposes the original native touch event stream through
`on_touch_event`. For semantic tap, pan, or swipe recognition, create the
XComponent as an ArkUI native node and attach the system recognizers provided by
`ohos-arkui-binding`; see that crate's XComponent gesture example.

## License

MIT OR Apache-2.0
