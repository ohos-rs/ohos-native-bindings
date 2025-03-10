use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use ohos_udmf_sys::{
    OH_UdmfData, OH_UdmfData_AddRecord, OH_UdmfData_Create, OH_UdmfData_GetPrimaryHtml,
    OH_UdmfData_GetPrimaryPlainText, OH_UdmfData_GetRecord, OH_UdmfData_GetRecordCount,
    OH_UdmfData_GetRecords, OH_UdmfData_IsLocal, OH_Udmf_GetUnifiedData, OH_Udmf_SetUnifiedData,
};

use crate::{UdmfError, UdmfIntention, UdsHtml, UdsPlainText};

use super::UdmfRecord;

pub struct UdmfData {
    raw: NonNull<OH_UdmfData>,
}

impl UdmfData {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdmfData_Create() }).expect("OH_UdmfData_create failed");
        Self { raw }
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

    pub fn add_record(&self, record: &UdmfRecord) -> Result<(), UdmfError> {
        let ret = unsafe { OH_UdmfData_AddRecord(self.raw.as_ptr(), record.raw.as_ptr()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    pub fn count(&self) -> i32 {
        let ret = unsafe { OH_UdmfData_GetRecordCount(self.raw.as_ptr()) };
        ret
    }

    pub fn record(&self, index: u32) -> Result<UdmfRecord, UdmfError> {
        let ret = unsafe { OH_UdmfData_GetRecord(self.raw.as_ptr(), index) };
        if ret.is_null() {
            return Err(UdmfError::UdmfInitError(String::from(
                "UdmfData::record get record failed",
            )));
        }
        Ok(UdmfRecord::from_raw(ret))
    }

    pub fn records(&self) -> Result<Vec<UdmfRecord>, UdmfError> {
        let mut count = 0;
        let ret = unsafe { OH_UdmfData_GetRecords(self.raw.as_ptr(), &mut count) };
        if ret.is_null() {
            return Err(UdmfError::InternalError(-1));
        }
        if count == 0 {
            return Ok(vec![]);
        } else {
            let mut records = Vec::with_capacity(count as usize);
            for i in 0..count {
                let record_ptr = unsafe { *ret.offset(i as isize) };
                if !record_ptr.is_null() {
                    records.push(UdmfRecord::from_raw(record_ptr));
                }
            }
            Ok(records)
        }
    }

    pub fn is_local(&self) -> bool {
        unsafe { OH_UdmfData_IsLocal(self.raw.as_ptr()) }
    }

    pub fn primary_plain_text(&self) -> Result<UdsPlainText, UdmfError> {
        let text = UdsPlainText::new();
        let ret = unsafe { OH_UdmfData_GetPrimaryPlainText(self.raw.as_ptr(), text.raw.as_ptr()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(text)
    }

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
                return UdmfError::CommonError(format!(
                    "UdmfData::save convert to str failed: {}",
                    e
                ));
            })?;
        Ok(key.to_owned())
    }

    /// Save Udmf to database with custom key size
    pub fn save_with_key_size(
        &self,
        intension: UdmfIntention,
        size: i32,
    ) -> Result<String, UdmfError> {
        let mut key = Vec::with_capacity(size as _);
        key.resize(size as _, 0);
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
                return UdmfError::CommonError(format!(
                    "UdmfData::save_with_key_size convert to str failed: {}",
                    e
                ));
            })?;
        Ok(key.to_owned())
    }
}
