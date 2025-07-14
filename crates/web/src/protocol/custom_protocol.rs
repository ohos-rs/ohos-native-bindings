use std::{
    mem::ManuallyDrop,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_ResourceHandler, ArkWeb_ResourceRequest, ArkWeb_SchemeHandler,
    OH_ArkWebSchemeHandler_GetUserData, OH_ArkWebSchemeHandler_SetOnRequestStart,
    OH_ArkWebSchemeHandler_SetOnRequestStop, OH_ArkWeb_CreateSchemeHandler,
    OH_ArkWeb_DestroySchemeHandler,
};

pub struct CustomProtocolHandlerContext {
    request: Option<Box<dyn FnMut() -> bool>>,
    response: Option<Box<dyn FnMut()>>,
}

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

    pub fn on_request_start<F>(&self, mut handler: F)
    where
        F: FnMut() -> bool,
    {
        let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(self.raw.as_ptr()) };

        let mut user_data = unsafe {
            ManuallyDrop::new(Box::from_raw(
                user_data_raw as *mut CustomProtocolHandlerContext,
            ))
        };
        let static_on_request_start = unsafe {
            std::mem::transmute::<Box<dyn FnMut() -> bool>, Box<dyn FnMut() -> bool + 'static>>(
                Box::new(move || handler()),
            )
        };
        user_data.request = Some(Box::new(static_on_request_start));

        unsafe {
            OH_ArkWebSchemeHandler_SetOnRequestStart(self.raw.as_ptr(), Some(on_request_start));
        }
    }

    pub fn on_request_stop<F>(&self, mut handler: F)
    where
        F: FnMut(),
    {
        let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(self.raw.as_ptr()) };

        let mut user_data = unsafe {
            ManuallyDrop::new(Box::from_raw(
                user_data_raw as *mut CustomProtocolHandlerContext,
            ))
        };

        let static_on_request_stop = unsafe {
            std::mem::transmute::<Box<dyn FnMut()>, Box<dyn FnMut() + 'static>>(
                Box::new(move || handler()),
            )
        };

        user_data.response = Some(Box::new(static_on_request_stop));

        unsafe {
            OH_ArkWebSchemeHandler_SetOnRequestStop(self.raw.as_ptr(), Some(on_request_stop));
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

    let mut user_data = unsafe {
        ManuallyDrop::new(Box::from_raw(
            user_data_raw as *mut CustomProtocolHandlerContext,
        ))
    };

    if let Some(request) = &mut user_data.request {
        let ret = request();
        unsafe {
            *intercept = ret;
        }
    }
}

extern "C" fn on_request_stop(
    schema_handle: *const ArkWeb_SchemeHandler,
    resource_request: *const ArkWeb_ResourceRequest,
) {
    let user_data_raw = unsafe { OH_ArkWebSchemeHandler_GetUserData(schema_handle) };

    let mut user_data = unsafe {
        ManuallyDrop::new(Box::from_raw(
            user_data_raw as *mut CustomProtocolHandlerContext,
        ))
    };

    if let Some(response) = &mut user_data.response {
        response();
    }
}
