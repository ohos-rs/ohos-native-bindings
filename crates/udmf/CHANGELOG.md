# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add API-12 general-entry byte copies and type enumeration for owned and
  borrowed records, plus data-container type enumeration and membership queries.
- Add API-12 `plain_text()` / `html()` typed getters on owned and borrowed
  records for interoperation with standard text clipboards.
- Reject empty general-entry input before native calls with `UdmfError::EmptyEntry`.

- Add fallible file-URI records and typed file URI/type getters and setters (API 13).
- Add fallible data and record allocation.

### Breaking

- `UdmfError` has a new `EmptyEntry` variant. Update exhaustive matches.

### Fixed

- Release owned UDMF containers on drop. Return lifetime-bound, non-owning records from `record`/`records`; these methods and `add_record` now require mutable container access.

## [0.0.6](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-udmf-binding-v0.0.5...ohos-udmf-binding-v0.0.6) - 2026-08-26

### Other

- updated the following local packages: ohos-udmf-sys, ohos-image-native-binding
