//! Module gesture::gesture_data wrappers and related types.

use std::os::raw::c_void;

use ohos_arkui_input_binding::{UIInputAction, UIInputEvent, UIInputSourceType, UIInputToolType};

use crate::GestureEventAction;

/// Owned snapshot of the raw pointer input that produced a gesture callback.
///
/// ArkUI owns the underlying input event and only keeps it alive for the duration of the native
/// callback. These values can safely be retained and forwarded by higher-level runtimes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureInputData {
    /// Input event category reported by ArkUI.
    pub event_type: UIInputEvent,
    /// Raw pointer action associated with this gesture callback.
    pub action: UIInputAction,
    /// Device that produced the input.
    pub source_type: UIInputSourceType,
    /// Tool that produced the input.
    pub tool_type: UIInputToolType,
    /// Position relative to the upper-left corner of the gesture node.
    pub x: f32,
    /// Position relative to the upper-left corner of the gesture node.
    pub y: f32,
    /// Position relative to the upper-left corner of the window.
    pub window_x: f32,
    /// Position relative to the upper-left corner of the window.
    pub window_y: f32,
    /// Position relative to the upper-left corner of the display.
    pub display_x: f32,
    /// Position relative to the upper-left corner of the display.
    pub display_y: f32,
    /// Event timestamp supplied by ArkUI.
    pub timestamp: i64,
    /// Number of pointer contacts carried by the raw input event.
    pub pointer_count: u32,
    /// Primary pointer identifier when the raw input contains at least one pointer.
    pub pointer_id: Option<i32>,
}

/// Event payload passed to gesture callbacks.
pub struct GestureEventData {
    /// Gesture callback action type.
    pub event_action_type: GestureEventAction,
    /// Strongly typed gesture data.
    pub event_action_data: GestureData,
    /// Owned snapshot of the raw input event associated with the gesture callback.
    pub input: Option<GestureInputData>,
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
