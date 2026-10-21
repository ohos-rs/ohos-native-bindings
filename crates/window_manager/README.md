# ohos-window-manager-binding

Safe Rust wrappers for OpenHarmony's native window manager API.

```shell
cargo add ohos-window-manager-binding --features api-15
```

Create a borrowed window handle from the ID exposed by ArkTS window
properties, then query or update it:

```rust,no_run
use ohos_window_manager_binding::Window;

let window = Window::from_id(1);
let properties = window.properties()?;
println!("window size: {}x{}", properties.window_rect.width, properties.window_rect.height);
# Ok::<(), ohos_window_manager_binding::Error>(())
```

APIs introduced after API 15 are enabled by the matching `api-*` feature.

## License

MIT OR Apache-2.0

The optional `key-filter` feature adds API-12 `WindowKeyFilter` with owned
`KeyEvent` snapshots. Its 32 trampoline slots bound concurrent live, retiring
and quarantined registrations; successful native unregister permits reuse.
Drop disables delivery first. Ordinary Drop unregisters synchronously, while
Drop inside any filter callback uses one shared cleanup worker to avoid the
native callback mutex. Failed cleanup reserves the slot with no handler.
Own each window's native filter exclusively for the lifetime of this guard.
