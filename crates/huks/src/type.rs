use ohos_enum_derive::EnumFrom;
use ohos_huks_sys::*;

/// Key algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_KeyAlg, "OH_Huks_KeyAlg_")]
pub enum HuksKeyAlg {
    #[suffix("OH_HUKS_ALG_RSA")]
    Rsa,
    #[suffix("OH_HUKS_ALG_ECC")]
    Ecc,
    #[suffix("OH_HUKS_ALG_DSA")]
    Dsa,
    #[suffix("OH_HUKS_ALG_AES")]
    Aes,
    #[suffix("OH_HUKS_ALG_HMAC")]
    Hmac,
    #[suffix("OH_HUKS_ALG_HKDF")]
    Hkdf,
    #[suffix("OH_HUKS_ALG_PBKDF2")]
    Pbkdf2,
    #[suffix("OH_HUKS_ALG_ECDH")]
    Ecdh,
    #[suffix("OH_HUKS_ALG_X25519")]
    X25519,
    #[suffix("OH_HUKS_ALG_ED25519")]
    Ed25519,
    #[suffix("OH_HUKS_ALG_DH")]
    Dh,
    #[suffix("OH_HUKS_ALG_SM2")]
    Sm2,
    #[suffix("OH_HUKS_ALG_SM3")]
    Sm3,
    #[suffix("OH_HUKS_ALG_SM4")]
    Sm4,
    #[cfg(feature = "api-18")]
    #[suffix("OH_HUKS_ALG_DES")]
    Des,
    #[cfg(feature = "api-18")]
    #[suffix("OH_HUKS_ALG_3DES")]
    TripleDes,
    #[cfg(feature = "api-18")]
    #[suffix("OH_HUKS_ALG_CMAC")]
    Cmac,
}

/// Key purposes; combine several via `ParamSetBuilder::purposes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_KeyPurpose, "OH_Huks_KeyPurpose_")]
pub enum HuksKeyPurpose {
    #[suffix("OH_HUKS_KEY_PURPOSE_ENCRYPT")]
    Encrypt,
    #[suffix("OH_HUKS_KEY_PURPOSE_DECRYPT")]
    Decrypt,
    #[suffix("OH_HUKS_KEY_PURPOSE_SIGN")]
    Sign,
    #[suffix("OH_HUKS_KEY_PURPOSE_VERIFY")]
    Verify,
    #[suffix("OH_HUKS_KEY_PURPOSE_DERIVE")]
    Derive,
    #[suffix("OH_HUKS_KEY_PURPOSE_WRAP")]
    Wrap,
    #[suffix("OH_HUKS_KEY_PURPOSE_UNWRAP")]
    Unwrap,
    #[suffix("OH_HUKS_KEY_PURPOSE_MAC")]
    Mac,
    #[suffix("OH_HUKS_KEY_PURPOSE_AGREE")]
    Agree,
}

/// Digests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_KeyDigest, "OH_Huks_KeyDigest_")]
pub enum HuksKeyDigest {
    #[suffix("OH_HUKS_DIGEST_NONE")]
    None,
    #[suffix("OH_HUKS_DIGEST_MD5")]
    Md5,
    #[suffix("OH_HUKS_DIGEST_SM3")]
    Sm3,
    #[suffix("OH_HUKS_DIGEST_SHA1")]
    Sha1,
    #[suffix("OH_HUKS_DIGEST_SHA224")]
    Sha224,
    #[suffix("OH_HUKS_DIGEST_SHA256")]
    Sha256,
    #[suffix("OH_HUKS_DIGEST_SHA384")]
    Sha384,
    #[suffix("OH_HUKS_DIGEST_SHA512")]
    Sha512,
}

/// Paddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_KeyPadding, "OH_Huks_KeyPadding_")]
pub enum HuksKeyPadding {
    #[suffix("OH_HUKS_PADDING_NONE")]
    None,
    #[suffix("OH_HUKS_PADDING_OAEP")]
    Oaep,
    #[suffix("OH_HUKS_PADDING_PSS")]
    Pss,
    #[suffix("OH_HUKS_PADDING_PKCS1_V1_5")]
    Pkcs1V15,
    #[suffix("OH_HUKS_PADDING_PKCS5")]
    Pkcs5,
    #[suffix("OH_HUKS_PADDING_PKCS7")]
    Pkcs7,
    #[cfg(feature = "api-18")]
    #[suffix("OH_HUKS_PADDING_ISO_IEC_9796_2")]
    IsoIec97962,
    #[cfg(feature = "api-18")]
    #[suffix("OH_HUKS_PADDING_ISO_IEC_9797_1")]
    IsoIec97971,
}

