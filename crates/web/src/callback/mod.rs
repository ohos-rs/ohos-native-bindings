use std::sync::LazyLock;
use std::sync::RwLock;

use crate::web::Web;

pub struct Callback {
    pub(crate) controller_attach: Option<Box<dyn FnMut()>>,
    pub(crate) page_begin: Option<Box<dyn FnMut()>>,
    pub(crate) page_end: Option<Box<dyn FnMut()>>,
    pub(crate) destroy: Option<Box<dyn FnMut()>>,
}

unsafe impl Send for Callback {}
unsafe impl Sync for Callback {}

pub static CALLBACK_MAP: LazyLock<papaya::HashMap<String, RwLock<Callback>>> =
    LazyLock::new(papaya::HashMap::new);        

pub unsafe extern "C" fn on_controller_attach(web_tag: String) {
    let map = CALLBACK_MAP.pin();
    if let Some(callback) = map.get(&web_tag) {
        if let Some(callback) = callback.write().unwrap().controller_attach.as_mut() {
            callback();
        }
    }
}
