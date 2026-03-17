use std::{cell::RefCell, rc::Rc};

use crate::{Event, NodeEventType, ARK_UI_NATIVE_NODE_API_1};

use super::ArkUIAttributeBasic;

pub trait ArkUIEvent: ArkUIAttributeBasic {
    fn on_event<T: Fn(&Event) + 'static>(&mut self, event_type: NodeEventType, cb: T) {
        let node = self.borrow_mut();
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.register_node_event(node, event_type))
            .expect("ArkUI_NativeNodeAPI_1::registerNodeEvent is None");
        node.event_handle
            .set_event_callback(event_type, Rc::new(RefCell::new(cb)));
    }

    fn on_event_no_param<T: Fn() + 'static>(&mut self, event_type: NodeEventType, cb: T) {
        self.on_event(event_type, move |_| cb());
    }

    fn on_click<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::OnClick, cb);
    }

    fn on_click_with_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnClick, cb);
    }

    fn on_appear<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::EventOnAppear, cb);
    }

    fn on_disappear<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::EventOnDisappear, cb);
    }

    fn on_focus<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::OnFocus, cb);
    }

    fn on_blur<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::OnBlur, cb);
    }

    fn on_attach<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::EventOnAttach, cb);
    }

    fn on_detach<T: Fn() + 'static>(&mut self, cb: T) {
        self.on_event_no_param(NodeEventType::EventOnDetach, cb);
    }

    fn on_hover<T: Fn(bool) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnHover, move |event| {
            cb(event.i32_value(0).unwrap_or_default() != 0);
        });
    }
}
