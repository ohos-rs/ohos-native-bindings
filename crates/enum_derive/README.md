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

## License

MIT OR Apache-2.0
