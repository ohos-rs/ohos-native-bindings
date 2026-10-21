//! Owned API-12 resource-configuration snapshots.

use crate::{RawFileError, ResourceManager};
use std::ffi::CStr;

/// Device configuration copied out of the native manager's allocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
    /// System locale identifier.
    pub locale: String,
    /// Platform device type: phone 0, tablet 1, car 2, PC 3, TV 4, wearable 6, 2-in-1 7.
    pub device_type: u32,
    /// Whether the configured appearance is dark.
    pub dark: bool,
}

struct Snapshot {
    raw: ohos_resource_manager_sys::ResourceManager_Configuration,
    initialized: bool,
}
impl Drop for Snapshot {
    fn drop(&mut self) {
        if self.initialized || !self.raw.locale.is_null() {
            // SAFETY: this output owns the only native locale allocation; its
            // scalars and nullable pointer were initialized before the call.
            unsafe {
                ohos_resource_manager_sys::OH_ResourceManager_ReleaseConfiguration(&mut self.raw);
            }
        }
    }
}

impl ResourceManager {
    /// Copies locale, device type and appearance using API 12.
    /// Serialize this operation with other calls on the same manager.
    pub fn configuration(&self) -> Result<Configuration, RawFileError> {
        let mut snapshot = Snapshot {
            // SAFETY: the SDK output contains only integer fields and a nullable pointer.
            raw: unsafe { std::mem::zeroed() },
            initialized: false,
        };
        // SAFETY: the live manager writes into this owned output synchronously.
        let status = unsafe {
            ohos_resource_manager_sys::OH_ResourceManager_GetConfiguration(
                self.resource_manager.as_ptr(),
                &mut snapshot.raw,
            )
        };
        if status != ohos_resource_manager_sys::ResourceManager_ErrorCode_SUCCESS {
            return Err(RawFileError::FfiInnerError(format!(
                "get configuration failed: {status}"
            )));
        }
        snapshot.initialized = true;
        let locale = if snapshot.raw.locale.is_null() {
            String::new()
        } else {
            // SAFETY: the successful output owns a terminated string until Snapshot drops.
            unsafe { CStr::from_ptr(snapshot.raw.locale) }
                .to_str()
                .map(str::to_owned)
                .map_err(|error| RawFileError::FfiInnerError(error.to_string()))?
        };
        Ok(Configuration {
            locale,
            device_type: snapshot.raw.deviceType,
            dark: snapshot.raw.colorMode
                == ohos_resource_manager_sys::ResourceManager_ColorMode_COLOR_MODE_DARK,
        })
    }
}
