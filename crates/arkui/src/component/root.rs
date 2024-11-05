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

    pub fn set_node(&mut self, node: ArkUINode) {
        self.base = Some(node);
    }

    pub fn handle(&self) -> &ArkUIHandle {
        &self.handle
    }

    pub fn mount(&self) -> Result<()> {
        if let Some(base) = self.base.as_ref() {
            let ret = unsafe {
                OH_ArkUI_NodeContent_AddNode(self.handle.raw(), base.raw())
            };
            // unsafe {
            //     check_status!(
            //         OH_ArkUI_NodeContent_AddNode(self.handle.raw(), base.raw()),
            //         "Mount root node failed"
            //     )
            // }?;
            Ok(())
        } else {
            return Err(Error::from_reason("root node is empty, can't be mounted"));
        }
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        if let Some(base) = self.base.as_mut() {
            unsafe { OH_ArkUI_NodeContent_RemoveNode(self.handle.raw(), base.raw()) };
            base.children_mut().clear();
        }
    }
}

unsafe impl Send for RootNode {}
unsafe impl Sync for RootNode {}