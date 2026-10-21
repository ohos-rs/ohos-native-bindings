# ohos-drawing-binding

This crate is a binding for the drawing module in OpenHarmony.

## Install

```shell
cargo add ohos-drawing-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_drawing_binding as drawing;

// Use the safe Rust APIs exposed by `ohos-drawing-binding` from your native module.
```

## License

MIT OR Apache-2.0

`ImageFilter` owns API-12 bitmap filters; `Filter::set_image_filter` retains the
native filter value. `ColorFilter::blend` blends an ARGB color (API 11).
`Matrix::from_values`, `values` and cloning preserve all nine row-major
coefficients, including perspective. These owners stay on their drawing thread.

Typography exposes copied font metrics, line counts, UTF-16 range boxes and
placeholder rectangles. Returned rectangles own their data; native arrays are
released before return. Decorations and ellipsis use API11/12 setters. Text
input preserves embedded NUL on API20's length-aware path; earlier APIs render
NUL as U+FFFD while preserving the rest of the UTF-8 text.
