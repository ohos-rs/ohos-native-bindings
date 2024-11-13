use std::os::raw::c_void;

use crate::GestureEventAction;

pub struct GestureEventData {
    pub event_action_type: GestureEventAction,
    pub event_action_data: GestureData,
    pub data: Option<*mut c_void>,
}

pub enum GestureData {
    Tap,
    Pan(PanGestureData),
    Pinch(PinchGestureData),
    Rotation(RotationGestureData),
    LongPress(LongPressGestureData),
    Swipe(SwipeGestureData),
}

pub struct PanGestureData {
    pub velocity: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

pub struct PinchGestureData {
    pub scale: f32,
    pub center_x: f32,
    pub center_y: f32,
}

pub struct RotationGestureData {
    pub angle: f32,
}

pub struct LongPressGestureData {
    pub repeat: i32,
}

pub struct SwipeGestureData {
    pub angle: f32,
    pub velocity: f32,
}
