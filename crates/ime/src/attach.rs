use std::ptr;

use ohos_input_method_sys::{
    InputMethod_AttachOptions, OH_AttachOptions_Create, OH_AttachOptions_Destroy,
    OH_AttachOptions_IsShowKeyboard,
};

/// Map an NDK Create result to an owning pointer.
///
/// `OH_AttachOptions_Create` returns NULL on OOM. That must not be wrapped as
/// an always-Ok leftover owner.
pub(crate) fn leftover_from_create<T>(raw: *mut T) -> Option<*mut T> {
    if raw.is_null() {
        None
    } else {
        Some(raw)
    }
}

/// Destroy an owned NDK pointer. Skip null so leftover Drop never Destroy(null).
///
/// The NDK has no refcount: one owner, one Destroy.
pub(crate) fn leftover_destroy<T>(raw: *mut T, destroy: impl FnOnce(*mut T)) {
    if !raw.is_null() {
        destroy(raw);
    }
}

/// Host-testable leftover owner with a stub Destroy counter.
///
/// Leftover `Clone` aliased `raw`. That impl is removed: NDK has no refcount,
/// so clone-then-drop would Destroy twice.
#[cfg(test)]
pub(crate) struct LeftoverAttachOwner<'a, T> {
    raw: *mut T,
    destroy_count: &'a std::cell::Cell<usize>,
}

#[cfg(test)]
impl<'a, T> LeftoverAttachOwner<'a, T> {
    pub(crate) fn from_create(
        raw: *mut T,
        destroy_count: &'a std::cell::Cell<usize>,
    ) -> Option<Self> {
        leftover_from_create(raw).map(|raw| Self { raw, destroy_count })
    }
}

#[cfg(test)]
impl<T> Drop for LeftoverAttachOwner<'_, T> {
    fn drop(&mut self) {
        leftover_destroy(self.raw, |_| {
            self.destroy_count.set(self.destroy_count.get() + 1);
        });
        self.raw = ptr::null_mut();
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct AttachOptions {
    pub(crate) raw: *mut InputMethod_AttachOptions,
}

impl AttachOptions {
    /// Panic-free constructor. Returns `None` when Create returns NULL.
    pub fn try_new(show_keyboard: bool) -> Option<Self> {
        leftover_from_create(unsafe { OH_AttachOptions_Create(show_keyboard) })
            .map(|raw| AttachOptions { raw })
    }

    pub fn new(show_keyboard: bool) -> Self {
        Self::try_new(show_keyboard).expect("OH_AttachOptions_Create failed")
    }

    pub fn is_showing_keyboard(&self) -> bool {
        let mut is_show = false;
        unsafe { OH_AttachOptions_IsShowKeyboard(self.raw, &mut is_show) };
        is_show
    }
}

impl Default for AttachOptions {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Clone for AttachOptions {
    fn clone(&self) -> Self {
        // NDK has no refcount: leftover Clone aliased `raw` and both Drops
        // called Destroy. Create a new instance instead.
        Self::new(self.is_showing_keyboard())
    }
}

impl Drop for AttachOptions {
    fn drop(&mut self) {
        leftover_destroy(self.raw, |raw| unsafe {
            OH_AttachOptions_Destroy(raw);
        });
        self.raw = ptr::null_mut();
    }
}

#[cfg(test)]
mod leftover_tests {
    use super::*;
    use std::cell::Cell;

    fn dummy_ptr() -> *mut u8 {
        0x10 as *mut u8
    }

    #[test]
    fn leftover_clone_removed_drop_destroys_once() {
        let destroys = Cell::new(0);
        {
            let owner = LeftoverAttachOwner::from_create(dummy_ptr(), &destroys)
                .expect("non-null Create must wrap");
            // leftover Clone is removed: there is no second owner of `raw`.
            drop(owner);
        }
        assert_eq!(destroys.get(), 1);
    }

    #[test]
    fn leftover_null_create_is_not_always_ok_and_skips_destroy() {
        let destroys = Cell::new(0);
        let owner = LeftoverAttachOwner::from_create(ptr::null_mut::<u8>(), &destroys);
        assert!(
            owner.is_none(),
            "null Create leftover must not wrap as always-Ok owning pointer"
        );
        leftover_destroy(ptr::null_mut::<u8>(), |p| {
            assert!(!p.is_null(), "leftover must not Destroy(null)");
            destroys.set(destroys.get() + 1);
        });
        assert_eq!(destroys.get(), 0);
    }
}
