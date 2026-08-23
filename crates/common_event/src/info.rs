use crate::error::{check, CommonEventError, Result};
use ohos_common_event_sys as sys;
use std::ffi::{c_char, CString};
use std::ptr::NonNull;

/// The events a [`Subscriber`](crate::Subscriber) listens to, plus the optional
/// filters on who published them.
///
/// The native handle is created by `OH_CommonEvent_CreateSubscribeInfo` and
/// released on drop. A [`Subscriber`](crate::Subscriber) copies everything it
/// needs out of the info when it is created, so the info may be dropped or
/// reused afterwards.
pub struct SubscribeInfo {
    raw: NonNull<sys::CommonEvent_SubscribeInfo>,
}

impl SubscribeInfo {
    /// Subscribe to the given event names, for example
    /// [`event::SCREEN_ON`](crate::event::SCREEN_ON).
    ///
    /// The list must not be empty, and no name may contain an interior NUL byte.
    pub fn new(events: &[&str]) -> Result<Self> {
        if events.is_empty() {
            return Err(CommonEventError::InvalidParameter);
        }
        let owned = events
            .iter()
            .map(|event| CString::new(*event))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut pointers: Vec<*const c_char> = owned.iter().map(|event| event.as_ptr()).collect();
        let count = i32::try_from(pointers.len()).map_err(|_| CommonEventError::TooLong)?;

        // SAFETY: `pointers` holds `count` valid pointers into `owned`, which
        // outlives the call, and the native side copies every name into its own
        // storage before returning.
        let raw = unsafe { sys::OH_CommonEvent_CreateSubscribeInfo(pointers.as_mut_ptr(), count) };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(CommonEventError::Alloc)
    }

    /// Only receive events published by an application holding this permission.
    pub fn set_publisher_permission(&mut self, permission: &str) -> Result<()> {
        let permission = CString::new(permission)?;
        // SAFETY: the handle is live and the string outlives the call, which
        // copies it.
        unsafe {
            check(sys::OH_CommonEvent_SetPublisherPermission(
                self.raw.as_ptr(),
                permission.as_ptr(),
            ))
        }
    }

    /// Only receive events published by this bundle.
    pub fn set_publisher_bundle_name(&mut self, bundle_name: &str) -> Result<()> {
        let bundle_name = CString::new(bundle_name)?;
        // SAFETY: the handle is live and the string outlives the call, which
        // copies it.
        unsafe {
            check(sys::OH_CommonEvent_SetPublisherBundleName(
                self.raw.as_ptr(),
                bundle_name.as_ptr(),
            ))
        }
    }

    pub(crate) fn as_ptr(&self) -> *const sys::CommonEvent_SubscribeInfo {
        self.raw.as_ptr()
    }
}

impl Drop for SubscribeInfo {
    fn drop(&mut self) {
        // SAFETY: the handle came from OH_CommonEvent_CreateSubscribeInfo, is
        // owned here and is released once.
        unsafe { sys::OH_CommonEvent_DestroySubscribeInfo(self.raw.as_ptr()) };
    }
}
