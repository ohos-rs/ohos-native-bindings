//! Module api::node::receiver wrappers and related types.

use std::{cell::RefCell, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_NodeCustomEvent, ArkUI_NodeEvent, OH_ArkUI_NodeEvent_GetEventType,
    OH_ArkUI_NodeEvent_GetNodeHandle,
};

use crate::{Event, NodeEventType};

pub(super) unsafe extern "C" fn node_event_receiver(event: *mut ArkUI_NodeEvent) {
    if event.is_null() {
        return;
    }
    let handle = OH_ArkUI_NodeEvent_GetNodeHandle(event);
    let Some(user_data) = super::ARK_UI_NATIVE_NODE_API_1
        .with(|api| api.get_user_data(handle))
        .ok()
        .flatten()
    else {
        return;
    };

    let user_data_rc: &Rc<RefCell<crate::ArkUINode>> =
        &*(user_data.as_ptr() as *const Rc<RefCell<crate::ArkUINode>>);

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

pub(super) unsafe extern "C" fn node_custom_event_receiver(event: *mut ArkUI_NodeCustomEvent) {
    let Some(event) = crate::NodeCustomEvent::from_raw(event) else {
        return;
    };
    let Some(node) = event.node_handle() else {
        return;
    };
    let key = super::node_custom_event_map_key(node.raw(), event.event_type());
    let callback = {
        let callbacks = match super::node_custom_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        callbacks.get(&key).copied()
    };
    let Some(callback) = callback else {
        return;
    };
    let callback = &*(callback as *const super::NodeCustomEventCallbackContext);
    (callback.callback)(&event);
}
