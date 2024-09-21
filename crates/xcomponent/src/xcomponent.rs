use napi_ohos::{bindgen_prelude::check_status, Env, Error, JsObject, NapiRaw, Result};
use napi_sys_ohos as sys;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_Callback, OH_NativeXComponent_GetXComponentId,
    OH_NativeXComponent_GetXComponentSize, OH_NativeXComponent_RegisterCallback,
    OH_NATIVE_XCOMPONENT_OBJ, OH_XCOMPONENT_ID_LEN_MAX,
};
use std::{os::raw::c_void, ptr, sync::LazyLock};

use crate::{
    code::XComponentResultCode,
    r#type::{NativeXComponentCallback, PersistedPerInstanceHashMap, Window, XComponentSize},
};

#[cfg(feature = "log")]
use ohos_hilog_binding::hilog_warn;

pub(crate) type XComponentMap = PersistedPerInstanceHashMap<
    String,
    Box<dyn Fn(XComponent, Window) -> Result<()> + 'static + Send>,
>;

static X_COMPONENT_MAP: LazyLock<XComponentMap> = LazyLock::new(Default::default);

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
pub struct XComponent(*mut OH_NativeXComponent);

pub type XComponentCallback = fn(xcomponent: XComponent, window: Window) -> Result<()>;

/// Allow to add custom event callback
/// ### Example
/// ```no_run
/// let id = xcomponent.id()?;
/// let mut callbacks = XComponentCallbacks::new(id);
/// callbacks.set_on_surface_created(|_xcomponent, _win| {
///     hilog_info!("xcomponent_create");
///     Ok(())
/// });
///
/// callbacks.set_on_surface_changed(|_xcomponent, _win| {
///     hilog_info!("xcomponent_changed");
///     Ok(())
/// });
///
/// callbacks.set_on_surface_destroyed(|_xcomponent, _win| {
///     hilog_info!("xcomponent_destroy");
///     Ok(())
/// });
///
/// callbacks.set_dispatch_touch_event(|_xcomponent, _win| {
///     hilog_info!("xcomponent_dispatch");
///     Ok(())
/// });
///
/// xcomponent.register_callback(callbacks)?;
/// ```
///
pub struct XComponentCallbacks {
    pub inner: NativeXComponentCallback,
    pub id: String,
}

impl XComponentCallbacks {
    pub fn new(id: String) -> Self {
        XComponentCallbacks {
            inner: NativeXComponentCallback::new(),
            id,
        }
    }

    /// set OnSurfaceCreated callback
    pub fn set_on_surface_created<F>(&mut self, callback: F)
    where
        F: Fn(XComponent, Window) -> Result<()> + 'static + Send,
    {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_created = Some(on_surface_created::<F>);

        let map_id = format!("{}_OnSurfaceCreated", self.id.clone());
        X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
    }

    /// set OnSurfaceChanged callback
    pub fn set_on_surface_changed<F>(&mut self, callback: F)
    where
        F: Fn(XComponent, Window) -> Result<()> + 'static + Send,
    {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_changed = Some(on_surface_changed::<F>);

        let map_id = format!("{}_OnSurfaceChanged", self.id.clone());
        X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
    }

    /// set OnSurfaceDestroyed callback
    pub fn set_on_surface_destroyed<F>(&mut self, callback: F)
    where
        F: Fn(XComponent, Window) -> Result<()> + 'static + Send,
    {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_destroyed = Some(on_surface_destroyed::<F>);

        let map_id = format!("{}_OnSurfaceDestroyed", self.id.clone());
        X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
    }

    /// set DispatchTouchEvent callback
    pub fn set_dispatch_touch_event<F>(&mut self, callback: F)
    where
        F: Fn(XComponent, Window) -> Result<()> + 'static + Send,
    {
        let boxed_callback = Box::new(callback);
        self.inner.dispatch_touch_event = Some(dispatch_touch_event::<F>);

        let map_id = format!("{}_DispatchTouchEvent", self.id.clone());
        X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
    }
}

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

        Ok(XComponent(instance))
    }

    /// Get current xcomponent instance's id
    pub fn id(&self) -> Result<String> {
        let current_id = resolve_id(self.0);
        if let Some(id_str) = current_id {
            return Ok(id_str);
        }
        Err(Error::from_reason("Get XComponent id failed."))
    }

    /// register callbacks
    pub fn register_callback(&self, callbacks: XComponentCallbacks) -> Result<()> {
        let cbs = Box::new(OH_NativeXComponent_Callback {
            OnSurfaceCreated: callbacks.inner.on_surface_created,
            OnSurfaceChanged: callbacks.inner.on_surface_changed,
            OnSurfaceDestroyed: callbacks.inner.on_surface_destroyed,
            DispatchTouchEvent: callbacks.inner.dispatch_touch_event,
        });
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_RegisterCallback(self.0, Box::leak(cbs) as *mut _).into()
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
            OH_NativeXComponent_GetXComponentSize(self.0, window.0, &mut width, &mut height).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent get size failed"));
        }
        Ok(XComponentSize { width, height })
    }
}

/// get xcomponent id
fn resolve_id(component: *mut OH_NativeXComponent) -> Option<String> {
    let mut id_len: u64 = (OH_XCOMPONENT_ID_LEN_MAX + 1).into();
    let mut origin_id = vec![0; id_len as usize];

    let ret: XComponentResultCode = unsafe {
        OH_NativeXComponent_GetXComponentId(component, origin_id.as_mut_ptr(), &mut id_len).into()
    };

    if ret != XComponentResultCode::Success {
        return None;
    }

    // id_len will change to real length if OH_NativeXComponent_GetXComponentId call successfully.
    let id_str: Vec<u8> = origin_id
        .into_iter()
        .take(id_len as usize)
        .map(|x| x as u8)
        .collect();
    let id = String::from_utf8_lossy(&id_str).into_owned();
    return Some(id);
}

macro_rules! callback {
    ($func: ident, $name: expr) => {
        unsafe extern "C" fn $func<F>(
            component: *mut OH_NativeXComponent,
            window: *mut ::std::os::raw::c_void,
        ) where
            F: Fn(XComponent, Window) -> Result<()>,
        {
            let id = resolve_id(component);
            if let Some(id) = id {
                X_COMPONENT_MAP.borrow_mut(|map| {
                    let map_id = format!("{}_{}", &id, &$name);
                    if let Some(callback) = map.get(&map_id) {
                        #[allow(unused_variables)]
                        if let Err(e) = callback(XComponent(component), Window(window)) {
                            #[cfg(feature = "log")]
                            hilog_warn!(format!("XComponent {} run failed: {}", &$name, e.reason));
                        }
                    } else {
                        #[cfg(feature = "log")]
                        hilog_warn!(format!(
                            "XComponent {} run failed: can't resolve current {} callback.",
                            &$name, &$name
                        ));
                    }
                });
            } else {
                #[cfg(feature = "log")]
                hilog_warn!(format!(
                    "XComponent {} run failed: can't resolve current xcomponent's id",
                    &$name
                ));
            }
        }
    };
}

callback!(on_surface_created, "OnSurfaceCreated");
callback!(on_surface_changed, "OnSurfaceChanged");
callback!(on_surface_destroyed, "OnSurfaceDestroyed");
callback!(dispatch_touch_event, "DispatchTouchEvent");
