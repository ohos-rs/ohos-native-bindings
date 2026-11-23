# ohos-audio-binding

Safe Rust bindings for OpenHarmony OHAudio (`OH_AudioStreamBuilder`, `OH_AudioRenderer`, and `OH_AudioSessionManager`).

## Install

```shell
cargo add ohos-audio-binding
```

## Usage

```rust
use ohos_audio_binding::{AudioSessionManager, AudioStreamBuilder};

// Configure an `AudioStreamBuilder` to create an `AudioRenderer`, or use
// `AudioSessionManager::get()` to manage audio session concurrency.
```

## License

MIT OR Apache-2.0
