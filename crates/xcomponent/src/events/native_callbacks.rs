use ohos_xcomponent_sys::OH_NativeXComponent;

use crate::{WindowRaw, XComponentRaw};

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

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = cb.on_surface_created {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = callback.on_surface_created {
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

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = cb.on_surface_changed {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = callback.on_surface_changed {
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

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = cb.on_surface_destroyed {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = callback.on_surface_destroyed {
                callback(xcomponent, window).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn dispatch_touch_event(
    xcomponent: *mut OH_NativeXComponent,
    window: *mut std::os::raw::c_void,
) {
    let window = WindowRaw(window);
    let xcomponent = XComponentRaw(xcomponent);

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = cb.dispatch_touch_event {
            callback(xcomponent, window).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = callback.dispatch_touch_event {
                callback(xcomponent, window).unwrap();
            }
        }
    })
}

pub unsafe extern "C" fn on_frame_change(
    xcomponent: *mut OH_NativeXComponent,
    width: u64,
    height: u64,
) {
    let xcomponent = XComponentRaw(xcomponent);

    #[cfg(feature = "single_mode")]
    X_COMPONENT_CALLBACKS.with_borrow(|cb| {
        if let Some(callback) = cb.on_frame_change {
            callback(xcomponent, width, height).unwrap();
        }
    });

    #[cfg(feature = "multi_mode")]
    X_COMPONENT_CALLBACKS_MAP.with_borrow(|cb| {
        let id = resolve_id(xcomponent.0).unwrap();
        if let Some(callback) = cb.get(&id) {
            if let Some(callback) = callback.on_frame_change {
                callback(xcomponent, width, height).unwrap();
            }
        }
    })
}
