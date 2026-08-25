//! Module component::root wrappers and related types.

use std::cell::RefCell;
use std::rc::Rc;

use crate::{ArkUIError, ArkUIHandle, ArkUINode, ArkUIResult, ARK_UI_NATIVE_NODE_API_1};
use ohos_arkui_input_binding::ArkUIErrorCode;

/// root node for arkui
/// ```rust
/// let mut root = RootNode::new();
///
/// ```
pub struct RootNode {
    base: Option<Rc<RefCell<ArkUINode>>>,
    handle: ArkUIHandle,
}

impl RootNode {
    pub fn new(handle: ArkUIHandle) -> Self {
        RootNode { base: None, handle }
    }

    pub fn handle(&self) -> &ArkUIHandle {
        &self.handle
    }

    pub fn mount<T: Into<ArkUINode>>(&mut self, node: T) -> ArkUIResult<()> {
        if self.base.is_some() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ChildNodeExist,
                "Mount root node failed, a base node is already mounted",
            ));
        }
        let base = Rc::new(RefCell::new(node.into()));
        // Same event-dispatch contract as every wrapper-mounted child: the
        // receiver resolves callbacks through the wrapper box installed here,
        // so events registered on the root wrapper actually fire.
        crate::common::user_data::install_wrapper_user_data(&base.borrow(), base.clone())?;
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_event_receiver(&base.borrow()))?;
        self.handle.add_node(&base.borrow())?;
        self.base = Some(base);
        Ok(())
    }

    pub fn unmount(&mut self) -> ArkUIResult<()> {
        // If root node is empty, just ignore it.
        if let Some(base) = self.base.as_ref() {
            self.handle.remove_node(&base.borrow())?;
            base.borrow_mut().dispose()?;
            self.base = None;
        }
        Ok(())
    }

    /// Drop this root without touching the native slot.
    ///
    /// Use when the underlying `NodeContent` (or its subtree) has already been
    /// destroyed outside this wrapper: `remove_node` on a dead handle would be
    /// a use-after-free, and so would reclaiming the event-dispatch user data
    /// through it. Rust-side state is released; the native handle and the
    /// wrapper box installed at mount are deliberately abandoned.
    pub fn into_inert(mut self) {
        self.base = None;
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        if let Some(base) = self.base.take() {
            let _ = self.handle.remove_node(&base.borrow());
            // The native node stays alive (this drop path deliberately does
            // not dispose it), so stop event dispatch first and only then
            // reclaim the wrapper box it references.
            let _ = ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(&base.borrow()));
            crate::common::user_data::release_wrapper_user_data(&base.borrow());
            base.borrow_mut().children_mut().clear();
        }
    }
}
