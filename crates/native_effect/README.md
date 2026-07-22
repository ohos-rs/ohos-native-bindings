# ohos-native-effect-binding

This crate is a binding for the native effect module in OpenHarmony.

The native effect module applies image filters. A filter is created from a
source pixel map, effects are added to it in order, and the result is rendered
into a new pixel map. This crate wraps the native `effect_filter.h` C API with a
safe layer.

## Install

```shell
cargo add ohos-native-effect-binding
```

## Usage

```rust
use ohos_native_effect_binding::{ColorMatrix, Filter, PixelMapHandle};

// `source` is a native pixel map pointer owned by the caller.
let handle = unsafe { PixelMapHandle::from_raw(source) }.expect("null pixel map");

let mut filter = Filter::create(handle)?;
filter.blur(10.0)?;
filter.brighten(0.5)?;
filter.set_color_matrix(&ColorMatrix::IDENTITY)?;

// The result is a new pixel map; releasing it is up to the caller.
let result = filter.effect_pixel_map()?;
println!("effect pixel map at {:?}", result.as_raw());
```

## Coverage

Available without any feature (API 12):

- Filter lifetime: `Filter::create`, released on drop.
- Effects: `blur`, `brighten`, `gray_scale`, `invert`, `set_color_matrix`.
- Result: `effect_pixel_map`.
- Helpers: `ColorMatrix` (5x4, `IDENTITY` by default), `PixelMapHandle`.

Behind `api-*` features:

| Feature | Adds |
|---|---|
| `api-14` | `TileMode` and `Filter::blur_with_tile_mode` |

## Notes

- Pixel maps belong to the image bindings. They cross the boundary as
  `PixelMapHandle`, a borrowed pointer view without `Drop`.
- The source pixel map passed to `Filter::create` stays owned by the caller and
  must outlive the filter.
- The pixel map returned by `effect_pixel_map` is a newly created object: the
  caller takes the ownership and releases it through the image bindings.
- Effects apply in the order they are added.
- Error codes are wrapped in `EffectError`; `describe` maps a raw
  `EffectErrorCode` to a short description.

## License

MIT OR Apache-2.0
