use once_cell::sync::Lazy;

use crate::SysConfig;

pub const CRYPTO: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-crypto-sys",
    headers: vec![
        "CryptoArchitectureKit/crypto_common.h",
        "CryptoArchitectureKit/crypto_sym_key.h",
        "CryptoArchitectureKit/crypto_sym_cipher.h",
        "CryptoArchitectureKit/crypto_asym_key.h",
        "CryptoArchitectureKit/crypto_asym_cipher.h",
        "CryptoArchitectureKit/crypto_digest.h",
        "CryptoArchitectureKit/crypto_mac.h",
        "CryptoArchitectureKit/crypto_kdf.h",
        "CryptoArchitectureKit/crypto_key_agreement.h",
        "CryptoArchitectureKit/crypto_rand.h",
        "CryptoArchitectureKit/crypto_signature.h",
    ],
    white_list: vec![
        // Functions / opaque contexts: OH_CryptoSymKey, OH_CryptoDigest, OH_CryptoSign, ...
        // plus OH_Crypto_ErrCode and OH_Crypto_FreeDataBlob.
        "OH_Crypto.*",
        // Naming outlier in crypto_digest.h: the digest destructor is OH_DigestCrypto_Destroy,
        // not OH_CryptoDigest_Destroy, so it is not covered by "OH_Crypto.*".
        "OH_DigestCrypto.*",
        // Crypto_DataBlob / Crypto_CipherMode / Crypto_EncodingType and the
        // CryptoXxx_ParamType enum type names (CryptoKdf_ParamType, CryptoMac_ParamType, ...).
        "Crypto.*",
        // Enum constants: CRYPTO_SUCCESS, CRYPTO_KDF_*, CRYPTO_ECC_*, CRYPTO_PSS_*, ...
        "CRYPTO_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohcrypto"],
    extra: "",
});
