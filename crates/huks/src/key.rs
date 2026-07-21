use crate::error::{check, HuksError, Result};
use crate::param::ParamSet;
use ohos_huks_sys::*;

/// The name a key is stored under.
///
/// Checked on construction so an over-long alias fails here rather than as an
/// opaque error from HUKS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuksAlias<'a>(&'a [u8]);

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
        Ok(HuksAlias(bytes))
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0
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

/// Wrap a byte slice as an input `OH_Huks_Blob`. The blob borrows `data`, so it
/// must not outlive the call it is passed to.
pub(crate) fn blob_in(data: &[u8]) -> OH_Huks_Blob {
    debug_assert!(
        data.len() <= u32::MAX as usize,
        "blob length does not fit in OH_Huks_Blob::size (u32)"
    );
    OH_Huks_Blob {
        size: data.len() as u32,
        data: data.as_ptr() as *mut u8,
    }
}

/// Generate a key under `alias` using the given parameters
/// (algorithm, purpose, key size, digest, padding, ...).
pub fn generate_key(alias: HuksAlias<'_>, params: &ParamSet) -> Result<()> {
    let alias = blob_in(alias.as_bytes());
    // SAFETY: alias/params are valid for the call; no out param set is requested.
    unsafe {
        check(OH_Huks_GenerateKeyItem(
            &alias,
            params.as_ptr(),
            std::ptr::null_mut(),
        ))
    }
}

/// Import raw key material under `alias`.
pub fn import_key(alias: HuksAlias<'_>, params: &ParamSet, key_material: &[u8]) -> Result<()> {
    let alias = blob_in(alias.as_bytes());
    let key = blob_in(key_material);
    // SAFETY: all three blobs are valid for the duration of the call.
    unsafe { check(OH_Huks_ImportKeyItem(&alias, params.as_ptr(), &key)) }
}

/// Export the public part of an asymmetric key stored under `alias`.
pub fn export_public_key(alias: HuksAlias<'_>, params: &ParamSet) -> Result<Vec<u8>> {
    let alias = blob_in(alias.as_bytes());
    let mut buf = vec![0u8; OH_HUKS_MAX_KEY_SIZE as usize];
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
    buf.truncate(out.size as usize);
    Ok(buf)
}

/// Delete the key stored under `alias`.
pub fn delete_key(alias: HuksAlias<'_>) -> Result<()> {
    let alias = blob_in(alias.as_bytes());
    let empty = ParamSet::empty()?;
    // SAFETY: alias and the empty param set are valid for the call.
    unsafe { check(OH_Huks_DeleteKeyItem(&alias, empty.as_ptr())) }
}

/// Whether a key exists under `alias`.
pub fn key_exists(alias: HuksAlias<'_>) -> Result<bool> {
    let alias = blob_in(alias.as_bytes());
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
