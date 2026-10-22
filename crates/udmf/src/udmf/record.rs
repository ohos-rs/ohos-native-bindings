use std::{
    ffi::{CStr, CString},
    mem::size_of,
    os::raw::c_char,
    ptr::NonNull,
};

use ohos_udmf_sys::{
    OH_UdmfRecord, OH_UdmfRecord_AddGeneralEntry, OH_UdmfRecord_AddHtml,
    OH_UdmfRecord_AddPlainText, OH_UdmfRecord_Create, OH_UdmfRecord_Destroy,
    OH_UdmfRecord_GetGeneralEntry, OH_UdmfRecord_GetHtml, OH_UdmfRecord_GetPlainText,
    OH_UdmfRecord_GetTypes,
};

#[cfg(feature = "api-13")]
use ohos_udmf_sys::OH_UdmfRecord_AddPixelMap;

use crate::{UdmfError, Uds, UdsHtml, UdsPlainText};

pub struct UdmfRecord {
    pub(crate) raw: NonNull<OH_UdmfRecord>,
}

impl UdmfRecord {
    /// Adds custom bytes under a native UDMF type identifier (API 12).
    ///
    /// The SDK copies the input and validates its supported type and size.
    /// Interior NULs in `type_id` are rejected without calling the SDK.
    /// Empty input returns [`UdmfError::EmptyEntry`] without calling native code:
    /// upstream `OH_UdmfRecord_AddGeneralEntry` rejects `count == 0`.
    /// General entries use `ARRAY_BUFFER`; use typed UDS methods for plain text
    /// and HTML rather than treating built-in UDS records as raw bytes.
    ///
    /// See [upstream udmf.cpp](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp).
    pub fn add_general_entry(&self, type_id: &str, bytes: &[u8]) -> Result<(), UdmfError> {
        if bytes.is_empty() {
            return Err(UdmfError::EmptyEntry);
        }
        let id = CString::new(type_id).map_err(|e| UdmfError::CommonError(e.to_string()))?;
        let count =
            u32::try_from(bytes.len()).map_err(|e| UdmfError::CommonError(e.to_string()))?;
        // SAFETY: the owner keeps the record live. The SDK copies the identifier
        // and bytes synchronously; despite the older mutable declaration, it
        // does not write to the input buffer or retain either input pointer.
        check_status(unsafe {
            OH_UdmfRecord_AddGeneralEntry(
                self.raw.as_ptr(),
                id.as_ptr(),
                bytes.as_ptr().cast_mut(),
                count,
            )
        })
    }

    /// Copies a general entry out of record-owned native storage (API 12).
    ///
    /// Upstream `GetValueFromUint8Array` reads an `ARRAY_BUFFER` payload;
    /// built-in plain-text/HTML UDS records are not general byte entries.
    /// Use [`Self::plain_text`] or [`Self::html`] for those types.
    /// See [upstream udmf.cpp](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp).
    pub fn general_entry(&self, type_id: &str) -> Result<Vec<u8>, UdmfError> {
        read_general_entry(self.raw, type_id)
    }

    /// Copies every native type identifier in this record (API 12).
    pub fn types(&self) -> Result<Vec<String>, UdmfError> {
        read_record_types(self.raw)
    }

    /// Reads this record's plain-text UDS into an independently owned object (API 12).
    pub fn plain_text(&self) -> Result<UdsPlainText, UdmfError> {
        read_plain_text(self.raw)
    }

    /// Reads this record's HTML UDS into an independently owned object (API 12).
    pub fn html(&self) -> Result<UdsHtml, UdmfError> {
        read_html(self.raw)
    }

    pub fn new() -> Self {
        Self::try_new().expect("OH_UdmfRecord_Create failed")
    }

    pub fn try_new() -> Result<Self, UdmfError> {
        NonNull::new(unsafe { OH_UdmfRecord_Create() })
            .map(|raw| Self { raw })
            .ok_or_else(|| UdmfError::UdmfInitError("OH_UdmfRecord_Create returned null".into()))
    }

    pub fn from_raw(raw: *mut OH_UdmfRecord) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_UdmfRecord_Create from a raw ptr failed"),
        }
    }

    #[cfg(feature = "api-13")]
    pub fn add_file_uri(&self, uri: &crate::UdsFileUri) -> Result<(), UdmfError> {
        let status =
            unsafe { ohos_udmf_sys::OH_UdmfRecord_AddFileUri(self.raw.as_ptr(), uri.raw.as_ptr()) };
        if status != 0 {
            return Err(UdmfError::InternalError(status));
        }
        Ok(())
    }

    pub fn add(&self, value: Uds) -> Result<(), UdmfError> {
        match value {
            Uds::PlainText(text) => {
                let ret =
                    unsafe { OH_UdmfRecord_AddPlainText(self.raw.as_ptr(), text.raw.as_ptr()) };
                if ret != 0 {
                    return Err(UdmfError::InternalError(ret));
                }
            }
            Uds::Html(html) => {
                let ret = unsafe { OH_UdmfRecord_AddHtml(self.raw.as_ptr(), html.raw.as_ptr()) };
                if ret != 0 {
                    return Err(UdmfError::InternalError(ret));
                }
            }
            #[cfg(feature = "api-13")]
            Uds::PixelMap(pixel_map) => {
                let ret =
                    unsafe { OH_UdmfRecord_AddPixelMap(self.raw.as_ptr(), pixel_map.raw.as_ptr()) };
                if ret != 0 {
                    return Err(UdmfError::InternalError(ret));
                }
            }
        }
        Ok(())
    }
}

