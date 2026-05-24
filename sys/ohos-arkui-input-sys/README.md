# ohos-arkui-input-sys

This crate provides low-level sys bindings for the arkui input event module in OpenHarmony.

## Install

```shell
cargo add ohos-arkui-input-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_arkui_input_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
