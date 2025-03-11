use std::os::raw::c_void;

use crate::{ArkUIError, ArkUINode, ARK_UI_NATIVE_NODE_API_1};
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

    pub fn mount<T: Into<ArkUINode>>(&mut self, node: T) -> Result<(), ArkUIError> {
        let node_raw = node.into();
        self.base = Some(node_raw.clone());
        if let Some(base) = self.base.as_ref() {
            #[cfg(feature = "napi")]
            let raw = self.handle.raw();

            #[cfg(not(feature = "napi"))]
            let raw = self.raw;

            ARK_UI_NATIVE_NODE_API_1
                .set_user_data(base, Box::into_raw(Box::new(base)) as *mut c_void)?;

            // Node will be mounted, we can think it as a event receiver.
            ARK_UI_NATIVE_NODE_API_1.add_event_receiver(base)?;
            unsafe {
                let ret = OH_ArkUI_NodeContent_AddNode(raw, base.raw());
                if ret != 0 {
                    Err(ArkUIError::InternalError(String::from(
                        "Mount root node failed",
                    )))
                } else {
                    Ok(())
                }
            }
        } else {
            Err(ArkUIError::InternalError(String::from(
                "Mount root node failed, base is None",
            )))
        }
    }

    pub fn unmount(&mut self) -> Result<(), ArkUIError> {
        // If root node is empty, just ignore it.
        if let Some(base) = self.base.as_mut() {
            #[cfg(feature = "napi")]
            let raw = self.handle.raw();

            #[cfg(not(feature = "napi"))]
            let raw = self.raw;
            unsafe {
                let ret = OH_ArkUI_NodeContent_RemoveNode(raw, base.raw());
                if ret != 0 {
                    return Err(ArkUIError::InternalError(String::from(
                        "Unmount root node failed",
                    )));
                }
            }
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
