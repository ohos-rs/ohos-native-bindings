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
    #[fallback]
    Unknown,
}
```

### Customizing Fallback Behavior

By default, `From<TargetEnum> for LocalEnum` panics on unrecognized raw values while `LocalEnum::try_from_raw` returns `None`. To handle future or device-specific OS discriminants without panicking across FFI boundaries, configure a fallback:

- **Unit variant mapped to an existing C constant**: `#[fallback] Unknown`
- **Unit variant with a custom raw value on reverse conversion**: `#[fallback(raw = -1)] Invalid`
- **Single-field tuple variant preserving the raw value**: `#[fallback] Other(u32)`
- **Enum-level custom fallback expression/closure**: `#[fallback(|raw| ...)]`

## License

MIT OR Apache-2.0
