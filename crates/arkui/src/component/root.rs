use napi_ohos::{bindgen_prelude::check_status, Error, Result};

use crate::{ArkUIHandle, ArkUINode};
use ohos_arkui_sys::{OH_ArkUI_NodeContent_AddNode, OH_ArkUI_NodeContent_RemoveNode};

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

    pub fn mount<T: Into<ArkUINode>>(&mut self, node: T) -> Result<()> {
        let node_raw = node.into();
        self.base = Some(node_raw.clone());
        if let Some(base) = self.base.as_ref() {
            unsafe {
                check_status!(
                    OH_ArkUI_NodeContent_AddNode(self.handle.raw(), base.raw()),
                    "Mount root node failed"
                )
            }?;
            Ok(())
        } else {
            Err(Error::from_reason("Mount root node failed, base is None"))
        }
    }

    pub fn unmount(&mut self) -> Result<()> {
        // If root node is empty, just ignore it.
        if let Some(base) = self.base.as_mut() {
            let ret = unsafe { OH_ArkUI_NodeContent_RemoveNode(self.handle.raw(), base.raw()) };

            base.dispose();
            // unsafe {
            //     check_status!(
            //         OH_ArkUI_NodeContent_RemoveNode(self.handle.raw(), base.raw()),
            //         "Mount root node failed"
            //     )
            // }?;
            self.base = None;
        }
        Ok(())
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        if let Some(base) = self.base.as_mut() {
            unsafe { OH_ArkUI_NodeContent_RemoveNode(self.handle.raw(), base.raw()) };
            base.children_mut().clear();
            self.base = None;
        }
    }
}
