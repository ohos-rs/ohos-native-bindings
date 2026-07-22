use napi_derive_ohos::napi;
use ohos_crypto_binding::{
    AsymKeyGenerator, CryptoCipherMode, CryptoSymCipherParamsType, Digest, Result, SymCipher,
    SymCipherParams, SymKeyGenerator,
};
const TAG: &str = "BINDTEST_CRYPTO";

// Test input, fed to the framework as a byte-string literal (`&[u8; N]`) in some
// calls and as a slice (`&[u8]`) in others.
const MESSAGE: &[u8; 34] = b"ohos-native-bindings crypto blob 1";

#[napi]
pub fn test_crypto() -> String {
    // The hardest FFI path: actually generate an RSA keypair through the framework.
    let msg = match AsymKeyGenerator::new("RSA1024") {
        Ok(gen) => match gen.generate() {
            Ok(kp) => {
                let has_pub = kp.pub_key().is_some();
                format!("RSA1024 keypair generated, pub_key_present={has_pub}")
            }
            Err(e) => format!("generate Err({e})"),
        },
        Err(e) => format!("new Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// SHA256 over the same input twice, fed once as `&[u8; N]` and once as `&[u8]`.
///
/// Self-checking: both digests must be 32 bytes and equal, so no digest constant
/// has to be baked into the test.
fn digest_twice() -> Result<String> {
    let mut first = Digest::new("SHA256")?;
    first.update(MESSAGE)?; // &[u8; N]
    let first = first.finish()?;

    let mut second = Digest::new("SHA256")?;
    second.update(&MESSAGE[..])?; // &[u8]
    let second = second.finish()?;

    let len_ok = first.len() == 32;
    let stable = first == second;
    Ok(format!(
        "SHA256 digest: len={} expected_len=32 len_ok={len_ok} stable={stable} ok={}",
        first.len(),
        len_ok && stable
    ))
}

#[napi]
pub fn test_crypto_digest() -> String {
    let msg = match digest_twice() {
        Ok(msg) => msg,
        Err(e) => format!("digest Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// AES-256-CBC encrypt then decrypt, checking the plaintext comes back.
///
/// The IV is derived from a digest of the message rather than being a fixed
/// constant, so every byte handed to the cipher is really computed here.
fn sym_roundtrip() -> Result<String> {
    let mut digest = Digest::new("SHA256")?;
    digest.update(MESSAGE)?; // &[u8; N]
    let iv = digest.finish()?[..16].to_vec();

    let key = SymKeyGenerator::new("AES256")?.generate()?;
    let mut params = SymCipherParams::new()?;
    params.set(CryptoSymCipherParamsType::Iv, &iv[..])?;

    let mut encrypt = SymCipher::new("AES256|CBC|PKCS7")?;
    encrypt.init(CryptoCipherMode::Encrypt, &key, Some(&params))?;
    let mut ciphertext = encrypt.update(MESSAGE)?; // &[u8; N]
    ciphertext.extend(encrypt.finish(None)?); // Option<CryptoDataBlob>, no last chunk

    let mut decrypt = SymCipher::new("AES256|CBC|PKCS7")?;
    decrypt.init(CryptoCipherMode::Decrypt, &key, Some(&params))?;
    let mut plaintext = decrypt.update(ciphertext.as_slice())?; // &[u8]
    plaintext.extend(decrypt.finish(None)?); // Option<CryptoDataBlob>, no last chunk

    let restored = plaintext == MESSAGE;
    let padded = ciphertext.len() > MESSAGE.len();
    Ok(format!(
        "AES256|CBC|PKCS7 roundtrip: plaintext_len={} ciphertext_len={} padded={padded} restored={restored} ok={}",
        MESSAGE.len(),
        ciphertext.len(),
        restored && padded
    ))
}

#[napi]
pub fn test_crypto_sym_roundtrip() -> String {
    let msg = match sym_roundtrip() {
        Ok(msg) => msg,
        Err(e) => format!("sym roundtrip Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// Export a generated AES key and import it back through
/// `SymKeyGenerator::convert`, which takes the key material as a `&Vec<u8>`.
///
/// Self-checking: the re-imported key must export the same material.
fn sym_key_convert() -> Result<String> {
    let generator = SymKeyGenerator::new("AES128")?;
    let material = generator.generate()?.key_data()?;
    let reimported = generator.convert(&material)?; // &Vec<u8>
    let material_again = reimported.key_data()?;

    let len_ok = material.len() == 16;
    let same = material == material_again;
    Ok(format!(
        "AES128 key convert: key_len={} expected_len=16 len_ok={len_ok} same={same} ok={}",
        material.len(),
        len_ok && same
    ))
}

#[napi]
pub fn test_crypto_sym_key_convert() -> String {
    let msg = match sym_key_convert() {
        Ok(msg) => msg,
        Err(e) => format!("sym key convert Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
