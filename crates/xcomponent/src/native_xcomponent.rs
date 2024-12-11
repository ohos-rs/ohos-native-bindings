use napi_ohos::{Error, Result};
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NativeXComponent_RegisterCallback,
    OH_NativeXComponent_RegisterKeyEventCallback, OH_NativeXComponent_RegisterOnFrameCallback,
};

use crate::{
    code::XComponentResultCode, dispatch_touch_event, key_event, on_frame_change,
    on_surface_changed, on_surface_created, on_surface_destroyed, raw::XComponentRaw,
    tool::resolve_id, KeyEventData, TouchEventData, WindowRaw, XComponentSize,
};

#[cfg(feature = "single_mode")]
use crate::X_COMPONENT_CALLBACKS;

#[cfg(feature = "multi_mode")]
use crate::X_COMPONENT_CALLBACKS_MAP;

pub struct NativeXComponent {
    pub raw: XComponentRaw,
    pub(crate) id: Option<String>,
}

impl NativeXComponent {
    pub fn new(raw: XComponentRaw) -> Self {
        Self { raw, id: None }
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

    /// Register callbacks   
    /// For multi-mode, it will use hashmap to store all of your callbacks closure.   
    /// This may cause xcomponent being slower, if you want to avoid this.    
    /// You can disable feature with `callbacks` and use `register_native_callback`   
    #[cfg(feature = "callbacks")]
    pub fn register_callback(&self) -> Result<()> {
        let cbs = Box::new(OH_NativeXComponent_Callback {
            OnSurfaceCreated: Some(on_surface_created),
            OnSurfaceChanged: Some(on_surface_changed),
            OnSurfaceDestroyed: Some(on_surface_destroyed),
            DispatchTouchEvent: Some(dispatch_touch_event),
        });
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterCallback(self.raw(), Box::leak(cbs) as *mut _).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent register callbacks failed"));
        }
        Ok(())
    }

    pub fn on_surface_changed<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_changed = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .on_surface_changed = Some(Box::new(cb));
            });
        }
    }

    pub fn on_surface_created<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_created = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .on_surface_created = Some(Box::new(cb));
            });
        }
    }

    pub fn on_surface_destroyed<T: Fn(XComponentRaw, WindowRaw) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) {
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_surface_destroyed = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .on_surface_destroyed = Some(Box::new(cb));
            });
        }
    }

    pub fn on_touch_event<
        T: Fn(XComponentRaw, WindowRaw, TouchEventData) -> Result<()> + 'static,
    >(
        &self,
        cb: T,
    ) {
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.dispatch_touch_event = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .dispatch_touch_event = Some(Box::new(cb));
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

    /// Register frame callback
    pub fn on_frame_callback<T: Fn(XComponentRaw, u64, u64) -> Result<()> + 'static>(
        &self,
        cb: T,
    ) -> Result<()> {
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_frame_change = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .on_frame_change = Some(Box::new(cb));
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
        #[cfg(feature = "single_mode")]
        X_COMPONENT_CALLBACKS.with_borrow_mut(|f| {
            f.on_key_event = Some(Box::new(cb));
        });

        #[cfg(feature = "multi_mode")]
        {
            let id = self.id().unwrap();
            X_COMPONENT_CALLBACKS_MAP.with_borrow_mut(|f| {
                f.entry(id)
                    .or_insert_with(|| Default::default())
                    .on_key_event = Some(Box::new(cb));
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
}
