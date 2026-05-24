# ohos-asset-sys

This crate provides low-level sys bindings for the asset module in OpenHarmony.

## Install

```shell
cargo add ohos-asset-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_asset_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
