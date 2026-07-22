# ohos-crypto-binding

This crate is a binding for the Crypto Architecture Kit in OpenHarmony.

The kit is the framework's general-purpose software crypto library: keys live in
the application process and can be imported and exported, which is what sets it
apart from HUKS. This crate wraps the native `libohcrypto` C API with RAII
contexts and `Result`-based error handling.

Output buffers are allocated by the framework; the safe layer copies them out
and releases them, so every operation returns a plain `Vec<u8>`.

## Install

```shell
cargo add ohos-crypto-binding
```

## Usage

```rust
use ohos_crypto_binding as crypto;
use crypto::{CryptoCipherMode, CryptoSymCipherParamsType, SymCipher, SymCipherParams, SymKeyGenerator};

// AES-256-GCM encryption under a freshly generated key.
let key = SymKeyGenerator::new("AES256")?.generate()?;

let mut params = SymCipherParams::new()?;
params.set(CryptoSymCipherParamsType::Iv, &[0u8; 12][..])?;

let mut cipher = SymCipher::new("AES256|GCM|PKCS7")?;
cipher.init(CryptoCipherMode::Encrypt, &key, Some(&params))?;
let mut ciphertext = cipher.update(b"hello")?;
ciphertext.extend(cipher.finish(None)?);
```

The raw bindings are re-exported as `crypto::sys` for anything not yet covered
by the safe layer.

## Coverage

Available with the default features (API 12):

- Symmetric crypto: `SymKeyGenerator`, `SymKey`, `SymCipherParams`, `SymCipher`.
- Message digests: `Digest`.
- Asymmetric keys: `AsymKeyGenerator`, `KeyPair`, `PubKey`.
- Signature verification: `Verify`.

Behind the `api-20` feature:

- Signing: `Sign`, and the private half of a key pair as `PrivKey`.
- `Mac`, `Kdf` / `KdfParams`, `KeyAgreement`, `AsymCipher`, `Rand`
  (`Rand::enable_hardware_entropy` needs `api-21`).

`PubKey` and `PrivKey` are views into the `KeyPair` that produced them — the C
API has no destructor for either — so they borrow it and cannot outlive it.

Not covered yet: the key specification family (`OH_CryptoAsymKeySpec`,
`OH_CryptoAsymKeyGeneratorWithSpec`, `OH_CryptoEcPoint`), the signature and
ciphertext component specs (`OH_CryptoEccSignatureSpec`,
`OH_CryptoSm2CiphertextSpec`) and password-protected private key encoding
(`OH_CryptoPrivKeyEncodingParams`).

## License

MIT OR Apache-2.0
