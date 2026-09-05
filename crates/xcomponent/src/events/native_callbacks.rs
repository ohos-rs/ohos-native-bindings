//! `extern "C"` entry points for XComponent surface and input callbacks.
//!
//! Failure policy: these run on the UI thread inside native dispatch, where a
//! panic aborts the process (Rust >= 1.81 aborts at `extern "C"` boundaries).
//! Recoverable failures — an unresolvable instance id, a failed event query,
//! or a user callback returning `Err` — therefore drop the single event
//! instead of taking the application down.

use std::mem::MaybeUninit;

use ohos_arkui_input_binding::ArkUIInputEvent;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_GetKeyEvent, OH_NativeXComponent_GetKeyEventAction,
    OH_NativeXComponent_GetKeyEventCode, OH_NativeXComponent_GetKeyEventDeviceId,
    OH_NativeXComponent_GetKeyEventSourceType, OH_NativeXComponent_GetKeyEventTimestamp,
    OH_NativeXComponent_GetMouseEvent, OH_NativeXComponent_GetTouchEvent,
    OH_NativeXComponent_GetTouchPointToolType, OH_NativeXComponent_MouseEvent,
    OH_NativeXComponent_TouchEvent,
};

use crate::{Action, EventSource, KeyCode, KeyEventData, MouseEventData, WindowRaw, XComponentRaw};

use super::{remove_raw_window, store_raw_window, RawWindow, TouchEventData, XComponentCallbacks};

#[cfg(not(feature = "multi_mode"))]
use super::X_COMPONENT_CALLBACKS;

#[cfg(feature = "multi_mode")]
use super::{callback_key, X_COMPONENT_CALLBACKS_MAP};

/// Resolve the callback set registered for `xcomponent`, if any.
fn callbacks_for(xcomponent: *mut OH_NativeXComponent) -> Option<XComponentCallbacks> {
    #[cfg(not(feature = "multi_mode"))]
    {
        let _ = xcomponent;
        Some(X_COMPONENT_CALLBACKS.with_borrow(|cb| cb.clone()))
    }

    #[cfg(feature = "multi_mode")]
    {
        X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| cb.get(&callback_key(xcomponent)).cloned())
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_surface_created(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    store_raw_window(xcomponent.cast(), RawWindow::new(window.0));

    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_surface_created) {
        let _ = callback(XComponentRaw(xcomponent), window);
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_surface_changed(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    store_raw_window(xcomponent.cast(), RawWindow::new(window.0));

    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_surface_changed) {
        let _ = callback(XComponentRaw(xcomponent), window);
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_surface_destroyed(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    remove_raw_window(xcomponent.cast());

    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_surface_destroyed) {
        let _ = callback(XComponentRaw(xcomponent), WindowRaw(window));
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn dispatch_touch_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.dispatch_touch_event) else {
        return;
    };

    let mut touch_event = MaybeUninit::<OH_NativeXComponent_TouchEvent>::uninit();
    let ret = OH_NativeXComponent_GetTouchEvent(xcomponent, window, touch_event.as_mut_ptr());
    if ret != 0 {
        return;
    }

    let touch_event_data = touch_event.assume_init();
    let mut data = TouchEventData::from(touch_event_data);

    data.touch_points.iter_mut().for_each(|point| {
        let mut tool = 0;
        let ret = OH_NativeXComponent_GetTouchPointToolType(xcomponent, point.id as _, &mut tool);
        if ret == 0 {
            point.event_tool_type = tool.into();
        }
    });

    let _ = callback(XComponentRaw(xcomponent), WindowRaw(window), data);
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_frame_change(
    xcomponent: *mut OH_NativeXComponent,
    timestamp: u64,
    target_timestamp: u64,
) {
    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_frame_change) {
        let _ = callback(XComponentRaw(xcomponent), timestamp, target_timestamp);
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn key_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_key_event) else {
        return;
    };

    let mut event = std::ptr::null_mut();
    if OH_NativeXComponent_GetKeyEvent(xcomponent, &mut event) != 0 || event.is_null() {
        return;
    }

    let mut action = 0;
    let mut code = 0;
    let mut device_id = 0;
    let mut source = 0;
    let mut timestamp = 0;
    if OH_NativeXComponent_GetKeyEventAction(event, &mut action) != 0
        || OH_NativeXComponent_GetKeyEventCode(event, &mut code) != 0
        || OH_NativeXComponent_GetKeyEventDeviceId(event, &mut device_id) != 0
        || OH_NativeXComponent_GetKeyEventSourceType(event, &mut source) != 0
        || OH_NativeXComponent_GetKeyEventTimestamp(event, &mut timestamp) != 0
    {
        return;
    }

    let key_event_data = KeyEventData {
        code: KeyCode::from(code),
        action: Action::from(action),
        device_id,
        source: EventSource::from(source),
        timestamp,
    };

    let _ = callback(XComponentRaw(xcomponent), WindowRaw(window), key_event_data);
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_mouse_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_mouse_event) else {
        return;
    };

    let mut mouse_event = MaybeUninit::<OH_NativeXComponent_MouseEvent>::uninit();
    let ret = OH_NativeXComponent_GetMouseEvent(xcomponent, window, mouse_event.as_mut_ptr());
    if ret != 0 {
        return;
    }

    let mouse_event_data = mouse_event.assume_init();
    let data = MouseEventData::from(mouse_event_data);

    let _ = callback(XComponentRaw(xcomponent), WindowRaw(window), data);
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_hover_event(xcomponent: *mut OH_NativeXComponent, is_hover: bool) {
    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_hover_event) {
        let _ = callback(XComponentRaw(xcomponent), is_hover);
    }
}

/// # Safety
///
/// NDK callback entry point: must only be invoked by ArkUI with a live
/// `OH_NativeXComponent` and the pointers it documents for this callback.
pub unsafe extern "C" fn on_ui_input_event(
    xcomponent: *mut OH_NativeXComponent,
    event: *mut ohos_arkui_input_binding::sys::ArkUI_UIInputEvent,
    _type_: ohos_arkui_input_binding::sys::ArkUI_UIInputEvent_Type,
) {
    if let Some(callback) = callbacks_for(xcomponent).and_then(|cb| cb.on_ui_input_event) {
        let arkui_input_event = ArkUIInputEvent::from_raw(event);
        let _ = callback(XComponentRaw(xcomponent), arkui_input_event);
    }
}
