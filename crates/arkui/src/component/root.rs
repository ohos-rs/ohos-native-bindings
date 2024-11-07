use crate::{check_arkui_status, ArkUIError, ArkUIErrorCode, ArkUINode, ArkUIResult};
use ohos_arkui_sys::{OH_ArkUI_NodeContent_AddNode, OH_ArkUI_NodeContent_RemoveNode};

#[cfg(not(feature = "napi"))]
use ohos_arkui_sys::ArkUI_NodeContentHandle;

#[cfg(feature = "napi")]
use crate::ArkUIHandle;

/// root node for arkui   
/// ```rust
/// let mut root = RootNode::new();
///
/// ```
pub struct RootNode {
    base: Option<ArkUINode>,
    #[cfg(feature = "napi")]
    handle: ArkUIHandle,
    #[cfg(not(feature = "napi"))]
    raw: ArkUI_NodeContentHandle,
}

impl RootNode {
    #[cfg(feature = "napi")]
    pub fn new(handle: ArkUIHandle) -> Self {
        RootNode { base: None, handle }
    }

    #[cfg(not(feature = "napi"))]
    pub fn new(handle: ArkUI_NodeContentHandle) -> Self {
        RootNode {
            base: None,
            raw: handle,
        }
    }

    #[cfg(feature = "napi")]
    pub fn handle(&self) -> &ArkUIHandle {
        &self.handle
    }

    #[cfg(not(feature = "napi"))]
    pub fn handle(&self) -> &ArkUI_NodeContentHandle {
        &self.raw
    }

    pub fn mount<T: Into<ArkUINode>>(&mut self, node: T) -> ArkUIResult<()> {
        let node_raw = node.into();
        self.base = Some(node_raw.clone());
        if let Some(base) = self.base.as_ref() {
            #[cfg(feature = "napi")]
            let raw = self.handle.raw();

            #[cfg(not(feature = "napi"))]
            let raw = self.raw;
            unsafe {
                check_arkui_status!(
                    OH_ArkUI_NodeContent_AddNode(raw, base.raw()),
                    "Mount root node failed"
                )
            }
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
            #[cfg(feature = "napi")]
            let raw = self.handle.raw();

            #[cfg(not(feature = "napi"))]
            let raw = self.raw;
            unsafe {
                check_arkui_status!(
                    OH_ArkUI_NodeContent_RemoveNode(raw, base.raw()),
                    "Mount root node failed"
                )
            }?;
            base.dispose()?;
            self.base = None;
        }
        Ok(())
    }
}

impl Drop for RootNode {
    fn drop(&mut self) {
        if let Some(base) = self.base.as_mut() {
            #[cfg(feature = "napi")]
            let raw = self.handle.raw();

            #[cfg(not(feature = "napi"))]
            let raw = self.raw;
            unsafe { OH_ArkUI_NodeContent_RemoveNode(raw, base.raw()) };
            base.children_mut().clear();
            self.base = None;
        }
    }
}
