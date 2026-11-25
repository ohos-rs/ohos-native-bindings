# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-display-binding-v0.1.6...ohos-display-binding-v0.2.0) - 2026-11-25

### Fixed

- *(display)* [**breaking**] return Result instead of asserting on native display errors ([#216](https://github.com/ohos-rs/ohos-native-bindings/pull/216))

### Breaking

- `default_display_cutout_info`, `default_display_density_dpi`,
  `default_display_density_pixel`, `default_display_density_x_dpi`,
  `default_display_density_y_dpi`, `default_display_height`,
  `default_display_width`, `default_display_id`,
  `default_display_orientation`, `default_display_refresh_rate`,
  `default_display_rotation`, `default_display_scaled_density`,
  `default_display_virtual_pixel_ratio`, and `fold_display_mode` now return
  `Result<T, DisplayError>` instead of panicking with `assert_eq!(ret, 0)` on
  non-zero native DisplayManager error codes.

### Fixed

- Guard `default_display_cutout_info` against null native cutout pointers
  (`DisplayError::UnexpectedNull`) and release the native cutout handle with an
  RAII drop guard while propagating destroy errors.
- Guard `DisplayCutInfo::from` against null or empty `boundingRects` and fix
  the swapped `left`/`top` coordinates on `bounding_rect`.

## [0.1.6](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-display-binding-v0.1.5...ohos-display-binding-v0.1.6) - 2026-11-23

### Added

- add audio & multimodal-input bindings and complete display/udmf/pasteboard/avplayer/arkui/web APIs ([#214](https://github.com/ohos-rs/ohos-native-bindings/pull/214))

## [0.1.3](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-display-binding-v0.1.2...ohos-display-binding-v0.1.3) - 2026-08-26

### Other

- updated the following local packages: ohos-display-sys
