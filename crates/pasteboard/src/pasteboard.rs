use std::{ffi::CString, ptr::NonNull};

use ohos_pasteboard_sys::{
    OH_Pasteboard, OH_Pasteboard_Create, OH_Pasteboard_Destroy, OH_Pasteboard_GetData,
    OH_Pasteboard_HasData, OH_Pasteboard_HasType, OH_Pasteboard_IsRemoteData,
};
use ohos_udmf_binding::UdmfData;

use crate::error::PasteboardError;

pub struct Pasteboard {
    raw: NonNull<OH_Pasteboard>,
}

impl Pasteboard {
    pub fn new() -> Self {
        let raw = unsafe { OH_Pasteboard_Create() };
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "OH_Pasteboard_Create failed");
        Self {
            raw: NonNull::new(raw).expect("OH_Pasteboard_Create failed"),
        }
    }

    pub fn data(&self) -> Result<UdmfData, PasteboardError> {
        let mut ret = 0;
        let raw = unsafe { OH_Pasteboard_GetData(self.raw.as_ptr(), &mut ret) };
        if ret != 0 {
            return Err(PasteboardError::IntervalError(ret));
        }
        let udmf_data = UdmfData::from_raw(raw);
        Ok(udmf_data)
    }

    // pub fn source(&self) -> Result<String, PasteboardError> {
    //     let mut len: u32 = 0;
    //     let ret = unsafe {
    //         OH_Pasteboard_GetDataSource(self.raw.as_ptr(), std::ptr::null_mut(), &mut len)
    //     };
    //     if ret != 0 {
    //         return Err(PasteboardError::IntervalError(ret));
    //     }

    //     let mut buf = vec![0; len as usize];
    //     let ret =
    //         unsafe { OH_Pasteboard_GetDataSource(self.raw.as_ptr(), buf.as_mut_ptr(), &mut len) };
    //     if ret != 0 {
    //         return Err(PasteboardError::IntervalError(ret));
    //     }
    //     Ok(String::from_utf8(buf).expect("Invalid UTF-8"))
    // }

    pub fn has_data(&self) -> bool {
        let ret = unsafe { OH_Pasteboard_HasData(self.raw.as_ptr()) };
        ret
    }

    pub fn has_type<T: AsRef<str>>(&self, t: T) -> bool {
        let s = CString::new(t.as_ref()).expect("Invalid CString");
        let ret = unsafe { OH_Pasteboard_HasType(self.raw.as_ptr(), s.as_ptr().cast()) };
        ret
    }

    pub fn is_remote_data(&self) -> bool {
        unsafe { OH_Pasteboard_IsRemoteData(self.raw.as_ptr()) }
    }
}

impl Drop for Pasteboard {
    fn drop(&mut self) {
        unsafe { OH_Pasteboard_Destroy(self.raw.as_ptr()) }
    }
}
