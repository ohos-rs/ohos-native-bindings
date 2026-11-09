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

### Text line height

Use half leading to divide the extra space in a fixed line height equally above
and below the text. The setting is opt-in and available since API 11, without a
Cargo feature:

```rust
use ohos_drawing_binding::TextStyle;

let mut style = TextStyle::new();
style.set_font_size(11.0);
style.set_font_height(20.0 / 11.0).unwrap();
style.set_half_leading(true);
```

`set_half_leading(false)` restores the native default, which distributes leading
according to the font's ascent and descent proportions. This controls line-box
spacing; it does not optically center the visible ink of individual characters.

## License

MIT OR Apache-2.0
