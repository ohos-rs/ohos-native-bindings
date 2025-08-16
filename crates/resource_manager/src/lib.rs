use ohos_resource_manager_sys::{
    ArkUI_DrawableDescriptor, OH_ResourceManager_GetDrawableDescriptor,
    OH_ResourceManager_GetDrawableDescriptorByName, OH_ResourceManager_GetMedia,
    OH_ResourceManager_GetMediaBase64, OH_ResourceManager_GetMediaBase64ByName,
    OH_ResourceManager_GetMediaByName, ResourceManager_ErrorCode_SUCCESS,
};
use std::ffi::CString;
use std::ptr;

use std::ptr::NonNull;

#[cfg(feature = "napi")]
use napi_ohos::{bindgen_prelude::Object, Env, JsValue};
#[cfg(feature = "napi")]
use ohos_resource_manager_sys::OH_ResourceManager_InitNativeResourceManager;

use ohos_resource_manager_sys::{
    NativeResourceManager, OH_ResourceManager_IsRawDir, OH_ResourceManager_OpenRawDir,
    OH_ResourceManager_ReleaseNativeResourceManager,
};

mod error;
mod info;
mod raw_dir;

pub use error::*;
pub use info::*;
pub use raw_dir::*;

/// Resource Manager
pub struct ResourceManager {
    pub resource_manager: NonNull<NativeResourceManager>,
}

impl ResourceManager {
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
    #[cfg(feature = "napi")]
    pub fn new(env: Env, resource_manager: Object) -> Self {
        let raw = unsafe {
            OH_ResourceManager_InitNativeResourceManager(env.raw(), resource_manager.raw())
        };

        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "Raw is null");

        ResourceManager {
            resource_manager: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn from_raw(raw: *mut NativeResourceManager) -> Self {
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "Raw is null");

        ResourceManager {
            resource_manager: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    /// get raw file dirs
    pub fn open_dir<S: AsRef<str>>(&self, path: S) -> Result<RawDir, RawFileError> {
        let dir = CString::new(path.as_ref()).expect("Can't crate CString.");

        let raw = unsafe {
            OH_ResourceManager_OpenRawDir(self.resource_manager.as_ptr(), dir.as_ptr().cast())
        };
        #[cfg(debug_assertions)]
        assert!(!raw.is_null(), "RawDir is null");

        Ok(RawDir::from_raw(
            unsafe { NonNull::new_unchecked(raw) },
            path.as_ref().to_string(),
        ))
    }

    pub fn is_raw_dir<S: AsRef<str>>(&self, path: S) -> bool {
        let dir = CString::new(path.as_ref()).expect("Can't crate CString.");
        unsafe { OH_ResourceManager_IsRawDir(self.resource_manager.as_ptr(), dir.as_ptr().cast()) }
    }

    pub fn drawable_descriptor(
        &self,
        id: u32,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, RawFileError> {
        let use_density = density.unwrap_or_default();
        let use_icon = icon_type.unwrap_or_default();
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let code = unsafe {
            OH_ResourceManager_GetDrawableDescriptor(
                self.resource_manager.as_ptr(),
                id,
                ret,
                use_density.into(),
                use_icon.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Drawable descriptor by name failed: {}",
                code
            )))
        }
    }

    pub fn drawable_descriptor_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, RawFileError> {
        let use_name = CString::new(name).expect("Create CString failed");
        let use_density = density.unwrap_or_default();
        let use_icon = icon_type.unwrap_or_default();
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let code = unsafe {
            OH_ResourceManager_GetDrawableDescriptorByName(
                self.resource_manager.as_ptr(),
                use_name.as_ptr().cast(),
                ret,
                use_density.into(),
                use_icon.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Drawable descriptor by name failed: {}",
                code
            )))
        }
    }

    pub fn media(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, RawFileError> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let code = unsafe {
            OH_ResourceManager_GetMedia(
                self.resource_manager.as_ptr(),
                res_id,
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Media failed: {}",
                code
            )))
        }
    }

    pub fn media_base64(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, RawFileError> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let code = unsafe {
            OH_ResourceManager_GetMediaBase64(
                self.resource_manager.as_ptr(),
                res_id,
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Media base64 failed: {}",
                code
            )))
        }
    }

    pub fn media_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, RawFileError> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let use_name = CString::new(name).expect("Create CString failed");

        let code = unsafe {
            OH_ResourceManager_GetMediaByName(
                self.resource_manager.as_ptr(),
                use_name.as_ptr().cast(),
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Media by name failed: {}",
                code
            )))
        }
    }

    pub fn media_base64_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, RawFileError> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let use_name = CString::new(name).expect("Create CString failed");

        let code = unsafe {
            OH_ResourceManager_GetMediaBase64ByName(
                self.resource_manager.as_ptr(),
                use_name.as_ptr().cast(),
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(RawFileError::FfiInnerError(format!(
                "Media base64 by name failed: {}",
                code
            )))
        }
    }
}

impl Drop for ResourceManager {
    fn drop(&mut self) {
        unsafe {
            OH_ResourceManager_ReleaseNativeResourceManager(self.resource_manager.as_ptr());
        }
    }
}
