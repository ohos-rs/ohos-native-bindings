# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add thin `NativeImage` consumer ownership (`ConsumerSurface`), fenced buffer guards (`Frame`), separate producer-window/identity queries, and caller-selected typed `BufferUsage`.
- Expose the native `FrameAvailableListener` descriptor with an explicit unsafe context lifetime, callback-quiescence, and producer-thread contract.
