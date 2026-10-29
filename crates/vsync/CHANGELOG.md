# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.2](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-vsync-binding-v0.3.1...ohos-vsync-binding-v0.3.2) - 2026-10-29

### Added

- *(arkui)* read and write node attributes without heap allocation  ([#204](https://github.com/ohos-rs/ohos-native-bindings/pull/204))

### Added

- `Vsync::set_dvsync` (api-14) switches DVSync on or off
  (`OH_NativeVSync_DVSyncSwitch`).

## [0.3.0](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-vsync-binding-v0.2.2...ohos-vsync-binding-v0.3.0) - 2026-08-26

### Fixed

- *(ime)* isolate editor sessions and recover stale proxies ([#142](https://github.com/ohos-rs/ohos-native-bindings/pull/142))
- fix some binding issues ([#133](https://github.com/ohos-rs/ohos-native-bindings/pull/133))
