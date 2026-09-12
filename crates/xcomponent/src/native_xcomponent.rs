#![allow(clippy::missing_safety_doc)]

use std::rc::Rc;

use napi_ohos::{Error, Result};
#[cfg(all(feature = "accessibility", feature = "api-13"))]
use ohos_accessibility_binding::{AccessibilityError, Provider};
use ohos_arkui_input_binding::ArkUIInputEvent;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NativeXComponent_ExpectedRateRange,
    OH_NativeXComponent_MouseEvent_Callback, OH_NativeXComponent_RegisterCallback,
    OH_NativeXComponent_RegisterKeyEventCallback, OH_NativeXComponent_RegisterMouseEventCallback,
    OH_NativeXComponent_RegisterOnFrameCallback, OH_NativeXComponent_RegisterUIInputEventCallback,
    OH_NativeXComponent_SetExpectedFrameRateRange,
};

#[cfg(all(feature = "accessibility", feature = "api-13"))]
use ohos_xcomponent_sys::OH_NativeXComponent_GetNativeAccessibilityProvider;

use crate::{
    code::XComponentResultCode, dispatch_touch_event, events::lookup_raw_window, key_event,
    on_frame_change, on_hover_event, on_mouse_event, on_surface_changed, on_surface_created,
    on_surface_destroyed, on_ui_input_event, raw::XComponentRaw, tool::resolve_id, KeyEventData,
    MouseEventData, RawWindow, TouchEventData, WindowRaw, XComponentOffset, XComponentSize,
};

#[cfg(not(feature = "multi_mode"))]
use crate::X_COMPONENT_CALLBACKS;

#[cfg(feature = "multi_mode")]
use crate::{events::callback_key, X_COMPONENT_CALLBACKS_MAP};

#[derive(Debug, Clone)]
pub struct NativeXComponent {
    pub raw: XComponentRaw,
    pub(crate) id: Option<String>,
}

impl NativeXComponent {
    pub fn new(raw: XComponentRaw) -> Self {
        Self { raw, id: None }
    }

    pub fn with_id(raw: XComponentRaw, id: String) -> Self {
        Self { raw, id: Some(id) }
    }

    /// Get current xcomponent instance's id
    pub fn id(&self) -> Result<String> {
        if let Some(id) = &self.id {
            return Ok(id.clone());
        }
        let current_id = resolve_id(self.raw());
        if let Some(id_str) = current_id {
            return Ok(id_str);
        }
        Err(Error::from_reason("Get XComponent id failed."))
    }

    /// get raw point
    pub fn raw(&self) -> *mut OH_NativeXComponent {
        self.raw.0
    }

