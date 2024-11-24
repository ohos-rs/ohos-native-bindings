use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::ArkUI_AnimateCompleteCallback;

use crate::AnimationFinishCallbackType;

pub struct AnimationFinishContext {
    pub(crate) callback: Rc<RefCell<Option<Box<dyn Fn(*mut c_void) -> ()>>>>,
    pub(crate) data: Rc<RefCell<Option<*mut c_void>>>,
    pub(crate) callback_type: Rc<RefCell<AnimationFinishCallbackType>>,
}

impl Default for AnimationFinishContext {
    fn default() -> Self {
        AnimationFinishContext {
            callback: Rc::new(RefCell::new(None)),
            data: Rc::new(RefCell::new(None)),
            callback_type: Rc::new(RefCell::new(AnimationFinishCallbackType::Removed)),
        }
    }
}

impl AnimationFinishContext {
    pub fn raw(&self) -> *mut ArkUI_AnimateCompleteCallback {
        let t = self.callback_type.borrow();
        let ctx = Box::new(ArkUI_AnimateCompleteCallback {
            callback: Some(finish),
            userData: self as *const AnimationFinishContext as *mut AnimationFinishContext
                as *mut c_void,
            type_: (*t).into(),
        });
        Box::into_raw(ctx)
    }
}

pub unsafe extern "C" fn finish(data: *mut c_void) {
    if !data.is_null() {
        let context = &*(data as *const AnimationFinishContext);

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
