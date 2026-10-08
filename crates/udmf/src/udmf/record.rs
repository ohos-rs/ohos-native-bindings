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
    // Reading a borrowed record requires API 13, but API 12 can still enumerate it.
    #[cfg_attr(not(feature = "api-13"), expect(dead_code))]
    raw: NonNull<OH_UdmfRecord>,
    _data: std::marker::PhantomData<&'data super::UdmfData>,
}

impl UdmfRecordRef<'_> {
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
