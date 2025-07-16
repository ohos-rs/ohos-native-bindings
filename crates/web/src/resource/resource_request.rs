use std::ptr::{self, NonNull};

use ohos_web_sys::{
    ArkWeb_ResourceRequest, OH_ArkWebResourceRequest_Destroy, OH_ArkWebResourceRequest_GetFrameUrl,
    OH_ArkWebResourceRequest_GetHttpBodyStream, OH_ArkWebResourceRequest_GetMethod,
    OH_ArkWebResourceRequest_GetReferrer, OH_ArkWebResourceRequest_GetResourceType,
    OH_ArkWebResourceRequest_GetUrl, OH_ArkWebResourceRequest_HasGesture,
    OH_ArkWebResourceRequest_IsMainFrame, OH_ArkWebResourceRequest_IsRedirect,
};

use crate::{HttpBodyStream, Method, Referrer, ResourceType, Url};

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

    pub fn http_body_stream(&self) -> Option<HttpBodyStream> {
        let mut raw = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetHttpBodyStream(self.raw.as_ptr(), &mut raw);
            if raw.is_null() {
                None
            } else {
                Some(HttpBodyStream::new(raw))
            }
        }
    }

    pub fn frame_url(&self) -> String {
        let mut url = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetFrameUrl(self.raw.as_ptr(), &mut url);
            Url { raw: url }.into()
        }
    }

    pub fn method(&self) -> String {
        let mut method = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetMethod(self.raw.as_ptr(), &mut method);
            Method { raw: method }.into()
        }
    }

    pub fn referrer(&self) -> String {
        let mut referrer = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetReferrer(self.raw.as_ptr(), &mut referrer);
            Referrer { raw: referrer }.into()
        }
    }

    pub fn resource_type(&self) -> ResourceType {
        unsafe {
            let ret = OH_ArkWebResourceRequest_GetResourceType(self.raw.as_ptr());
            ResourceType::from(ret as u32)
        }
    }

    pub fn url(&self) -> String {
        let mut url = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetUrl(self.raw.as_ptr(), &mut url);
            Url { raw: url }.into()
        }
    }

    pub fn has_gesture(&self) -> bool {
        unsafe { OH_ArkWebResourceRequest_HasGesture(self.raw.as_ptr()) }
    }

    pub fn is_main_frame(&self) -> bool {
        unsafe { OH_ArkWebResourceRequest_IsMainFrame(self.raw.as_ptr()) }
    }

    pub fn is_redirect(&self) -> bool {
        unsafe { OH_ArkWebResourceRequest_IsRedirect(self.raw.as_ptr()) }
    }
}

impl Drop for ResourceRequest {
    fn drop(&mut self) {
        unsafe {
            OH_ArkWebResourceRequest_Destroy(self.raw.as_ptr());
        }
    }
}
