use ohos_arkui_sys::ArkUI_NodeEvent;
use std::{cell::RefCell, rc::Rc};

pub struct Event(*mut ArkUI_NodeEvent);

impl Event {
    pub fn new(event: *mut ArkUI_NodeEvent) -> Self {
        Self(event)
    }

    pub fn raw(&self) -> *mut ArkUI_NodeEvent {
        self.0
    }
}

pub type NoParamClause = Rc<RefCell<dyn Fn() -> ()>>;

#[derive(Default, Clone)]
pub struct EventHandle {
    pub(crate) click: Option<NoParamClause>,
}
