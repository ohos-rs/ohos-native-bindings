use napi_ohos::{Error, Result};
use ohos_xcomponent_sys::OH_NativeXComponent;
use ohos_xcomponent_sys::OH_NativeXComponent_GetXComponentSize;
use std::os::raw::c_void;

use crate::{code::XComponentResultCode, XComponentSize};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct WindowRaw(pub *mut c_void);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct XComponentRaw(pub *mut OH_NativeXComponent);

impl XComponentRaw {
    pub fn size(&self, win: WindowRaw) -> Result<XComponentSize> {
        let mut width: u64 = 0;
        let mut height: u64 = 0;
        let ret: XComponentResultCode = unsafe {
            OH_NativeXComponent_GetXComponentSize(self.0, win.0, &mut width, &mut height).into()
        };
        if ret != XComponentResultCode::Success {
            return Err(Error::from_reason("XComponent get size failed"));
        }
        Ok(XComponentSize { width, height })
    }
}
