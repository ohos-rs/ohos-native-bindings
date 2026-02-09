use crate::{MouseAction, MouseButton};
use ohos_xcomponent_sys::OH_NativeXComponent_MouseEvent;

#[derive(Debug, Clone)]
pub struct MouseEventData {
    pub x: f32,
    pub y: f32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub timestamp: i64,
    pub action: MouseAction,
    pub button: MouseButton,
}

impl From<OH_NativeXComponent_MouseEvent> for MouseEventData {
    fn from(value: OH_NativeXComponent_MouseEvent) -> Self {
        Self {
            x: value.x,
            y: value.y,
            screen_x: value.screenX,
            screen_y: value.screenY,
            timestamp: value.timestamp,
            action: value.action.into(),
            button: value.button.into(),
        }
    }
}
