use crate::error::{check, HuksError, Result};
use crate::param::ParamSet;
use ohos_huks_sys::*;

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
pub fn generate_key(alias: &[u8], params: &ParamSet) -> Result<()> {
    let alias = blob_in(alias);
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
pub fn import_key(alias: &[u8], params: &ParamSet, key_material: &[u8]) -> Result<()> {
    let alias = blob_in(alias);
    let key = blob_in(key_material);
    // SAFETY: all three blobs are valid for the duration of the call.
    unsafe { check(OH_Huks_ImportKeyItem(&alias, params.as_ptr(), &key)) }
}

/// Export the public part of an asymmetric key stored under `alias`.
pub fn export_public_key(alias: &[u8], params: &ParamSet) -> Result<Vec<u8>> {
    let alias = blob_in(alias);
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
pub fn delete_key(alias: &[u8]) -> Result<()> {
    let alias = blob_in(alias);
    let empty = ParamSet::empty()?;
    // SAFETY: alias and the empty param set are valid for the call.
    unsafe { check(OH_Huks_DeleteKeyItem(&alias, empty.as_ptr())) }
}

/// Whether a key exists under `alias`.
pub fn key_exists(alias: &[u8]) -> Result<bool> {
    let alias = blob_in(alias);
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
