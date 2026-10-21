use std::{
    ffi::{CStr, CString},
    mem::size_of,
    os::raw::c_char,
    ptr::NonNull,
};

use ohos_udmf_sys::{
    OH_UdmfData, OH_UdmfData_GetTypes, OH_UdmfData_HasType, OH_UdmfRecord,
    OH_UdmfRecord_AddGeneralEntry, OH_UdmfRecord_GetGeneralEntry, OH_UdmfRecord_GetTypes,
};

use crate::UdmfError;

fn type_id(value: &str) -> Result<CString, UdmfError> {
    CString::new(value).map_err(|e| UdmfError::CommonError(e.to_string()))
}

fn check_status(status: i32) -> Result<(), UdmfError> {
    if status == 0 {
        Ok(())
    } else {
        Err(UdmfError::InternalError(status))
    }
}

pub(super) fn add_general_entry(
    record: NonNull<OH_UdmfRecord>,
    id: &str,
    bytes: &[u8],
) -> Result<(), UdmfError> {
    let id = type_id(id)?;
    let count = u32::try_from(bytes.len()).map_err(|e| UdmfError::CommonError(e.to_string()))?;
    // SAFETY: the owner keeps the record live. The SDK copies the identifier
    // and bytes synchronously; despite the older mutable declaration, it
    // does not write to the input buffer or retain either input pointer.
    check_status(unsafe {
        OH_UdmfRecord_AddGeneralEntry(
            record.as_ptr(),
            id.as_ptr(),
            bytes.as_ptr().cast_mut(),
            count,
        )
    })
}

pub(super) fn general_entry(
    record: NonNull<OH_UdmfRecord>,
    id: &str,
) -> Result<Vec<u8>, UdmfError> {
    let id = type_id(id)?;
    let mut bytes = std::ptr::null_mut();
    let mut count = 0;
    // SAFETY: the record and identifier are live and both out-pointers are writable.
    check_status(unsafe {
        OH_UdmfRecord_GetGeneralEntry(record.as_ptr(), id.as_ptr(), &mut bytes, &mut count)
    })?;
    let len = count as usize;
    if len == 0 {
        return Ok(Vec::new());
    }
    if bytes.is_null() || len > isize::MAX as usize {
        return Err(UdmfError::CommonError(
            "invalid native general-entry buffer".into(),
        ));
    }
    let mut copy = Vec::new();
    copy.try_reserve_exact(len)
        .map_err(|e| UdmfError::CommonError(e.to_string()))?;
    // SAFETY: the SDK returns count initialized bytes owned by this live
    // record. Copy before another SDK call can replace its cached buffer;
    // never free or adopt the record-owned allocation.
    copy.extend_from_slice(unsafe { std::slice::from_raw_parts(bytes, len) });
    Ok(copy)
}

pub(super) fn record_types(record: NonNull<OH_UdmfRecord>) -> Result<Vec<String>, UdmfError> {
    let mut count = 0;
    // SAFETY: the owner keeps this record live and count is writable.
    let types = unsafe { OH_UdmfRecord_GetTypes(record.as_ptr(), &mut count) };
    // SAFETY: this SDK-owned array is valid until the next mutation/destruction.
    unsafe { copy_types(types, count) }
}

pub(super) fn data_types(data: NonNull<OH_UdmfData>) -> Result<Vec<String>, UdmfError> {
    let mut count = 0;
    // SAFETY: the owner keeps this data live and count is writable.
    let types = unsafe { OH_UdmfData_GetTypes(data.as_ptr(), &mut count) };
    // SAFETY: this SDK-owned array is valid until the next mutation/destruction.
    unsafe { copy_types(types, count) }
}

pub(super) fn has_type(data: NonNull<OH_UdmfData>, id: &str) -> Result<bool, UdmfError> {
    let id = type_id(id)?;
    // SAFETY: both pointers remain live throughout this synchronous query.
    Ok(unsafe { OH_UdmfData_HasType(data.as_ptr(), id.as_ptr()) })
}

unsafe fn copy_types(types: *mut *mut c_char, count: u32) -> Result<Vec<String>, UdmfError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let len = count as usize;
    if types.is_null() || len > isize::MAX as usize / size_of::<*mut c_char>() {
        return Err(UdmfError::CommonError("invalid native type array".into()));
    }
    let mut copy = Vec::new();
    copy.try_reserve_exact(len)
        .map_err(|e| UdmfError::CommonError(e.to_string()))?;
    // SAFETY: the caller guarantees an array of count readable pointers.
    for &value in unsafe { std::slice::from_raw_parts(types, len) } {
        if value.is_null() {
            return Err(UdmfError::CommonError("null native type string".into()));
        }
        // SAFETY: the SDK provides a live NUL-terminated type identifier.
        let value = unsafe { CStr::from_ptr(value) }
            .to_str()
            .map_err(|e| UdmfError::CommonError(e.to_string()))?;
        copy.push(value.to_owned());
    }
    Ok(copy)
}
