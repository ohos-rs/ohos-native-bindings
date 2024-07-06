use std::ffi::CString;

use napi_ohos::{bindgen_prelude::Object, Env, NapiRaw};
use ohos_raw_sys::{
    NativeResourceManager, OH_ResourceManager_InitNativeResourceManager,
    OH_ResourceManager_OpenRawDir, OH_ResourceManager_ReleaseNativeResourceManager,
};

mod data;

pub use data::*;

/// Raw file manager
pub struct Raw {
    pub resource_manager: *mut NativeResourceManager,
}

impl Raw {
    /// Use the resource manager from arkts to create a raw file manager.
    /// Be aware that none of these methods are thread-safe.
    /// ### Example
    /// ```no_run
    /// use napi_derive_ohos::napi;
    /// use napi_ohos::{bindgen_prelude::Object, Env};
    /// use ohos_raw_binding::Raw;
    ///
    /// #[napi]
    /// pub fn raw_example(
    ///     env: Env,
    ///     #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object,
    /// ) -> i32 {
    ///     let raw_manager = Raw::new(env, resource_manager);
    ///     let raw_dir = raw_manager.open_dir("");
    ///     let count = raw_dir.count();
    ///     count
    /// }
    /// ```
    pub fn new(env: Env, resource_manager: Object) -> Self {
        Raw {
            resource_manager: unsafe {
                OH_ResourceManager_InitNativeResourceManager(env.raw(), resource_manager.raw())
            },
        }
    }

    /// get raw file dirs
    pub fn open_dir<S: AsRef<str>>(&self, path: S) -> data::RawDir {
        let dir = CString::new(path.as_ref()).expect("Can't crate CString.");
        let raw =
            unsafe { OH_ResourceManager_OpenRawDir(self.resource_manager, dir.as_ptr().cast()) };
        data::RawDir::new(raw)
    }
}

impl Drop for Raw {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_ReleaseNativeResourceManager(self.resource_manager);
        }
    }
}
