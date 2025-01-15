use ohos_display_binding::{default_display_height, default_display_width};
use raw_window_handle::{OhosNdkWindowHandle, RawWindowHandle};
use std::{
    os::raw::c_void,
    ptr::NonNull,
    sync::{LazyLock, RwLock},
};

// Same with WindowRaw, but thread safe.
pub struct RawWindow(pub(crate) *mut c_void);

unsafe impl Send for RawWindow {}
unsafe impl Sync for RawWindow {}

impl Clone for RawWindow {
    fn clone(&self) -> Self {
        RawWindow(self.0)
    }
}

impl RawWindow {
    pub fn new(window: *mut c_void) -> Self {
        RawWindow(window)
    }

    pub fn raw(&self) -> *mut c_void {
        self.0
    }

    /// Get window handle
    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        let guard = (*RAW_WINDOW).read();
        if let Ok(guard) = guard {
            if let Some(win) = &*guard {
                let win = NonNull::new(win.0);
                if let Some(win) = win {
                    return Some(RawWindowHandle::OhosNdk(OhosNdkWindowHandle::new(win)));
                }
                return None;
            }
            return None;
        }
        None
    }

    pub fn width(&self) -> i32 {
        default_display_width()
    }
    pub fn height(&self) -> i32 {
        default_display_height()
    }
}

pub(crate) static RAW_WINDOW: LazyLock<RwLock<Option<RawWindow>>> =
    LazyLock::new(|| RwLock::new(None));
