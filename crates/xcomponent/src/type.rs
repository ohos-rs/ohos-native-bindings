use napi_ohos::{Error, Result};
use ohos_xcomponent_sys::{OH_NativeXComponent, OH_NativeXComponent_GetXComponentSize};
use std::os::raw::c_void;

use crate::{code::XComponentResultCode, tool::resolve_id};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Window(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct NativeXComponent(pub *mut OH_NativeXComponent);

impl NativeXComponent {
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

    /// Get current xcomponent instance's id
    pub fn id(&self) -> Result<String> {
        let current_id = resolve_id(self.0);
        if let Some(id_str) = current_id {
            return Ok(id_str);
        }
        Err(Error::from_reason("Get XComponent id failed."))
    }
}

pub(crate) type CallbackClosure =
    Box<dyn Fn(NativeXComponent, Window) -> Result<()> + 'static + Send>;

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
