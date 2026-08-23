# ohos-native-color-space-manager-binding

This crate is a binding for the native color space manager module in OpenHarmony.

A color space manager instance describes a color space, either a standard one
selected by name or a custom one described by its primaries and gamma. Once
created, the instance reports its color space name, white point and gamma;
graphics APIs take it to tag surfaces and images with a color space. This crate
wraps the native `native_color_space_manager.h` C API with a safe layer.

## Install

```shell
cargo add ohos-native-color-space-manager-binding --features api-13
```

## Usage

```rust
use ohos_native_color_space_manager_binding as color_space;
use color_space::{Chromaticity, ColorSpaceManager, ColorSpaceName, ColorSpacePrimaries};

// A standard color space, queried back by name.
let srgb = ColorSpaceManager::from_name(ColorSpaceName::Srgb)?;
assert_eq!(srgb.name()?, ColorSpaceName::Srgb);

// A custom color space, described by its primaries and gamma.
let custom = ColorSpaceManager::from_primaries_and_gamma(
    ColorSpacePrimaries {
        red: Chromaticity::new(0.64, 0.33),
        green: Chromaticity::new(0.30, 0.60),
        blue: Chromaticity::new(0.15, 0.06),
        white: Chromaticity::new(0.3127, 0.3290),
    },
    2.2,
)?;
println!("white point: {:?}", custom.white_point()?);
println!("gamma: {}", custom.gamma()?);
```

## Coverage

The whole module was introduced in API 13, so everything below is behind the
`api-13` feature; without it the crate only re-exports `sys`.

| Feature | Adds |
|---|---|
| `api-13` | `ColorSpaceManager` (`from_name`, `from_primaries_and_gamma`, `name`, `white_point`, `gamma`), `ColorSpaceName`, `ColorSpacePrimaries`, `Chromaticity` |

## Notes

- `ColorSpaceManager` owns the native instance and destroys it on drop, so
  `OH_NativeColorSpaceManager_Destroy` is never called twice and never missed.
- The handle is neither `Send` nor `Sync`: the native library gives no guarantee
  about sharing one instance across threads.
- The native API carries no error codes. Creation returns a null pointer on
  failure and every getter has a documented failure sentinel (`0` for the color
  space name, `(0.0, 0.0)` for the white point, `0.0` for the gamma); those
  sentinels become `ColorSpaceError` variants.
- Several native color space names are aliases of one another (`DISPLAY_SRGB` is
  `SRGB`, `LINEAR_BT709` is `LINEAR_SRGB`, `DISPLAY_P3_SRGB` is `DISPLAY_P3`,
  `DISPLAY_P3_HLG` is `P3_HLG`, `DISPLAY_P3_PQ` is `P3_PQ`). Each distinct value
  is one `ColorSpaceName` variant, with the alias spellings available as
  associated constants.

## License

MIT OR Apache-2.0
