# ohos-image-native-sys

This crate provides low-level sys bindings for the native image module in OpenHarmony.

## Install

```shell
cargo add ohos-image-native-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_image_native_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
