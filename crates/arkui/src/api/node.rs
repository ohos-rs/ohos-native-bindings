use std::cell::RefCell;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::{Mutex, OnceLock};
use std::{cell::LazyCell, ffi::CString};

use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_IntOffset, ArkUI_IntSize, ArkUI_LayoutConstraint,
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE, ArkUI_NativeNodeAPI_1, ArkUI_NodeCustomEvent,
    ArkUI_NodeEvent, ArkUI_NodeEventType, OH_ArkUI_NodeEvent_GetEventType,
    OH_ArkUI_NodeEvent_GetNodeHandle, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{check_arkui_status, ArkUINodeAttributeType, ArkUINodeType, Event, NodeEventType};

use crate::common::{ArkUIError, ArkUINode, ArkUINodeAttributeItem, ArkUIResult};

thread_local! {
    /// ArkUI_NativeNodeAPI_1 struct
    /// Only can be used in main thread
    pub(crate) static ARK_UI_NATIVE_NODE_API_1: LazyCell<ArkUINativeNodeAPI1> =
    LazyCell::new(ArkUINativeNodeAPI1::new);
}

struct NodeCustomEventCallbackContext {
    callback: Box<dyn Fn(&crate::NodeCustomEvent)>,
}

type NodeCustomEventCallbackMap = HashMap<(usize, usize), usize>;

static NODE_CUSTOM_EVENT_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeCustomEventCallbackMap>> =
    OnceLock::new();

fn node_custom_event_callback_contexts() -> &'static Mutex<NodeCustomEventCallbackMap> {
    NODE_CUSTOM_EVENT_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn node_custom_event_map_key(
    node_handle: ArkUI_NodeHandle,
    event_type: crate::NodeCustomEventType,
) -> (usize, usize) {
    let raw_event_type: ohos_arkui_sys::ArkUI_NodeCustomEventType = event_type.into();
    (node_handle as usize, raw_event_type as usize)
}

pub(crate) struct ArkUINativeNodeAPI1(pub(crate) NonNull<ArkUI_NativeNodeAPI_1>);

impl ArkUINativeNodeAPI1 {
    /// allow us to get the pointer of ArkUI_NativeNodeAPI_1 and use it directly
    pub(crate) fn raw(&self) -> *mut ArkUI_NativeNodeAPI_1 {
        self.0.as_ptr()
    }