    /// Obtain the ArkUI accessibility provider owned by this XComponent.
    #[cfg(all(feature = "accessibility", feature = "api-13"))]
    pub fn accessibility_provider(&self) -> ohos_accessibility_binding::Result<Provider<'_>> {
        let mut provider = std::ptr::null_mut();
        let result = unsafe {
            OH_NativeXComponent_GetNativeAccessibilityProvider(self.raw(), &mut provider)
        };
        if result != 0 {
            return Err(AccessibilityError::UnknownCode(result));
        }
        unsafe { Provider::from_raw(provider) }
    }

    /// The live native window of **this** XComponent instance, if its surface
    /// is currently created.
    pub fn native_window(&self) -> Option<RawWindow> {
        lookup_raw_window(self.raw().cast())
    }

    /// Register callbacks.
    ///
    /// With `multi_mode`, closures are routed by native XComponent instance,
    /// so reused or duplicate application-level IDs remain isolated.
    #[cfg(feature = "callbacks")]
    pub fn register_callback(&self) -> Result<()> {
        // The callback table is identical for every registration, so one
        // process-wide allocation serves all instances instead of leaking a
        // fresh box per call.
        static CALLBACK_TABLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        let table = *CALLBACK_TABLE.get_or_init(|| {
            Box::into_raw(Box::new(OH_NativeXComponent_Callback {
                OnSurfaceCreated: Some(on_surface_created),
                OnSurfaceChanged: Some(on_surface_changed),
                OnSurfaceDestroyed: Some(on_surface_destroyed),
                DispatchTouchEvent: Some(dispatch_touch_event),
            })) as usize
        }) as *mut OH_NativeXComponent_Callback;
        let ret: XComponentResultCode =
            unsafe { OH_NativeXComponent_RegisterCallback(self.raw(), table).into() };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent register callbacks failed"));
        }
        Ok(())
    }

    /// Drop every closure registered for this instance.
    ///
    /// Native surface callbacks may still arrive after the consumer's owner
    /// is gone (ArkUI does not guarantee `OnSurfaceDestroyed` ordering versus
    /// consumer teardown); after this call they find no closure and are
    /// dropped, and captured state (channels, contexts) is released instead
    /// of living in the registry forever.
    #[cfg(feature = "callbacks")]
    pub fn unregister_callbacks(&self) {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            *f = Default::default();
        });

        #[cfg(feature = "multi_mode")]
        X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
            f.remove(&callback_key(self.raw()));
        });
    }

    pub fn on_surface_changed<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_changed = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_surface_changed = Some(Rc::new(cb));
            });
        }
    }

    pub fn on_surface_created<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_created = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_surface_created = Some(Rc::new(cb));
            });
        }
    }

    pub fn on_surface_destroyed<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_destroyed = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_surface_destroyed = Some(Rc::new(cb));
            });
        }
    }

    pub fn on_touch_event<
        T: Fn(XComponentRaw, WindowRaw, TouchEventData) -> Result<()> + 'static,
    >(
        &self,
        cb: T,
    ) {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.dispatch_touch_event = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .dispatch_touch_event = Some(Rc::new(cb));
            });
        }
    }

    /// Use ffi to register callbacks directly.
    pub unsafe fn register_native_callback(
        &self,
        callbacks: Box<OH_NativeXComponent_Callback>,
    ) -> Result<()> {
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterCallback(self.raw(), Box::leak(callbacks) as *mut _).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent register callbacks failed"));
        }
        Ok(())
    }

    /// Get current XComponent's size info include width and height.
    pub fn size(&self, window: WindowRaw) -> Result<XComponentSize> {
        self.raw.size(window)
    }

    pub fn offset(&self, window: WindowRaw) -> Result<XComponentOffset> {
        self.raw.offset(window)
    }

    pub fn set_frame_rate(&self, min: i32, max: i32, expected: i32) -> Result<()> {
        let mut range = OH_NativeXComponent_ExpectedRateRange { min, max, expected };
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_SetExpectedFrameRateRange(self.raw(), &mut range as *mut _).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent set frame rate failed"));
        }
        Ok(())
    }

    /// Register frame callback
    pub fn on_frame_callback<T: Fn(XComponentRaw, u64, u64) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_frame_change = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_frame_change = Some(Rc::new(cb));
            });
        }

        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterOnFrameCallback(self.raw(), Some(on_frame_change)).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason(
                "XComponent register frame callback failed",
            ));
        }
        Ok(())
    }

    pub fn on_key_event<T: Fn(XComponentRaw, WindowRaw, KeyEventData) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_key_event = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw())).or_default().on_key_event = Some(Rc::new(cb));
            });
        }

        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterKeyEventCallback(self.raw(), Some(key_event)).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason(
                "XComponent register key event callback failed",
            ));
        }
        Ok(())
    }

    pub fn on_hover_event<T: Fn(XComponentRaw, bool) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_hover_event = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_hover_event = Some(Rc::new(cb));
            });
        }
        Ok(())
    }

    pub fn on_mouse_event<
        T: Fn(XComponentRaw, WindowRaw, MouseEventData) -> Result<()> + 'static,
    >(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_mouse_event = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_mouse_event = Some(Rc::new(cb));
            });
        }
        Ok(())
    }

    pub fn register_mouse_event_callback(&self) -> Result<()> {
        // Same table-sharing rationale as `register_callback`.
        static MOUSE_CALLBACK_TABLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
        let table = *MOUSE_CALLBACK_TABLE.get_or_init(|| {
            Box::into_raw(Box::new(OH_NativeXComponent_MouseEvent_Callback {
                DispatchMouseEvent: Some(on_mouse_event),
                DispatchHoverEvent: Some(on_hover_event),
            })) as usize
        }) as *mut OH_NativeXComponent_MouseEvent_Callback;
        let ret: XComponentResultCode =
            unsafe { OH_NativeXComponent_RegisterMouseEventCallback(self.raw(), table).into() };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason(
                "XComponent register mouse event callback failed",
            ));
        }
        Ok(())
    }

    pub fn on_ui_input_event<T: Fn(XComponentRaw, ArkUIInputEvent) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(not(feature = "multi_mode"))]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_ui_input_event = Some(Rc::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(callback_key(self.raw()))
                    .or_default()
                    .on_ui_input_event = Some(Rc::new(cb));
            });
        }
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterUIInputEventCallback(
                self.raw(),
                Some(on_ui_input_event),
                ohos_arkui_input_binding::UIInputEvent::Axis.into(),
            )
            .into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason(
                "XComponent register ui input event callback failed",
            ));
        }
        Ok(())
    }
}
