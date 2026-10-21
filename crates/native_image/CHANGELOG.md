# Changelog

## [Unreleased]

### Added

- Add thin NativeImage consumer ownership, fenced buffer guards, separate
  producer-window/identity queries and caller-selected typed usage.
- Expose the native listener descriptor with an explicit unsafe context
  lifetime, callback-quiescence and producer-thread contract.

### Breaking

- Withdraw the unpublished closure registry, `id`, `RgbaFrame` and
  `Frame::copy_rgba`; consumers own pixel mapping/conversion and scheduling.
- The default constructor takes no callback or implicit Rust-selected usage;
  `with_usage` / `set_usage` require API 13.

### Fixed

- Name the native no-buffer status and log failed native cleanup.
- Run native-substitute owner tests from target/ in CI.
