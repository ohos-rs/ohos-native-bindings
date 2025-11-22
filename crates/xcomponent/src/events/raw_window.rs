use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, NativeWindowOperation_GET_BUFFER_GEOMETRY,
    OH_NativeWindow_NativeWindowHandleOpt,
};
use raw_window_handle::{OhosNdkWindowHandle, RawWindowHandle};
use std::{
    os::raw::c_void,
    ptr::NonNull,
    sync::{LazyLock, RwLock},
};

// Same with WindowRaw, but thread safe.
#[derive(Debug, Clone, Copy)]
pub struct RawWindow {
    pub(crate) raw: *mut c_void,
    pub(crate) width: i32,
    pub(crate) height: i32,
}

unsafe impl Send for RawWindow {}
unsafe impl Sync for RawWindow {}

impl RawWindow {
    pub fn new(window: *mut c_void) -> Self {
        let mut width = 0;
        let mut height = 0;
        unsafe {
            OH_NativeWindow_NativeWindowHandleOpt(
                window as *mut NativeWindowRaw,
                NativeWindowOperation_GET_BUFFER_GEOMETRY as _,
                &mut height,
                &mut width,
            )
        };
        RawWindow {
            raw: window,
            width,
            height,
        }
    }

    pub fn raw(&self) -> *mut c_void {
        self.raw
    }

    /// Get window handle
    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        let guard = (*RAW_WINDOW).read();
        if let Ok(guard) = guard {
            if let Some(win) = &*guard {
                let win = NonNull::new(win.raw);
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
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
}

pub(crate) static RAW_WINDOW: LazyLock<RwLock<Option<RawWindow>>> =
    LazyLock::new(|| RwLock::new(None));
