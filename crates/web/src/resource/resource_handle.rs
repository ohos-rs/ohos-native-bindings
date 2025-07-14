use std::ptr::NonNull;

use ohos_web_sys::{
    ArkWeb_ResourceHandler, OH_ArkWebResourceHandler_Destroy, OH_ArkWebResourceHandler_DidFinish,
    OH_ArkWebResourceHandler_DidReceiveData, OH_ArkWebResourceHandler_DidReceiveResponse,
};

use crate::ArkWebResponse;

pub struct ResourceHandle {
    raw: NonNull<ArkWeb_ResourceHandler>,
}

impl ResourceHandle {
    pub fn new(raw: *mut ArkWeb_ResourceHandler) -> Self {
        unsafe {
            Self {
                raw: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn receive_data<D: Into<Vec<u8>>>(&self, data: D) {
        let buf: Vec<u8> = data.into();
        unsafe {
            OH_ArkWebResourceHandler_DidReceiveData(
                self.raw.as_ptr(),
                buf.as_ptr(),
                buf.len() as _,
            );
        }
    }

    pub fn receive_response(&self, response: ArkWebResponse) {
        unsafe {
            OH_ArkWebResourceHandler_DidReceiveResponse(self.raw.as_ptr(), response.raw());
        }
    }

    pub fn finish(&self) {
        unsafe {
            OH_ArkWebResourceHandler_DidFinish(self.raw.as_ptr());
        }
    }
}

impl Drop for ResourceHandle {
    fn drop(&mut self) {
        unsafe {
            OH_ArkWebResourceHandler_Destroy(self.raw.as_ptr());
        }
    }
}
