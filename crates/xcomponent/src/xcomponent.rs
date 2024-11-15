use napi_ohos::{bindgen_prelude::check_status, Env, JsObject, NapiRaw, Result};
use napi_sys_ohos as sys;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NATIVE_XCOMPONENT_OBJ,
};
use std::{os::raw::c_void, ptr};

use crate::{
    callbacks::XComponentCallbacks,
    native_xcomponent::{NativeXComponent, Window, XComponentSize},
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
        self.0.id()
    }

    /// get raw point
    pub fn raw(&self) -> *mut OH_NativeXComponent {
        self.0.raw()
    }

    /// Register callbacks   
    /// For multi-mode, it will use hashmap to store all of your callbacks closure.   
    /// This may cause xcomponent being slower, if you want to avoid this.    
    /// You can disable feature with `callbacks` and use `register_native_callback`   
    #[cfg(feature = "callbacks")]
    pub fn register_callback(&self, callbacks: XComponentCallbacks) -> Result<()> {
        self.0.register_callback(callbacks)
    }

    /// Use ffi to register callbacks directly.
    pub unsafe fn register_native_callback(
        &self,
        callbacks: Box<OH_NativeXComponent_Callback>,
    ) -> Result<()> {
        self.0.register_native_callback(callbacks)
    }

    /// Get current XComponent's size info include width and height.
    pub fn size(&self, window: Window) -> Result<XComponentSize> {
        self.0.size(window)
    }
}
