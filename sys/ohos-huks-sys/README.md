# ohos-huks-sys

This crate provides low-level sys bindings for the HUKS (Universal Keystore) module in OpenHarmony.

## Install

```shell
cargo add ohos-huks-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_huks_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

For a safe wrapper, use [`ohos-huks-binding`](../../crates/huks).

## License

MIT OR Apache-2.0
