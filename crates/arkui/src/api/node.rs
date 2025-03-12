use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;
use std::{cell::LazyCell, ptr::NonNull};

use ohos_arkui_sys::{
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE, ArkUI_NativeNodeAPI_1, ArkUI_NodeEvent,
    ArkUI_NodeEventType, ArkUI_NodeHandle, OH_ArkUI_NodeEvent_GetEventType,
    OH_ArkUI_NodeEvent_GetNodeHandle, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{ArkUINodeAttributeType, ArkUINodeType, NodeEventType};

use crate::common::{ArkUIError, ArkUINode, ArkUINodeAttributeItem};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_NODE_API_1: LazyCell<ArkUINativeNodeAPI1> = LazyCell::new(|| {
    let api = ArkUINativeNodeAPI1::new();
    api
});

pub struct ArkUINativeNodeAPI1 {
    pub(crate) raw: NonNull<ArkUI_NativeNodeAPI_1>,
}

impl ArkUINativeNodeAPI1 {
    /// allow us to get the pointer of ArkUI_NativeNodeAPI_1 and use it directly
    pub fn raw(&self) -> NonNull<ArkUI_NativeNodeAPI_1> {
        self.raw
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeNodeAPI_1 = std::ptr::null_mut();
        let struct_name = c"ArkUI_NativeNodeAPI_1";
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeNodeAPI_1 is NULL");
        api = raw_ptr.cast();
        Self {
            raw: unsafe { NonNull::new_unchecked(api) },
        }
    }

    pub fn create_node(&self, node_type: ArkUINodeType) -> Result<ArkUI_NodeHandle, ArkUIError> {
        unsafe {
            if let Some(create_node) = (*self.raw.as_ptr()).createNode {
                let handle = create_node(node_type.into());
                if handle.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeNodeAPI1::createNode",
                    )))
                } else {
                    Ok(handle)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::createNode is None",
                )))
            }
        }
    }

    pub fn set_attribute(
        &self,
        node: &ArkUINode,
        attr: ArkUINodeAttributeType,
        value: ArkUINodeAttributeItem,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_attribute) = (*self.raw.as_ptr()).setAttribute {
                let ret = set_attribute(node.raw(), attr.into(), &value.into());
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::setAttribute, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::setAttribute is None",
                )))
            }
        }
    }

    pub fn add_child(&self, parent: &ArkUINode, child: &ArkUINode) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(add_child) = (*self.raw.as_ptr()).addChild {
                let ret = add_child(parent.raw(), child.raw());
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::addChild, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::addChild is None",
                )))
            }
        }
    }

    pub fn remove_child(&self, parent: &ArkUINode, child: &ArkUINode) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(remove_child) = (*self.raw.as_ptr()).removeChild {
                let ret = remove_child(parent.raw(), child.raw());
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::removeChild, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::removeChild is None",
                )))
            }
        }
    }

    pub fn insert_child(
        &self,
        parent: &ArkUINode,
        child: &ArkUINode,
        index: i32,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(insert_child_at) = (*self.raw.as_ptr()).insertChildAt {
                let ret = insert_child_at(parent.raw(), child.raw(), index);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::insertChildAt, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::insertChild is None",
                )))
            }
        }
    }

    pub fn dispose(&self, node: &ArkUINode) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(dispose_node) = (*self.raw.as_ptr()).disposeNode {
                dispose_node(node.raw());
                Ok(())
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::disposeNode is None",
                )))
            }
        }
    }

    pub fn register_node_event(
        &self,
        node: &ArkUINode,
        event_type: NodeEventType,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(register_node_event) = (*self.raw.as_ptr()).registerNodeEvent {
                let t: ArkUI_NodeEventType = event_type.into();
                let ret = register_node_event(node.raw(), t, 0, std::ptr::null_mut());
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::registerNodeEvent, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::registerNodeEvent is None",
                )))
            }
        }
    }

    pub fn add_event_receiver(&self, node: &ArkUINode) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(add_node_event_receiver) = (*self.raw.as_ptr()).addNodeEventReceiver {
                let ret = add_node_event_receiver(node.raw(), Some(node_event_receiver));
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::addNodeEventReceiver, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::addNodeEventReceiver is None",
                )))
            }
        }
    }

    pub fn remove_event_receiver(&self, node: &ArkUINode) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(remove_node_event_receiver) = (*self.raw.as_ptr()).removeNodeEventReceiver {
                let ret = remove_node_event_receiver(node.raw(), Some(node_event_receiver));
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::removeNodeEventReceiver, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::removeNodeEventReceiver is None",
                )))
            }
        }
    }

    pub fn set_user_data(
        &self,
        node: &ArkUINode,
        user_data: *mut c_void,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_user_data) = (*self.raw.as_ptr()).setUserData {
                let ret = set_user_data(node.raw(), user_data);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeNodeAPI1::setUserData, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::setUserData is None",
                )))
            }
        }
    }

    pub fn get_user_data(&self, node_handle: ArkUI_NodeHandle) -> Result<*mut c_void, ArkUIError> {
        unsafe {
            if let Some(get_user_data) = (*self.raw.as_ptr()).getUserData {
                Ok(get_user_data(node_handle))
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeNodeAPI_1::setUserData is None",
                )))
            }
        }
    }
}

unsafe extern "C" fn node_event_receiver(event: *mut ArkUI_NodeEvent) {
    let handle = OH_ArkUI_NodeEvent_GetNodeHandle(event);
    let user_data = ARK_UI_NATIVE_NODE_API_1.get_user_data(handle).unwrap();

    #[cfg(debug_assertions)]
    assert!(!user_data.is_null(), "user_data is null");

    let user_data_rc: &Rc<RefCell<ArkUINode>> = &*(user_data as *const Rc<RefCell<ArkUINode>>);

    let node = user_data_rc.borrow();

    let event_type = OH_ArkUI_NodeEvent_GetEventType(event);
    let event_type = NodeEventType::from(event_type);
    match event_type {
        NodeEventType::OnClick => {
            if let Some(cb) = node.event_handle.click.as_ref() {
                cb.borrow()();
            }
        }
        _ => {}
    }
}
