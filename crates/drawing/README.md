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
