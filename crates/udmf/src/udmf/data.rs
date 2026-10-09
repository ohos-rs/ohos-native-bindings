use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use ohos_udmf_sys::*;

use crate::{UdmfError, UdmfIntention};

#[cfg(feature = "api-13")]
use crate::{UdsHtml, UdsPlainText};

use super::{UdmfRecord, UdmfRecordRef};

pub struct UdmfData {
    raw: NonNull<OH_UdmfData>,
}

impl UdmfData {
    pub fn new() -> Self {
        Self::try_new().expect("OH_UdmfData_Create failed")
    }

    pub fn try_new() -> Result<Self, UdmfError> {
        NonNull::new(unsafe { OH_UdmfData_Create() })
            .map(|raw| Self { raw })
            .ok_or_else(|| UdmfError::UdmfInitError("OH_UdmfData_Create returned null".into()))
    }

    pub fn from_raw(raw: *mut OH_UdmfData) -> Self {
        let raw = NonNull::new(raw).expect("Create UdmfData from raw failed");
        Self { raw }
    }

    pub fn raw(&self) -> NonNull<OH_UdmfData> {
        self.raw
    }

    pub fn create_from_database<T: AsRef<str>>(key: T, intention: UdmfIntention) -> Self {
        let raw = unsafe { OH_UdmfData_Create() };
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "OH_UdmfData_Create failed");

        let s = CString::new(key.as_ref()).expect("CString::new failed");

        let ret = unsafe { OH_Udmf_GetUnifiedData(s.as_ptr().cast(), intention.into(), raw) };

        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_Udmf_GetUnifiedData failed");
        Self {
            raw: NonNull::new(raw).expect("Create UdmfData from database failed"),
        }
    }

    pub fn add_record(&mut self, record: &UdmfRecord) -> Result<(), UdmfError> {
        let ret = unsafe { OH_UdmfData_AddRecord(self.raw.as_ptr(), record.raw.as_ptr()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    #[cfg(feature = "api-13")]
    pub fn count(&self) -> i32 {
        unsafe { OH_UdmfData_GetRecordCount(self.raw.as_ptr()) }
    }

    #[cfg(feature = "api-13")]
    pub fn record(&mut self, index: u32) -> Result<UdmfRecordRef<'_>, UdmfError> {
        let ret = unsafe { OH_UdmfData_GetRecord(self.raw.as_ptr(), index) };
        if ret.is_null() {
            return Err(UdmfError::UdmfInitError(String::from(
                "UdmfData::record get record failed",
            )));
        }
        Ok(unsafe { UdmfRecordRef::from_raw(ret) })
    }

    pub fn records(&mut self) -> Result<Vec<UdmfRecordRef<'_>>, UdmfError> {
        let mut count = 0;
        let ret = unsafe { OH_UdmfData_GetRecords(self.raw.as_ptr(), &mut count) };
        if count == 0 {
            Ok(vec![])
        } else if ret.is_null() {
            Err(UdmfError::InternalError(-1))
        } else {
            let mut records = Vec::with_capacity(count as usize);
            for i in 0..count {
                let record_ptr = unsafe { *ret.offset(i as isize) };
                if !record_ptr.is_null() {
                    records.push(unsafe { UdmfRecordRef::from_raw(record_ptr) });
                }
            }
            Ok(records)
        }
    }

    #[cfg(feature = "api-13")]
    pub fn is_local(&self) -> bool {
        unsafe { OH_UdmfData_IsLocal(self.raw.as_ptr()) }
    }

    #[cfg(feature = "api-13")]
    pub fn primary_plain_text(&self) -> Result<UdsPlainText, UdmfError> {
        let text = UdsPlainText::new();
        let ret = unsafe { OH_UdmfData_GetPrimaryPlainText(self.raw.as_ptr(), text.raw.as_ptr()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(text)
    }

    #[cfg(feature = "api-13")]
    pub fn primary_html(&self) -> Result<UdsHtml, UdmfError> {
        let html = UdsHtml::new();
        let ret = unsafe { OH_UdmfData_GetPrimaryHtml(self.raw.as_ptr(), html.raw.as_ptr()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(html)
    }

    /// Save Udmf to database
    pub fn save(&self, intension: UdmfIntention) -> Result<String, UdmfError> {
        const MIN_KEY_SIZE: u32 = 512;
        let mut key = [0; MIN_KEY_SIZE as _];
        let ret = unsafe {
            OH_Udmf_SetUnifiedData(
                intension.into(),
                self.raw.as_ptr(),
                key.as_mut_ptr(),
                MIN_KEY_SIZE,
            )
        };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        let key = unsafe { CStr::from_ptr(key.as_ptr()) }
            .to_str()
            .map_err(|e| {
                UdmfError::CommonError(format!("UdmfData::save convert to str failed: {}", e))
            })?;
        Ok(key.to_owned())
    }

    /// Save Udmf to database with custom key size
    pub fn save_with_key_size(
        &self,
        intension: UdmfIntention,
        size: i32,
    ) -> Result<String, UdmfError> {
        let mut key = vec![0; size as _];
        let ret = unsafe {
            OH_Udmf_SetUnifiedData(
                intension.into(),
                self.raw.as_ptr(),
                key.as_mut_ptr(),
                size as _,
            )
        };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        let key = unsafe { CStr::from_ptr(key.as_ptr()) }
            .to_str()
            .map_err(|e| {
                UdmfError::CommonError(format!(
                    "UdmfData::save_with_key_size convert to str failed: {}",
                    e
                ))
            })?;
        Ok(key.to_owned())
    }
}

impl Default for UdmfData {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdmfData {
    fn drop(&mut self) {
        // GetRecord(s) returns container-owned views, not independently owned records.
        unsafe { OH_UdmfData_Destroy(self.raw.as_ptr()) };
    }
}
