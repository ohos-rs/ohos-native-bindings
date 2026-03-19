//! Module component::root wrappers and related types.

use std::os::raw::c_void;

use crate::{ArkUIError, ArkUIHandle, ArkUINode, ArkUIResult, ARK_UI_NATIVE_NODE_API_1};
use ohos_arkui_input_binding::ArkUIErrorCode;

/// root node for arkui   
/// ```rust
/// let mut root = RootNode::new();
///
/// ```
pub struct RootNode {
    base: Option<ArkUINode>,
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
        let node_raw = node.into();
        self.base = Some(node_raw.clone());
        if let Some(base) = self.base.as_ref() {
            ARK_UI_NATIVE_NODE_API_1.with(|api| {
                api.set_user_data(base, Box::into_raw(Box::new(base)) as *mut c_void)
            })?;

            // Node will be mounted, we can think it as a event receiver.
            ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_event_receiver(base))?;
            self.handle.add_node(base)
        } else {
            Err(ArkUIError::new(
                ArkUIErrorCode::ChildNodeExist,
                "Mount root node failed, base is None",
            ))
        }
    }

    pub fn unmount(&mut self) -> ArkUIResult<()> {
        // If root node is empty, just ignore it.
        if let Some(base) = self.base.as_mut() {
            self.handle.remove_node(base)?;
            base.dispose()?;
            self.base = None;
        }
        Ok(())
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        if let Some(base) = self.base.as_mut() {
            let _ = self.handle.remove_node(base);
            base.children_mut().clear();
            self.base = None;
        }
    }
}
