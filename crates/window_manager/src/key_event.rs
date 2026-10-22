use std::{marker::PhantomData, ptr::NonNull};

use crate::Input_KeyEvent;

/// A key event borrowed for the duration of a native window-filter callback.
pub struct KeyEventRef<'event> {
    raw: NonNull<Input_KeyEvent>,
    lifetime: PhantomData<&'event Input_KeyEvent>,
}

impl<'event> KeyEventRef<'event> {
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

    /// Returns the native window ID associated with this event (API 15+).
    pub fn window_id(&self) -> i32 {
        // SAFETY: both headers declare the same opaque Input_KeyEvent C type;
        // from_raw's caller guarantees its lifetime covers this callback.
        unsafe {
            ohos_multi_modal_input_sys::OH_Input_GetKeyEventWindowId(self.raw.as_ptr().cast())
        }
    }
}
