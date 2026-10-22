use std::{ffi::CString, ptr::NonNull};

use ohos_native_pasteboard_sys::*;
use ohos_udmf_binding::{UdmfData, UdsValue};

use crate::error::PasteboardError;

pub struct Pasteboard {
    raw: NonNull<OH_Pasteboard>,
}

impl Pasteboard {
    /// Clears the native system pasteboard (API 13).
    pub fn clear(&self) -> Result<(), PasteboardError> {
        // SAFETY: the owner keeps the pasteboard instance live for this call.
        let status = unsafe { OH_Pasteboard_ClearData(self.raw.as_ptr()) };
        if status == 0 {
            Ok(())
        } else {
            Err(PasteboardError::InternalError(status))
        }
    }

    /// Copies the native MIME-type list (API 14).
    ///
    /// The native array and strings belong to this pasteboard instance and
    /// are released by the SDK, not by the caller.
    #[cfg(feature = "api-14")]
    pub fn mime_types(&self) -> Result<Vec<String>, PasteboardError> {
        let mut count = 0;
        // SAFETY: the pasteboard instance is live and count is writable.
        let types = unsafe { OH_Pasteboard_GetMimeTypes(self.raw.as_ptr(), &mut count) };
        if count == 0 {
            return Ok(Vec::new());
        }
        let len = count as usize;
        if types.is_null()
            || len > isize::MAX as usize / std::mem::size_of::<*mut std::os::raw::c_char>()
        {
            return Err(PasteboardError::CommonError(
                "invalid native MIME-type array".into(),
            ));
        }
        let mut copy = Vec::new();
        copy.try_reserve_exact(len)
            .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        // SAFETY: the SDK returns count pointers owned by this pasteboard.
        // The SDK replaces/frees this cache on later queries/destruction; copy
        // both the strings and the array without freeing either native owner.
        for &value in unsafe { std::slice::from_raw_parts(types, len) } {
            if value.is_null() {
                return Err(PasteboardError::CommonError(
                    "null native MIME-type string".into(),
                ));
            }
            // SAFETY: every SDK MIME-type pointer names a live NUL-terminated string.
            let value = unsafe { std::ffi::CStr::from_ptr(value) }
                .to_str()
                .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
            copy.push(value.to_owned());
        }
        Ok(copy)
    }

    /// Returns the native content-change counter (API 18).
    ///
    /// Zero also represents a failed query. The counter resets when the
    /// service/system restarts; clearing data does not reset it.
    #[cfg(feature = "api-18")]
    pub fn change_count(&self) -> u32 {
        // SAFETY: the owner keeps the pasteboard instance live for this query.
        unsafe { OH_Pasteboard_GetChangeCount(self.raw.as_ptr()) }
    }

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
