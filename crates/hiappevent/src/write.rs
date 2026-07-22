use crate::constant::config_item;
use crate::error::{HiAppEventError, Result};
use crate::params::EventParams;
use crate::r#type::AppEventType;
use ohos_hiappevent_sys as sys;
use std::ffi::CString;

/// Log an application event to the local event file.
///
/// Returns `0` when every parameter passed verification, or a positive code when
/// the event was written after the invalid parameters were dropped. A negative
/// code means the event was rejected and is returned as an error. Either way the
/// code is explained by [`describe`](crate::describe).
pub fn write(
    domain: &str,
    name: &str,
    event_type: AppEventType,
    params: &EventParams,
) -> Result<i32> {
    let domain = CString::new(domain)?;
    let name = CString::new(name)?;
    // SAFETY: both strings outlive the call; the list is owned by `params`.
    let code = unsafe {
        sys::OH_HiAppEvent_Write(
            domain.as_ptr(),
            name.as_ptr(),
            event_type.into(),
            params.as_raw(),
        )
    };
    if code < 0 {
        Err(HiAppEventError::Native(code))
    } else {
        Ok(code)
    }
}

/// Set one of the [`config_item`](crate::config_item) values.
pub fn configure(name: &str, value: &str) -> Result<()> {
    let name = CString::new(name)?;
    let value = CString::new(value)?;
    // SAFETY: both strings outlive the call.
    let ok = unsafe { sys::OH_HiAppEvent_Configure(name.as_ptr(), value.as_ptr()) };
    if ok {
        Ok(())
    } else {
        Err(HiAppEventError::Failed)
    }
}

/// Turn application event logging off or back on.
pub fn set_logging_disabled(disabled: bool) -> Result<()> {
    configure(
        config_item::DISABLE,
        if disabled { "true" } else { "false" },
    )
}

/// Set the quota of the event file directory, e.g. `"100M"`.
pub fn set_max_storage(quota: &str) -> Result<()> {
    configure(config_item::MAX_STORAGE, quota)
}

/// Clear the event data stored locally for this application.
pub fn clear_data() {
    // SAFETY: the call takes no arguments and transfers no ownership.
    unsafe { sys::OH_HiAppEvent_ClearData() };
}
