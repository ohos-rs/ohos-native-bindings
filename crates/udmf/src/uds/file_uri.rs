//! Owned UDS file-URI records (API 13+).
use crate::UdmfError;
use ohos_udmf_sys::*;
use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

pub struct UdsFileUri {
    pub(crate) raw: NonNull<OH_UdsFileUri>,
}

impl UdsFileUri {
    pub fn new() -> Result<Self, UdmfError> {
        NonNull::new(unsafe { OH_UdsFileUri_Create() })
            .map(|raw| Self { raw })
            .ok_or_else(|| UdmfError::UdsInitError("OH_UdsFileUri_Create returned null".into()))
    }

    pub fn set_file_uri(&self, value: &str) -> Result<(), UdmfError> {
        let value = CString::new(value).map_err(|e| UdmfError::CommonError(e.to_string()))?;
        let status = unsafe { OH_UdsFileUri_SetFileUri(self.raw.as_ptr(), value.as_ptr()) };
        if status != 0 {
            return Err(UdmfError::InternalError(status));
        }
        Ok(())
    }

    pub fn set_file_type(&self, value: &str) -> Result<(), UdmfError> {
        let value = CString::new(value).map_err(|e| UdmfError::CommonError(e.to_string()))?;
        let status = unsafe { OH_UdsFileUri_SetFileType(self.raw.as_ptr(), value.as_ptr()) };
        if status != 0 {
            return Err(UdmfError::InternalError(status));
        }
        Ok(())
    }

    pub fn file_uri(&self) -> Result<String, UdmfError> {
        self.read_string(unsafe { OH_UdsFileUri_GetFileUri(self.raw.as_ptr()) })
    }

    pub fn file_type(&self) -> Result<String, UdmfError> {
        self.read_string(unsafe { OH_UdsFileUri_GetFileType(self.raw.as_ptr()) })
    }

    fn read_string(&self, value: *const std::ffi::c_char) -> Result<String, UdmfError> {
        if value.is_null() {
            return Err(UdmfError::InternalError(-1));
        }
        // The native string is owned by this live UDS instance. Copy without lossy URI conversion.
        unsafe { CStr::from_ptr(value) }
            .to_str()
            .map(str::to_owned)
            .map_err(|e| UdmfError::CommonError(e.to_string()))
    }
}

impl Drop for UdsFileUri {
    fn drop(&mut self) {
        unsafe { OH_UdsFileUri_Destroy(self.raw.as_ptr()) };
    }
}
