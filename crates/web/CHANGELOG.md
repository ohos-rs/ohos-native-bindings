# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking changes

- HTTP body reads consume the stream and initialize lazily. Single reads return
  ownership with the bytes; full reads and errors close the stream.
- Report typed errors through one completion callback; initialize explicitly only
  when metadata is needed before reading.
- Prevent overlapping operations through ownership, preserve the current position,
  and handle synchronous callbacks without recursive reads through EOF.
- Keep native streams and output buffers owned by the pending operation until
  completion, without shared handles or self-retaining reference cycles.

## [0.2.3](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-web-binding-v0.2.2...ohos-web-binding-v0.2.3) - 2026-08-26

### Other

- migrate examples UI workspace ([#143](https://github.com/ohos-rs/ohos-native-bindings/pull/143))
