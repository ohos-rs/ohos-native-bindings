use ohos_resource_manager_sys::{
    OH_ResourceManager_CloseRawDir, OH_ResourceManager_CloseRawFile,
    OH_ResourceManager_CloseRawFile64, OH_ResourceManager_GetRawFileCount,
    OH_ResourceManager_GetRawFileDescriptor64, OH_ResourceManager_GetRawFileDescriptorData,
    OH_ResourceManager_GetRawFileName, OH_ResourceManager_GetRawFileOffset,
    OH_ResourceManager_GetRawFileOffset64, OH_ResourceManager_GetRawFileRemainingLength,
    OH_ResourceManager_GetRawFileRemainingLength64, OH_ResourceManager_GetRawFileSize,
    OH_ResourceManager_GetRawFileSize64, OH_ResourceManager_ReadRawFile,
    OH_ResourceManager_ReadRawFile64, OH_ResourceManager_SeekRawFile,
    OH_ResourceManager_SeekRawFile64, RawFileDescriptor, RawFileDescriptor64,
};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::{ffi::CStr, fmt::Display, os::raw::c_void};

/// RawDir
pub struct RawDir {
    raw: NonNull<ohos_resource_manager_sys::RawDir>,
    path: String,

    /// Current file folder file name and index
    pub files: HashMap<String, i32>,
}

impl RawDir {
    pub fn from_raw(raw: NonNull<ohos_resource_manager_sys::RawDir>, path: String) -> Self {
        let mut files = HashMap::new();

        let count = unsafe { OH_ResourceManager_GetRawFileCount(raw.as_ptr()) };
        for i in 0..count {
            let name = unsafe { OH_ResourceManager_GetRawFileName(raw.as_ptr(), i) };
            files.insert(
                unsafe { CStr::from_ptr(name).to_str().unwrap_or("").to_string() },
                i,
            );
        }

        RawDir { raw, files, path }
    }
}

impl Drop for RawDir {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_CloseRawDir(self.raw.as_ptr());
        }
    }
}

impl Display for RawDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawDir: {}", self.path)
    }
}

pub struct RawFile {
    raw: NonNull<ohos_resource_manager_sys::RawFile>,
}

impl RawFile {
    pub fn from_raw(raw: *mut ohos_resource_manager_sys::RawFile) -> Self {
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "RawFile is null");

        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn file_size(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileSize(self.raw.as_ptr()) }
    }

    pub fn seek(&self, offset: i64, whence: i32) -> i32 {
        unsafe { OH_ResourceManager_SeekRawFile(self.raw.as_ptr(), offset, whence) }
    }

    pub fn offset(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileOffset(self.raw.as_ptr()) }
    }

    pub fn read(&self, len: usize) -> (&str, i32) {
        let mut ret = Vec::with_capacity(len as usize);
        let buf_ptr = ret.as_mut_ptr();
        let offset = unsafe {
            OH_ResourceManager_ReadRawFile(self.raw.as_ptr(), buf_ptr as *mut c_void, len as usize)
        };
        unsafe { (CStr::from_ptr(buf_ptr).to_str().unwrap_or(""), offset) }
    }

    pub fn remain(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileRemainingLength(self.raw.as_ptr()) }
    }

    /// try to get fd with start and length
    /// get failed it will return -1 and then return fd
    pub fn fd(&self, start: i64, len: i64) -> i32 {
        let mut file_descriptor = RawFileDescriptor {
            fd: 0,
            start,
            length: len,
        };
        let ret = unsafe {
            OH_ResourceManager_GetRawFileDescriptorData(self.raw.as_ptr(), &mut file_descriptor)
        };
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
            OH_ResourceManager_CloseRawFile(self.raw.as_ptr());
        }
    }
}

pub struct RawFile64 {
    raw: NonNull<ohos_resource_manager_sys::RawFile64>,
}

impl RawFile64 {
    pub fn new(raw: *mut ohos_resource_manager_sys::RawFile64) -> Self {
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "RawFile64 is null");
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn file_size(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileSize64(self.raw.as_ptr()) }
    }

    pub fn seek(&self, offset: i64, whence: i32) -> i32 {
        unsafe { OH_ResourceManager_SeekRawFile64(self.raw.as_ptr(), offset, whence) }
    }

    pub fn offset(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileOffset64(self.raw.as_ptr()) }
    }

    pub fn read(&self, len: i64) -> (&str, i64) {
        let mut ret = Vec::with_capacity(len as usize);
        let buf_ptr = ret.as_mut_ptr();
        let offset = unsafe {
            OH_ResourceManager_ReadRawFile64(self.raw.as_ptr(), buf_ptr as *mut c_void, len)
        };
        unsafe { (CStr::from_ptr(buf_ptr).to_str().unwrap_or(""), offset) }
    }

    pub fn remain(&self) -> i64 {
        unsafe { OH_ResourceManager_GetRawFileRemainingLength64(self.raw.as_ptr()) }
    }

    /// try to get fd with start and length
    /// get failed it will return -1 and then return fd
    pub fn fd(&self, start: i64, len: i64) -> i32 {
        let mut file_descriptor = RawFileDescriptor64 {
            fd: 0,
            start,
            length: len,
        };
        let ret = unsafe {
            OH_ResourceManager_GetRawFileDescriptor64(self.raw.as_ptr(), &mut file_descriptor)
        };
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
            OH_ResourceManager_CloseRawFile64(self.raw.as_ptr());
        }
    }
}
