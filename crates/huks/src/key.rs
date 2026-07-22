use crate::error::{check, HuksError, Result};
use crate::param::ParamSet;
use ohos_huks_sys::*;

/// Borrowed bytes passed to HUKS as an `OH_Huks_Blob`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuksBlob<'a>(&'a [u8]);

impl<'a> HuksBlob<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        HuksBlob(data)
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// The blob borrows `self`, so it must not outlive the call it is passed to.
    pub(crate) fn to_raw(self) -> Result<OH_Huks_Blob> {
        let size = u32::try_from(self.0.len()).map_err(|_| {
            HuksError::illegal_argument("blob length does not fit in OH_Huks_Blob::size")
        })?;
        Ok(OH_Huks_Blob {
            size,
            data: self.0.as_ptr().cast_mut(),
        })
    }
}

impl<'a> From<&'a [u8]> for HuksBlob<'a> {
    fn from(data: &'a [u8]) -> Self {
        HuksBlob::new(data)
    }
}

/// Head room added on top of an input length when sizing an output buffer.
///
/// A single call never expands its input by more than one cipher block, one
/// padding block, one AEAD tag or one signature, all of which are bounded by
/// `OH_HUKS_MAX_KEY_SIZE`. Allowing that much on top of the input therefore
/// leaves the buffer strictly larger than any output HUKS can write into it,
/// which is what makes [`take_output`] able to recognise an untouched length.
pub(crate) const OUT_HEAD_ROOM: usize = OH_HUKS_MAX_KEY_SIZE as usize;

/// An output buffer for a call that consumes `input_len` bytes.
pub(crate) fn out_buf(input_len: usize) -> Vec<u8> {
    vec![0u8; input_len + OUT_HEAD_ROOM]
}

/// Cut an output buffer down to what HUKS actually wrote into it.
///
/// `OH_Huks_Blob::size` is an in/out field: the caller states the capacity
/// available, and HUKS overwrites it with the length it produced — but only when
/// it produces output at all. Sign, verify, mac and derive sessions absorb their
/// input during `update` and emit everything at `finish`, and for those stages
/// the field comes back still holding the capacity that was passed in. Reading
/// it as a length there would hand out the whole zero-filled buffer as if it
/// were real output.
///
/// The capacity is picked so no genuine output can reach it (see
/// [`OUT_HEAD_ROOM`]), so a reported length that does means "nothing written"
/// and yields an empty slice. A caller can tell the two apart: an empty result
/// is a stage that produced nothing.
pub(crate) fn take_output(mut buf: Vec<u8>, reported: u32) -> Vec<u8> {
    let capacity = buf.len();
    if reported as usize >= capacity {
        buf.clear();
    } else {
        buf.truncate(reported as usize);
    }
    buf
}

/// The name a key is stored under.
///
/// Checked on construction so an over-long alias fails here rather than as an
/// opaque error from HUKS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuksAlias<'a>(HuksBlob<'a>);

impl<'a> HuksAlias<'a> {
    /// Wrap `bytes` as an alias. Rejects an empty alias, and one longer than
    /// `OH_HUKS_MAX_KEY_ALIAS_LEN`.
    pub fn new(bytes: &'a [u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(HuksError::illegal_argument("key alias is empty"));
        }
        if bytes.len() > OH_HUKS_MAX_KEY_ALIAS_LEN as usize {
            return Err(HuksError::illegal_argument(format!(
                "key alias is {} bytes, limit is {OH_HUKS_MAX_KEY_ALIAS_LEN}",
                bytes.len()
            )));
        }
        Ok(HuksAlias(HuksBlob::new(bytes)))
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.0.as_bytes()
    }

    pub(crate) fn to_raw(self) -> Result<OH_Huks_Blob> {
        self.0.to_raw()
    }

    /// Generate a key under this alias using the given parameters
    /// (algorithm, purpose, key size, digest, padding, ...).
    pub fn generate(self, params: &ParamSet) -> Result<()> {
        let alias = self.to_raw()?;
        // SAFETY: alias/params are valid for the call; no out param set is requested.
        unsafe {
            check(OH_Huks_GenerateKeyItem(
                &alias,
                params.as_ptr(),
                std::ptr::null_mut(),
            ))
        }
    }

    /// Import raw key material under this alias.
    pub fn import<'m>(
        self,
        params: &ParamSet,
        key_material: impl Into<HuksBlob<'m>>,
    ) -> Result<()> {
        let alias = self.to_raw()?;
        let key = key_material.into().to_raw()?;
        // SAFETY: all three blobs are valid for the duration of the call.
        unsafe { check(OH_Huks_ImportKeyItem(&alias, params.as_ptr(), &key)) }
    }

    /// Export the public part of the asymmetric key stored under this alias.
    pub fn export_public_key(self, params: &ParamSet) -> Result<Vec<u8>> {
        let alias = self.to_raw()?;
        let mut buf = out_buf(0);
        let mut out = OH_Huks_Blob {
            size: buf.len() as u32,
            data: buf.as_mut_ptr(),
        };
        // SAFETY: out points at a `buf.len()`-byte buffer; HUKS writes the actual
        // length back into `out.size`.
        unsafe {
            check(OH_Huks_ExportPublicKeyItem(
                &alias,
                params.as_ptr(),
                &mut out,
            ))?;
        }
        Ok(take_output(buf, out.size))
    }

    /// Delete the key stored under this alias.
    pub fn delete(self) -> Result<()> {
        let alias = self.to_raw()?;
        let empty = ParamSet::empty()?;
        // SAFETY: alias and the empty param set are valid for the call.
        unsafe { check(OH_Huks_DeleteKeyItem(&alias, empty.as_ptr())) }
    }

    /// Whether a key exists under this alias.
    pub fn exists(self) -> Result<bool> {
        let alias = self.to_raw()?;
        let empty = ParamSet::empty()?;
        // SAFETY: alias and the empty param set are valid for the call.
        let result = unsafe { OH_Huks_IsKeyItemExist(&alias, empty.as_ptr()) };
        if result.errorCode == OH_Huks_ErrCode_OH_HUKS_SUCCESS as i32 {
            Ok(true)
        } else if result.errorCode == OH_Huks_ErrCode_OH_HUKS_ERR_CODE_ITEM_NOT_EXIST as i32 {
            Ok(false)
        } else {
            Err(HuksError::from_result(result))
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for HuksAlias<'a> {
    type Error = HuksError;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        HuksAlias::new(bytes)
    }
}

impl<'a> TryFrom<&'a str> for HuksAlias<'a> {
    type Error = HuksError;

    fn try_from(s: &'a str) -> Result<Self> {
        HuksAlias::new(s.as_bytes())
    }
}