    pub(crate) fn new() -> Self {
        let struct_name = CString::new("ArkUI_NativeNodeAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE,
                struct_name.as_ptr().cast(),
            )
        };
        let api =
            NonNull::new(raw_ptr.cast()).unwrap_or_else(|| panic!("ArkUI_NativeNodeAPI_1 is NULL"));
        Self(api)
    }

    pub(crate) fn create_node(&self, node_type: ArkUINodeType) -> ArkUIResult<ArkUI_NodeHandle> {
        unsafe {
            if let Some(create_node) = (*self.raw()).createNode {
                let handle = create_node(node_type.into());
                if handle.is_null() {
                    return Err(ArkUIError::new(
                        ArkUIErrorCode::ArkTSNodeNotSupported,
                        "Create node failed",
                    ));
                }
                Ok(handle)
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::createNode is None",
                ))
            }
        }
    }

    pub(crate) fn set_attribute(
        &self,
        node: &ArkUINode,
        attr: ArkUINodeAttributeType,
        value: ArkUINodeAttributeItem,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_attribute) = (*self.raw()).setAttribute {
                check_arkui_status!(set_attribute(node.raw(), attr.into(), &value.into()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setAttribute is None",
                ))
            }
        }
    }

    pub(crate) fn get_attribute(
        &self,
        node: &ArkUINode,
        attr: ArkUINodeAttributeType,
    ) -> ArkUIResult<ArkUINodeAttributeItem> {
        unsafe {
            (*self.raw())
                .getAttribute
                .map(|get_attribute| get_attribute(node.raw(), attr.into()))
                .and_then(|attr_ptr| ArkUINodeAttributeItem::try_from(attr_ptr).ok())
                .map(ArkUIResult::Ok)
                .unwrap_or_else(|| {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUI_NativeNodeAPI_1::getAttribute failed to get or convert attribute",
                    ))
                })
        }
    }

    pub(crate) fn add_child(&self, parent: &ArkUINode, child: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_child) = (*self.raw()).addChild {
                check_arkui_status!(add_child(parent.raw(), child.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::addChild is None",
                ))
            }
        }
    }

    pub(crate) fn remove_child(&self, parent: &ArkUINode, child: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_child) = (*self.raw()).removeChild {
                check_arkui_status!(remove_child(parent.raw(), child.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeChild is None",
                ))
            }
        }
    }

    pub(crate) fn insert_child(
        &self,
        parent: &ArkUINode,
        child: &ArkUINode,
        index: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(insert_child_at) = (*self.raw()).insertChildAt {
                check_arkui_status!(insert_child_at(parent.raw(), child.raw(), index))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::insertChild is None",
                ))
            }
        }
    }

    pub(crate) fn insert_child_after(
        &self,
        parent: &ArkUINode,
        child: &ArkUINode,
        sibling: Option<&ArkUINode>,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(insert_child_after) = (*self.raw()).insertChildAfter {
                let sibling_handle = sibling.map_or(std::ptr::null_mut(), |node| node.raw());
                check_arkui_status!(insert_child_after(
                    parent.raw(),
                    child.raw(),
                    sibling_handle
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::insertChildAfter is None",
                ))
            }
        }
    }

    pub(crate) fn insert_child_before(
        &self,
        parent: &ArkUINode,
        child: &ArkUINode,
        sibling: Option<&ArkUINode>,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(insert_child_before) = (*self.raw()).insertChildBefore {
                let sibling_handle = sibling.map_or(std::ptr::null_mut(), |node| node.raw());
                check_arkui_status!(insert_child_before(
                    parent.raw(),
                    child.raw(),
                    sibling_handle
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::insertChildBefore is None",
                ))
            }
        }
    }

    pub(crate) fn dispose(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.clear_node_custom_event_callbacks_for_node(node.raw());
        let _ = self.remove_node_custom_event_receiver(node);
        unsafe {
            if let Some(dispose_node) = (*self.raw()).disposeNode {
                dispose_node(node.raw());
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::disposeNode is None",
                ))
            }
        }
    }

    pub(crate) fn register_node_event(
        &self,
        node: &ArkUINode,
        event_type: NodeEventType,
    ) -> ArkUIResult<()> {
        self.register_node_event_with_data(node, event_type, 0, std::ptr::null_mut())
    }

    pub(crate) fn register_node_event_with_data(
        &self,
        node: &ArkUINode,
        event_type: NodeEventType,
        target_id: i32,
        user_data: *mut c_void,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_event) = (*self.raw()).registerNodeEvent {
                let t: ArkUI_NodeEventType = event_type.into();
                check_arkui_status!(register_node_event(node.raw(), t, target_id, user_data))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeEvent is None",
                ))
            }
        }
    }

    pub(crate) fn unregister_node_event(
        &self,
        node: &ArkUINode,
        event_type: NodeEventType,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(unregister_node_event) = (*self.raw()).unregisterNodeEvent {
                let t: ArkUI_NodeEventType = event_type.into();
                unregister_node_event(node.raw(), t);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::unregisterNodeEvent is None",
                ))
            }
        }
    }

    pub(crate) fn add_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.add_node_event_receiver(node)
    }

    pub(crate) fn remove_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.remove_node_event_receiver(node)
    }

    fn add_node_event_receiver_raw(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_node_event_receiver) = (*self.raw()).addNodeEventReceiver {
                check_arkui_status!(add_node_event_receiver(
                    node.raw(),
                    Some(node_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::addNodeEventReceiver is None",
                ))
            }
        }
    }

    fn remove_node_event_receiver_raw(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_node_event_receiver) = (*self.raw()).removeNodeEventReceiver {
                check_arkui_status!(remove_node_event_receiver(
                    node.raw(),
                    Some(node_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeNodeEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn add_node_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.add_node_event_receiver_raw(node)
    }

    pub(crate) fn remove_node_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.remove_node_event_receiver_raw(node)
    }

    fn register_node_event_receiver_raw(&self) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_event_receiver) = (*self.raw()).registerNodeEventReceiver {
                register_node_event_receiver(Some(node_event_receiver));
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeEventReceiver is None",
                ))
            }
        }
    }

    fn clear_node_event_receiver_raw(&self) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_event_receiver) = (*self.raw()).registerNodeEventReceiver {
                register_node_event_receiver(None);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn register_node_event_receiver(&self) -> ArkUIResult<()> {
        self.register_node_event_receiver_raw()
    }

    pub(crate) fn unregister_node_event_receiver(&self) -> ArkUIResult<()> {
        self.clear_node_event_receiver_raw()?;
        unsafe {
            if let Some(unregister_node_event_receiver) = (*self.raw()).unregisterNodeEventReceiver
            {
                unregister_node_event_receiver();
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::unregisterNodeEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn reset_attribute(
        &self,
        node: &ArkUINode,
        attr: ArkUINodeAttributeType,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(reset_attribute) = (*self.raw()).resetAttribute {
                check_arkui_status!(reset_attribute(node.raw(), attr.into()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::resetAttribute is None",
                ))
            }
        }
    }

    pub(crate) fn mark_dirty(
        &self,
        node: &ArkUINode,
        dirty_flag: crate::NodeDirtyFlag,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(mark_dirty) = (*self.raw()).markDirty {
                mark_dirty(node.raw(), dirty_flag.into());
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::markDirty is None",
                ))
            }
        }
    }

    pub(crate) fn get_total_child_count(&self, node: &ArkUINode) -> ArkUIResult<u32> {
        unsafe {
            if let Some(get_total_child_count) = (*self.raw()).getTotalChildCount {
                Ok(get_total_child_count(node.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getTotalChildCount is None",
                ))
            }
        }
    }

    pub(crate) fn get_child_at(
        &self,
        node: &ArkUINode,
        position: i32,
    ) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_child_at) = (*self.raw()).getChildAt {
                let child = get_child_at(node.raw(), position);
                if child.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(child))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getChildAt is None",
                ))
            }
        }
    }

    pub(crate) fn get_first_child(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_first_child) = (*self.raw()).getFirstChild {
                let child = get_first_child(node.raw());
                if child.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(child))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getFirstChild is None",
                ))
            }
        }
    }

    pub(crate) fn get_last_child(&self, node: &ArkUINode) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_last_child) = (*self.raw()).getLastChild {
                let child = get_last_child(node.raw());
                if child.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(child))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getLastChild is None",
                ))
            }
        }
    }

    pub(crate) fn get_previous_sibling(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_previous_sibling) = (*self.raw()).getPreviousSibling {
                let sibling = get_previous_sibling(node.raw());
                if sibling.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(sibling))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getPreviousSibling is None",
                ))
            }
        }
    }

    pub(crate) fn get_next_sibling(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_next_sibling) = (*self.raw()).getNextSibling {
                let sibling = get_next_sibling(node.raw());
                if sibling.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(sibling))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getNextSibling is None",
                ))
            }
        }
    }

    pub(crate) fn register_node_custom_event(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeCustomEventType,
        target_id: i32,
        user_data: *mut c_void,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_custom_event) = (*self.raw()).registerNodeCustomEvent {
                check_arkui_status!(register_node_custom_event(
                    node.raw(),
                    event_type.into(),
                    target_id,
                    user_data
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeCustomEvent is None",
                ))
            }
        }
    }

    pub(crate) fn unregister_node_custom_event(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeCustomEventType,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(unregister_node_custom_event) = (*self.raw()).unregisterNodeCustomEvent {
                unregister_node_custom_event(node.raw(), event_type.into());
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::unregisterNodeCustomEvent is None",
                ))
            }
        }
    }

    pub(crate) fn register_node_custom_event_callback<T: Fn(&crate::NodeCustomEvent) + 'static>(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeCustomEventType,
        target_id: i32,
        callback: T,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(NodeCustomEventCallbackContext {
            callback: Box::new(callback),
        }));
        let result = self.register_node_custom_event(node, event_type, target_id, callback.cast());
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        if let Err(err) = self.add_node_custom_event_receiver(node) {
            let _ = self.unregister_node_custom_event(node, event_type);
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        if let Err(err) = self.register_node_custom_event_receiver() {
            let _ = self.remove_node_custom_event_receiver(node);
            let _ = self.unregister_node_custom_event(node, event_type);
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match node_custom_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        let key = node_custom_event_map_key(node.raw(), event_type);
        if let Some(old) = callbacks.insert(key, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut NodeCustomEventCallbackContext));
            }
        }
        Ok(())
    }

    pub(crate) fn unregister_node_custom_event_callback(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeCustomEventType,
    ) -> ArkUIResult<()> {
        self.unregister_node_custom_event(node, event_type)?;
        self.clear_node_custom_event_callback(node.raw(), event_type);
        Ok(())
    }

    fn clear_node_custom_event_callback(
        &self,
        node_handle: ArkUI_NodeHandle,
        event_type: crate::NodeCustomEventType,
    ) {
        let mut callbacks = match node_custom_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        let key = node_custom_event_map_key(node_handle, event_type);
        if let Some(callback) = callbacks.remove(&key) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut NodeCustomEventCallbackContext,
                ));
            }
        }
    }

    fn clear_node_custom_event_callbacks_for_node(&self, node_handle: ArkUI_NodeHandle) {
        let mut callbacks = match node_custom_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut keys = Vec::new();
        for key in callbacks.keys() {
            if key.0 == node_handle as usize {
                keys.push(*key);
            }
        }
        for key in keys {
            if let Some(callback) = callbacks.remove(&key) {
                unsafe {
                    drop(Box::from_raw(
                        callback as *mut NodeCustomEventCallbackContext,
                    ));
                }
            }
        }
    }

    fn register_node_custom_event_receiver_raw(&self) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_custom_event_receiver) =
                (*self.raw()).registerNodeCustomEventReceiver
            {
                register_node_custom_event_receiver(Some(node_custom_event_receiver));
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeCustomEventReceiver is None",
                ))
            }
        }
    }

    fn clear_node_custom_event_receiver_raw(&self) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_custom_event_receiver) =
                (*self.raw()).registerNodeCustomEventReceiver
            {
                register_node_custom_event_receiver(None);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeCustomEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn register_node_custom_event_receiver(&self) -> ArkUIResult<()> {
        self.register_node_custom_event_receiver_raw()
    }

    pub(crate) fn unregister_node_custom_event_receiver(&self) -> ArkUIResult<()> {
        self.clear_node_custom_event_receiver_raw()?;
        unsafe {
            if let Some(unregister_node_custom_event_receiver) =
                (*self.raw()).unregisterNodeCustomEventReceiver
            {
                unregister_node_custom_event_receiver();
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::unregisterNodeCustomEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn set_measured_size(
        &self,
        node: &ArkUINode,
        width: i32,
        height: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_measured_size) = (*self.raw()).setMeasuredSize {
                check_arkui_status!(set_measured_size(node.raw(), width, height))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setMeasuredSize is None",
                ))
            }
        }
    }

    pub(crate) fn set_layout_position(
        &self,
        node: &ArkUINode,
        position_x: i32,
        position_y: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_layout_position) = (*self.raw()).setLayoutPosition {
                check_arkui_status!(set_layout_position(node.raw(), position_x, position_y))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setLayoutPosition is None",
                ))
            }
        }
    }

    pub(crate) fn get_measured_size(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_IntSize> {
        unsafe {
            if let Some(get_measured_size) = (*self.raw()).getMeasuredSize {
                Ok(get_measured_size(node.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getMeasuredSize is None",
                ))
            }
        }
    }

    pub(crate) fn get_layout_position(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_IntOffset> {
        unsafe {
            if let Some(get_layout_position) = (*self.raw()).getLayoutPosition {
                Ok(get_layout_position(node.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getLayoutPosition is None",
                ))
            }
        }
    }

    pub(crate) fn measure_node(
        &self,
        node: &ArkUINode,
        constraint: &mut ArkUI_LayoutConstraint,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(measure_node) = (*self.raw()).measureNode {
                check_arkui_status!(measure_node(
                    node.raw(),
                    constraint as *mut ArkUI_LayoutConstraint
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::measureNode is None",
                ))
            }
        }
    }

    pub(crate) fn layout_node(
        &self,
        node: &ArkUINode,
        position_x: i32,
        position_y: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(layout_node) = (*self.raw()).layoutNode {
                check_arkui_status!(layout_node(node.raw(), position_x, position_y))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::layoutNode is None",
                ))
            }
        }
    }

    fn add_node_custom_event_receiver_raw(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_node_custom_event_receiver) = (*self.raw()).addNodeCustomEventReceiver {
                check_arkui_status!(add_node_custom_event_receiver(
                    node.raw(),
                    Some(node_custom_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::addNodeCustomEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn add_node_custom_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.add_node_custom_event_receiver_raw(node)
    }

    fn remove_node_custom_event_receiver_raw(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_node_custom_event_receiver) =
                (*self.raw()).removeNodeCustomEventReceiver
            {
                check_arkui_status!(remove_node_custom_event_receiver(
                    node.raw(),
                    Some(node_custom_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeNodeCustomEventReceiver is None",
                ))
            }
        }
    }

    pub(crate) fn remove_node_custom_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.remove_node_custom_event_receiver_raw(node)
    }

    pub(crate) fn set_length_metric_unit(
        &self,
        node: &ArkUINode,
        unit: crate::LengthMetricUnit,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_length_metric_unit) = (*self.raw()).setLengthMetricUnit {
                check_arkui_status!(set_length_metric_unit(node.raw(), unit.into()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setLengthMetricUnit is None",
                ))
            }
        }
    }

    pub(crate) fn get_parent(&self, node: &ArkUINode) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
        unsafe {
            if let Some(get_parent) = (*self.raw()).getParent {
                let parent = get_parent(node.raw());
                if parent.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(parent))
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getParent is None",
                ))
            }
        }
    }

    pub(crate) fn remove_all_children(&self, parent: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_all_children) = (*self.raw()).removeAllChildren {
                check_arkui_status!(remove_all_children(parent.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeAllChildren is None",
                ))
            }
        }
    }

    pub(crate) fn set_user_data(
        &self,
        node: &ArkUINode,
        user_data: *mut c_void,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_user_data) = (*self.raw()).setUserData {
                check_arkui_status!(set_user_data(node.raw(), user_data))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::getUserData is None",
                ))
            }
        }
    }

    pub(crate) fn get_user_data(
        &self,
        node_handle: ArkUI_NodeHandle,
    ) -> ArkUIResult<Option<NonNull<c_void>>> {
        unsafe {
            if let Some(get_user_data) = (*self.raw()).getUserData {
                Ok(NonNull::new(get_user_data(node_handle)))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setUserData is None",
                ))
            }
        }
    }
}

