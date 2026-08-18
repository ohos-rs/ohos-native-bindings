//! Native node `user_data` ownership for wrapper-created children.
//!
//! ArkUI's `setUserData`/`getUserData` pair stores an opaque pointer on a
//! native node. The wrapper layer uses it to route native events back to the
//! Rust `Rc<RefCell<ArkUINode>>` wrapper (see `api::node::receiver`). Mounting
//! installs a wrapper box; native handles are long-lived and can be
//! re-attached many times, so a naive overwrite leaks the previous box and a
//! `dispose` that ignores the pointer leaks it permanently.
//!
//! Ownership is encoded in the pointer itself instead of a global registry:
//! every pointer installed by this layer points at a [`UserDataBox`] whose
//! header carries a magic value plus the box's own address. Cleanup checks
//! both before reclaiming, so pointers owned by external code (ArkTS
//! `FrameNode` bridges) are recognized and left untouched. This keeps the hot
//! mount path lock-free and free of global state.
//!
//! Release ordering is a hard invariant: every box in a subtree must be
//! reclaimed **before** `disposeNode` destroys that subtree, because
//! `getUserData`/`setUserData` on a destroyed handle is a use-after-free at
//! the native layer, and boxes that survive the native teardown become
//! unreachable and leak the entire Rust-side wrapper tree.

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
/// `#[repr(C)]` so the header lives at a fixed offset: ownership checks read
/// the magic and the self-address without touching the rest of the box. A
/// foreign pointer would need to carry the magic *and* its own address at the
/// right offset to be misclassified, which cannot happen by accident.
#[repr(C)]
pub(crate) struct UserDataBox {
    magic: u32,
    /// The box's own heap address, written at install time. Second half of
    /// the classification check; rules out magic collisions in foreign data.
    this: *const UserDataBox,
    /// The wrapper routed to by native event callbacks. Read by
    /// `api::node::receiver`; nothing else touches the box's interior.
    pub(crate) wrapper: Rc<RefCell<ArkUINode>>,
}

/// Classify `pointer` and return it as a live wrapper box when it was
/// installed by this module.
///
/// # Safety
///
/// `pointer` must be a value previously stored in a live native node's
/// `user_data` slot. Classification reads the header words of the referenced
/// allocation; the self-address check makes a false positive on foreign data
/// practically impossible.
pub(crate) unsafe fn classify_wrapper_user_data<'a>(
    pointer: *mut c_void,
) -> Option<&'a UserDataBox> {
    if pointer.is_null() {
        return None;
    }
    let candidate = pointer as *const UserDataBox;
    // SAFETY: the caller guarantees `pointer` came from a live node's
    // user_data slot, so reading the header of that allocation is valid.
    let header_matches =
        unsafe { (*candidate).magic == USER_DATA_MAGIC && (*candidate).this == candidate };
    if !header_matches {
        return None;
    }
    // SAFETY: magic + self-address prove this module installed the box and it
    // has not been reclaimed (release nulls the slot before freeing).
    Some(unsafe { &*candidate })
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
    let mut boxed = Box::new(UserDataBox {
        magic: USER_DATA_MAGIC,
        this: std::ptr::null(),
        wrapper,
    });
    boxed.this = &*boxed as *const UserDataBox;
    let new_ptr = Box::into_raw(boxed);
    if let Err(error) =
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, new_ptr as *mut c_void))
    {
        // SAFETY: the pointer was created above and never handed to native
        // code, so reclaiming it here is the only ownership path.
        drop(unsafe { Box::from_raw(new_ptr) });
        return Err(error);
    }

    if let Some(previous) = previous {
        // SAFETY: the pointer came from this node's live user_data slot and
        // the node now references `new_ptr`, so a positively classified
        // previous box is exclusively ours to reclaim.
        if let Some(previous_box) = unsafe { classify_wrapper_user_data(previous.as_ptr()) } {
            drop(unsafe { Box::from_raw(previous_box as *const UserDataBox as *mut UserDataBox) });
        }
    }
    Ok(())
}

/// Release and null out a wrapper-owned `user_data` pointer, if any.
///
/// Must run while the native handle is still alive (before `disposeNode`).
/// External pointers are left untouched.
pub(crate) fn release_wrapper_user_data(node: &ArkUINode) {
    let Some(previous) = crate::ARK_UI_NATIVE_NODE_API_1
        .with(|api| api.get_user_data(node.raw_handle()))
        .ok()
        .flatten()
    else {
        return;
    };
    // SAFETY: the pointer came from this node's live user_data slot; a
    // positive classification proves a live `Box<UserDataBox>` installed by
    // this module. The slot is nulled before the box is reused elsewhere.
    let Some(previous_box) = (unsafe { classify_wrapper_user_data(previous.as_ptr()) }) else {
        return;
    };
    let _ =
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, std::ptr::null_mut()));
    // SAFETY: classified above; the native slot no longer references it.
    drop(unsafe { Box::from_raw(previous_box as *const UserDataBox as *mut UserDataBox) });
}

/// Release the wrapper-owned `user_data` box of `node` and of every
/// descendant wrapper reachable through the wrapper `children` lists.
///
/// `dispose` calls this before `disposeNode`: the native teardown is
/// recursive, so this is the last moment every descendant handle is valid.
/// A child wrapper that is currently borrowed elsewhere is skipped (its box
/// leaks) instead of panicking mid-teardown; that situation indicates a
/// reentrant dispose, which the single-threaded UI contract already forbids.
pub(crate) fn release_wrapper_user_data_recursive(node: &ArkUINode) {
    release_wrapper_user_data(node);
    for child in node.children() {
        if let Ok(child) = child.try_borrow() {
            release_wrapper_user_data_recursive(&child);
        }
    }
}
