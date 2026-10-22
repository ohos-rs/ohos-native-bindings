use std::{marker::PhantomData, ptr::NonNull};

use crate::Input_KeyEvent;

/// A key event borrowed for the duration of a native window-filter callback.
pub struct KeyEventRef<'event> {
    raw: NonNull<Input_KeyEvent>,
    lifetime: PhantomData<&'event Input_KeyEvent>,
}

impl<'event> KeyEventRef<'event> {
    /// Native cancellation action, without translating the SDK value.
    pub const ACTION_CANCEL: i32 =
        ohos_multi_modal_input_sys::Input_KeyEventAction_KEY_ACTION_CANCEL as i32;
    /// Native key-down action.
    pub const ACTION_DOWN: i32 =
        ohos_multi_modal_input_sys::Input_KeyEventAction_KEY_ACTION_DOWN as i32;
    /// Native key-up action.
    pub const ACTION_UP: i32 =
        ohos_multi_modal_input_sys::Input_KeyEventAction_KEY_ACTION_UP as i32;

    /// Borrows an event supplied by the native window event-filter API.
    ///
    /// # Safety
    ///
    /// A non-null pointer must refer to a live native key event for `'event`.
    /// The caller must not destroy or retain it beyond its native callback.
    pub unsafe fn from_raw(raw: *mut Input_KeyEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
            lifetime: PhantomData,
        })
    }

    /// Returns the unmodified native action (native API 12).
    pub fn action(&self) -> i32 {
        // SAFETY: from_raw guarantees this callback-borrowed native event is live.
        unsafe { ohos_multi_modal_input_sys::OH_Input_GetKeyEventAction(self.raw.as_ptr().cast()) }
    }

    /// Returns the unmodified native key code, including unknown future codes (native API 12).
    pub fn key_code(&self) -> i32 {
        // SAFETY: from_raw guarantees this callback-borrowed native event is live.
        unsafe { ohos_multi_modal_input_sys::OH_Input_GetKeyEventKeyCode(self.raw.as_ptr().cast()) }
    }

    /// Returns the monotonic native event time in microseconds (native API 12).
    /// This accessor does not deduplicate or otherwise interpret timestamps.
    pub fn action_time(&self) -> i64 {
        // SAFETY: from_raw guarantees this callback-borrowed native event is live.
        unsafe {
            ohos_multi_modal_input_sys::OH_Input_GetKeyEventActionTime(self.raw.as_ptr().cast())
        }
    }

    /// Returns the native window ID associated with this event (API 15+).
    pub fn window_id(&self) -> i32 {
        // SAFETY: both headers declare the same opaque Input_KeyEvent C type;
        // from_raw's caller guarantees its lifetime covers this callback.
        unsafe {
            ohos_multi_modal_input_sys::OH_Input_GetKeyEventWindowId(self.raw.as_ptr().cast())
        }
    }
}
