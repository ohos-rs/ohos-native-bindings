//! Module gesture::gesture_data wrappers and related types.

use std::os::raw::c_void;

use crate::GestureEventAction;

/// Event payload passed to gesture callbacks.
pub struct GestureEventData {
    /// Gesture callback action type.
    pub event_action_type: GestureEventAction,
    /// Strongly typed gesture data.
    pub event_action_data: GestureData,
    /// Optional user data pointer provided during registration.
    pub data: Option<*mut c_void>,
}

/// Concrete gesture payload variants.
pub enum GestureData {
    /// Tap gesture with no extra payload.
    Tap,
    /// Pan gesture payload.
    Pan(PanGestureData),
    /// Pinch gesture payload.
    Pinch(PinchGestureData),
    /// Rotation gesture payload.
    Rotation(RotationGestureData),
    /// Long-press gesture payload.
    LongPress(LongPressGestureData),
    /// Swipe gesture payload.
    Swipe(SwipeGestureData),
}

/// Pan gesture data.
pub struct PanGestureData {
    /// Composite velocity.
    pub velocity: f32,
    /// Velocity on X axis.
    pub velocity_x: f32,
    /// Velocity on Y axis.
    pub velocity_y: f32,
    /// Offset on X axis.
    pub offset_x: f32,
    /// Offset on Y axis.
    pub offset_y: f32,
}

/// Pinch gesture data.
pub struct PinchGestureData {
    /// Scale factor.
    pub scale: f32,
    /// Pinch center x.
    pub center_x: f32,
    /// Pinch center y.
    pub center_y: f32,
}

/// Rotation gesture data.
pub struct RotationGestureData {
    /// Rotation angle.
    pub angle: f32,
}

/// Long-press gesture data.
pub struct LongPressGestureData {
    /// Repeat count.
    pub repeat: i32,
}

/// Swipe gesture data.
pub struct SwipeGestureData {
    /// Swipe angle.
    pub angle: f32,
    /// Swipe velocity.
    pub velocity: f32,
}
