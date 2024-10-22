use std::sync::{LazyLock, RwLock};

use crate::r#type::CallbackClosure;

pub struct SingleXComponent {
    pub on_surface_created: Option<CallbackClosure>,
    pub on_surface_changed: Option<CallbackClosure>,
    pub on_surface_destroyed: Option<CallbackClosure>,
    pub dispatch_touch_event: Option<CallbackClosure>,
}

impl Default for SingleXComponent {
    fn default() -> Self {
        SingleXComponent {
            on_surface_changed: None,
            on_surface_created: None,
            on_surface_destroyed: None,
            dispatch_touch_event: None,
        }
    }
}

pub struct SingleCallbacks(RwLock<SingleXComponent>);

impl SingleCallbacks {
    pub(crate) fn borrow_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut SingleXComponent) -> R,
    {
        let mut write_lock = self.0.write().unwrap();
        f(&mut *write_lock)
    }

    pub(crate) fn borrow<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&SingleXComponent) -> R,
    {
        let write_lock: std::sync::RwLockReadGuard<'_, SingleXComponent> = self.0.read().unwrap();
        f(&*write_lock)
    }
}

impl Default for SingleCallbacks {
    fn default() -> Self {
        Self(RwLock::new(Default::default()))
    }
}

unsafe impl Send for SingleCallbacks {}
unsafe impl Sync for SingleCallbacks {}

pub(crate) static X_COMPONENT_SINGLE_MAP: LazyLock<SingleCallbacks> =
    LazyLock::new(Default::default);
