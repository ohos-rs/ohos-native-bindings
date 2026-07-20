use crate::error::{check, Result};
use crate::r#type::{
    HuksCipherMode, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose, HuksTag,
};
use ohos_huks_sys::*;

/// A built, owned HUKS parameter set. Frees the native allocation on drop.
///
/// Build one with [`ParamSet::builder`]. A parameter set describes an
/// operation (algorithm, purpose, key size, digest, padding, IV, ...).
pub struct ParamSet {
    raw: *mut OH_Huks_ParamSet,
    // Backing storage for byte-blob parameters, kept alive for the lifetime of
    // the set (the native set may reference this memory).
    _blobs: Vec<Vec<u8>>,
}

impl ParamSet {
    /// Start building a parameter set.
    pub fn builder() -> ParamSetBuilder {
        ParamSetBuilder::default()
    }

    /// Build an empty parameter set — valid input for calls that require a set
    /// but no parameters (e.g. deleting a key).
    pub fn empty() -> Result<ParamSet> {
        ParamSetBuilder::default().build()
    }

    pub(crate) fn as_ptr(&self) -> *const OH_Huks_ParamSet {
        self.raw
    }
}

impl Drop for ParamSet {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: raw was produced by OH_Huks_BuildParamSet and is freed once.
            unsafe { OH_Huks_FreeParamSet(&mut self.raw) };
        }
    }
}

/// Builder for a [`ParamSet`]. Add typed parameters — either via the convenience
/// methods ([`algorithm`](Self::algorithm), [`purposes`](Self::purposes), ...)
/// or the generic `add_*` setters keyed by a [`HuksTag`] — then
/// [`build`](Self::build).
#[derive(Default)]
pub struct ParamSetBuilder {
    params: Vec<OH_Huks_Param>,
    blobs: Vec<Vec<u8>>,
}

impl ParamSetBuilder {
    fn push_scalar(
        mut self,
        tag: HuksTag,
        set: impl FnOnce(&mut OH_Huks_Param__bindgen_ty_1),
    ) -> Self {
        // SAFETY: OH_Huks_Param is a plain-old-data C struct; zeroed is a valid value.
        let mut param: OH_Huks_Param = unsafe { std::mem::zeroed() };
        param.tag = tag.into();
        set(&mut param.__bindgen_anon_1);
        self.params.push(param);
        self
    }

    /// Add a boolean parameter.
    pub fn add_bool(self, tag: HuksTag, value: bool) -> Self {
        self.push_scalar(tag, |u| u.boolParam = value)
    }

    /// Add a signed 32-bit parameter.
    pub fn add_i32(self, tag: HuksTag, value: i32) -> Self {
        self.push_scalar(tag, |u| u.int32Param = value)
    }

    /// Add an unsigned 32-bit parameter.
    pub fn add_u32(self, tag: HuksTag, value: u32) -> Self {
        self.push_scalar(tag, |u| u.uint32Param = value)
    }

    /// Add an unsigned 64-bit parameter.
    pub fn add_u64(self, tag: HuksTag, value: u64) -> Self {
        self.push_scalar(tag, |u| u.uint64Param = value)
    }

    /// Add a byte-blob parameter (IV, nonce, AAD, ...). The bytes are copied and
    /// kept alive by the resulting [`ParamSet`].
    pub fn add_bytes(mut self, tag: HuksTag, data: &[u8]) -> Self {
        let mut owned = data.to_vec();
        let blob = OH_Huks_Blob {
            size: owned.len() as u32,
            data: owned.as_mut_ptr(),
        };
        // SAFETY: zeroed OH_Huks_Param is valid POD; we set tag + blob field.
        let mut param: OH_Huks_Param = unsafe { std::mem::zeroed() };
        param.tag = tag.into();
        param.__bindgen_anon_1.blob = blob;
        self.params.push(param);
        // Moving `owned` into the Vec keeps its heap buffer address stable, so the
        // pointer stored in `blob` stays valid.
        self.blobs.push(owned);
        self
    }

    /// Set the key algorithm.
    pub fn algorithm(self, alg: HuksKeyAlg) -> Self {
        self.add_u32(HuksTag::Algorithm, alg.into())
    }

    /// Set the key purpose(s); several are OR-ed together.
    pub fn purposes(self, purposes: &[HuksKeyPurpose]) -> Self {
        let bits = purposes.iter().fold(0u32, |acc, p| acc | u32::from(*p));
        self.add_u32(HuksTag::Purpose, bits)
    }

    /// Set the key size in bits (e.g. `2048` for RSA, `256` for AES).
    pub fn key_size(self, bits: u32) -> Self {
        self.add_u32(HuksTag::KeySize, bits)
    }

    /// Set the digest.
    pub fn digest(self, digest: HuksKeyDigest) -> Self {
        self.add_u32(HuksTag::Digest, digest.into())
    }

    /// Set the padding.
    pub fn padding(self, padding: HuksKeyPadding) -> Self {
        self.add_u32(HuksTag::Padding, padding.into())
    }

    /// Set the block / cipher mode.
    pub fn block_mode(self, mode: HuksCipherMode) -> Self {
        self.add_u32(HuksTag::BlockMode, mode.into())
    }

    /// Initialise, populate and finalise the native parameter set.
    pub fn build(self) -> Result<ParamSet> {
        let mut raw: *mut OH_Huks_ParamSet = std::ptr::null_mut();
        // SAFETY: standard HUKS Init -> AddParams -> Build sequence; the set is
        // freed on any early error and otherwise owned by the returned ParamSet.
        unsafe {
            check(OH_Huks_InitParamSet(&mut raw))?;
            if !self.params.is_empty() {
                if let Err(e) = check(OH_Huks_AddParams(
                    raw,
                    self.params.as_ptr(),
                    self.params.len() as u32,
                )) {
                    OH_Huks_FreeParamSet(&mut raw);
                    return Err(e);
                }
            }
            if let Err(e) = check(OH_Huks_BuildParamSet(&mut raw)) {
                OH_Huks_FreeParamSet(&mut raw);
                return Err(e);
            }
        }
        Ok(ParamSet {
            raw,
            _blobs: self.blobs,
        })
    }
}
