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
