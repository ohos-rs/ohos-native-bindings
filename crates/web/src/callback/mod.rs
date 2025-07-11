use std::ffi::c_char;
use std::ffi::c_void;
use std::mem::ManuallyDrop;

pub(crate) struct WebComponentLifeCycleContext {
    pub(crate) callback: Box<dyn FnMut()>,
}

pub(crate) type OnControllerAttachContext = WebComponentLifeCycleContext;
pub(crate) type OnPageBeginContext = WebComponentLifeCycleContext;
pub(crate) type OnPageEndContext = WebComponentLifeCycleContext;
pub(crate) type OnDestroyContext = WebComponentLifeCycleContext;

pub unsafe extern "C" fn on_controller_attach(_web_tag: *const c_char, user_data: *mut c_void) {
    let mut ctx =
        unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut OnControllerAttachContext)) };
    let cb = &mut ctx.callback;
    cb();
}

pub unsafe extern "C" fn on_page_begin(_web_tag: *const c_char, user_data: *mut c_void) {
    let mut ctx = unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut OnPageBeginContext)) };
    let cb = &mut ctx.callback;
    cb();
}

pub unsafe extern "C" fn on_page_end(_web_tag: *const c_char, user_data: *mut c_void) {
    let mut ctx = unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut OnPageEndContext)) };
    let cb = &mut ctx.callback;
    cb();
}

pub unsafe extern "C" fn on_destroy(_web_tag: *const c_char, user_data: *mut c_void) {
    let mut ctx = unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut OnDestroyContext)) };
    let cb = &mut ctx.callback;
    cb();
}
