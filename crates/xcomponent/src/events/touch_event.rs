use ohos_xcomponent_sys::{OH_NativeXComponent_TouchEvent, OH_NativeXComponent_TouchPoint};

use crate::TouchEvent;

#[derive(Debug, Clone)]
pub struct TouchEventData {
    pub id: i32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub x: f32,
    pub y: f32,
    pub event_type: TouchEvent,
    pub size: f64,
    pub force: f32,
    pub device_id: i64,
    pub timestamp: i64,
    pub touch_points: Vec<TouchPointData>,
    pub num_points: u32,
}

impl Default for TouchEventData {
    fn default() -> Self {
        Self {
            id: 0,
            screen_x: 0.0,
            screen_y: 0.0,
            x: 0.0,
            y: 0.0,
            event_type: TouchEvent::Unknown,
            size: 0.0,
            force: 0.0,
            device_id: 0,
            timestamp: 0,
            touch_points: vec![TouchPointData::default(); 10],
            num_points: 0,
        }
    }
}

impl From<TouchEventData> for OH_NativeXComponent_TouchEvent {
    fn from(value: TouchEventData) -> Self {
        let default_value = OH_NativeXComponent_TouchPoint {
            id: 0,
            screenX: 0.0,
            screenY: 0.0,
            x: 0.0,
            y: 0.0,
            type_: 0,
            size: 0.0,
            force: 0.0,
            timeStamp: 0,
            isPressed: false,
        };
        let mut touch_points = [default_value; 10];

        for (i, point) in value.touch_points.iter().take(10).enumerate() {
            touch_points[i] = point.clone().into();
        }

        OH_NativeXComponent_TouchEvent {
            id: value.id,
            screenX: value.screen_x,
            screenY: value.screen_y,
            x: value.x,
            y: value.y,
            type_: value.event_type.into(),
            size: value.size,
            force: value.force,
            deviceId: value.device_id,
            timeStamp: value.timestamp,
            touchPoints: touch_points,
            numPoints: value.num_points,
        }
    }
}

impl From<OH_NativeXComponent_TouchEvent> for TouchEventData {
    fn from(value: OH_NativeXComponent_TouchEvent) -> Self {
        let mut touch_points = Vec::with_capacity(value.numPoints as usize);
        for point in value.touchPoints.iter().take(value.numPoints as usize) {
            touch_points.push(TouchPointData {
                id: point.id,
                screen_x: point.screenX,
                screen_y: point.screenY,
                x: point.x,
                y: point.y,
                event_type: point.type_.into(),
                size: point.size,
                force: point.force,
                timestamp: point.timeStamp,
                is_pressed: point.isPressed,
            });
        }

        Self {
            id: value.id,
            screen_x: value.screenX,
            screen_y: value.screenY,
            x: value.x,
            y: value.y,
            event_type: value.type_.into(),
            size: value.size,
            force: value.force,
            device_id: value.deviceId,
            timestamp: value.timeStamp,
            touch_points,
            num_points: value.numPoints,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TouchPointData {
    pub id: i32,
    pub screen_x: f32,
    pub screen_y: f32,
    pub x: f32,
    pub y: f32,
    pub event_type: TouchEvent,
    pub size: f64,
    pub force: f32,
    pub timestamp: i64,
    pub is_pressed: bool,
}

impl Default for TouchPointData {
    fn default() -> Self {
        Self {
            id: 0,
            screen_x: 0.0,
            screen_y: 0.0,
            x: 0.0,
            y: 0.0,
            event_type: TouchEvent::Unknown,
            size: 0.0,
            force: 0.0,
            timestamp: 0,
            is_pressed: false,
        }
    }
}

impl From<TouchPointData> for OH_NativeXComponent_TouchPoint {
    fn from(value: TouchPointData) -> Self {
        OH_NativeXComponent_TouchPoint {
            id: value.id,
            screenX: value.screen_x,
            screenY: value.screen_y,
            x: value.x,
            y: value.y,
            type_: value.event_type.into(),
            size: value.size,
            force: value.force,
            timeStamp: value.timestamp,
            isPressed: value.is_pressed,
        }
    }
}
