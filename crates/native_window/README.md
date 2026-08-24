# ohos-native-window-binding

This crate is a binding for the native window module in OpenHarmony.

## Install

```shell
cargo add ohos-native-window-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_native_window_binding::NativeWindow;

let window = NativeWindow::clone_from_ptr(raw_window);
let surface_id = window.surface_id()?;
```

## License

MIT OR Apache-2.0
