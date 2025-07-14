use std::ptr::{self, NonNull};

use ohos_web_sys::{ArkWeb_ResourceRequest, OH_ArkWebResourceRequest_GetHttpBodyStream};

use crate::HttpBodyStream;

pub struct ResourceRequest {
    raw: NonNull<ArkWeb_ResourceRequest>,
}

impl ResourceRequest {
    pub fn new(raw: *mut ArkWeb_ResourceRequest) -> Self {
        unsafe {
            Self {
                raw: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn http_body_stream(&self) -> HttpBodyStream {
        let mut raw = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetHttpBodyStream(self.raw.as_ptr(), &mut raw);
            HttpBodyStream::new(raw)
        }
    }
}
