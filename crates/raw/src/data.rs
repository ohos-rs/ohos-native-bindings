use ohos_raw_sys::{
    NativeResourceManager, OH_ResourceManager_CloseRawDir, OH_ResourceManager_CloseRawFile,
    OH_ResourceManager_CloseRawFile64, OH_ResourceManager_GetRawFileCount,
    OH_ResourceManager_GetRawFileDescriptor64, OH_ResourceManager_GetRawFileDescriptorData,
    OH_ResourceManager_GetRawFileName, OH_ResourceManager_GetRawFileOffset,
    OH_ResourceManager_GetRawFileOffset64, OH_ResourceManager_GetRawFileRemainingLength,
    OH_ResourceManager_GetRawFileRemainingLength64, OH_ResourceManager_GetRawFileSize,
    OH_ResourceManager_GetRawFileSize64, OH_ResourceManager_IsRawDir,
    OH_ResourceManager_ReadRawFile, OH_ResourceManager_ReadRawFile64,
    OH_ResourceManager_SeekRawFile, OH_ResourceManager_SeekRawFile64, RawFileDescriptor,
    RawFileDescriptor64,
};
use std::ffi::CString;
use std::{ffi::CStr, os::raw::c_void};

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

    pub fn is_raw_dir(&self, dir: String) -> bool {
        let d = CString::new(dir).expect("Create CString failed.");
        unsafe {
            OH_ResourceManager_IsRawDir(self.raw as *const NativeResourceManager, d.as_ptr().cast())
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
        let mut ret = Vec::with_capacity(len as _);
        let buf_ptr = ret.as_mut_ptr();
        let offset =
            unsafe { OH_ResourceManager_ReadRawFile(self.raw, buf_ptr as *mut c_void, len as _) };
        unsafe { (CStr::from_ptr(buf_ptr).to_str().unwrap_or(""), offset) }
    }

    pub fn remain(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileRemainingLength(self.raw) }
    }

    /// try to get fd with start and length
    /// get failed it will return -1 and then return fd
    pub fn fd(&self, start: i64, len: i64) -> i32 {
        let mut file_descriptor = RawFileDescriptor {
            fd: 0,
            start,
            length: len,
        };
        let ret =
            unsafe { OH_ResourceManager_GetRawFileDescriptorData(self.raw, &mut file_descriptor) };
        if ret {
            file_descriptor.fd
        } else {
            -1
        }
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

    pub fn file_size(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileSize64(self.raw) }
    }

    pub fn seek(&self, offset: i64, whence: i32) -> i32 {
        unsafe { OH_ResourceManager_SeekRawFile64(self.raw, offset, whence) }
    }

    pub fn offset(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileOffset64(self.raw) }
    }

    pub fn read(&self, len: i64) -> (&str, i64) {
        let mut ret = Vec::with_capacity(len as usize);
        let buf_ptr = ret.as_mut_ptr();
        let offset =
            unsafe { OH_ResourceManager_ReadRawFile64(self.raw, buf_ptr as *mut c_void, len) };
        unsafe { (CStr::from_ptr(buf_ptr).to_str().unwrap_or(""), offset) }
    }

    pub fn remain(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileRemainingLength64(self.raw) }
    }

    /// try to get fd with start and length
    /// get failed it will return -1 and then return fd
    pub fn fd(&self, start: i64, len: i64) -> i32 {
        let mut file_descriptor = RawFileDescriptor64 {
            fd: 0,
            start,
            length: len,
        };
        let ret =
            unsafe { OH_ResourceManager_GetRawFileDescriptor64(self.raw, &mut file_descriptor) };
        if ret {
            file_descriptor.fd
        } else {
            -1
        }
    }
}

impl Drop for RawFile64 {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_CloseRawFile64(self.raw);
        }
    }
}
