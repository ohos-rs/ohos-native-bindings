use std::{ffi::CStr, os::raw::c_void};

use ohos_raw_sys::{
    OH_ResourceManager_CloseRawDir, OH_ResourceManager_CloseRawFile,
    OH_ResourceManager_CloseRawFile64, OH_ResourceManager_GetRawFileCount,
    OH_ResourceManager_GetRawFileName, OH_ResourceManager_GetRawFileOffset,
    OH_ResourceManager_GetRawFileSize, OH_ResourceManager_ReadRawFile,
    OH_ResourceManager_SeekRawFile,
};

pub struct RawDir {
    raw: *mut ohos_raw_sys::RawDir,
}

impl RawDir {
    pub fn new(raw: *mut ohos_raw_sys::RawDir) -> Self {
        RawDir { raw }
    }

    pub fn count(&self) -> i32 {
        unsafe { OH_ResourceManager_GetRawFileCount(self.raw) }
    }

    pub fn get_file_name_by_index(&self, index: i32) -> &str {
        unsafe {
            let ret = OH_ResourceManager_GetRawFileName(self.raw, index);
            CStr::from_ptr(ret).to_str().unwrap_or("")
        }
    }
}

impl Drop for RawDir {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_CloseRawDir(self.raw);
        }
    }
}

pub struct RawFile {
    raw: *mut ohos_raw_sys::RawFile,
}

impl RawFile {
    pub fn new(raw: *mut ohos_raw_sys::RawFile) -> Self {
        Self { raw }
    }

    pub fn file_size(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileSize(self.raw) }
    }

    pub fn seek(&self, offset: i64, whence: i32) -> i32 {
        unsafe { OH_ResourceManager_SeekRawFile(self.raw, offset, whence) }
    }

    pub fn offset(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileOffset(self.raw) }
    }

    pub fn read(&self, len: usize) -> (&str, i32) {
        let mut ret = Vec::with_capacity(len as usize);
        let buf_ptr = ret.as_mut_ptr();
        let offset = unsafe {
            OH_ResourceManager_ReadRawFile(self.raw, buf_ptr as *mut c_void, len as usize)
        };
        unsafe { (CStr::from_ptr(buf_ptr).to_str().unwrap_or(""), offset) }
    }
}

impl Drop for RawFile {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_CloseRawFile(self.raw);
        }
    }
}

pub struct RawFile64 {
    raw: *mut ohos_raw_sys::RawFile64,
}

impl RawFile64 {
    pub fn new(raw: *mut ohos_raw_sys::RawFile64) -> Self {
        Self { raw }
    }
}

impl Drop for RawFile64 {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_CloseRawFile64(self.raw);
        }
    }
}
