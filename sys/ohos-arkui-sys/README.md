# ohos-arkui-sys

This crate provides low-level sys bindings for the arkui module in OpenHarmony.

## Install

```shell
cargo add ohos-arkui-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_arkui_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
