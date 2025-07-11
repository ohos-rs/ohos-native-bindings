use std::ffi::c_void;

use crate::{
    callback::{OnControllerAttachContext, OnDestroyContext, OnPageBeginContext, OnPageEndContext},
    error::ArkWebError,
    ARK_WEB_COMPONENT_API,
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
}
