use std::ffi::{c_void, CString};

use crate::{ArkWebError, JsApiCallbackContext, ARK_WEB_CONTROLLER_API};
use ohos_web_sys::{ArkWeb_ProxyMethod, ArkWeb_ProxyObject};

use super::ark_web_proxy_method;

pub struct WebProxy {
    web_tag: String,
    #[allow(dead_code)]
    object_name: String,
}

impl WebProxy {
    pub fn new(web_tag: String, object_name: String) -> Self {
        Self {
            web_tag,
            object_name,
        }
    }

    /// Refresh current page
    ///
    /// If you want register new proxy without onControllerAttach, you can call this method to refresh current page
    pub fn refresh(&self) -> Result<(), ArkWebError> {
        ARK_WEB_CONTROLLER_API.refresh(self.web_tag.clone())?;

        Ok(())
    }
}

struct ArkWebProxyMethod {
    js_name: CString, // Store CString instead of raw pointer
    user_data: *mut c_void,
}

/// Builder for WebProxy
///
/// # Example
///
/// ```ignore
/// let proxy = WebProxyBuilder::new("webview_tag", "ipc")
///     .add_method("postMessage", |args: Vec<String>| {
///         println!("postMessage called with args: {:?}", args);
///     })
///     .build()
///     .unwrap();
/// ```
///
pub struct WebProxyBuilder {
    web_tag: String,
    object_name: String,

    js_methods: Vec<ArkWebProxyMethod>,
}

impl WebProxyBuilder {
    pub fn new<S: Into<String>>(web_tag: S, object_name: S) -> Self {
        Self {
            web_tag: web_tag.into(),
            object_name: object_name.into(),
            js_methods: Vec::new(),
        }
    }

    /// Add a method to the proxy
    ///
    /// # Arguments
    ///
    /// * `js_name` - The name of the method in JS
    /// * `callback` - The callback function to be called when the method is called
    ///
    pub fn add_method<S, F>(self, js_name: S, callback: F) -> WebProxyBuilder
    where
        S: Into<String>,
        F: FnMut(String, Vec<String>),
    {
        let js_name =
            CString::new(js_name.into()).expect("Failed to create CString for method name");

        let cb: Box<dyn FnMut(String, Vec<String>)> = unsafe {
            std::mem::transmute::<
                Box<dyn FnMut(String, Vec<String>)>,
                Box<dyn FnMut(String, Vec<String>) + 'static>,
            >(Box::new(callback))
        };

        let ctx: Box<JsApiCallbackContext> = Box::new(JsApiCallbackContext { callback: cb });
        let user_data = Box::into_raw(ctx) as *mut c_void;

        let method = ArkWebProxyMethod { js_name, user_data };

        let mut methods = self.js_methods;
        methods.push(method);

        WebProxyBuilder {
            js_methods: methods,
            ..self
        }
    }

    pub fn build(self) -> Result<WebProxy, ArkWebError> {
        let obj_name = CString::new(self.object_name.clone())
            .expect("Failed to create CString for object name");

        let method_list = self
            .js_methods
            .iter()
            .map(|method| ArkWeb_ProxyMethod {
                methodName: method.js_name.as_ptr().cast(),
                callback: Some(ark_web_proxy_method),
                userData: method.user_data,
            })
            .collect::<Vec<_>>();

        let obj = ArkWeb_ProxyObject {
            objName: obj_name.as_ptr().cast(),
            methodList: method_list.as_ptr().cast(),
            size: method_list.len(),
        };

        ARK_WEB_CONTROLLER_API
            .register_javascript_proxy(self.web_tag.clone(), &obj as *const ArkWeb_ProxyObject)?;

        Ok(WebProxy {
            web_tag: self.web_tag,
            object_name: self.object_name,
        })
    }
}
