use ohos_resource_manager_sys::{
    OH_ResourceManager_CloseRawDir, OH_ResourceManager_CloseRawFile,
    OH_ResourceManager_CloseRawFile64, OH_ResourceManager_GetRawFileCount,
    OH_ResourceManager_GetRawFileDescriptor64, OH_ResourceManager_GetRawFileDescriptorData,
    OH_ResourceManager_GetRawFileName, OH_ResourceManager_GetRawFileOffset,
    OH_ResourceManager_GetRawFileOffset64, OH_ResourceManager_GetRawFileRemainingLength,
    OH_ResourceManager_GetRawFileRemainingLength64, OH_ResourceManager_GetRawFileSize,
    OH_ResourceManager_GetRawFileSize64, OH_ResourceManager_IsRawDir,
    OH_ResourceManager_OpenRawDir, OH_ResourceManager_OpenRawFile,
    OH_ResourceManager_OpenRawFile64, OH_ResourceManager_ReadRawFile,
    OH_ResourceManager_ReadRawFile64, OH_ResourceManager_SeekRawFile,
    OH_ResourceManager_SeekRawFile64, RawFileDescriptor, RawFileDescriptor64,
};
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr::NonNull;
use std::{ffi::CStr, fmt::Display, os::raw::c_void};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMetaInfo {
    index: i32,
    is_dir: bool,
}

/// RawDir
pub struct RawDir {
    #[allow(dead_code)]
    mgr: NonNull<ohos_resource_manager_sys::NativeResourceManager>,
    path: String,

    /// Current file folder file name and index
    pub files: HashMap<String, FileMetaInfo>,
}

impl RawDir {
    pub fn from_raw(
        mgr: NonNull<ohos_resource_manager_sys::NativeResourceManager>,
        path: String,
        recursive: bool,
    ) -> Self {
        let mut files = HashMap::new();
        let mut dirs = Vec::new();

        let dir = CString::new(path.clone()).expect("Can't crate CString.");
        let raw = unsafe { OH_ResourceManager_OpenRawDir(mgr.as_ptr(), dir.as_ptr().cast()) };
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "RawDir is null");

        let count = unsafe { OH_ResourceManager_GetRawFileCount(raw) };
        for i in 0..count {
            let name = unsafe { OH_ResourceManager_GetRawFileName(raw, i) };
            let name_ret = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") }.to_string();

            // avoid double //
            let mut format_name = format!("{}/{}", path, name_ret).replace("//", "/");

            if format_name.starts_with("/") {
                format_name = format_name.split_at(1).1.to_string();
            }

            let full_path = CString::new(format_name.clone()).unwrap();

            let is_dir =
                unsafe { OH_ResourceManager_IsRawDir(mgr.as_ptr(), full_path.as_ptr().cast()) };

            files.insert(format_name.clone(), FileMetaInfo { index: i, is_dir });

            // if recursive, and is dir, then add to dirs
            if recursive && is_dir {
                dirs.push(format_name.clone());
            }
        }

        // close raw dir
        unsafe { OH_ResourceManager_CloseRawDir(raw) };

        while let Some(dir) = dirs.pop() {
            let dir_c = CString::new(dir.clone()).unwrap();

            let dir_raw =
                unsafe { OH_ResourceManager_OpenRawDir(mgr.as_ptr(), dir_c.as_ptr().cast()) };
            #[cfg(debug_assertions)]
            assert!(!dir_raw.is_null(), "RawDir is null");

            let count = unsafe { OH_ResourceManager_GetRawFileCount(dir_raw) };
            for i in 0..count {
                let name = unsafe { OH_ResourceManager_GetRawFileName(dir_raw, i) };
                let name_ret = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") }.to_string();

                let mut format_name = format!("{}/{}", dir.clone(), name_ret).replace("//", "/");

                if format_name.starts_with("/") {
                    format_name = format_name.split_at(1).1.to_string();
                }

                let full_path = CString::new(format_name.clone()).unwrap();

                let is_dir =
                    unsafe { OH_ResourceManager_IsRawDir(mgr.as_ptr(), full_path.as_ptr().cast()) };

                files.insert(format_name.clone(), FileMetaInfo { index: i, is_dir });

                // if recursive, and is dir, then add to dirs
                if recursive && is_dir {
                    dirs.push(format_name.clone());
                }
            }

            unsafe { OH_ResourceManager_CloseRawDir(dir_raw) };
        }

        RawDir { mgr, files, path }
    }

    pub fn open_file<S: AsRef<str>>(&self, file_name: S) -> RawFile {
        let file_name = file_name.as_ref();
        let file_name_c = CString::new(file_name).unwrap();

        let raw = unsafe {
            OH_ResourceManager_OpenRawFile(self.mgr.as_ptr(), file_name_c.as_ptr().cast())
        };
        RawFile::from_raw(raw)
    }

    pub fn open_file64<S: AsRef<str>>(&self, file_name: S) -> RawFile64 {
        let file_name = file_name.as_ref();
        let file_name_c = CString::new(file_name).unwrap();

        let raw = unsafe {
            OH_ResourceManager_OpenRawFile64(self.mgr.as_ptr(), file_name_c.as_ptr().cast())
        };
        RawFile64::from_raw(raw)
    }
}

impl Display for RawDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawDir: {}", self.path)
    }
}

/// Use for file size less than 2G
/// If the file size is greater than 2G, there is a risk of precision loss, it is recommended to use RawFile64
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

    pub fn file_size(&self) -> i32 {
        unsafe { OH_ResourceManager_GetRawFileSize(self.raw.as_ptr()) as _ }
    }

    pub fn seek(&self, offset: i32, whence: i32) -> i32 {
        unsafe { OH_ResourceManager_SeekRawFile(self.raw.as_ptr(), offset as _, whence) }
    }

    pub fn offset(&self) -> i32 {
        unsafe { OH_ResourceManager_GetRawFileOffset(self.raw.as_ptr()) as _ }
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
        unsafe { OH_ResourceManager_GetRawFileRemainingLength(self.raw.as_ptr()) as i64 }
    }

    /// try to get fd with start and length
    /// get failed it will return -1 and then return fd
    pub fn fd(&self, start: i32, len: i32) -> i32 {
        let mut file_descriptor = RawFileDescriptor {
            fd: 0,
            start: start as _,
            length: len as _,
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

/// Using for file size.
/// Recommended to use this type if the file size is greater than 2G
impl RawFile64 {
    pub fn from_raw(raw: *mut ohos_resource_manager_sys::RawFile64) -> Self {
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
