use std::{cell::RefCell, os::raw::c_void};

use ohos_arkui_sys::ArkUI_ContextCallback;

struct UpdateCallbackContext {
    callback: Option<Box<dyn Fn()>>,
}

pub struct AnimationUpdateContext {
    callback_context: Box<RefCell<UpdateCallbackContext>>,
    raw: RefCell<ArkUI_ContextCallback>,
}

impl Default for AnimationUpdateContext {
    fn default() -> Self {
        let callback_context = Box::new(RefCell::new(UpdateCallbackContext { callback: None }));
        let user_data =
            callback_context.as_ref() as *const RefCell<UpdateCallbackContext> as *mut c_void;
        let raw = RefCell::new(ArkUI_ContextCallback {
            callback: Some(update_callback_trampoline),
            userData: user_data,
        });
        Self {
            callback_context,
            raw,
        }
    }
}

impl AnimationUpdateContext {
    pub(crate) fn raw(&self) -> *mut ArkUI_ContextCallback {
        self.raw.as_ptr()
    }

    pub fn callback<T: Fn() + 'static>(&self, callback: T) {
        self.callback_context.borrow_mut().callback = Some(Box::new(callback));
    }

    pub fn clear_callback(&self) {
        self.callback_context.borrow_mut().callback = None;
    }
}

unsafe extern "C" fn update_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let context = unsafe { &*(user_data as *const RefCell<UpdateCallbackContext>) };
    if let Some(callback) = context.borrow().callback.as_ref() {
        callback();
    }
}
