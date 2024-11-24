use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::ArkUI_ContextCallback;

pub struct AnimationUpdateContext {
    pub(crate) callback: Rc<RefCell<Option<Box<dyn Fn(*mut c_void) -> ()>>>>,
    pub(crate) data: Rc<RefCell<Option<*mut c_void>>>,
}

impl AnimationUpdateContext {
    pub fn raw(&self) -> *mut ArkUI_ContextCallback {
        let ctx = Box::new(ArkUI_ContextCallback {
            callback: Some(update),
            userData: self as *const AnimationUpdateContext as *mut AnimationUpdateContext
                as *mut c_void,
        });
        Box::into_raw(ctx)
    }
}

pub unsafe extern "C" fn update(data: *mut c_void) {
    if !data.is_null() {
        let context = &*(data as *const AnimationUpdateContext);

        let ctx_callback = context.callback.borrow_mut();
        let user_data = context.data.borrow_mut();

        if let Some(callback) = &*ctx_callback {
            if let Some(d) = *user_data {
                callback(d);
            } else {
                callback(std::ptr::null_mut())
            }
        }
    }
}
