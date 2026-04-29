use std::{ffi::CStr, ptr::NonNull, str::FromStr};

use ohos_image_native_binding::PixelMapNativeHandle;
use ohos_udmf_sys::{
    OH_UdsPixelMap, OH_UdsPixelMap_Create, OH_UdsPixelMap_Destroy, OH_UdsPixelMap_GetPixelMap,
    OH_UdsPixelMap_GetType, OH_UdsPixelMap_SetPixelMap,
};

use crate::{UdmfError, UdmfMeta};

use super::{Uds, UdsValue};

pub struct UdsPixelMap {
    pub(crate) raw: NonNull<OH_UdsPixelMap>,
}

impl UdsPixelMap {
    pub fn new() -> Self {
        let raw =
            NonNull::new(unsafe { OH_UdsPixelMap_Create() }).expect("OH_UdsPixelMap_Create failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdsPixelMap) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_UdsPixelMap_Create from a raw pointer failed"),
        }
    }

    pub fn set_pixel_map(&self, pixelmap: &PixelMapNativeHandle) -> Result<(), UdmfError> {
        let ret =
            unsafe { OH_UdsPixelMap_SetPixelMap(self.raw.as_ptr(), pixelmap.as_raw().cast()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    pub fn get_pixel_map(&self, pixelmap: &PixelMapNativeHandle) {
        unsafe { OH_UdsPixelMap_GetPixelMap(self.raw.as_ptr(), pixelmap.as_raw().cast()) }
    }
}

impl Drop for UdsPixelMap {
    fn drop(&mut self) {
        unsafe { OH_UdsPixelMap_Destroy(self.raw.as_ptr()) }
    }
}

impl From<UdsPixelMap> for Uds {
    fn from(value: UdsPixelMap) -> Self {
        Uds::PixelMap(value)
    }
}

impl UdsValue for UdsPixelMap {
    fn get_type(&self) -> Result<UdmfMeta, UdmfError> {
        let ret = unsafe { OH_UdsPixelMap_GetType(self.raw.as_ptr()) };
        let s = unsafe { CStr::from_ptr(ret) }
            .to_str()
            .map_err(|e| UdmfError::CommonError(e.to_string()))?;
        UdmfMeta::from_str(s)
            .map_err(|_| UdmfError::CommonError(String::from("UdmfMeta::from_str failed")))
    }
}

impl Default for UdsPixelMap {
    fn default() -> Self {
        Self::new()
    }
}
