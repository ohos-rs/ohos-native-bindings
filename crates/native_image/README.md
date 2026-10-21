# NativeImage

Owned OpenHarmony NativeImage consumer surfaces and fenced CPU frames.
The API-12 default uses native usage flags; `api-13` configures CPU-readable,
GPU-renderable usage and exposes consumer resizing. Newer API features require
the matching runtime and forward to every direct native dependency.

`ConsumerSurface` serializes native operations through mutable ownership.
`Frame` retains its consumer, acquired native buffer and fence until release.
Wait on a rendering worker before copying pixels. Copies validate the complete
mapped row extent and return packed RGBA while preserving producer alpha values;
RGBX formats become opaque. Dimensions and allocation sizes are checked; callers
choose their own frame and aggregate memory budgets.

Producer callbacks must only enqueue a wake. They must not call NativeImage
interfaces or retire consumers from the callback. Callback identities are never reused, so late callbacks cannot
address released Rust storage.

Native declarations are generated from `native_image/native_image.h` through
`tools/generate/build/config/native_image.rs`; extend that configuration and
regenerate when adding native APIs.
