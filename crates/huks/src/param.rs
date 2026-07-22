use crate::error::{check, HuksError, Result};
use crate::r#type::{
    HuksCipherMode, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose, HuksTag, HuksTagType,
};
use ohos_huks_sys::*;
use std::ptr::NonNull;

/// A value carried by a HUKS parameter.
///
/// Which variant a parameter takes is fixed by its [`HuksTag`], not chosen by the
/// caller — see [`HuksTag::value_type`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HuksValue {
    Bool(bool),
    Int(i32),
    Uint(u32),
    Ulong(u64),
    Bytes(Vec<u8>),
}

impl HuksValue {
    fn value_type(&self) -> HuksTagType {
        match self {
            HuksValue::Bool(_) => HuksTagType::Bool,
            HuksValue::Int(_) => HuksTagType::Int,
            HuksValue::Uint(_) => HuksTagType::Uint,
            HuksValue::Ulong(_) => HuksTagType::Ulong,
            HuksValue::Bytes(_) => HuksTagType::Bytes,
        }
    }
}

/// Conversion into a [`HuksValue`], so one `add` covers every parameter type.
pub trait IntoHuksValue {
    fn into_huks_value(self) -> HuksValue;
}

macro_rules! into_huks_value {
    ($($ty:ty => $variant:ident $(as $cast:ty)?),* $(,)?) => {
        $(impl IntoHuksValue for $ty {
            fn into_huks_value(self) -> HuksValue {
                HuksValue::$variant(self $(as $cast)?)
            }
        })*
    };
}

into_huks_value! {
    bool => Bool,
    i32 => Int,
    u32 => Uint,
    u64 => Ulong,
    Vec<u8> => Bytes,
}

/// Same, for the `EnumFrom` enums.
///
/// These carry no explicit discriminants, so an `as u32` cast would yield the
/// variant's declaration index rather than the constant its `#[suffix(...)]`
/// names. The number HUKS expects only comes out of the generated `From` impl,
/// which is built from the header constants — so that is what is used here.
macro_rules! into_huks_value_via_from {
    ($($ty:ty),* $(,)?) => {
        $(impl IntoHuksValue for $ty {
            fn into_huks_value(self) -> HuksValue {
                HuksValue::Uint(u32::from(self))
            }
        })*
    };
}

into_huks_value_via_from! {
    HuksKeyAlg,
    HuksKeyDigest,
    HuksKeyPadding,
    HuksCipherMode,
    HuksKeyPurpose,
}

impl IntoHuksValue for &[u8] {
    fn into_huks_value(self) -> HuksValue {
        HuksValue::Bytes(self.to_vec())
    }
}

/// A built, owned HUKS parameter set. Frees the native allocation on drop.
///
/// Build one with [`ParamSet::builder`]. A parameter set describes an
/// operation (algorithm, purpose, key size, digest, padding, IV, ...).
pub struct ParamSet {
    raw: NonNull<OH_Huks_ParamSet>,
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
        self.raw.as_ptr()
    }
}

impl Drop for ParamSet {
    fn drop(&mut self) {
        let mut raw = self.raw.as_ptr();
        // SAFETY: raw was produced by OH_Huks_BuildParamSet and is freed once.
        unsafe { OH_Huks_FreeParamSet(&mut raw) };
    }
}

/// Builder for a [`ParamSet`]. Add parameters via the convenience methods
/// ([`algorithm`](Self::algorithm), [`purposes`](Self::purposes), ...) or the generic
/// [`add`](Self::add), then [`build`](Self::build).
///
/// A value whose type does not match its tag is rejected. The error surfaces from
/// `build` rather than `add`, so the setters stay chainable.
#[derive(Default)]
pub struct ParamSetBuilder {
    params: Vec<OH_Huks_Param>,
    blobs: Vec<Vec<u8>>,
    error: Option<HuksError>,
}

impl ParamSetBuilder {
    /// Add a parameter. The value must match the type the tag declares.
    pub fn add(mut self, tag: HuksTag, value: impl IntoHuksValue) -> Self {
        let value = value.into_huks_value();
        let expected = tag.value_type();
        if expected != value.value_type() {
            if self.error.is_none() {
                self.error = Some(HuksError::illegal_argument(format!(
                    "tag {tag:?} expects {expected:?}, got {:?}",
                    value.value_type()
                )));
            }
            return self;
        }

        // SAFETY: OH_Huks_Param is a plain-old-data C struct; zeroed is a valid value.
        let mut param: OH_Huks_Param = unsafe { std::mem::zeroed() };
        param.tag = tag.into();
        match value {
            HuksValue::Bool(v) => param.__bindgen_anon_1.boolParam = v,
            HuksValue::Int(v) => param.__bindgen_anon_1.int32Param = v,
            HuksValue::Uint(v) => param.__bindgen_anon_1.uint32Param = v,
            HuksValue::Ulong(v) => param.__bindgen_anon_1.uint64Param = v,
            HuksValue::Bytes(bytes) => {
                let mut owned = bytes;
                let Ok(size) = u32::try_from(owned.len()) else {
                    self.error = Some(HuksError::illegal_argument(
                        "blob length does not fit in OH_Huks_Blob::size",
                    ));
                    return self;
                };
                param.__bindgen_anon_1.blob = OH_Huks_Blob {
                    size,
                    data: owned.as_mut_ptr(),
                };
                // Moving `owned` into the Vec keeps its heap buffer address stable, so
                // the pointer stored in the blob stays valid.
                self.blobs.push(owned);
            }
        }
        self.params.push(param);
        self
    }

    /// Set the key algorithm.
    pub fn algorithm(self, alg: HuksKeyAlg) -> Self {
        self.add(HuksTag::Algorithm, alg)
    }

    /// Set the key purpose(s); several are OR-ed together.
    pub fn purposes(self, purposes: &[HuksKeyPurpose]) -> Self {
        let bits = purposes.iter().fold(0u32, |acc, p| acc | u32::from(*p));
        self.add(HuksTag::Purpose, bits)
    }

    /// Set the key size in bits (e.g. `2048` for RSA, `256` for AES).
    pub fn key_size(self, bits: u32) -> Self {
        self.add(HuksTag::KeySize, bits)
    }

    /// Set the digest.
    pub fn digest(self, digest: HuksKeyDigest) -> Self {
        self.add(HuksTag::Digest, digest)
    }

    /// Set the padding.
    pub fn padding(self, padding: HuksKeyPadding) -> Self {
        self.add(HuksTag::Padding, padding)
    }

    /// Set the block / cipher mode.
    pub fn block_mode(self, mode: HuksCipherMode) -> Self {
        self.add(HuksTag::BlockMode, mode)
    }

    /// Initialise, populate and finalise the native parameter set.
    pub fn build(self) -> Result<ParamSet> {
        if let Some(e) = self.error {
            return Err(e);
        }
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
        let raw = NonNull::new(raw)
            .ok_or_else(|| HuksError::illegal_argument("HUKS returned a null parameter set"))?;
        Ok(ParamSet {
            raw,
            _blobs: self.blobs,
        })
    }
}