impl Default for ArkUINativeNodeAPI1 {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "C" fn node_event_receiver(event: *mut ArkUI_NodeEvent) {
    if event.is_null() {
        return;
    }
    let handle = OH_ArkUI_NodeEvent_GetNodeHandle(event);
    let Some(user_data) = ARK_UI_NATIVE_NODE_API_1
        .with(|api| api.get_user_data(handle))
        .ok()
        .flatten()
    else {
        return;
    };

    let user_data_rc: &Rc<RefCell<ArkUINode>> =
        &*(user_data.as_ptr() as *const Rc<RefCell<ArkUINode>>);

    let node = user_data_rc.borrow();

    let raw_event_type = OH_ArkUI_NodeEvent_GetEventType(event);
    let Some(event_type) = NodeEventType::try_from_raw(raw_event_type) else {
        return;
    };

    if let Some(cb) = node.event_handle.get_event_callback(event_type) {
        let node_event = Event::new(event);
        cb.borrow()(&node_event);
    }
}

unsafe extern "C" fn node_custom_event_receiver(event: *mut ArkUI_NodeCustomEvent) {
    let Some(event) = crate::NodeCustomEvent::from_raw(event) else {
        return;
    };
    let Some(node) = event.node_handle() else {
        return;
    };
    let key = node_custom_event_map_key(node.raw(), event.event_type());
    let callback = {
        let callbacks = match node_custom_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        callbacks.get(&key).copied()
    };
    let Some(callback) = callback else {
        return;
    };
    let callback = unsafe { &*(callback as *const NodeCustomEventCallbackContext) };
    (callback.callback)(&event);
}
