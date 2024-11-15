use napi_ohos::{Error, Result};
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NativeXComponent_GetXComponentSize,
    OH_NativeXComponent_RegisterCallback,
};
use std::os::raw::c_void;

use crate::{callbacks::XComponentCallbacks, code::XComponentResultCode, tool::resolve_id};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Window(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct NativeXComponent(pub *mut OH_NativeXComponent);

impl NativeXComponent {
    /// Get current xcomponent instance's id
    pub fn id(&self) -> Result<String> {
        let current_id = resolve_id(self.raw());
        if let Some(id_str) = current_id {
            return Ok(id_str);
        }
        Err(Error::from_reason("Get XComponent id failed."))
    }

    /// get raw point
    pub fn raw(&self) -> *mut OH_NativeXComponent {
        self.0
    }

    /// Register callbacks   
    /// For multi-mode, it will use hashmap to store all of your callbacks closure.   
    /// This may cause xcomponent being slower, if you want to avoid this.    
    /// You can disable feature with `callbacks` and use `register_native_callback`   
    #[cfg(feature = "callbacks")]
    pub fn register_callback(&self, callbacks: XComponentCallbacks) -> Result<()> {
        let cbs = Box::new(OH_NativeXComponent_Callback {
            OnSurfaceCreated: callbacks.inner.on_surface_created,
            OnSurfaceChanged: callbacks.inner.on_surface_changed,
            OnSurfaceDestroyed: callbacks.inner.on_surface_destroyed,
            DispatchTouchEvent: callbacks.inner.dispatch_touch_event,
        });
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterCallback(self.raw(), Box::leak(cbs) as *mut _).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent register callbacks failed"));
        }
        Ok(())
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
    pub fn size(&self, window: Window) -> Result<XComponentSize> {
        let mut width: u64 = 0;
        let mut height: u64 = 0;
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_GetXComponentSize(self.raw(), window.0, &mut width, &mut height)
                .into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent get size failed"));
        }
        Ok(XComponentSize { width, height })
    }
}

pub(crate) type CallbackClosure = Box<fn(NativeXComponent, Window) -> Result<()>>;

#[repr(C)]
pub struct NativeXComponentCallback {
    pub on_surface_created:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub on_surface_changed:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub on_surface_destroyed:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
    pub dispatch_touch_event:
        Option<unsafe extern "C" fn(*mut OH_NativeXComponent, *mut std::os::raw::c_void)>,
}

impl NativeXComponentCallback {
    pub fn new() -> Self {
        NativeXComponentCallback {
            on_surface_created: None,
            on_surface_changed: None,
            on_surface_destroyed: None,
            dispatch_touch_event: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XComponentSize {
    pub width: u64,
    pub height: u64,
}