impl Default for UdmfRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdmfRecord {
    fn drop(&mut self) {
        unsafe { OH_UdmfRecord_Destroy(self.raw.as_ptr()) }
    }
}

/// A container-owned record. Dropping this view never destroys the native record.
pub struct UdmfRecordRef<'data> {
    raw: NonNull<OH_UdmfRecord>,
    _data: std::marker::PhantomData<&'data super::UdmfData>,
}

impl UdmfRecordRef<'_> {
    /// Copies a general entry while its owning data container is borrowed (API 12).
    ///
    /// This reads the native `ARRAY_BUFFER`, not built-in plain-text/HTML UDS.
    /// Use [`Self::plain_text`] or [`Self::html`] for standard text records.
    /// See [upstream udmf.cpp](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp).
    pub fn general_entry(&self, type_id: &str) -> Result<Vec<u8>, UdmfError> {
        read_general_entry(self.raw, type_id)
    }

    /// Copies every native type identifier in this borrowed record (API 12).
    pub fn types(&self) -> Result<Vec<String>, UdmfError> {
        read_record_types(self.raw)
    }

    /// Reads plain-text UDS while the container is borrowed (API 12).
    ///
    /// The returned native UDS object is independently owned.
    pub fn plain_text(&self) -> Result<UdsPlainText, UdmfError> {
        read_plain_text(self.raw)
    }

    /// Reads HTML UDS while the container is borrowed (API 12).
    ///
    /// The returned native UDS object is independently owned.
    pub fn html(&self) -> Result<UdsHtml, UdmfError> {
        read_html(self.raw)
    }

    pub(crate) unsafe fn from_raw(raw: *mut OH_UdmfRecord) -> Self {
        Self {
            raw: NonNull::new(raw).expect("non-null record"),
            _data: std::marker::PhantomData,
        }
    }

    #[cfg(feature = "api-13")]
    pub fn file_uri(&self) -> Result<crate::UdsFileUri, UdmfError> {
        let uri = crate::UdsFileUri::new()?;
        let status =
            unsafe { ohos_udmf_sys::OH_UdmfRecord_GetFileUri(self.raw.as_ptr(), uri.raw.as_ptr()) };
        if status != 0 {
            return Err(UdmfError::InternalError(status));
        }
        Ok(uri)
    }
}

fn check_status(status: i32) -> Result<(), UdmfError> {
    if status == 0 {
        Ok(())
    } else {
        Err(UdmfError::InternalError(status))
    }
}

fn read_general_entry(record: NonNull<OH_UdmfRecord>, id: &str) -> Result<Vec<u8>, UdmfError> {
    let id = CString::new(id).map_err(|e| UdmfError::CommonError(e.to_string()))?;
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

fn read_plain_text(record: NonNull<OH_UdmfRecord>) -> Result<UdsPlainText, UdmfError> {
    let text = UdsPlainText::new();
    // SAFETY: both the live record and the independently owned output UDS
    // instance remain valid throughout this synchronous conversion.
    check_status(unsafe { OH_UdmfRecord_GetPlainText(record.as_ptr(), text.raw.as_ptr()) })?;
    Ok(text)
}

fn read_html(record: NonNull<OH_UdmfRecord>) -> Result<UdsHtml, UdmfError> {
    let html = UdsHtml::new();
    // SAFETY: both native instances are live. On failure, the temporary output
    // is dropped normally; on success, the returned UDS owns its native instance.
    check_status(unsafe { OH_UdmfRecord_GetHtml(record.as_ptr(), html.raw.as_ptr()) })?;
    Ok(html)
}

fn read_record_types(record: NonNull<OH_UdmfRecord>) -> Result<Vec<String>, UdmfError> {
    let mut count = 0;
    // SAFETY: the owner keeps this record live and count is writable.
    let types = unsafe { OH_UdmfRecord_GetTypes(record.as_ptr(), &mut count) };
    // SAFETY: this SDK-owned array is valid until the next mutation/destruction.
    unsafe { copy_types(types, count) }
}

pub(super) unsafe fn copy_types(
    types: *mut *mut c_char,
    count: u32,
) -> Result<Vec<String>, UdmfError> {
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
