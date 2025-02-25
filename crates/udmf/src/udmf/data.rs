use std::{ffi::CString, ptr::NonNull};

use ohos_udmf_sys::{
    OH_UdmfData, OH_UdmfData_AddRecord, OH_UdmfData_Create, OH_UdmfData_GetRecordCount,
    OH_Udmf_GetUnifiedData, OH_Udmf_SetUnifiedData,
};

use crate::{UdmfError, UdmfIntention};

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
            return Err(UdmfError::IntervalError(ret));
        }
        Ok(())
    }

    pub fn count(&self) -> i32 {
        let ret = unsafe { OH_UdmfData_GetRecordCount(self.raw.as_ptr()) };
        ret
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
            return Err(UdmfError::IntervalError(ret));
        }
        let key =
            CString::from_vec_with_nul(key.to_vec()).expect("CString::from_vec_with_nul failed");
        Ok(key.to_str().expect("CString::to_str failed").to_owned())
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
            return Err(UdmfError::IntervalError(ret));
        }
        let key =
            CString::from_vec_with_nul(key.to_vec()).expect("CString::from_vec_with_nul failed");
        Ok(key.to_str().expect("CString::to_str failed").to_owned())
    }
}
