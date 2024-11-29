use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::ArkUI_ContextCallback;

struct CallbackContext {
    pub(crate) callback: Rc<RefCell<Option<Box<dyn Fn(*mut c_void) -> ()>>>>,
    pub(crate) data: Rc<RefCell<Option<*mut c_void>>>,
}

pub struct AnimationUpdateContext {
    inner: Rc<RefCell<CallbackContext>>,
}

impl Default for AnimationUpdateContext {
    fn default() -> Self {
        AnimationUpdateContext {
            inner: Rc::new(RefCell::new(CallbackContext {
                callback: Rc::new(RefCell::new(None)),
                data: Rc::new(RefCell::new(None)),
            })),
        }
    }
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

    pub fn callback<T: Fn(*mut c_void) + 'static>(&self, callback: T) {
        let inner = self.inner.borrow();
        let mut cb = inner.callback.borrow_mut();
        *cb = Some(Box::new(callback));
    }

    pub fn data(&self, data: *mut c_void) {
        let inner = self.inner.borrow();
        let mut d = inner.data.borrow_mut();
        *d = Some(data);
    }
}

pub unsafe extern "C" fn update(data: *mut c_void) {
    let context_ptr = Box::from_raw(data as *mut Rc<RefCell<Option<CallbackContext>>>);
    let context = context_ptr.borrow();

    if let Some(ctx) = context.as_ref() {
        let cb = ctx.callback.borrow();
        if let Some(f) = cb.as_ref() {
            let d = ctx.data.borrow();
            if let Some(data) = *d {
                f(data);
            } else {
                f(std::ptr::null_mut());
            }
        }
    }
}
