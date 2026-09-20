# ohos-audio-sys

This crate provides low-level sys bindings for the audio module in OpenHarmony.
Shared channel-layout types are provided by and re-exported from
`ohos-audio-base-sys`.

## Install

```shell
cargo add ohos-audio-sys
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_audio_sys as sys;

unsafe {
    // Call the raw FFI functions exposed by `sys` after checking
    // the corresponding OpenHarmony or HarmonyOS API requirements.
}
```

## License

MIT OR Apache-2.0
