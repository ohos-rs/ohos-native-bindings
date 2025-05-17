use std::sync::LazyLock;

use crate::error::ArkWebError;

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

    pub fn on_controller_attach<T>(&self, callback: T)
    where
        T: Fn(&mut Self),
    {
    }
}

impl Drop for Web {
    fn drop(&mut self) {
        let map = WEB_VIEW_INSTANCE.pin();
        map.remove(&self.web_tag);
    }
}
