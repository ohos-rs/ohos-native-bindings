use std::cell::RefCell;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::rc::Rc;
use std::sync::LazyLock;

#[derive(Default, Clone)]
pub struct Callback {
    pub(crate) controller_attach: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
    pub(crate) page_begin: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
    pub(crate) page_end: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
    pub(crate) destroy: Rc<RefCell<Option<Box<dyn FnMut()>>>>,
}

unsafe impl Send for Callback {}
unsafe impl Sync for Callback {}

pub static CALLBACK_MAP: LazyLock<papaya::HashMap<String, Callback>> =
    LazyLock::new(papaya::HashMap::new);

pub unsafe extern "C" fn on_controller_attach(web_tag: *const c_char, _data: *mut c_void) {
    let web_tag = CStr::from_ptr(web_tag).to_string_lossy().into_owned();

    let map = CALLBACK_MAP.pin();
    if let Some(callback) = map.get(&web_tag) {
        let mut callback = callback.controller_attach.borrow_mut();
        if let Some(callback) = &mut *callback {
            callback();
        }
    }
}

pub unsafe extern "C" fn on_page_begin(web_tag: *const c_char, _data: *mut c_void) {
    let web_tag = CStr::from_ptr(web_tag).to_string_lossy().into_owned();

    let map = CALLBACK_MAP.pin();
    if let Some(callback) = map.get(&web_tag) {
        let mut callback = callback.page_begin.borrow_mut();
        if let Some(callback) = &mut *callback {
            callback();
        }
    }
}

pub unsafe extern "C" fn on_page_end(web_tag: *const c_char, _data: *mut c_void) {
    let web_tag = CStr::from_ptr(web_tag).to_string_lossy().into_owned();

    let map = CALLBACK_MAP.pin();
    if let Some(callback) = map.get(&web_tag) {
        let mut callback = callback.page_end.borrow_mut();
        if let Some(callback) = &mut *callback {
            callback();
        }
    }
}

pub unsafe extern "C" fn on_destroy(web_tag: *const c_char, _data: *mut c_void) {
    let web_tag = CStr::from_ptr(web_tag).to_string_lossy().into_owned();

    let map = CALLBACK_MAP.pin();
    map.remove(&web_tag);
}
