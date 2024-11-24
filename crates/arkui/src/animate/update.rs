use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::ArkUI_ContextCallback;

pub struct AnimationUpdateContext {
    pub(crate) callback: Rc<RefCell<Option<Box<dyn Fn(*mut c_void) -> ()>>>>,
    pub(crate) data: Rc<RefCell<Option<*mut c_void>>>
}


pub unsafe extern "C" fn update() {

}
