//! Identity registry used by native node-event dispatch.
//!
//! ArkUI node events already carry their originating node handle. Routing by
//! that handle avoids occupying the public native `user_data` slot and avoids
//! dereferencing opaque pointers owned by another layer. The registry stores
//! `Weak` references only, so event dispatch never becomes a second owner of a
//! node and cannot keep a detached subtree alive.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use ohos_arkui_input_binding::{sys::ArkUI_NodeHandle, ArkUIErrorCode};

use crate::common::node::ArkUINode;
use crate::{ArkUIError, ArkUIResult};

type NodeWrapper = Rc<RefCell<ArkUINode>>;

thread_local! {
    static NODE_EVENT_WRAPPERS: RefCell<HashMap<usize, Weak<RefCell<ArkUINode>>>> =
        RefCell::new(HashMap::new());
}

/// Register the single identity-stable wrapper for a native node handle.
pub(crate) fn register(node: &ArkUINode, wrapper: NodeWrapper) -> ArkUIResult<()> {
    let key = node.raw_handle() as usize;
    if key == 0 {
        return Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            "cannot register events for a null ArkUI node",
        ));
    }

    NODE_EVENT_WRAPPERS.with(|wrappers| {
        let mut wrappers = wrappers.borrow_mut();
        if let Some(existing) = wrappers.get(&key).and_then(Weak::upgrade) {
            if !Rc::ptr_eq(&existing, &wrapper) {
                return Err(ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "a different Rust wrapper is already registered for this ArkUI node",
                ));
            }
        }
        wrappers.insert(key, Rc::downgrade(&wrapper));
        Ok(())
    })
}

/// Remove event routing for a node that is about to be disposed.
pub(crate) fn unregister(node: &ArkUINode) {
    NODE_EVENT_WRAPPERS.with(|wrappers| {
        wrappers.borrow_mut().remove(&(node.raw_handle() as usize));
    });
}

/// Resolve an event's native handle to its live identity-stable wrapper.
pub(crate) fn resolve(handle: ArkUI_NodeHandle) -> Option<NodeWrapper> {
    let key = handle as usize;
    NODE_EVENT_WRAPPERS.with(|wrappers| {
        let mut wrappers = wrappers.borrow_mut();
        let wrapper = wrappers.get(&key).and_then(Weak::upgrade);
        if wrapper.is_none() {
            wrappers.remove(&key);
        }
        wrapper
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArkUINodeType, EventHandle};

    fn fake_node(id: usize) -> NodeWrapper {
        Rc::new(RefCell::new(ArkUINode {
            raw: id as ArkUI_NodeHandle,
            tag: ArkUINodeType::Custom,
            children: Vec::new(),
            event_handle: EventHandle::default(),
            owns_raw: true,
        }))
    }

    #[test]
    fn registry_is_non_owning_and_identity_stable() {
        let first = fake_node(41);
        register(&first.borrow(), first.clone()).expect("register first wrapper");
        assert!(Rc::ptr_eq(
            &resolve(41 as ArkUI_NodeHandle).unwrap(),
            &first
        ));

        let duplicate = fake_node(41);
        assert!(register(&duplicate.borrow(), duplicate.clone()).is_err());

        unregister(&first.borrow());
        assert!(resolve(41 as ArkUI_NodeHandle).is_none());
    }

    #[test]
    fn stale_entries_do_not_retain_wrappers() {
        let node = fake_node(42);
        register(&node.borrow(), node.clone()).expect("register wrapper");
        drop(node);
        assert!(resolve(42 as ArkUI_NodeHandle).is_none());
    }
}
