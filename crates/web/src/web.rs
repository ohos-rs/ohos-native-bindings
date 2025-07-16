use std::ffi::{c_void, CString};

use ohos_web_sys::OH_ArkWeb_SetSchemeHandler;

use crate::{
    callback::{OnControllerAttachContext, OnDestroyContext, OnPageBeginContext, OnPageEndContext},
    error::ArkWebError,
    CustomProtocolHandler, ARK_WEB_COMPONENT_API,
};

#[derive(Debug, Clone)]
pub struct Web {
    web_tag: String,
}

impl Web {
    pub fn new(web_tag: String) -> Self {
        Self { web_tag }
    }

    pub fn on_controller_attach<F>(&self, mut callback: F) -> Result<(), ArkWebError>
    where
        F: FnMut(),
    {
        let cb = unsafe {
            std::mem::transmute::<Box<dyn FnMut()>, Box<dyn FnMut() + 'static>>(Box::new(
                move || {
                    callback();
                },
            ))
        };

        let ctx: Box<OnControllerAttachContext> =
            Box::new(OnControllerAttachContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        ARK_WEB_COMPONENT_API.on_controller_attached(self.web_tag.clone(), user_data)?;
        Ok(())
    }

    pub fn on_page_begin<F>(&self, mut callback: F) -> Result<(), ArkWebError>
    where
        F: FnMut(),
    {
        let cb = unsafe {
            std::mem::transmute::<Box<dyn FnMut()>, Box<dyn FnMut() + 'static>>(Box::new(
                move || {
                    callback();
                },
            ))
        };

        let ctx: Box<OnPageBeginContext> = Box::new(OnPageBeginContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        ARK_WEB_COMPONENT_API.on_page_begin(self.web_tag.clone(), user_data)?;
        Ok(())
    }

    pub fn on_page_end<F>(&self, mut callback: F) -> Result<(), ArkWebError>
    where
        F: FnMut(),
    {
        let cb = unsafe {
            std::mem::transmute::<Box<dyn FnMut()>, Box<dyn FnMut() + 'static>>(Box::new(
                move || {
                    callback();
                },
            ))
        };

        let ctx: Box<OnPageEndContext> = Box::new(OnPageEndContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        ARK_WEB_COMPONENT_API.on_page_end(self.web_tag.clone(), user_data)?;
        Ok(())
    }

    pub fn on_destroy<F>(&self, mut callback: F) -> Result<(), ArkWebError>
    where
        F: FnMut(),
    {
        let cb = unsafe {
            std::mem::transmute::<Box<dyn FnMut()>, Box<dyn FnMut() + 'static>>(Box::new(
                move || {
                    callback();
                },
            ))
        };

        let ctx: Box<OnDestroyContext> = Box::new(OnDestroyContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        ARK_WEB_COMPONENT_API.on_destroy(self.web_tag.clone(), user_data)?;
        Ok(())
    }

    /// Register a custom protocol handler for the web view.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol to register the handler for.
    /// * `handle` - The handler to register.
    ///
    /// ```ignore
    /// let web = Web::new("web_tag".to_string());
    ///
    /// let handler = CustomProtocolHandler::new();
    /// handler.on_request_start(|request, handle| {
    ///     handle.receive_data("Hello, world!");
    ///     true
    /// });
    /// handler.on_request_stop(|request| {
    ///     println!("Request stopped: {:?}", request);
    /// });
    ///
    /// web.custom_protocol("custom", handler).unwrap();
    /// ```
    pub fn custom_protocol<S>(
        &self,
        protocol: S,
        handle: CustomProtocolHandler,
    ) -> Result<bool, ArkWebError>
    where
        S: Into<String>,
    {
        let protocol: String = protocol.into();

        let tag = CString::new(self.web_tag.clone()).unwrap();
        let protocol = CString::new(protocol).unwrap();

        let ret = unsafe {
            OH_ArkWeb_SetSchemeHandler(protocol.as_ptr().cast(), tag.as_ptr().cast(), handle.raw())
        };
        let _ = Box::leak(Box::new(handle));
        Ok(ret)
    }
}
