use std::ptr::NonNull;

use ohos_udmf_sys::{
    OH_UdmfRecord, OH_UdmfRecord_AddHtml, OH_UdmfRecord_AddPlainText, OH_UdmfRecord_Create,
    OH_UdmfRecord_Destroy,
};

#[cfg(feature = "api-13")]
use ohos_udmf_sys::OH_UdmfRecord_AddPixelMap;

use crate::{UdmfError, Uds};

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
        super::native_data::add_general_entry(self.raw, type_id, bytes)
    }

    /// Copies a general entry out of record-owned native storage (API 12).
    ///
    /// Upstream `GetValueFromUint8Array` reads an `ARRAY_BUFFER` payload;
    /// built-in plain-text/HTML UDS records are not general byte entries.
    /// Use [`Self::plain_text`] or [`Self::html`] for those types.
    /// See [upstream udmf.cpp](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp).
    pub fn general_entry(&self, type_id: &str) -> Result<Vec<u8>, UdmfError> {
        super::native_data::general_entry(self.raw, type_id)
    }

    /// Copies every native type identifier in this record (API 12).
    pub fn types(&self) -> Result<Vec<String>, UdmfError> {
        super::native_data::record_types(self.raw)
    }

    /// Reads this record's plain-text UDS into an independently owned object (API 12).
    pub fn plain_text(&self) -> Result<crate::UdsPlainText, UdmfError> {
        super::native_data::plain_text(self.raw)
    }

    /// Reads this record's HTML UDS into an independently owned object (API 12).
    pub fn html(&self) -> Result<crate::UdsHtml, UdmfError> {
        super::native_data::html(self.raw)
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
        super::native_data::general_entry(self.raw, type_id)
    }

    /// Copies every native type identifier in this borrowed record (API 12).
    pub fn types(&self) -> Result<Vec<String>, UdmfError> {
        super::native_data::record_types(self.raw)
    }

    /// Reads plain-text UDS while the container is borrowed (API 12).
    ///
    /// The returned native UDS object is independently owned.
    pub fn plain_text(&self) -> Result<crate::UdsPlainText, UdmfError> {
        super::native_data::plain_text(self.raw)
    }

    /// Reads HTML UDS while the container is borrowed (API 12).
    ///
    /// The returned native UDS object is independently owned.
    pub fn html(&self) -> Result<crate::UdsHtml, UdmfError> {
        super::native_data::html(self.raw)
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
