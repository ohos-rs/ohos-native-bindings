# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-pasteboard-binding-v0.1.0...ohos-pasteboard-binding-v0.1.1) - 2026-11-27

### Other

- updated the following local packages: ohos-enum-derive, ohos-udmf-binding

## [0.1.0](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-pasteboard-binding-v0.0.8...ohos-pasteboard-binding-v0.1.0) - 2026-11-23

### Added

- add audio & multimodal-input bindings and complete display/udmf/pasteboard/avplayer/arkui/web APIs ([#214](https://github.com/ohos-rs/ohos-native-bindings/pull/214))

### Added

- Add native `Pasteboard::clear` (API 13), copied `mime_types` (API 14)
  and native `change_count` revision queries (API 18).

### Breaking

- Withdraw the unpublished ArkTS Clipboard bridge in favor of native pasteboard
  and UDMF entries. Revision queries use the native change counter.

## [0.0.6](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-pasteboard-binding-v0.0.5...ohos-pasteboard-binding-v0.0.6) - 2026-08-26

### Other

- updated the following local packages: ohos-native-pasteboard-sys, ohos-udmf-binding
