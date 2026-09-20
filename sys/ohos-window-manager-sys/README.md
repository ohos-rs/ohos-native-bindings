# ohos-window-manager-sys

Raw Rust FFI bindings for OpenHarmony's native window manager API.

```shell
cargo add ohos-window-manager-sys --features api-15
```

The bindings are generated from `oh_window_comm.h`, `oh_window.h`, and
`oh_window_event_filter.h`, and link to `libnative_window_manager.so`.

## License

MIT OR Apache-2.0