/// Block / cipher modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_CipherMode, "OH_Huks_CipherMode_")]
pub enum HuksCipherMode {
    #[suffix("OH_HUKS_MODE_ECB")]
    Ecb,
    #[suffix("OH_HUKS_MODE_CBC")]
    Cbc,
    #[suffix("OH_HUKS_MODE_CTR")]
    Ctr,
    #[suffix("OH_HUKS_MODE_OFB")]
    Ofb,
    #[suffix("OH_HUKS_MODE_CFB")]
    Cfb,
    #[suffix("OH_HUKS_MODE_CCM")]
    Ccm,
    #[suffix("OH_HUKS_MODE_GCM")]
    Gcm,
}

/// HUKS parameter tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Huks_Tag, "OH_Huks_Tag_")]
pub enum HuksTag {
    #[suffix("OH_HUKS_TAG_ALGORITHM")]
    Algorithm,
    #[suffix("OH_HUKS_TAG_PURPOSE")]
    Purpose,
    #[suffix("OH_HUKS_TAG_KEY_SIZE")]
    KeySize,
    #[suffix("OH_HUKS_TAG_DIGEST")]
    Digest,
    #[suffix("OH_HUKS_TAG_PADDING")]
    Padding,
    #[suffix("OH_HUKS_TAG_BLOCK_MODE")]
    BlockMode,
    #[suffix("OH_HUKS_TAG_KEY_TYPE")]
    KeyType,
    #[suffix("OH_HUKS_TAG_ASSOCIATED_DATA")]
    AssociatedData,
    #[suffix("OH_HUKS_TAG_NONCE")]
    Nonce,
    #[suffix("OH_HUKS_TAG_IV")]
    Iv,
    #[suffix("OH_HUKS_TAG_INFO")]
    Info,
    #[suffix("OH_HUKS_TAG_SALT")]
    Salt,
    #[suffix("OH_HUKS_TAG_ITERATION")]
    Iteration,
    #[suffix("OH_HUKS_TAG_KEY_GENERATE_TYPE")]
    KeyGenerateType,
    #[suffix("OH_HUKS_TAG_AGREE_ALG")]
    AgreeAlg,
    #[suffix("OH_HUKS_TAG_AGREE_PUBLIC_KEY_IS_KEY_ALIAS")]
    AgreePublicKeyIsKeyAlias,
    #[suffix("OH_HUKS_TAG_AGREE_PRIVATE_KEY_ALIAS")]
    AgreePrivateKeyAlias,
    #[suffix("OH_HUKS_TAG_AGREE_PUBLIC_KEY")]
    AgreePublicKey,
    #[suffix("OH_HUKS_TAG_KEY_ALIAS")]
    KeyAlias,
    #[suffix("OH_HUKS_TAG_DERIVE_KEY_SIZE")]
    DeriveKeySize,
    #[suffix("OH_HUKS_TAG_IMPORT_KEY_TYPE")]
    ImportKeyType,
    #[suffix("OH_HUKS_TAG_UNWRAP_ALGORITHM_SUITE")]
    UnwrapAlgorithmSuite,
    #[suffix("OH_HUKS_TAG_DERIVED_AGREED_KEY_STORAGE_FLAG")]
    DerivedAgreedKeyStorageFlag,
    #[suffix("OH_HUKS_TAG_RSA_PSS_SALT_LEN_TYPE")]
    RsaPssSaltLenType,
    #[suffix("OH_HUKS_TAG_ALL_USERS")]
    AllUsers,
    #[suffix("OH_HUKS_TAG_USER_ID")]
    UserId,
    #[suffix("OH_HUKS_TAG_NO_AUTH_REQUIRED")]
    NoAuthRequired,
    #[suffix("OH_HUKS_TAG_USER_AUTH_TYPE")]
    UserAuthType,
    #[suffix("OH_HUKS_TAG_AUTH_TIMEOUT")]
    AuthTimeout,
    #[suffix("OH_HUKS_TAG_AUTH_TOKEN")]
    AuthToken,
    #[suffix("OH_HUKS_TAG_KEY_AUTH_ACCESS_TYPE")]
    KeyAuthAccessType,
    #[suffix("OH_HUKS_TAG_KEY_SECURE_SIGN_TYPE")]
    KeySecureSignType,
    #[suffix("OH_HUKS_TAG_CHALLENGE_TYPE")]
    ChallengeType,
    #[suffix("OH_HUKS_TAG_CHALLENGE_POS")]
    ChallengePos,
    #[suffix("OH_HUKS_TAG_KEY_AUTH_PURPOSE")]
    KeyAuthPurpose,
    #[suffix("OH_HUKS_TAG_AUTH_STORAGE_LEVEL")]
    AuthStorageLevel,
    #[suffix("OH_HUKS_TAG_USER_AUTH_MODE")]
    UserAuthMode,
    #[suffix("OH_HUKS_TAG_ATTESTATION_CHALLENGE")]
    AttestationChallenge,
    #[suffix("OH_HUKS_TAG_ATTESTATION_APPLICATION_ID")]
    AttestationApplicationId,
    #[suffix("OH_HUKS_TAG_ATTESTATION_ID_ALIAS")]
    AttestationIdAlias,
    #[suffix("OH_HUKS_TAG_ATTESTATION_ID_SEC_LEVEL_INFO")]
    AttestationIdSecLevelInfo,
    #[suffix("OH_HUKS_TAG_ATTESTATION_ID_VERSION_INFO")]
    AttestationIdVersionInfo,
    #[cfg(feature = "api-20")]
    #[suffix("OH_HUKS_TAG_KEY_OVERRIDE")]
    KeyOverride,
    #[cfg(feature = "api-22")]
    #[suffix("OH_HUKS_TAG_AE_TAG_LEN")]
    AeTagLen,
    #[cfg(feature = "api-22")]
    #[suffix("OH_HUKS_TAG_KEY_CLASS")]
    KeyClass,
    #[cfg(feature = "api-23")]
    #[suffix("OH_HUKS_TAG_KEY_ACCESS_GROUP")]
    KeyAccessGroup,
    #[suffix("OH_HUKS_TAG_IS_KEY_ALIAS")]
    IsKeyAlias,
    #[suffix("OH_HUKS_TAG_KEY_STORAGE_FLAG")]
    KeyStorageFlag,
    #[suffix("OH_HUKS_TAG_IS_ALLOWED_WRAP")]
    IsAllowedWrap,
    #[suffix("OH_HUKS_TAG_KEY_WRAP_TYPE")]
    KeyWrapType,
    #[suffix("OH_HUKS_TAG_KEY_AUTH_ID")]
    KeyAuthId,
    #[suffix("OH_HUKS_TAG_KEY_ROLE")]
    KeyRole,
    #[suffix("OH_HUKS_TAG_KEY_FLAG")]
    KeyFlag,
    #[suffix("OH_HUKS_TAG_IS_ASYNCHRONIZED")]
    IsAsynchronized,
    #[suffix("OH_HUKS_TAG_KEY_DOMAIN")]
    KeyDomain,
    #[suffix("OH_HUKS_TAG_IS_DEVICE_PASSWORD_SET")]
    IsDevicePasswordSet,
    #[suffix("OH_HUKS_TAG_AE_TAG")]
    AeTag,
    #[suffix("OH_HUKS_TAG_SYMMETRIC_KEY_DATA")]
    SymmetricKeyData,
    #[suffix("OH_HUKS_TAG_ASYMMETRIC_PUBLIC_KEY_DATA")]
    AsymmetricPublicKeyData,
    #[suffix("OH_HUKS_TAG_ASYMMETRIC_PRIVATE_KEY_DATA")]
    AsymmetricPrivateKeyData,
}

/// The value type a [`HuksTag`] declares, encoded in the tag's top bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HuksTagType {
    Invalid,
    Int,
    Uint,
    Ulong,
    Bool,
    Bytes,
}

impl HuksTag {
    /// The value type this tag expects. HUKS encodes it in the tag's top bits, so
    /// a parameter's type is fixed by its tag rather than chosen by the caller.
    pub fn value_type(self) -> HuksTagType {
        const MASK: u32 = 0xF << 28;
        match u32::from(self) & MASK {
            ohos_huks_sys::OH_Huks_TagType_OH_HUKS_TAG_TYPE_INT => HuksTagType::Int,
            ohos_huks_sys::OH_Huks_TagType_OH_HUKS_TAG_TYPE_UINT => HuksTagType::Uint,
            ohos_huks_sys::OH_Huks_TagType_OH_HUKS_TAG_TYPE_ULONG => HuksTagType::Ulong,
            ohos_huks_sys::OH_Huks_TagType_OH_HUKS_TAG_TYPE_BOOL => HuksTagType::Bool,
            ohos_huks_sys::OH_Huks_TagType_OH_HUKS_TAG_TYPE_BYTES => HuksTagType::Bytes,
            _ => HuksTagType::Invalid,
        }
    }
}
