use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use crate::r#type::CallbackClosure;

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

pub(crate) type XComponentMap = PersistedPerInstanceHashMap<String, CallbackClosure>;

// store callback by id
pub static X_COMPONENT_MAP: LazyLock<XComponentMap> = LazyLock::new(Default::default);
