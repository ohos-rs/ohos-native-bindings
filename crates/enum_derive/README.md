# ohos-enum-derive

This crate provides derive macros for converting Rust enums to and from OpenHarmony FFI enum values.

## Install

```shell
cargo add ohos-enum-derive
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_enum_derive::EnumFrom;

#[derive(EnumFrom)]
#[config(TargetEnum, "TARGET_PREFIX_")]
enum LocalEnum {
    Example,
}
```

## Never cast a derived enum with `as`

The derive generates conversions; it does **not** assign explicit discriminants to
the Rust enum. `LocalEnum::Example as u32` is therefore the variant's position in
the declaration, not the FFI constant it is mapped to — the two agree only when
the native enum happens to number from zero without gaps. Across FFI that quietly
selects a different, still-valid setting instead of failing, so always convert
through the generated impl:

```rust,ignore
let raw = u32::from(LocalEnum::Example); // correct: the FFI constant
let bad = LocalEnum::Example as u32;     // wrong: the declaration index
```

## License

MIT OR Apache-2.0
