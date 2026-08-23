use crate::error::{check, CommonEventError, Result};
use crate::parameters::Parameters;
use ohos_common_event_sys as sys;
use std::ffi::{c_char, CString};
use std::ptr::NonNull;

/// Publish a common event under `event`, with no extra information.
pub fn publish(event: &str) -> Result<()> {
    let event = CString::new(event)?;
    // SAFETY: the name outlives the call, which copies it.
    unsafe { check(sys::OH_CommonEvent_Publish(event.as_ptr())) }
}

/// Publish a common event under `event`, carrying `info`.
pub fn publish_with_info(event: &str, info: &PublishInfo) -> Result<()> {
    let event = CString::new(event)?;
    // SAFETY: the name outlives the call, which copies it, and `info` is a live
    // handle.
    unsafe {
        check(sys::OH_CommonEvent_PublishWithInfo(
            event.as_ptr(),
            info.as_ptr(),
        ))
    }
}

/// The content and attributes of a common event to [`publish_with_info`].
///
/// The native handle is created by `OH_CommonEvent_CreatePublishInfo` and
/// released on drop.
pub struct PublishInfo {
    raw: NonNull<sys::CommonEvent_PublishInfo>,
    // The native info stores the payload pointer without taking ownership, so
    // the payload is kept here and outlives every use of the handle.
    parameters: Option<Parameters>,
}

impl PublishInfo {
    /// Create publish information for an ordered or unordered event.
    ///
    /// An ordered event reaches its subscribers one at a time, each of which
    /// can pass a result on or abort the event; an unordered one reaches them
    /// all at once.
    pub fn new(ordered: bool) -> Result<Self> {
        // SAFETY: the call yields an owned handle or null.
        let raw = unsafe { sys::OH_CommonEvent_CreatePublishInfo(ordered) };
        NonNull::new(raw)
            .map(|raw| Self {
                raw,
                parameters: None,
            })
            .ok_or(CommonEventError::Alloc)
    }

    /// Only deliver the event to this bundle.
    pub fn set_bundle_name(&mut self, bundle_name: &str) -> Result<()> {
        let bundle_name = CString::new(bundle_name)?;
        // SAFETY: the handle is live and the string outlives the call, which
        // copies it.
        unsafe {
            check(sys::OH_CommonEvent_SetPublishInfoBundleName(
                self.raw.as_ptr(),
                bundle_name.as_ptr(),
            ))
        }
    }

    /// Only deliver the event to subscribers holding all of these permissions.
    pub fn set_permissions(&mut self, permissions: &[&str]) -> Result<()> {
        let owned = permissions
            .iter()
            .map(|permission| CString::new(*permission))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut pointers: Vec<*const c_char> =
            owned.iter().map(|permission| permission.as_ptr()).collect();
        let count = i32::try_from(pointers.len()).map_err(|_| CommonEventError::TooLong)?;
        // SAFETY: the handle is live and `pointers` holds `count` valid pointers
        // into `owned`, which outlives the call; the native side copies every
        // permission into its own storage.
        unsafe {
            check(sys::OH_CommonEvent_SetPublishInfoPermissions(
                self.raw.as_ptr(),
                pointers.as_mut_ptr(),
                count,
            ))
        }
    }

    /// The result code to publish with the event.
    pub fn set_code(&mut self, code: i32) -> Result<()> {
        // SAFETY: the handle is live.
        unsafe {
            check(sys::OH_CommonEvent_SetPublishInfoCode(
                self.raw.as_ptr(),
                code,
            ))
        }
    }

    /// The string payload to publish with the event.
    pub fn set_data(&mut self, data: &str) -> Result<()> {
        let data = CString::new(data)?;
        let length = data.as_bytes().len();
        // SAFETY: the handle is live and the string of `length` bytes outlives
        // the call, which copies it.
        unsafe {
            check(sys::OH_CommonEvent_SetPublishInfoData(
                self.raw.as_ptr(),
                data.as_ptr(),
                length,
            ))
        }
    }

    /// The key/value payload to publish with the event.
    ///
    /// The payload is moved in and kept alive by this info, because the native
    /// side stores nothing but a pointer to it and reads through that pointer
    /// at publish time. Any payload set earlier is released once the new one is
    /// in place.
    pub fn set_parameters(&mut self, parameters: Parameters) -> Result<()> {
        let pointer = parameters.as_ptr();
        let previous = self.parameters.replace(parameters);
        // SAFETY: the handle is live, and the payload behind `pointer` is now
        // owned by `self` and released no earlier than the handle.
        let result = unsafe {
            check(sys::OH_CommonEvent_SetPublishInfoParameters(
                self.raw.as_ptr(),
                pointer,
            ))
        };
        // Only now that the native side no longer refers to it.
        drop(previous);
        result
    }

    pub(crate) fn as_ptr(&self) -> *const sys::CommonEvent_PublishInfo {
        self.raw.as_ptr()
    }
}

impl Drop for PublishInfo {
    fn drop(&mut self) {
        // SAFETY: the handle came from OH_CommonEvent_CreatePublishInfo, is owned
        // here and is released once. The `parameters` field is dropped after
        // this, so the handle never outlives the payload it points at.
        unsafe { sys::OH_CommonEvent_DestroyPublishInfo(self.raw.as_ptr()) };
    }
}
