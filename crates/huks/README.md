# ohos-huks-binding

This crate is a binding for the HUKS (Universal Keystore) module in OpenHarmony.

HUKS is the HarmonyOS hardware-backed key store: keys are generated and used
inside the keystore and never leave it. This crate wraps the native
`native_huks_api.h` C API with a safe layer — an RAII `ParamSet` builder,
`Result`-based error handling, key management, and a three-stage crypto session.

## Install

```shell
cargo add ohos-huks-binding
```

## Usage

```rust
use ohos_huks_binding as huks;
use huks::{HuksAlias, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose, ParamSet};

// Generate an RSA-2048 sign/verify key.
let params = ParamSet::builder()
    .algorithm(HuksKeyAlg::Rsa)
    .purposes(&[HuksKeyPurpose::Sign, HuksKeyPurpose::Verify])
    .key_size(2048)
    .digest(HuksKeyDigest::Sha256)
    .padding(HuksKeyPadding::Pss)
    .build()?;

let alias = HuksAlias::new(b"my_key")?;
alias.generate(&params)?;
assert!(alias.exists()?);
alias.delete()?;
```

The raw bindings are re-exported as `huks::sys` for anything not yet covered by
the safe layer.

## Coverage

- Key management: `HuksAlias::generate`, `import`, `export_public_key`, `delete`,
  `exists`.
- Crypto sessions: `HuksAlias::init_session` → `Session::update` → `Session::finish` /
  `Session::abort` (sign / verify / encrypt / decrypt / mac / derive), with the
  auth token from `init` available via `Session::token`.
- Parameter building: `ParamSet` / `ParamSetBuilder`, with a single `add` keyed by
  `HuksTag` plus named setters for the common ones. HUKS encodes a parameter's type
  in its tag, so a value that does not match its tag is rejected.
- Types: `HuksBlob` for borrowed byte inputs, `HuksAlias` for key names, and `EnumFrom`-derived enums (`HuksKeyAlg`,
  `HuksKeyPurpose`, `HuksKeyDigest`, `HuksKeyPadding`, `HuksCipherMode`, `HuksTag`).

## License

MIT OR Apache-2.0
