# ohos-audio-base-sys

This crate provides low-level shared audio types used by the OHAudio and
OHAudioSuite native APIs in OpenHarmony.

The bindings are generated from:

- `multimedia/native_audio_channel_layout.h`

This header does not have a standalone native library. The crate is intended
to be consumed through `ohos-audio-sys` and `ohos-audiosuite-sys`, both of
which re-export its public items.

## License

MIT OR Apache-2.0
