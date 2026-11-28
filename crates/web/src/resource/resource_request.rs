use std::{
    collections::HashMap,
    ffi::c_void,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_ResourceRequest, OH_ArkWebRequestHeaderList_Destroy,
    OH_ArkWebRequestHeaderList_GetHeader, OH_ArkWebRequestHeaderList_GetSize,
    OH_ArkWebResourceRequest_GetFrameUrl, OH_ArkWebResourceRequest_GetHttpBodyStream,
    OH_ArkWebResourceRequest_GetMethod, OH_ArkWebResourceRequest_GetReferrer,
    OH_ArkWebResourceRequest_GetRequestHeaders, OH_ArkWebResourceRequest_GetResourceType,
    OH_ArkWebResourceRequest_GetUrl, OH_ArkWebResourceRequest_GetUserData,
    OH_ArkWebResourceRequest_HasGesture, OH_ArkWebResourceRequest_IsMainFrame,
    OH_ArkWebResourceRequest_IsRedirect, OH_ArkWebResourceRequest_SetUserData,
};

use crate::{
    ArkWebError, HttpBodyStream, Method, Referrer, ResourceType, ResponseHeaderKey,
    ResponseHeaderValue, Url,
};

#[derive(Debug, PartialEq, Eq, Hash)]
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

    /// Returns the underlying native `ArkWeb_ResourceRequest` pointer.
    pub fn raw(&self) -> *mut ArkWeb_ResourceRequest {
        self.raw.as_ptr()
    }

    /// Returns the numeric address of the native `ArkWeb_ResourceRequest` handle,
    /// suitable for correlating the same request across `on_request_start` and
    /// `on_request_stop`.
    pub fn as_id(&self) -> usize {
        self.raw.as_ptr() as usize
    }

    /// Stores an opaque integer token on the native `ArkWeb_ResourceRequest` via
    /// `OH_ArkWebResourceRequest_SetUserData`.
    pub fn set_user_token(&self, token: usize) -> Result<(), ArkWebError> {
        let ret = unsafe {
            OH_ArkWebResourceRequest_SetUserData(
                self.raw.as_ptr(),
                ptr::without_provenance_mut(token),
            )
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(ArkWebError::NativeError {
                operation: "OH_ArkWebResourceRequest_SetUserData",
                code: ret as u32,
            })
        }
    }

    /// Reads back the integer token previously stored with [`Self::set_user_token`]
    /// (`0` if unset).
    pub fn user_token(&self) -> usize {
        unsafe { OH_ArkWebResourceRequest_GetUserData(self.raw.as_ptr()) }.addr()
    }

    /// Attaches a raw `user_data` pointer to this native `ArkWeb_ResourceRequest`
    /// via `OH_ArkWebResourceRequest_SetUserData`.
    ///
    /// # Safety
    ///
    /// Callers that later dereference [`Self::user_data`] must ensure `user_data`
    /// remains valid and properly aligned for that access.
    pub unsafe fn set_user_data(&self, user_data: *mut c_void) -> Result<(), ArkWebError> {
        let ret = unsafe { OH_ArkWebResourceRequest_SetUserData(self.raw.as_ptr(), user_data) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ArkWebError::NativeError {
                operation: "OH_ArkWebResourceRequest_SetUserData",
                code: ret as u32,
            })
        }
    }

    /// Returns the raw `user_data` pointer attached to this native
    /// `ArkWeb_ResourceRequest` via `OH_ArkWebResourceRequest_GetUserData`.
    pub fn user_data(&self) -> *mut c_void {
        unsafe { OH_ArkWebResourceRequest_GetUserData(self.raw.as_ptr()) }
    }

    /// Obtain an owned body stream on the ArkWeb I/O thread. Reads initialize
    /// automatically; use `initialize` first if metadata is needed before reading.
    pub fn http_body_stream(&self) -> Option<HttpBodyStream> {
        let mut raw = ptr::null_mut();
        unsafe {
            OH_ArkWebResourceRequest_GetHttpBodyStream(self.raw.as_ptr(), &mut raw);
            // Construction only checks null; registration and initialization
            // errors are reported by the operation's completion callback.
            HttpBodyStream::new(raw).ok()
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
