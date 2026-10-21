use std::ptr::NonNull;

use ohos_native_pasteboard_sys::OH_Pasteboard;

use crate::error::PasteboardError;

pub(super) fn clear(pasteboard: NonNull<OH_Pasteboard>) -> Result<(), PasteboardError> {
    // SAFETY: the owner keeps the pasteboard instance live for this call.
    let status =
        unsafe { ohos_native_pasteboard_sys::OH_Pasteboard_ClearData(pasteboard.as_ptr()) };
    if status == 0 {
        Ok(())
    } else {
        Err(PasteboardError::InternalError(status))
    }
}

#[cfg(feature = "api-14")]
pub(super) fn mime_types(
    pasteboard: NonNull<OH_Pasteboard>,
) -> Result<Vec<String>, PasteboardError> {
    let mut count = 0;
    // SAFETY: the pasteboard instance is live and count is writable.
    let types = unsafe {
        ohos_native_pasteboard_sys::OH_Pasteboard_GetMimeTypes(pasteboard.as_ptr(), &mut count)
    };
    if count == 0 {
        return Ok(Vec::new());
    }
    let len = count as usize;
    if types.is_null()
        || len > isize::MAX as usize / std::mem::size_of::<*mut std::os::raw::c_char>()
    {
        return Err(PasteboardError::CommonError(
            "invalid native MIME-type array".into(),
        ));
    }
    let mut copy = Vec::new();
    copy.try_reserve_exact(len)
        .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
    // SAFETY: the SDK returns count pointers owned by this pasteboard.
    // The SDK replaces/frees this cache on later queries/destruction; copy
    // both the strings and the array without freeing either native owner.
    for &value in unsafe { std::slice::from_raw_parts(types, len) } {
        if value.is_null() {
            return Err(PasteboardError::CommonError(
                "null native MIME-type string".into(),
            ));
        }
        // SAFETY: every SDK MIME-type pointer names a live NUL-terminated string.
        let value = unsafe { std::ffi::CStr::from_ptr(value) }
            .to_str()
            .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        copy.push(value.to_owned());
    }
    Ok(copy)
}

#[cfg(feature = "api-18")]
pub(super) fn change_count(pasteboard: NonNull<OH_Pasteboard>) -> u32 {
    // SAFETY: the owner keeps the pasteboard instance live for this query.
    unsafe { ohos_native_pasteboard_sys::OH_Pasteboard_GetChangeCount(pasteboard.as_ptr()) }
}
