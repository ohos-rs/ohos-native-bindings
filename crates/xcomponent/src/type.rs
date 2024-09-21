use ohos_xcomponent_sys::OH_NativeXComponent;
use std::{collections::HashMap, os::raw::c_void, sync::RwLock};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Window(pub *mut c_void);

#[repr(C)]
pub struct NativeXComponentCallback {
    pub on_surface_created:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub on_surface_changed:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub on_surface_destroyed:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub dispatch_touch_event:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
}

impl NativeXComponentCallback {
    pub fn new() -> Self {
        NativeXComponentCallback {
            on_surface_created: None,
            on_surface_changed: None,
            on_surface_destroyed: None,
            dispatch_touch_event: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XComponentSize {
    pub width: u64,
    pub height: u64,
}

pub(crate) struct PersistedPerInstanceHashMap<K, V>(RwLock<HashMap<K, V>>);

impl<K, V> PersistedPerInstanceHashMap<K, V> {
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn borrow_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut HashMap<K, V>) -> R,
    {
        let mut write_lock = self.0.write().unwrap();
        f(&mut *write_lock)
    }
}

impl<K, V> Default for PersistedPerInstanceHashMap<K, V> {
    fn default() -> Self {
        Self(RwLock::new(HashMap::default()))
    }
}

unsafe impl<K, V> Send for PersistedPerInstanceHashMap<K, V> {}
unsafe impl<K, V> Sync for PersistedPerInstanceHashMap<K, V> {}
