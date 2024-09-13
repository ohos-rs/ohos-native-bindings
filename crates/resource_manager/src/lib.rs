mod data;

use crate::data::{IconType, ScreenDensity};
use ohos_raw_binding::Raw;
use ohos_resource_manager_sys::{
    ArkUI_DrawableDescriptor, OH_ResourceManager_GetDrawableDescriptor,
    OH_ResourceManager_GetDrawableDescriptorByName, OH_ResourceManager_GetMedia,
    OH_ResourceManager_GetMediaBase64, OH_ResourceManager_GetMediaBase64ByName,
    OH_ResourceManager_GetMediaByName, ResourceManager_ErrorCode,
    ResourceManager_ErrorCode_SUCCESS,
};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

#[allow(unused_imports)]
pub use data::*;

pub trait ResourceManager {
    fn drawable_descriptor(
        &self,
        id: u32,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, ResourceManager_ErrorCode>;
    fn drawable_descriptor_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, ResourceManager_ErrorCode>;
    fn media(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, ResourceManager_ErrorCode>;
    fn media_base64(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<*mut *mut c_char, ResourceManager_ErrorCode>;
    fn media_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, ResourceManager_ErrorCode>;
    fn media_base64_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<*mut *mut c_char, ResourceManager_ErrorCode>;
}

impl ResourceManager for Raw {
    fn drawable_descriptor(
        &self,
        id: u32,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, ResourceManager_ErrorCode> {
        let use_density = density.unwrap_or_default();
        let use_icon = icon_type.unwrap_or_default();
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let code = unsafe {
            OH_ResourceManager_GetDrawableDescriptor(
                self.resource_manager,
                id,
                ret,
                use_density.into(),
                use_icon.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }

    fn drawable_descriptor_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
        icon_type: Option<IconType>,
    ) -> Result<*mut *mut ArkUI_DrawableDescriptor, ResourceManager_ErrorCode> {
        let use_name = CString::new(name).expect("Create CString failed");
        let use_density = density.unwrap_or_default();
        let use_icon = icon_type.unwrap_or_default();
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let code = unsafe {
            OH_ResourceManager_GetDrawableDescriptorByName(
                self.resource_manager,
                use_name.as_ptr().cast(),
                ret,
                use_density.into(),
                use_icon.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }

    fn media(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, ResourceManager_ErrorCode> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let code = unsafe {
            OH_ResourceManager_GetMedia(
                self.resource_manager,
                res_id,
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }

    fn media_base64(
        &self,
        res_id: u32,
        density: Option<ScreenDensity>,
    ) -> Result<*mut *mut c_char, ResourceManager_ErrorCode> {
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let code = unsafe {
            OH_ResourceManager_GetMediaBase64(
                self.resource_manager,
                res_id,
                ret,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }

    fn media_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<Vec<u8>, ResourceManager_ErrorCode> {
        let mut ret = Vec::new();
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let use_name = CString::new(name).expect("Create CString failed");

        let code = unsafe {
            OH_ResourceManager_GetMediaByName(
                self.resource_manager,
                use_name.as_ptr().cast(),
                ret.as_mut_ptr() as *mut *mut u8,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }

    fn media_base64_by_name(
        &self,
        name: String,
        density: Option<ScreenDensity>,
    ) -> Result<*mut *mut c_char, ResourceManager_ErrorCode> {
        let ret = Box::into_raw(Box::new(ptr::null_mut()));
        let mut len = 0;
        let use_density = density.unwrap_or_default();
        let use_name = CString::new(name).expect("Create CString failed");

        let code = unsafe {
            OH_ResourceManager_GetMediaBase64ByName(
                self.resource_manager,
                use_name.as_ptr().cast(),
                ret,
                &mut len,
                use_density.into(),
            )
        };
        if code == ResourceManager_ErrorCode_SUCCESS {
            Ok(ret)
        } else {
            Err(code)
        }
    }
}
