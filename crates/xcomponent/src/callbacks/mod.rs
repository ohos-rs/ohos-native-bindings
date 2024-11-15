use napi_ohos::Result;
use ohos_xcomponent_sys::OH_NativeXComponent;

use crate::native_xcomponent::{NativeXComponent, NativeXComponentCallback, Window};

#[cfg(feature = "single_mode")]
mod single;

#[cfg(feature = "multi_mode")]
mod multi;

#[cfg(feature = "log")]
use ohos_hilog_binding::hilog_warn;

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
    #[cfg(feature = "multi_mode")]
    pub fn new(id: String) -> Self {
        XComponentCallbacks {
            inner: NativeXComponentCallback::new(),
            id,
        }
    }

    #[cfg(feature = "single_mode")]
    pub fn new() -> Self {
        XComponentCallbacks {
            inner: NativeXComponentCallback::new(),
            id: String::from(""),
        }
    }

    /// set OnSurfaceCreated callback
    pub fn set_on_surface_created(&mut self, callback: fn(NativeXComponent, Window) -> Result<()>) {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_created = Some(on_surface_created);

        #[cfg(feature = "multi_mode")]
        {
            let map_id = format!("{}_OnSurfaceCreated", self.id.clone());
            multi::X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
        }

        #[cfg(feature = "single_mode")]
        single::X_COMPONENT_SINGLE_MAP.borrow_mut(|cb| cb.on_surface_created = Some(boxed_callback))
    }

    /// set OnSurfaceChanged callback
    pub fn set_on_surface_changed(&mut self, callback: fn(NativeXComponent, Window) -> Result<()>) {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_changed = Some(on_surface_changed);

        #[cfg(feature = "multi_mode")]
        {
            let map_id = format!("{}_OnSurfaceChanged", self.id.clone());
            multi::X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
        }

        #[cfg(feature = "single_mode")]
        single::X_COMPONENT_SINGLE_MAP.borrow_mut(|cb| cb.on_surface_changed = Some(boxed_callback))
    }

    /// set OnSurfaceDestroyed callback
    pub fn set_on_surface_destroyed(
        &mut self,
        callback: fn(NativeXComponent, Window) -> Result<()>,
    ) {
        let boxed_callback = Box::new(callback);
        self.inner.on_surface_destroyed = Some(on_surface_destroyed);

        #[cfg(feature = "multi_mode")]
        {
            let map_id = format!("{}_OnSurfaceDestroyed", self.id.clone());
            multi::X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
        }

        #[cfg(feature = "single_mode")]
        single::X_COMPONENT_SINGLE_MAP
            .borrow_mut(|cb| cb.on_surface_destroyed = Some(boxed_callback))
    }

    /// set DispatchTouchEvent callback
    pub fn set_dispatch_touch_event(
        &mut self,
        callback: fn(NativeXComponent, Window) -> Result<()>,
    ) {
        let boxed_callback = Box::new(callback);
        self.inner.dispatch_touch_event = Some(dispatch_touch_event);

        #[cfg(feature = "multi_mode")]
        {
            let map_id = format!("{}_DispatchTouchEvent", self.id.clone());
            multi::X_COMPONENT_MAP.borrow_mut(|m| m.insert(map_id, boxed_callback));
        }

        #[cfg(feature = "single_mode")]
        single::X_COMPONENT_SINGLE_MAP
            .borrow_mut(|cb| cb.dispatch_touch_event = Some(boxed_callback))
    }
}

macro_rules! callback {
    ($func: ident, $name: expr) => {
        unsafe extern "C" fn $func(
            component: *mut OH_NativeXComponent,
            win: *mut ::std::os::raw::c_void,
        ) {
            #[cfg(feature = "multi_mode")]
            {
                use crate::tool::resolve_id;

                let id = resolve_id(component);
                if let Some(id) = id {
                    multi::X_COMPONENT_MAP.borrow_mut(|map| {
                        let map_id = format!("{}_{}", &id, &$name);
                        if let Some(callback) = map.get(&map_id) {
                            #[allow(unused_variables)]
                            if let Err(e) = callback(NativeXComponent(component), Window(win)) {
                                #[cfg(feature = "log")]
                                hilog_warn!(format!(
                                    "XComponent {} run failed: {}",
                                    &$name, e.reason
                                ));
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

            #[cfg(feature = "single_mode")]
            single::X_COMPONENT_SINGLE_MAP.borrow(|cb| {
                if let Some(callback) = &cb.$func {
                    #[allow(unused_variables)]
                    if let Err(e) = callback(NativeXComponent(component), Window(win)) {
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
            })
        }
    };
}

callback!(on_surface_created, "OnSurfaceCreated");
callback!(on_surface_changed, "OnSurfaceChanged");
callback!(on_surface_destroyed, "OnSurfaceDestroyed");
callback!(dispatch_touch_event, "DispatchTouchEvent");
