//! Native node `user_data` ownership for wrapper-created children.
//!
//! ArkUI's `setUserData`/`getUserData` pair stores an opaque pointer on a
//! native node. The wrapper layer uses it to route native events back to the
//! Rust `Rc<RefCell<ArkUINode>>` wrapper (see `api::node::receiver`). Every
//! `add_child`/`insert_child` call installs a fresh wrapper box; native
//! handles are long-lived and can be re-attached many times, so a naive
//! overwrite leaks the previous box and a `dispose` that ignores the pointer
//! leaks it permanently.
//!
//! Ownership is encoded in the pointer itself instead of a global registry:
//! every pointer installed by this layer points at a [`UserDataBox`] whose
//! header carries a magic value. Cleanup checks the magic before reclaiming,
//! so pointers owned by external code (ArkTS `FrameNode` bridges, `RootNode`'s
//! base) are recognized and left untouched. This keeps the hot
//! `add_child`/`insert_child` path lock-free and free of global state.

use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;

use crate::common::node::ArkUINode;
use crate::ArkUIResult;

/// Magic value stamped on every pointer installed by this module. Also read by
/// `api::node::receiver` to classify event-dispatch user data before
/// dereferencing it as a wrapper box.
pub(crate) const USER_DATA_MAGIC: u32 = 0x5752_4150; // "WRAP"

/// Heap layout of a wrapper-installed `user_data` pointer.
///
/// `#[repr(C)]` so the magic lives at offset 0: ownership checks only need to
/// read the first word, which also means a stale or foreign pointer can be
/// classified without touching (and possibly faulting on) the rest.
#[repr(C)]
pub(crate) struct UserDataBox {
    magic: u32,
    /// The wrapper routed to by native event callbacks. Read by
    /// `api::node::receiver`; nothing else touches the box's interior.
    pub(crate) wrapper: Rc<RefCell<ArkUINode>>,
}

/// Whether `pointer` was installed by this module (points at a live
/// [`UserDataBox`] with a valid magic).
unsafe fn is_wrapper_user_data(pointer: *mut c_void) -> bool {
    !pointer.is_null() && unsafe { *(pointer as *const u32) } == USER_DATA_MAGIC
}

/// Install `wrapper` as the native node's event-dispatch user data, releasing
/// any previously wrapper-owned pointer in the process.
///
/// The new pointer is installed first so a failed `set_user_data` cannot leave
/// the node pointing at freed memory; the old box is reclaimed only after the
/// new pointer is live. `add_event_receiver` remains the caller's
/// responsibility so existing call sites keep their error semantics.
pub(crate) fn install_wrapper_user_data(
    node: &ArkUINode,
    wrapper: Rc<RefCell<ArkUINode>>,
) -> ArkUIResult<()> {
    let previous =
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.get_user_data(node.raw_handle()))?;
    let new_ptr = Box::into_raw(Box::new(UserDataBox {
        magic: USER_DATA_MAGIC,
        wrapper,
    })) as *mut c_void;
    crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, new_ptr))?;

    if let Some(previous) = previous {
        if unsafe { is_wrapper_user_data(previous.as_ptr()) } {
            // SAFETY: the magic proves the pointer was installed by this
            // module, so it is a live `Box<UserDataBox>` that the node no
            // longer references (the new pointer is already live).
            drop(unsafe { Box::from_raw(previous.as_ptr() as *mut UserDataBox) });
        }
    }
    Ok(())
}

/// Release and null out a wrapper-owned `user_data` pointer, if any.
///
/// Called from `dispose`: the native node is gone (or about to be), so the
/// retained `Rc` strong reference must be dropped and the stale pointer
/// cleared. External pointers are left untouched.
pub(crate) fn release_wrapper_user_data(node: &ArkUINode) {
    let Some(previous) = crate::ARK_UI_NATIVE_NODE_API_1
        .with(|api| api.get_user_data(node.raw_handle()))
        .ok()
        .flatten()
    else {
        return;
    };
    if !unsafe { is_wrapper_user_data(previous.as_ptr()) } {
        return;
    }
    // SAFETY: same ownership guarantee as `install_wrapper_user_data`:
    // the magic proves a live `Box<UserDataBox>` installed by this module.
    drop(unsafe { Box::from_raw(previous.as_ptr() as *mut UserDataBox) });
    let _ =
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, std::ptr::null_mut()));
}
