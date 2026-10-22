# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add owned typography ranges, placeholders, metrics, decorations and ellipsis;
  add fallible `TypographyBuilder::try_add_text` for earlier APIs' C-string limits.
- Add owned image blur and color blend filters, and preserve full 3×3 matrices.

### Breaking

- The unpublished placeholder helper now takes a complete SDK `PlaceholderSpan`;
  the caller selects alignment, baseline and offset.
- The unpublished decoration helper now takes a combinable `TextDecoration`
  mask, including overline. Interior NULs are not replaced on older text APIs;
  `try_add_text` returns an error and existing `add_text` retains its panic contract.

### Added

- Add owned text-shadow construction and `TextStyle::add_shadow` wrappers.
- Add API 25/26 feature forwarding to the native drawing sys crate.

## [0.2.3](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-drawing-binding-v0.2.2...ohos-drawing-binding-v0.2.3) - 2026-08-26

### Other

- updated the following local packages: ohos-native-drawing-sys
