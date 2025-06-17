use std::{
    cell::RefCell,
    ffi::{c_char, CStr, CString},
    rc::Rc,
    sync::{LazyLock, Mutex},
};

use crate::{error::ArkWebError, Callback, ARK_WEB_COMPONENT_API, CALLBACK_MAP};
use ohos_web_sys::OH_NativeArkWeb_RunJavaScript;

// store all web view instances
pub static WEB_VIEW_INSTANCE: LazyLock<papaya::HashMap<String, Web>> =
    LazyLock::new(papaya::HashMap::new);

pub static EVALUATE_SCRIPT_CALLBACK: LazyLock<Mutex<Option<Box<dyn Fn(String) + Sync + Send>>>> =
    LazyLock::new(|| Mutex::new(None));

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

    /// Evaluate js code and get result   
    /// Only one callback can be set at a time
    pub fn evaluate_js(
        &self,
        js: String,
        callback: Option<Box<dyn Fn(String) + Sync + Send + 'static>>,
    ) -> Result<(), ArkWebError> {
        if let Some(callback) = callback {
            let mut guard = EVALUATE_SCRIPT_CALLBACK
                .lock()
                .expect("Failed to lock EVALUATE_SCRIPT_CALLBACK");
            if (*guard).is_some() {
                return Err(ArkWebError::EvaluateScriptCallbackAlreadyExists);
            }
            *guard = Some(callback);
        }
        let js_code = CString::new(js).expect("Failed to create CString");
        unsafe {
            OH_NativeArkWeb_RunJavaScript(
                self.web_tag.as_ptr(),
                js_code.as_ptr().cast(),
                Some(on_evaluate_script_callback),
            );
        }

        Ok(())
    }
}

impl Drop for Web {
    fn drop(&mut self) {
        let map = WEB_VIEW_INSTANCE.pin();
        map.remove(&self.web_tag);
    }
}

extern "C" fn on_evaluate_script_callback(result: *const c_char) {
    let result = unsafe { CStr::from_ptr(result) };
    let result = result.to_string_lossy().to_string();
    let mut guard = EVALUATE_SCRIPT_CALLBACK
        .lock()
        .expect("Failed to lock EVALUATE_SCRIPT_CALLBACK");
    if let Some(callback) = (*guard).take() {
        callback(result);
    }
    *guard = None;
}
