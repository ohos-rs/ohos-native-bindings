use std::{cell::RefCell, rc::Rc, sync::LazyLock};

use crate::{
    ark_web_member_missing, error::ArkWebError, Callback, ARK_WEB_COMPONENT_API, CALLBACK_MAP,
};

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
        let t = map.insert(web_tag.clone(), new_instance.clone());
        if let Some(inst) = t {
            return Ok(inst.to_owned());
        }
        Err(ArkWebError::WebviewCreateFailed(web_tag))
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
        let api = ARK_WEB_COMPONENT_API.raw.as_ptr();
        if ark_web_member_missing!(api, onControllerAttached) {
            return Err(ArkWebError::ArkWebApiMemberMissing(
                "onControllerAttached".to_string(),
            ));
        }
        ARK_WEB_COMPONENT_API.on_controller_attached(self.web_tag.clone())?;
        Ok(())
    }
}

impl Drop for Web {
    fn drop(&mut self) {
        let map = WEB_VIEW_INSTANCE.pin();
        map.remove(&self.web_tag);
    }
}
