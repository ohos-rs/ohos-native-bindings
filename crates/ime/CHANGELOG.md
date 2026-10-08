# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Synchronize editor configuration, cursor, text and selection state; expose
  forward deletion, cursor movement, selection and extended editing callbacks.

### Fixed

- Preserve borrowed native text configuration/cursor ownership and UTF-16
  surrogate pairs in input callbacks.
- Preserve native text editor callbacks across stale input sessions and failed
  attach/detach operations, preventing delayed keyboard status callbacks from
  accessing a destroyed editor. Reclaim retired native tables after a successful
  lifetime transition and serialize access to the process-wide input proxy.

## [0.3.0](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-ime-binding-v0.2.1...ohos-ime-binding-v0.3.0) - 2026-08-26

### Fixed

- *(ime)* isolate editor sessions and recover stale proxies ([#142](https://github.com/ohos-rs/ohos-native-bindings/pull/142))
- fix some binding issues ([#133](https://github.com/ohos-rs/ohos-native-bindings/pull/133))
