# ohos-vsync-binding

This crate is a binding for the vsync module in OpenHarmony.

## Install

```shell
cargo add ohos-vsync-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_vsync_binding::Vsync;

let vsync = Vsync::try_new_for_associated_window(surface_id, "renderer")
    .expect("failed to create window-associated VSync");
vsync.request_frame_once(move |timestamp| {
    frame_sender.send(timestamp).ok();
});
```

## License

MIT OR Apache-2.0
