//! Module animate::context::finish wrappers and related types.

use std::{cell::RefCell, os::raw::c_void};

use ohos_arkui_sys::ArkUI_AnimateCompleteCallback;

use crate::AnimationFinishCallbackType;

struct FinishCallbackContext {
    callback: Option<Box<dyn Fn()>>,
}

/// Callback context used for animation finish notifications.
pub struct AnimationFinishContext {
    callback_context: Box<RefCell<FinishCallbackContext>>,
    raw: RefCell<ArkUI_AnimateCompleteCallback>,
}

impl AnimationFinishContext {
    /// Registers finish callback closure.
    pub fn callback<T: Fn() + 'static>(&self, callback: T) {
        self.callback_context.borrow_mut().callback = Some(Box::new(callback));
    }

    /// Clears previously registered finish callback.
    pub fn clear_callback(&self) {
        self.callback_context.borrow_mut().callback = None;
    }

    /// Sets the finish callback type consumed by ArkUI.
    pub fn callback_type(&self, callback_type: AnimationFinishCallbackType) {
        self.raw.borrow_mut().type_ = callback_type.into();
    }
}

impl Default for AnimationFinishContext {
    fn default() -> Self {
        let callback_context = Box::new(RefCell::new(FinishCallbackContext { callback: None }));
        let user_data =
            callback_context.as_ref() as *const RefCell<FinishCallbackContext> as *mut c_void;
        let raw = RefCell::new(ArkUI_AnimateCompleteCallback {
            callback: Some(finish_callback_trampoline),
            userData: user_data,
            type_: AnimationFinishCallbackType::Logically.into(),
        });
        Self {
            callback_context,
            raw,
        }
    }
}

impl AnimationFinishContext {
    pub(crate) fn raw(&self) -> *mut ArkUI_AnimateCompleteCallback {
        self.raw.as_ptr()
    }
}

unsafe extern "C" fn finish_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let context = unsafe { &*(user_data as *const RefCell<FinishCallbackContext>) };
    if let Some(callback) = context.borrow().callback.as_ref() {
        callback();
    }
}
