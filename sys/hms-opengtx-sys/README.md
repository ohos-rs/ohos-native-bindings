# hms-opengtx-sys

This crate provides low-level sys bindings for OpenGTX in HarmonyOS.

## Install

```shell
cargo add hms-opengtx-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use hms_opengtx_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
