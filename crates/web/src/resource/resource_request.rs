use std::{
    collections::HashMap,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_ResourceRequest, OH_ArkWebRequestHeaderList_Destroy,
    OH_ArkWebRequestHeaderList_GetHeader, OH_ArkWebRequestHeaderList_GetSize,
    OH_ArkWebResourceRequest_Destroy, OH_ArkWebResourceRequest_GetFrameUrl,
    OH_ArkWebResourceRequest_GetHttpBodyStream, OH_ArkWebResourceRequest_GetMethod,
    OH_ArkWebResourceRequest_GetReferrer, OH_ArkWebResourceRequest_GetRequestHeaders,
    OH_ArkWebResourceRequest_GetResourceType, OH_ArkWebResourceRequest_GetUrl,
    OH_ArkWebResourceRequest_HasGesture, OH_ArkWebResourceRequest_IsMainFrame,
    OH_ArkWebResourceRequest_IsRedirect,
};

use crate::{
    HttpBodyStream, Method, Referrer, ResourceType, ResponseHeaderKey, ResponseHeaderValue, Url,
};

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

    /// Get the url of the request
    pub fn url(&self) -> String {
        let mut url = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetUrl(self.raw.as_ptr(), &mut url);
            Url { raw: url }.into()
        }
    }

    /// Get all headers of the request
    pub fn headers(&self) -> HashMap<String, String> {
        let mut all_headers: HashMap<String, String> = HashMap::new();

        unsafe {
            let mut headers = ptr::null_mut();
            OH_ArkWebResourceRequest_GetRequestHeaders(self.raw.as_ptr(), &mut headers);

            let size = OH_ArkWebRequestHeaderList_GetSize(headers);

            for i in 0..size {
                let mut header_key = ptr::null_mut();
                let mut header_value = ptr::null_mut();
                OH_ArkWebRequestHeaderList_GetHeader(
                    headers,
                    i,
                    &mut header_key,
                    &mut header_value,
                );
                let key = ResponseHeaderKey { raw: header_key }.into();
                let value = ResponseHeaderValue { raw: header_value }.into();
                all_headers.insert(key, value);
            }
            OH_ArkWebRequestHeaderList_Destroy(headers);
        };

        all_headers
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
