# ohos-input-sys

Raw Rust FFI bindings for OpenHarmony's multimodal InputKit API.

```shell
cargo add ohos-input-sys --features api-22
```

The bindings are generated from `multimodalinput/oh_input_manager.h` and link
to `libohinput.so`. API12 is the baseline; APIs introduced in later platform
versions are enabled through the corresponding `api-*` feature.

The crate owns the canonical `Input_*` event types used by other native APIs,
including `ohos-window-manager-sys`.

## License

MIT OR Apache-2.0
