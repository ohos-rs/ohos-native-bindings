//! Native node `user_data` ownership for wrapper-created children.
//!
//! ArkUI's `setUserData`/`getUserData` pair stores an opaque pointer on a
//! native node. The wrapper layer uses it to route native events back to the
//! Rust `Rc<RefCell<ArkUINode>>` wrapper (see `api::node::receiver`). Every
//! `add_child`/`insert_child` call installs a fresh `Box<Rc<...>>`; native
//! handles are long-lived and can be re-attached many times, so a naive
//! overwrite leaks the previous box and a `dispose` that ignores the pointer
//! leaks it permanently.
//!
//! Ownership is tracked here by pointer address so cleanup can release only
//! pointers this layer actually installed. External owners (ArkTS `FrameNode`
//! bridges, `RootNode`'s base) never enter the registry and are never
//! dereferenced as wrapper types.

use std::cell::RefCell;
use std::collections::HashSet;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::{LazyLock, Mutex};

use crate::common::node::ArkUINode;
use crate::ArkUIResult;

static WRAPPER_USER_DATA: LazyLock<Mutex<HashSet<usize>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

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
    let new_ptr = Box::into_raw(Box::new(wrapper)) as *mut c_void;
    crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, new_ptr))?;

    let mut owned = WRAPPER_USER_DATA
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(previous) = previous {
        if owned.remove(&(previous.as_ptr() as usize)) {
            // SAFETY: the pointer was installed by this module's
            // `install_wrapper_user_data` (it is in the ownership registry),
            // so it is a `Box<Rc<RefCell<ArkUINode>>>` and is still live.
            drop(unsafe { Box::from_raw(previous.as_ptr() as *mut Rc<RefCell<ArkUINode>>) });
        }
    }
    owned.insert(new_ptr as usize);
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
    let address = previous.as_ptr() as usize;
    if !WRAPPER_USER_DATA
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .remove(&address)
    {
        return;
    }
    // SAFETY: same ownership guarantee as `install_wrapper_user_data`:
    // the pointer is registered here, hence a live `Box<Rc<...>>`.
    drop(unsafe { Box::from_raw(previous.as_ptr() as *mut Rc<RefCell<ArkUINode>>) });
    let _ =
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_user_data(node, std::ptr::null_mut()));
}
