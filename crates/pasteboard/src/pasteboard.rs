use std::{ffi::CString, ptr::NonNull};

use ohos_pasteboard_sys::{
    OH_Pasteboard, OH_Pasteboard_Create, OH_Pasteboard_Destroy, OH_Pasteboard_GetData,
    OH_Pasteboard_HasData, OH_Pasteboard_HasType, OH_Pasteboard_IsRemoteData,
    OH_Pasteboard_SetData,
};
use ohos_udmf_binding::{UdmfData, UdsValue};

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
            return Err(PasteboardError::InternalError(ret));
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
    //         return Err(PasteboardError::InternalError(ret));
    //     }

    //     let mut buf = vec![0; len as usize];
    //     let ret =
    //         unsafe { OH_Pasteboard_GetDataSource(self.raw.as_ptr(), buf.as_mut_ptr(), &mut len) };
    //     if ret != 0 {
    //         return Err(PasteboardError::InternalError(ret));
    //     }
    //     Ok(String::from_utf8(buf).expect("Invalid UTF-8"))
    // }

    pub fn has_data(&self) -> bool {
        unsafe { OH_Pasteboard_HasData(self.raw.as_ptr()) }
    }

    /// Current pasteboard has some types   
    /// https://developer.huawei.com/consumer/cn/doc/harmonyos-guides-V14/use_pasteboard_to_copy_and_paste-V14#js%E6%8E%A5%E5%8F%A3%E4%B8%8Endk%E6%8E%A5%E5%8F%A3%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B%E5%AF%B9%E5%BA%94%E5%85%B3%E7%B3%BB
    /// ```no_run
    /// let pasteboard = Pasteboard::new();
    /// let t = UdsPlainText::new();
    /// let has_type = pasteboard.has_type(t).unwrap();
    /// ```
    pub fn has_type<T: UdsValue>(&self, t: T) -> Result<bool, PasteboardError> {
        let types = t
            .get_type()
            .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        let s = CString::new(types.to_string()).expect("CString::new failed");
        let ret = unsafe { OH_Pasteboard_HasType(self.raw.as_ptr(), s.as_ptr().cast()) };
        Ok(ret)
    }

    pub fn is_remote_data(&self) -> bool {
        unsafe { OH_Pasteboard_IsRemoteData(self.raw.as_ptr()) }
    }

    pub fn set_data(&self, data: &UdmfData) -> Result<(), PasteboardError> {
        let ret = unsafe { OH_Pasteboard_SetData(self.raw.as_ptr(), data.raw().as_ptr()) };
        if ret != 0 {
            return Err(PasteboardError::InternalError(ret));
        }
        Ok(())
    }
}

impl Drop for Pasteboard {
    fn drop(&mut self) {
        unsafe { OH_Pasteboard_Destroy(self.raw.as_ptr()) }
    }
}

impl Default for Pasteboard {
    fn default() -> Self {
        Self::new()
    }
}
