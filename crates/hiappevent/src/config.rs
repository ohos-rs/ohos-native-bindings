use crate::error::{check, HiAppEventError, Result};
use ohos_hiappevent_sys as sys;
use std::ffi::CString;

/// Configuration of one OS event, such as
/// [`event::MAIN_THREAD_JANK`](crate::event::MAIN_THREAD_JANK).
///
/// Set the items, then [`apply`](Self::apply) the config to the event.
pub struct EventConfig {
    raw: *mut sys::HiAppEvent_Config,
}

impl EventConfig {
    /// Create an empty config.
    pub fn new() -> Result<Self> {
        // SAFETY: the call takes no arguments and yields an owned handle or null.
        let raw = unsafe { sys::OH_HiAppEvent_CreateConfig() };
        if raw.is_null() {
            return Err(HiAppEventError::Alloc);
        }
        Ok(EventConfig { raw })
    }

    /// Set a config item; the accepted names depend on the event the config is
    /// applied to (see [`param`](crate::param)).
    pub fn set_item(&mut self, name: &str, value: &str) -> Result<()> {
        let name = CString::new(name)?;
        let value = CString::new(value)?;
        // SAFETY: raw is a live config handle; both strings outlive the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetConfigItem(
                self.raw,
                name.as_ptr(),
                value.as_ptr(),
            ))
        }
    }

    /// Apply this config to the OS event `event_name`.
    pub fn apply(&mut self, event_name: &str) -> Result<()> {
        let event_name = CString::new(event_name)?;
        // SAFETY: raw is a live config handle; the name outlives the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetEventConfig(
                event_name.as_ptr(),
                self.raw,
            ))
        }
    }
}

impl Drop for EventConfig {
    fn drop(&mut self) {
        // SAFETY: raw came from OH_HiAppEvent_CreateConfig and is freed once.
        unsafe { sys::OH_HiAppEvent_DestroyConfig(self.raw) };
    }
}
