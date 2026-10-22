# ohos-native-image-binding

This crate is a binding for the native image module in OpenHarmony.

## Install

```shell
cargo add ohos-native-image-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_native_image_binding::ConsumerSurface;

let mut surface = ConsumerSurface::new()?;
let surface_id = surface.surface_id()?;
```

## License

MIT OR Apache-2.0
