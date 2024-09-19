use napi_derive_ohos::module_exports;
use napi_ohos::{Env, JsObject, Result};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::{XComponent, XComponentCallbacks};

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

    pub type EGLNativeWindowType = *const libc::c_void;
    pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = libc::c_long;
    pub type EGLint = i32;
    pub type EGLContext = *const libc::c_void;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type NativeDisplayType = EGLNativeDisplayType;
    pub type NativePixmapType = EGLNativePixmapType;
    pub type NativeWindowType = EGLNativeWindowType;

    pub use Egl as Gl;
}

#[module_exports]
pub fn init(exports: JsObject, env: Env) -> Result<()> {
    let xcomponent = XComponent::init(env, exports)?;

    let id = xcomponent.id()?;

    let mut callbacks = XComponentCallbacks::new(id);
    callbacks.set_on_surface_created(|_xcomponent, _win| {
        hilog_info!("xcomponent_create");
        Ok(())
    });

    callbacks.set_on_surface_changed(|_xcomponent, _win| {
        hilog_info!("xcomponent_changed");
        Ok(())
    });

    callbacks.set_on_surface_destroyed(|_xcomponent, _win| {
        hilog_info!("xcomponent_destroy");
        Ok(())
    });

    callbacks.set_dispatch_touch_event(|_xcomponent, _win| {
        hilog_info!("xcomponent_dispatch");
        Ok(())
    });

    xcomponent.register_callback(callbacks)?;

    Ok(())
}
