use ohos_crypto_sys::*;
use ohos_enum_derive::EnumFrom;

/// Whether a cipher context encrypts or decrypts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Crypto_CipherMode, "Crypto_CipherMode_")]
pub enum CryptoCipherMode {
    #[suffix("CRYPTO_ENCRYPT_MODE")]
    Encrypt,
    #[suffix("CRYPTO_DECRYPT_MODE")]
    Decrypt,
}

/// Encoding of an exported or imported key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Crypto_EncodingType, "Crypto_EncodingType_")]
pub enum CryptoEncodingType {
    #[suffix("CRYPTO_PEM")]
    Pem,
    #[suffix("CRYPTO_DER")]
    Der,
}

/// Symmetric cipher parameters (GCM / CCM need IV, AAD and tag).
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(CryptoSymCipher_ParamsType, "CryptoSymCipher_ParamsType_")]
pub enum CryptoSymCipherParamsType {
    #[suffix("CRYPTO_IV_DATABLOB")]
    Iv,
    #[suffix("CRYPTO_AAD_DATABLOB")]
    Aad,
    #[suffix("CRYPTO_TAG_DATABLOB")]
    Tag,
}

/// Asymmetric key components, keyed by algorithm family.
///
/// Values suffixed `Int` / `Str` in the C API are still exchanged as byte
/// blobs; only the interpretation differs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(CryptoAsymKey_ParamType, "CryptoAsymKey_ParamType_")]
pub enum CryptoAsymKeyParamType {
    #[suffix("CRYPTO_DSA_P_DATABLOB")]
    DsaP,
    #[suffix("CRYPTO_DSA_Q_DATABLOB")]
    DsaQ,
    #[suffix("CRYPTO_DSA_G_DATABLOB")]
    DsaG,
    #[suffix("CRYPTO_DSA_SK_DATABLOB")]
    DsaSk,
    #[suffix("CRYPTO_DSA_PK_DATABLOB")]
    DsaPk,
    #[suffix("CRYPTO_ECC_FP_P_DATABLOB")]
    EccFpP,
    #[suffix("CRYPTO_ECC_A_DATABLOB")]
    EccA,
    #[suffix("CRYPTO_ECC_B_DATABLOB")]
    EccB,
    #[suffix("CRYPTO_ECC_G_X_DATABLOB")]
    EccGx,
    #[suffix("CRYPTO_ECC_G_Y_DATABLOB")]
    EccGy,
    #[suffix("CRYPTO_ECC_N_DATABLOB")]
    EccN,
    #[suffix("CRYPTO_ECC_H_INT")]
    EccH,
    #[suffix("CRYPTO_ECC_SK_DATABLOB")]
    EccSk,
    #[suffix("CRYPTO_ECC_PK_X_DATABLOB")]
    EccPkX,
    #[suffix("CRYPTO_ECC_PK_Y_DATABLOB")]
    EccPkY,
    #[suffix("CRYPTO_ECC_FIELD_TYPE_STR")]
    EccFieldType,
    #[suffix("CRYPTO_ECC_FIELD_SIZE_INT")]
    EccFieldSize,
    #[suffix("CRYPTO_ECC_CURVE_NAME_STR")]
    EccCurveName,
    #[suffix("CRYPTO_RSA_N_DATABLOB")]
    RsaN,
    #[suffix("CRYPTO_RSA_D_DATABLOB")]
    RsaD,
    #[suffix("CRYPTO_RSA_E_DATABLOB")]
    RsaE,
    #[suffix("CRYPTO_DH_P_DATABLOB")]
    DhP,
    #[suffix("CRYPTO_DH_G_DATABLOB")]
    DhG,
    #[suffix("CRYPTO_DH_L_INT")]
    DhL,
    #[suffix("CRYPTO_DH_SK_DATABLOB")]
    DhSk,
    #[suffix("CRYPTO_DH_PK_DATABLOB")]
    DhPk,
    #[suffix("CRYPTO_ED25519_SK_DATABLOB")]
    Ed25519Sk,
    #[suffix("CRYPTO_ED25519_PK_DATABLOB")]
    Ed25519Pk,
    #[suffix("CRYPTO_X25519_SK_DATABLOB")]
    X25519Sk,
    #[suffix("CRYPTO_X25519_PK_DATABLOB")]
    X25519Pk,
}

/// Signature parameters (RSA-PSS tuning and the SM2 user id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(CryptoSignature_ParamType, "CryptoSignature_ParamType_")]
pub enum CryptoSignatureParamType {
    #[suffix("CRYPTO_PSS_MD_NAME_STR")]
    PssMdName,
    #[suffix("CRYPTO_PSS_MGF_NAME_STR")]
    PssMgfName,
    #[suffix("CRYPTO_PSS_MGF1_NAME_STR")]
    PssMgf1Name,
    #[suffix("CRYPTO_PSS_SALT_LEN_INT")]
    PssSaltLen,
    #[suffix("CRYPTO_PSS_TRAILER_FIELD_INT")]
    PssTrailerField,
    #[suffix("CRYPTO_SM2_USER_ID_DATABLOB")]
    Sm2UserId,
}

/// MAC parameters: HMAC takes a digest name, CMAC a cipher name.
#[cfg(feature = "api-20")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(CryptoMac_ParamType, "CryptoMac_ParamType_")]
pub enum CryptoMacParamType {
    #[suffix("CRYPTO_MAC_DIGEST_NAME_STR")]
    DigestName,
    #[suffix("CRYPTO_MAC_CIPHER_NAME_STR")]
    CipherName,
}

/// Key derivation parameters; which ones apply depends on the KDF algorithm.
#[cfg(feature = "api-20")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(CryptoKdf_ParamType, "CryptoKdf_ParamType_")]
pub enum CryptoKdfParamType {
    #[suffix("CRYPTO_KDF_KEY_DATABLOB")]
    Key,
    #[suffix("CRYPTO_KDF_SALT_DATABLOB")]
    Salt,
    #[suffix("CRYPTO_KDF_INFO_DATABLOB")]
    Info,
    #[suffix("CRYPTO_KDF_ITER_COUNT_INT")]
    IterCount,
    #[suffix("CRYPTO_KDF_SCRYPT_N_UINT64")]
    ScryptN,
    #[suffix("CRYPTO_KDF_SCRYPT_R_UINT64")]
    ScryptR,
    #[suffix("CRYPTO_KDF_SCRYPT_P_UINT64")]
    ScryptP,
    #[suffix("CRYPTO_KDF_SCRYPT_MAX_MEM_UINT64")]
    ScryptMaxMem,
}
