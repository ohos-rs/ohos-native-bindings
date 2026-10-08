use std::marker::PhantomData;
use std::ptr::{self, NonNull};

use ohos_native_window_manager_sys::{
    OH_WindowManager_DensityInfo, OH_WindowManager_DensityInfo_GetCustomDensity,
    OH_WindowManager_DensityInfo_GetDefaultDensity, OH_WindowManager_DensityInfo_GetSystemDensity,
    OH_WindowManager_DensityInfo_Release, OH_WindowManager_GetDensityInfoCopy,
    OH_WindowManager_RegisterDensityInfoChangeCallback,
    OH_WindowManager_UnregisterDensityInfoChangeCallback,
};

use crate::error::{check, Result};
use crate::Window;

pub type RawDensityInfo = OH_WindowManager_DensityInfo;
pub type DensityInfoChangeCallback =
    unsafe extern "C" fn(window_id: i32, info: *const RawDensityInfo);

#[derive(Debug)]
pub struct DensityInfo {
    raw: NonNull<RawDensityInfo>,
}

impl DensityInfo {
    pub fn default_density(&self) -> Result<f32> {
        self.as_ref().default_density()
    }

    pub fn system_density(&self) -> Result<f32> {
        self.as_ref().system_density()
    }

    pub fn custom_density(&self) -> Result<f32> {
        self.as_ref().custom_density()
    }

    pub fn as_ref(&self) -> DensityInfoRef<'_> {
        DensityInfoRef {
            raw: self.raw,
            marker: PhantomData,
        }
    }
}

impl Drop for DensityInfo {
    fn drop(&mut self) {
        let _ = unsafe { OH_WindowManager_DensityInfo_Release(self.raw.as_ptr()) };
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DensityInfoRef<'a> {
    raw: NonNull<RawDensityInfo>,
    marker: PhantomData<&'a RawDensityInfo>,
}

impl<'a> DensityInfoRef<'a> {
    /// Creates a borrowed density-info view from a native callback pointer.
    ///
    /// # Safety
    ///
    /// `raw` must remain valid and immutable for the returned view's lifetime.
    pub unsafe fn from_raw(raw: *const RawDensityInfo) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(|raw| Self {
            raw,
            marker: PhantomData,
        })
    }

    pub fn default_density(self) -> Result<f32> {
        self.read(OH_WindowManager_DensityInfo_GetDefaultDensity)
    }

    pub fn system_density(self) -> Result<f32> {
        self.read(OH_WindowManager_DensityInfo_GetSystemDensity)
    }

    pub fn custom_density(self) -> Result<f32> {
        self.read(OH_WindowManager_DensityInfo_GetCustomDensity)
    }

    pub const fn as_raw(self) -> *const RawDensityInfo {
        self.raw.as_ptr()
    }

    fn read(
        self,
        getter: unsafe extern "C" fn(*const RawDensityInfo, *mut f32) -> i32,
    ) -> Result<f32> {
        let mut value = 0.0;
        check(unsafe { getter(self.as_raw(), &mut value) })?;
        Ok(value)
    }
}

impl Window {
    pub fn density_info(self) -> Result<Option<DensityInfo>> {
        let mut raw = ptr::null();
        check(unsafe { OH_WindowManager_GetDensityInfoCopy(self.id(), &mut raw) })?;
        Ok(NonNull::new(raw.cast_mut()).map(|raw| DensityInfo { raw }))
    }

    pub fn register_density_info_change_callback(
        self,
        callback: DensityInfoChangeCallback,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_RegisterDensityInfoChangeCallback(self.id(), Some(callback))
        })
    }

    pub fn unregister_density_info_change_callback(
        self,
        callback: DensityInfoChangeCallback,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_UnregisterDensityInfoChangeCallback(self.id(), Some(callback))
        })
    }
}
