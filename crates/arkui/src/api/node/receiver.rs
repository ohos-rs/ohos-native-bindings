//! `extern "C"` entry points for node event dispatch.
//!
//! Panic policy: Rust (>= 1.81) aborts the process when a panic reaches an
//! `extern "C"` boundary, so a panicking user callback is a defined,
//! deliberate process failure — the consumer's panic hook still runs first.
//! These entry points therefore avoid *avoidable* panic sources themselves:
//! no `unwrap`, and no `RefCell` borrow held across the user callback.

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

    // Only wrapper-installed pointers carry the magic + self-address header;
    // foreign user_data (e.g. ArkTS bridges) is never dereferenced as a
    // wrapper. SAFETY: the pointer came from a live node's user_data slot.
    let Some(user_data_box) =
        (unsafe { crate::common::user_data::classify_wrapper_user_data(user_data.as_ptr()) })
    else {
        return;
    };

    let raw_event_type = OH_ArkUI_NodeEvent_GetEventType(event);
    let Some(event_type) = NodeEventType::try_from_raw(raw_event_type) else {
        return;
    };

    // Clone the callback out and end the wrapper borrow before invoking it:
    // a callback that touches its own node (set attributes, replace its
    // handler) must not trip a reentrant `RefCell` borrow.
    let callback = {
        let Ok(node) = user_data_box.wrapper.try_borrow() else {
            // The wrapper is mutably borrowed further up the stack (native
            // code invoked us synchronously mid-mutation). Delivering the
            // event would alias that borrow; drop it instead.
            return;
        };
        node.event_handle.get_event_callback(event_type).cloned()
    };
    if let Some(callback) = callback {
        let node_event = Event::new(event);
        (callback.borrow())(&node_event);
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
    // Clone the `Rc` out of the registry before invoking: the callback stays
    // alive for the duration of the call even if it unregisters itself.
    let callback =
        super::NODE_CUSTOM_EVENT_CALLBACKS.with(|callbacks| callbacks.borrow().get(&key).cloned());
    let Some(callback) = callback else {
        return;
    };
    (callback.callback)(&event);
}
