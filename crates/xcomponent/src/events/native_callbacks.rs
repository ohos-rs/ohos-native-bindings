#![allow(clippy::missing_safety_doc)]

use std::mem::MaybeUninit;

use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_GetKeyEvent, OH_NativeXComponent_GetKeyEventAction,
    OH_NativeXComponent_GetKeyEventCode, OH_NativeXComponent_GetKeyEventDeviceId,
    OH_NativeXComponent_GetKeyEventSourceType, OH_NativeXComponent_GetKeyEventTimestamp,
    OH_NativeXComponent_GetTouchEvent, OH_NativeXComponent_GetTouchPointToolType,
    OH_NativeXComponent_TouchEvent,
};

use crate::{Action, EventSource, KeyCode, KeyEventData, WindowRaw, XComponentRaw};

use super::{RawWindow, TouchEventData, RAW_WINDOW};

#[cfg(feature = "single_mode")]
use super::X_COMPONENT_CALLBACKS;

#[cfg(feature = "multi_mode")]
use super::X_COMPONENT_CALLBACKS_MAP;
#[cfg(feature = "multi_mode")]
use crate::tool::resolve_id;

pub unsafe extern "C" fn on_surface_created(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    {
        let mut guard = (*RAW_WINDOW).write().expect("read raw window failed");
        guard.replace(RawWindow(window.0));
    }

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.on_surface_created {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.on_surface_created {
                callback(xcomponent, window).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn on_surface_changed(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    {
        let mut guard = (*RAW_WINDOW).write().expect("read raw window failed");
        guard.replace(RawWindow(window.0));
    }

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.on_surface_changed {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.on_surface_changed {
                callback(xcomponent, window).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn on_surface_destroyed(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    {
        let mut guard = (*RAW_WINDOW).write().expect("read raw window failed");
        *guard = None;
    }

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.on_surface_destroyed {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.on_surface_destroyed {
                callback(xcomponent, window).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn dispatch_touch_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let mut touch_event = MaybeUninit::<OH_NativeXComponent_TouchEvent>::uninit();
    let ret = OH_NativeXComponent_GetTouchEvent(xcomponent, window, touch_event.as_mut_ptr());
    assert!(ret == 0, "Get touch event failed");

    let touch_event_data = touch_event.assume_init();
    let mut data = TouchEventData::from(touch_event_data);

    data.touch_points.iter_mut().for_each(|point| {
        let mut tool = 0;
        let ret = OH_NativeXComponent_GetTouchPointToolType(xcomponent, point.id as _, &mut tool);
        assert!(ret == 0, "Get touch point tool type failed");
        point.event_tool_type = tool.into();
    });

    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.dispatch_touch_event {
            callback(xcomponent, window, data).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.dispatch_touch_event {
                callback(xcomponent, window, data).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn on_frame_change(
    xcomponent: *mut OH_NativeXComponent,
    timestamp: u64,
    target_timestamp: u64,
) {
    let xcomponent = XComponentRaw(xcomponent);

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.on_frame_change {
            callback(xcomponent, timestamp, target_timestamp).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.on_frame_change {
                callback(xcomponent, timestamp, target_timestamp).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn key_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let mut event = std::ptr::null_mut();
    let ret = OH_NativeXComponent_GetKeyEvent(xcomponent, &mut event);
    assert!(ret == 0, "Get key event failed");

    let mut action = 0;
    let ret = OH_NativeXComponent_GetKeyEventAction(event, &mut action);
    assert!(ret == 0, "Get key event action failed");

    let mut code = 0;
    let ret = OH_NativeXComponent_GetKeyEventCode(event, &mut code);
    assert!(ret == 0, "Get key event code failed");

    let mut device_id = 0;
    let ret = OH_NativeXComponent_GetKeyEventDeviceId(event, &mut device_id);
    assert!(ret == 0, "Get key event device id failed");

    let mut source = 0;
    let ret = OH_NativeXComponent_GetKeyEventSourceType(event, &mut source);
    assert!(ret == 0, "Get key event source type failed");

    let mut timestamp = 0;
    let ret = OH_NativeXComponent_GetKeyEventTimestamp(event, &mut timestamp);
    assert!(ret == 0, "Get key event timestamp failed");

    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    let key_event_data = KeyEventData {
        code: KeyCode::from(code),
        action: Action::from(action),
        device_id,
        source: EventSource::from(source),
        timestamp,
    };

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = &cb.on_key_event {
            callback(xcomponent, window, key_event_data).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = &callback.on_key_event {
                callback(xcomponent, window, key_event_data).unwrap();
            }
        }
    })
}
