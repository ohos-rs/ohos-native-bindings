use std::{
    cell::RefCell,
    ffi::{c_void, CString},
    rc::Rc,
    sync::LazyLock,
};

use crate::{
    ark_web_proxy_method, error::ArkWebError, Callback, JsApiCallbackContext,
    ARK_WEB_COMPONENT_API, ARK_WEB_CONTROLLER_API, CALLBACK_MAP,
};
use ohos_web_sys::{ArkWeb_ProxyMethod, ArkWeb_ProxyObject};

// store all web view instances
pub static WEB_VIEW_INSTANCE: LazyLock<papaya::HashMap<String, Web>> =
    LazyLock::new(papaya::HashMap::new);

#[derive(Debug, Clone)]
pub struct Web {
    web_tag: String,
}

impl Web {
    pub fn new(web_tag: String) -> Result<Self, ArkWebError> {
        let map = WEB_VIEW_INSTANCE.pin();
        let instance = map.get(&web_tag);
        if let Some(inst) = instance {
            #[cfg(debug_assertions)]
            println!("Web view instance already exists: {}", web_tag);

            return Ok(inst.to_owned());
        }
        let new_instance = Self {
            web_tag: web_tag.clone(),
        };
        _ = map.insert(web_tag.clone(), new_instance.clone());
        Ok(new_instance.clone())
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
        let attach = Rc::new(RefCell::new(Some(cb)));
        let update = attach.clone();
        let map = CALLBACK_MAP.pin();
        map.update_or_insert(
            self.web_tag.clone(),
            move |e| {
                let mut e = e.clone();
                e.controller_attach = update.clone();
                e.clone()
            },
            Callback {
                controller_attach: attach.clone(),
                page_begin: Rc::new(RefCell::new(None)),
                page_end: Rc::new(RefCell::new(None)),
                destroy: Rc::new(RefCell::new(None)),
            },
        );
        ARK_WEB_COMPONENT_API.on_controller_attached(self.web_tag.clone())?;
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
        let attach = Rc::new(RefCell::new(Some(cb)));
        let update = attach.clone();
        let map = CALLBACK_MAP.pin();
        map.update_or_insert(
            self.web_tag.clone(),
            move |e| {
                let mut e = e.clone();
                e.page_begin = update.clone();
                e.clone()
            },
            Callback {
                controller_attach: Rc::new(RefCell::new(None)),
                page_begin: attach.clone(),
                page_end: Rc::new(RefCell::new(None)),
                destroy: Rc::new(RefCell::new(None)),
            },
        );
        ARK_WEB_COMPONENT_API.on_page_begin(self.web_tag.clone())?;
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
        let attach = Rc::new(RefCell::new(Some(cb)));
        let update = attach.clone();
        let map = CALLBACK_MAP.pin();
        map.update_or_insert(
            self.web_tag.clone(),
            move |e| {
                let mut e = e.clone();
                e.page_end = update.clone();
                e.clone()
            },
            Callback {
                controller_attach: Rc::new(RefCell::new(None)),
                page_begin: Rc::new(RefCell::new(None)),
                page_end: attach.clone(),
                destroy: Rc::new(RefCell::new(None)),
            },
        );
        ARK_WEB_COMPONENT_API.on_page_end(self.web_tag.clone())?;
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
        let attach = Rc::new(RefCell::new(Some(cb)));
        let update = attach.clone();
        let map = CALLBACK_MAP.pin();
        map.update_or_insert(
            self.web_tag.clone(),
            move |e| {
                let mut e = e.clone();
                e.destroy = update.clone();
                e.clone()
            },
            Callback {
                controller_attach: Rc::new(RefCell::new(None)),
                page_begin: Rc::new(RefCell::new(None)),
                page_end: Rc::new(RefCell::new(None)),
                destroy: attach.clone(),
            },
        );
        Ok(())
    }

    pub fn register_js_api<S, F>(
        &self,
        obj_name: S,
        method_name: S,
        callback: F,
    ) -> Result<(), ArkWebError>
    where
        S: Into<String>,
        F: FnMut(String, Vec<String>),
    {
        let obj_name = CString::new(obj_name.into())
            .map_err(|e| ArkWebError::JsApiRegisterFailed(e.to_string()))?;
        let method_name = CString::new(method_name.into())
            .map_err(|e| ArkWebError::JsApiRegisterFailed(e.to_string()))?;

        let cb: Box<dyn FnMut(String, Vec<String>)> = unsafe {
            std::mem::transmute::<
                Box<dyn FnMut(String, Vec<String>)>,
                Box<dyn FnMut(String, Vec<String>) + 'static>,
            >(Box::new(callback))
        };

        let ctx: Box<JsApiCallbackContext> = Box::new(JsApiCallbackContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        let method = ArkWeb_ProxyMethod {
            methodName: method_name.as_ptr().cast(),
            callback: Some(ark_web_proxy_method),
            userData: user_data,
        };

        let obj = ArkWeb_ProxyObject {
            objName: obj_name.as_ptr().cast(),
            methodList: &method as *const ArkWeb_ProxyMethod,
            size: 1,
        };

        // Keep CString and method alive during the function call
        let _ = ARK_WEB_CONTROLLER_API
            .register_javascript_proxy(self.web_tag.clone(), &obj as *const ArkWeb_ProxyObject)?;

        Ok(())
    }
}

impl Drop for Web {
    fn drop(&mut self) {
        let map = WEB_VIEW_INSTANCE.pin();
        map.remove(&self.web_tag);
    }
}
