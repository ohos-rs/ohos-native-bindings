use std::{
    mem::ManuallyDrop,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_ResourceHandler, ArkWeb_ResourceRequest, ArkWeb_SchemeHandler,
    OH_ArkWebResourceHandler_Destroy, OH_ArkWebResourceRequest_Destroy,
    OH_ArkWebSchemeHandler_GetUserData, OH_ArkWebSchemeHandler_SetOnRequestStart,
    OH_ArkWebSchemeHandler_SetOnRequestStop, OH_ArkWebSchemeHandler_SetUserData,
    OH_ArkWeb_CreateSchemeHandler, OH_ArkWeb_DestroySchemeHandler,
};

use crate::{ResourceHandle, ResourceRequest};

pub struct CustomProtocolHandlerContext {
    request: Option<Box<dyn FnMut(ResourceRequest, ResourceHandle) -> bool>>,
    response: Option<Box<dyn FnMut(ResourceRequest)>>,

    // can help us to release it
    resource_handle: Option<*mut ArkWeb_ResourceHandler>,
}

/// A custom protocol handler for the web view.
/// ```ignore
/// let handler = CustomProtocolHandler::new();
/// handler.on_request_start(|request, handle| {
///     handle.receive_data("Hello, world!");
///     true
/// });
/// ```
pub struct CustomProtocolHandler {
    raw: NonNull<ArkWeb_SchemeHandler>,
}

impl CustomProtocolHandler {
    pub fn new() -> Self {
        let mut raw = ptr::null_mut();

        unsafe {
            OH_ArkWeb_CreateSchemeHandler(&mut raw);

            #[cfg(debug_assertions)]
            assert!(!raw.is_null(), "Failed to create scheme handler");

            Self {
                raw: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn raw(&self) -> *mut ArkWeb_SchemeHandler {
        self.raw.as_ptr()
    }

    /// Set the callback for the request start event.
    ///
    /// # Arguments
    ///
    /// * `handler` - The callback to set.
    ///
    /// # Return
    /// * `true` if the request should be intercepted, `false` otherwise.
    ///
    /// # Example
    /// ```ignore   
    /// let handler = CustomProtocolHandler::new();
    /// handler.on_request_start(|request, handle| {
    ///     handle.receive_data("Hello, world!");
    ///     true
    /// });
    /// ```
    pub fn on_request_start<F>(&self, mut handler: F)
    where
        F: FnMut(ResourceRequest, ResourceHandle) -> bool,
    {
        let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(self.raw.as_ptr()) };

        let static_on_request_start = unsafe {
            std::mem::transmute::<
                Box<dyn FnMut(ResourceRequest, ResourceHandle) -> bool>,
                Box<dyn FnMut(ResourceRequest, ResourceHandle) -> bool + 'static>,
            >(Box::new(move |request, handle| handler(request, handle)))
        };

        match user_data_raw.is_null() {
            false => {
                let mut user_data = unsafe {
                    ManuallyDrop::new(Box::from_raw(
                        user_data_raw as *mut CustomProtocolHandlerContext,
                    ))
                };
                user_data.request = Some(Box::new(static_on_request_start));
            }
            true => {
                let user_data = Box::new(CustomProtocolHandlerContext {
                    request: Some(Box::new(static_on_request_start)),
                    response: None,
                    resource_handle: None,
                });
                let user_data_raw = Box::into_raw(user_data);
                unsafe {
                    OH_ArkWebSchemeHandler_SetUserData(self.raw.as_ptr(), user_data_raw as _);
                }
            }
        }

        unsafe {
            let ret =
                OH_ArkWebSchemeHandler_SetOnRequestStart(self.raw.as_ptr(), Some(on_request_start));
            #[cfg(debug_assertions)]
            assert!(ret == 0, "Failed to set on request start");
        }
    }

    /// Set the callback for the request stop event.
    pub fn on_request_stop<F>(&self, mut handler: F)
    where
        F: FnMut(ResourceRequest),
    {
        let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(self.raw.as_ptr()) };

        let static_on_request_stop = unsafe {
            std::mem::transmute::<
                Box<dyn FnMut(ResourceRequest)>,
                Box<dyn FnMut(ResourceRequest) + 'static>,
            >(Box::new(move |handle| handler(handle)))
        };

        match user_data_raw.is_null() {
            false => {
                let mut user_data = unsafe {
                    ManuallyDrop::new(Box::from_raw(
                        user_data_raw as *mut CustomProtocolHandlerContext,
                    ))
                };
                user_data.response = Some(Box::new(static_on_request_stop));
            }
            true => {
                let user_data = Box::new(CustomProtocolHandlerContext {
                    request: None,
                    response: Some(Box::new(static_on_request_stop)),
                    resource_handle: None,
                });
                let user_data_raw = Box::into_raw(user_data);
                unsafe {
                    OH_ArkWebSchemeHandler_SetUserData(self.raw.as_ptr(), user_data_raw as _);
                }
            }
        }

        unsafe {
            let ret =
                OH_ArkWebSchemeHandler_SetOnRequestStop(self.raw.as_ptr(), Some(on_request_stop));
            #[cfg(debug_assertions)]
            assert!(ret == 0, "Failed to set on request stop");
        }
    }
}

impl Drop for CustomProtocolHandler {
    fn drop(&mut self) {
        unsafe {
            OH_ArkWeb_DestroySchemeHandler(self.raw.as_ptr());
        }
    }
}

extern "C" fn on_request_start(
    schema_handle: *const ArkWeb_SchemeHandler,
    resource_request: *mut ArkWeb_ResourceRequest,
    resource_handler: *const ArkWeb_ResourceHandler,
    intercept: *mut bool,
) {
    let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(schema_handle) };

    if user_data_raw.is_null() {
        unsafe {
            *intercept = false;
        }
        return;
    }

    let mut user_data = unsafe {
        ManuallyDrop::new(Box::from_raw(
            user_data_raw as *mut CustomProtocolHandlerContext,
        ))
    };

    user_data.resource_handle = Some(resource_handler as _);

    if let Some(request) = &mut user_data.request {
        let ret = request(
            ResourceRequest::new(resource_request),
            ResourceHandle::new(resource_handler as _),
        );
        unsafe {
            *intercept = ret;
        }
    } else {
        unsafe {
            *intercept = false;
        }
    }
}

extern "C" fn on_request_stop(
    schema_handle: *const ArkWeb_SchemeHandler,
    resource_request: *const ArkWeb_ResourceRequest,
) {
    let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(schema_handle) };

    if user_data_raw.is_null() {
        return;
    }

    let mut user_data = unsafe {
        ManuallyDrop::new(Box::from_raw(
            user_data_raw as *mut CustomProtocolHandlerContext,
        ))
    };

    if let Some(response) = &mut user_data.response {
        response(ResourceRequest::new(resource_request as _));
    }

    // release the resource
    unsafe {
        OH_ArkWebResourceRequest_Destroy(resource_request);
        if let Some(resource_handle) = user_data.resource_handle {
            OH_ArkWebResourceHandler_Destroy(resource_handle);
        }
    }
}
