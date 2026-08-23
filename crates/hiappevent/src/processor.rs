use crate::array_len;
use crate::error::{check, HiAppEventError, Result};
use ohos_hiappevent_sys as sys;
use std::ffi::{c_char, CString};

/// A reporting processor: uploads logged events to a backend.
///
/// Configure it, then [`add`](Self::add) it. A registered processor is removed
/// when dropped; [`into_id`](Self::into_id) releases the handle but leaves it
/// registered.
pub struct Processor {
    raw: *mut sys::HiAppEvent_Processor,
    id: Option<i64>,
}

impl Processor {
    /// Create a processor identified by `name`.
    pub fn new(name: &str) -> Result<Self> {
        let name = CString::new(name)?;
        // SAFETY: name outlives the call.
        let raw = unsafe { sys::OH_HiAppEvent_CreateProcessor(name.as_ptr()) };
        if raw.is_null() {
            return Err(HiAppEventError::Alloc);
        }
        Ok(Processor { raw, id: None })
    }

    /// Set the application id and the server location events are reported to.
    pub fn set_report_route(&mut self, app_id: &str, route_info: &str) -> Result<()> {
        let app_id = CString::new(app_id)?;
        let route_info = CString::new(route_info)?;
        // SAFETY: raw is a live processor handle; both strings outlive the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetReportRoute(
                self.raw,
                app_id.as_ptr(),
                route_info.as_ptr(),
            ))
        }
    }

    /// Set when events are reported: every `period_report` seconds, once
    /// `batch_report` events have accumulated, on startup and on going to the
    /// background.
    pub fn set_report_policy(
        &mut self,
        period_report: i32,
        batch_report: i32,
        on_start_report: bool,
        on_background_report: bool,
    ) -> Result<()> {
        // SAFETY: raw is a live processor handle.
        unsafe {
            check(sys::OH_HiAppEvent_SetReportPolicy(
                self.raw,
                period_report,
                batch_report,
                on_start_report,
                on_background_report,
            ))
        }
    }

    /// Report the event `domain`/`name`, in real time or per the report policy.
    pub fn set_report_event(&mut self, domain: &str, name: &str, real_time: bool) -> Result<()> {
        let domain = CString::new(domain)?;
        let name = CString::new(name)?;
        // SAFETY: raw is a live processor handle; both strings outlive the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetReportEvent(
                self.raw,
                domain.as_ptr(),
                name.as_ptr(),
                real_time,
            ))
        }
    }

    /// Set a custom key/value carried by the processor.
    pub fn set_custom_config(&mut self, key: &str, value: &str) -> Result<()> {
        let key = CString::new(key)?;
        let value = CString::new(value)?;
        // SAFETY: raw is a live processor handle; both strings outlive the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetCustomConfig(
                self.raw,
                key.as_ptr(),
                value.as_ptr(),
            ))
        }
    }

    /// Set the id of the processor configuration.
    pub fn set_config_id(&mut self, config_id: i32) -> Result<()> {
        // SAFETY: raw is a live processor handle.
        unsafe { check(sys::OH_HiAppEvent_SetConfigId(self.raw, config_id)) }
    }

    /// Set the name of the processor configuration.
    #[cfg(feature = "api-20")]
    pub fn set_config_name(&mut self, config_name: &str) -> Result<()> {
        let config_name = CString::new(config_name)?;
        // SAFETY: raw is a live processor handle; the string outlives the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetConfigName(
                self.raw,
                config_name.as_ptr(),
            ))
        }
    }

    /// Set the names of the user ids reported with each event.
    pub fn set_report_user_id(&mut self, names: &[&str]) -> Result<()> {
        let (_names, ptrs) = c_strings(names)?;
        // SAFETY: raw is a live processor handle; the name array outlives the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetReportUserId(
                self.raw,
                ptrs.as_ptr(),
                array_len(ptrs.len()),
            ))
        }
    }

    /// Set the names of the user properties reported with each event.
    pub fn set_report_user_property(&mut self, names: &[&str]) -> Result<()> {
        let (_names, ptrs) = c_strings(names)?;
        // SAFETY: raw is a live processor handle; the name array outlives the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetReportUserProperty(
                self.raw,
                ptrs.as_ptr(),
                array_len(ptrs.len()),
            ))
        }
    }

    /// Register the processor and return the id it was assigned.
    ///
    /// Registering twice would strand the first registration, since only one id can be
    /// tracked, so a second call returns the existing id instead.
    pub fn add(&mut self) -> Result<i64> {
        if let Some(id) = self.id {
            return Ok(id);
        }
        // SAFETY: raw is a live processor handle.
        let id = unsafe { sys::OH_HiAppEvent_AddProcessor(self.raw) };
        if id < 0 {
            return Err(HiAppEventError::Native(id as i32));
        }
        self.id = Some(id);
        Ok(id)
    }

    /// The id assigned by [`add`](Self::add).
    pub fn id(&self) -> Option<i64> {
        self.id
    }

    /// Unregister the processor. Does nothing if it was never added.
    ///
    /// The id is kept on failure so a later retry, or the [`Drop`] fallback, can
    /// still remove the registration.
    pub fn remove(&mut self) -> Result<()> {
        match self.id.take() {
            Some(id) => match remove_processor(id) {
                Ok(()) => Ok(()),
                Err(e) => {
                    self.id = Some(id);
                    Err(e)
                }
            },
            None => Ok(()),
        }
    }

    /// Release the handle but leave the processor registered, returning the id
    /// needed to [`remove_processor`] it later.
    pub fn into_id(mut self) -> Option<i64> {
        self.id.take()
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        // SAFETY: id was returned by OH_HiAppEvent_AddProcessor; raw is a live
        // handle, destroyed once.
        unsafe {
            if let Some(id) = self.id {
                sys::OH_HiAppEvent_RemoveProcessor(id);
            }
            sys::OH_HiAppEvent_DestroyProcessor(self.raw);
        }
    }
}

/// Unregister the processor with the given id.
pub fn remove_processor(id: i64) -> Result<()> {
    // SAFETY: the id is passed by value.
    unsafe { check(sys::OH_HiAppEvent_RemoveProcessor(id)) }
}

/// Both halves are returned because the pointer array borrows the strings: the
/// caller must hold on to them until the native call has returned.
fn c_strings(values: &[&str]) -> Result<(Vec<CString>, Vec<*const c_char>)> {
    let owned = values
        .iter()
        .map(|value| CString::new(*value))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let ptrs = owned.iter().map(|value| value.as_ptr()).collect();
    Ok((owned, ptrs))
}
