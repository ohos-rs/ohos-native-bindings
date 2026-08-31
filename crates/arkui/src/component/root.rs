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
        self.mount_node(Rc::new(RefCell::new(node.into())))
    }

    /// Mount an identity-stable wrapper as the NodeContent root.
    ///
    /// Renderers must use this overload so the root owned by `RootNode` is the
    /// same wrapper whose child list they mutate. Cloning `ArkUINode` here
    /// would split ownership bookkeeping and make descendant teardown
    /// incomplete.
    pub fn mount_node(&mut self, base: Rc<RefCell<ArkUINode>>) -> ArkUIResult<()> {
        if self.base.is_some() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ChildNodeExist,
                "Mount root node failed, a base node is already mounted",
            ));
        }
        if !base.borrow().owns_raw {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Mount root node failed, the node handle is borrowed",
            ));
        }
        // Same event-dispatch contract as every wrapper-mounted child: the
        // receiver resolves callbacks through this identity-stable wrapper.
        crate::common::node_registry::register(&base.borrow(), base.clone())?;
        if let Err(error) =
            ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_event_receiver(&base.borrow()))
        {
            crate::common::node_registry::unregister(&base.borrow());
            return Err(error);
        }
        if let Err(error) = self.handle.add_node(&base.borrow()) {
            let _ = ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(&base.borrow()));
            crate::common::node_registry::unregister(&base.borrow());
            return Err(error);
        }
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
    /// event registry entry becomes a harmless stale `Weak` and is pruned on
    /// lookup or handle reuse.
    pub fn into_inert(mut self) {
        self.base = None;
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        let _ = self.unmount();
    }
}
