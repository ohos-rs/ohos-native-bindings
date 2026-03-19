//! Module component::attribute::event wrappers and related types.

use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "api-22")]
use crate::TouchTestInfo;
use crate::{Event, NodeEventType, ARK_UI_NATIVE_NODE_API_1};

use super::ArkUIAttributeBasic;

/// Event registration helpers shared by components.
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

    fn on_custom_event<T: Fn(&crate::NodeCustomEvent) + 'static>(
        &mut self,
        event_type: crate::NodeCustomEventType,
        cb: T,
    ) {
        let node = self.borrow_mut();
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.register_node_custom_event_callback(node, event_type, 0, cb))
            .expect("ArkUI_NativeNodeAPI_1::registerNodeCustomEvent is None");
    }

    fn on_custom_measure<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnMeasure, cb);
    }

    fn on_custom_layout<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnLayout, cb);
    }

    fn on_custom_draw<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnDraw, cb);
    }

    fn on_custom_foreground_draw<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnForegroundDraw, cb);
    }

    fn on_custom_overlay_draw<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnOverlayDraw, cb);
    }

    #[cfg(feature = "api-20")]
    fn on_custom_draw_front<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnDrawFront, cb);
    }

    #[cfg(feature = "api-20")]
    fn on_custom_draw_behind<T: Fn(&crate::NodeCustomEvent) + 'static>(&mut self, cb: T) {
        self.on_custom_event(crate::NodeCustomEventType::OnDrawBehind, cb);
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

    fn on_touch_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::TouchEvent, cb);
    }

    fn on_area_change<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::EventOnAreaChange, cb);
    }

    fn on_visible_area_change<T: Fn(bool, f32) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::EventOnVisibleAreaChange, move |event| {
            cb(
                event.i32_value(0).unwrap_or_default() != 0,
                event.f32_value(1).unwrap_or_default(),
            );
        });
    }

    fn on_touch_intercept<T: Fn(&Event) -> Option<bool> + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnTouchIntercept, move |event| {
            if let Some(value) = cb(event) {
                let _ = event.set_return_bool(value);
            }
        });
    }

    fn on_mouse<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnMouse, cb);
    }

    fn on_accessibility_actions<T: Fn(u32) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnAccessibilityActions, move |event| {
            cb(event.u32_value(0).unwrap_or_default());
        });
    }

    fn on_pre_drag<T: Fn(Option<crate::PreDragStatus>) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnPreDrag, move |event| {
            cb(event.pre_drag_status());
        });
    }

    fn on_drag_start<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDragStart, cb);
    }

    fn on_drag_enter<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDragEnter, cb);
    }

    fn on_drag_move<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDragMove, cb);
    }

    fn on_drag_leave<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDragLeave, cb);
    }

    fn on_drop<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDrop, cb);
    }

    fn on_drag_end<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnDragEnd, cb);
    }

    #[cfg(feature = "api-14")]
    fn on_key_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnKeyEvent, cb);
    }

    #[cfg(feature = "api-14")]
    fn on_key_pre_ime<T: Fn(&Event) -> Option<bool> + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnKeyPreIme, move |event| {
            if let Some(value) = cb(event) {
                let _ = event.set_return_bool(value);
            }
        });
    }

    #[cfg(feature = "api-15")]
    fn on_focus_axis<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnFocusAxis, cb);
    }

    #[cfg(feature = "api-15")]
    fn on_dispatch_key_event<T: Fn(&Event) -> Option<bool> + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::DispatchKeyEvent, move |event| {
            if let Some(value) = cb(event) {
                let _ = event.set_return_bool(value);
            }
        });
    }

    #[cfg(feature = "api-17")]
    fn on_axis<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnAxis, cb);
    }

    #[cfg(feature = "api-18")]
    fn on_click_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnClickEvent, cb);
    }

    #[cfg(feature = "api-17")]
    fn on_hover_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnHoverEvent, cb);
    }

    #[cfg(feature = "api-17")]
    fn on_visible_area_approximate_change<T: Fn(bool, f32) + 'static>(&mut self, cb: T) {
        self.on_event(
            NodeEventType::VisibleAreaApproximateChangeEvent,
            move |event| {
                cb(
                    event.i32_value(0).unwrap_or_default() != 0,
                    event.f32_value(1).unwrap_or_default(),
                );
            },
        );
    }

    #[cfg(feature = "api-15")]
    fn on_hover_move<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnHoverMove, cb);
    }

    #[cfg(feature = "api-21")]
    fn on_size_change<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnSizeChange, cb);
    }

    #[cfg(feature = "api-22")]
    fn on_coasting_axis_event<T: Fn(&Event) + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnCoastingAxisEvent, cb);
    }

    #[cfg(feature = "api-22")]
    fn on_child_touch_test<T: Fn(&Event) -> Option<bool> + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnChildTouchTest, move |event| {
            if let Some(value) = cb(event) {
                let _ = event.set_return_bool(value);
            }
        });
    }

    #[cfg(feature = "api-22")]
    fn on_child_touch_test_with_info<
        T: Fn(&Event, Option<TouchTestInfo>) -> Option<bool> + 'static,
    >(
        &mut self,
        cb: T,
    ) {
        self.on_event(NodeEventType::OnChildTouchTest, move |event| {
            if let Some(value) = cb(event, event.touch_test_info()) {
                let _ = event.set_return_bool(value);
            }
        });
    }

    fn on_will_scroll<T: Fn(f32, i32, i32) -> Option<f32> + 'static>(&mut self, cb: T) {
        self.on_event(NodeEventType::OnWillScroll, move |event| {
            let offset = event.f32_value(0).unwrap_or_default();
            let state = event.i32_value(1).unwrap_or_default();
            let source = event.i32_value(2).unwrap_or_default();
            if let Some(value) = cb(offset, state, source) {
                let _ = event.set_return_f32(value);
            }
        });
    }
}
