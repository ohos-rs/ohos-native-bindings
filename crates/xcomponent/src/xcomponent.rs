use napi_ohos::{bindgen_prelude::check_status, Env, Error, JsObject, NapiRaw, Result};
use napi_sys_ohos as sys;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NativeXComponent_GetXComponentSize,
    OH_NativeXComponent_RegisterCallback, OH_NATIVE_XCOMPONENT_OBJ,
};
use std::{os::raw::c_void, ptr};

use crate::{
    callbacks::XComponentCallbacks,
    code::XComponentResultCode,
    r#type::{NativeXComponent, Window, XComponentSize},
    tool::resolve_id,
};

/// Accept XComponent with env and exports
/// ### Example
/// ```no_run
/// #[module_exports]
/// pub fn init(exports: JsObject, env: Env) -> Result<()> {
///     let xcomponent = XComponent::init(env, exports)?;
///
///     Ok(())
/// }
/// ```
#[repr(transparent)]
pub struct XComponent(NativeXComponent);

impl XComponent {
    pub fn init(env: Env, exports: JsObject) -> Result<Self> {
        // Safety: static char * we can use it directly.
        // c char has \0, we should remove it.
        let xcomponent_obj_name: &str = unsafe {
            std::str::from_utf8_unchecked(
                &OH_NATIVE_XCOMPONENT_OBJ[..OH_NATIVE_XCOMPONENT_OBJ.len() - 1],
            )
        };

        let export_instance: JsObject = exports.get_named_property(xcomponent_obj_name)?;
        // env.unwrap will check type, so we just use ffi directly.
        let mut instance = ptr::null_mut();
        check_status!(
            unsafe {
                sys::napi_unwrap(
                    env.raw(),
                    export_instance.raw(),
                    &mut instance as *mut *mut OH_NativeXComponent as *mut *mut c_void,
                )
            },
            "Get OH_NativeXComponent failed."
        )?;

        Ok(XComponent(NativeXComponent(instance)))
    }

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
        self.0 .0
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
