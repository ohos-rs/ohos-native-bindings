use raw_window_handle::{OhosNdkWindowHandle, RawWindowHandle};
use std::{
    collections::HashMap,
    os::raw::c_void,
    ptr::NonNull,
    sync::{LazyLock, RwLock},
};

/// Address of a native window captured from a surface callback.
///
/// This is a plain pointer value: copying it or moving it across threads is
/// safe by itself. Dereferencing it (through NDK window APIs) is only valid
/// while the surface that produced it is alive; callers own that liveness
/// contract, typically by dropping the value from `OnSurfaceDestroyed`.
#[derive(Debug, Clone, Copy)]
pub struct RawWindow {
    pub(crate) raw: *mut c_void,
}

// SAFETY: `RawWindow` carries only an address with no interior state, so
// transferring it between threads is safe. Whether the pointee is alive is a
// separate contract documented on the type.
unsafe impl Send for RawWindow {}
// SAFETY: shared references expose only the address value; see `Send`.
unsafe impl Sync for RawWindow {}

impl RawWindow {
    pub fn new(window: *mut c_void) -> Self {
        RawWindow { raw: window }
    }

    pub fn raw(&self) -> *mut c_void {
        self.raw
    }

    /// Window handle for **this** window.
    pub fn raw_window_handle(&self) -> Option<RawWindowHandle> {
        NonNull::new(self.raw).map(|win| RawWindowHandle::OhosNdk(OhosNdkWindowHandle::new(win)))
    }
}

/// Live native windows keyed by their `OH_NativeXComponent` address.
///
/// Surface callbacks maintain one entry per XComponent instance, so multiple
/// simultaneously mounted XComponents do not overwrite one another (the
/// previous design used a single global slot, which broke `multi_mode`).
/// Entries are removed by `OnSurfaceDestroyed`.
pub(crate) static RAW_WINDOWS: LazyLock<RwLock<HashMap<usize, RawWindow>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) fn store_raw_window(xcomponent: *mut c_void, window: RawWindow) {
    if let Ok(mut windows) = RAW_WINDOWS.write() {
        windows.insert(xcomponent as usize, window);
    }
}

pub(crate) fn remove_raw_window(xcomponent: *mut c_void) {
    if let Ok(mut windows) = RAW_WINDOWS.write() {
        windows.remove(&(xcomponent as usize));
    }
}

pub(crate) fn lookup_raw_window(xcomponent: *mut c_void) -> Option<RawWindow> {
    RAW_WINDOWS
        .read()
        .ok()
        .and_then(|windows| windows.get(&(xcomponent as usize)).copied())
}
