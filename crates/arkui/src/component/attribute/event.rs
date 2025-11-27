use std::{cell::RefCell, rc::Rc};

use crate::{NodeEventType, ARK_UI_NATIVE_NODE_API_1};

use super::ArkUIAttributeBasic;

pub trait ArkUIEvent: ArkUIAttributeBasic {
    fn on_click<T: Fn() + 'static>(&mut self, cb: T) {
        let node = self.borrow_mut();
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.register_node_event(node, NodeEventType::OnClick))
            .expect("ArkUI_NativeNodeAPI_1::registerNodeEvent is None");
        node.event_handle.click = Some(Rc::new(RefCell::new(cb)));
    }
}
