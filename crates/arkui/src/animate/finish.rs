use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::ArkUI_AnimateCompleteCallback;

use crate::AnimationFinishCallbackType;

pub type FinishCallback = Rc<RefCell<Option<Box<dyn Fn(*mut c_void)>>>>;

struct CallbackContext {
    pub(crate) callback: FinishCallback,
    pub(crate) data: Rc<RefCell<Option<*mut c_void>>>,
    pub(crate) callback_type: Rc<RefCell<AnimationFinishCallbackType>>,
}

pub struct AnimationFinishContext {
    inner: Rc<RefCell<CallbackContext>>,
}

impl AnimationFinishContext {
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

    pub fn callback_type(&self, callback_type: AnimationFinishCallbackType) {
        let inner = self.inner.borrow();
        let mut t = inner.callback_type.borrow_mut();
        *t = callback_type;
    }
}

impl Default for AnimationFinishContext {
    fn default() -> Self {
        AnimationFinishContext {
            inner: Rc::new(RefCell::new(CallbackContext {
                callback: Rc::new(RefCell::new(None)),
                data: Rc::new(RefCell::new(None)),
                callback_type: Rc::new(RefCell::new(AnimationFinishCallbackType::Logically)),
            })),
        }
    }
}

impl AnimationFinishContext {
    pub fn raw(&self) -> *mut ArkUI_AnimateCompleteCallback {
        let inner_raw = self.inner.borrow();
        let t = inner_raw.callback_type.borrow();
        let inner = self.inner.clone();

        let ctx = Box::new(ArkUI_AnimateCompleteCallback {
            callback: Some(finish),
            userData: Box::into_raw(Box::new(inner)) as *mut c_void,
            type_: (*t).into(),
        });

        Box::into_raw(ctx)
    }
}

unsafe extern "C" fn finish(data: *mut c_void) {
    if data.is_null() {
        return;
    }
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
