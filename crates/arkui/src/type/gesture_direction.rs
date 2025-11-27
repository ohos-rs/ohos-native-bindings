use bitflags::bitflags;
use ohos_arkui_sys::{
    ArkUI_GestureDirection, ArkUI_GestureDirectionMask,
    ArkUI_GestureDirection_GESTURE_DIRECTION_ALL, ArkUI_GestureDirection_GESTURE_DIRECTION_DOWN,
    ArkUI_GestureDirection_GESTURE_DIRECTION_HORIZONTAL,
    ArkUI_GestureDirection_GESTURE_DIRECTION_LEFT, ArkUI_GestureDirection_GESTURE_DIRECTION_NONE,
    ArkUI_GestureDirection_GESTURE_DIRECTION_RIGHT, ArkUI_GestureDirection_GESTURE_DIRECTION_UP,
    ArkUI_GestureDirection_GESTURE_DIRECTION_VERTICAL,
};

bitflags! {
    #[derive(PartialEq)]
    pub struct GestureDirection: u32 {
        const All = 15;
        const Horizontal = 3;
        const Vertical = 12;
        const Left = 1;
        const Right = 2;
        const Up = 4;
        const Down = 8;
        const None = 0;
    }
}

impl From<GestureDirection> for ArkUI_GestureDirectionMask {
    fn from(value: GestureDirection) -> Self {
        let mut mask = 0;
        if value.contains(GestureDirection::All) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_ALL as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Horizontal) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_HORIZONTAL as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Vertical) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_VERTICAL as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Left) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_LEFT as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Right) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_RIGHT as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Up) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_UP as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::Down) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_DOWN as ArkUI_GestureDirection;
        }
        if value.contains(GestureDirection::None) {
            mask |= ArkUI_GestureDirection_GESTURE_DIRECTION_NONE as ArkUI_GestureDirection;
        }
        mask as ArkUI_GestureDirectionMask
    }
}

impl From<ArkUI_GestureDirection> for GestureDirection {
    fn from(value: ArkUI_GestureDirection) -> Self {
        match value {
            #![allow(non_upper_case_globals)]
            ArkUI_GestureDirection_GESTURE_DIRECTION_ALL => GestureDirection::All,
            ArkUI_GestureDirection_GESTURE_DIRECTION_HORIZONTAL => GestureDirection::Horizontal,
            ArkUI_GestureDirection_GESTURE_DIRECTION_VERTICAL => GestureDirection::Vertical,
            ArkUI_GestureDirection_GESTURE_DIRECTION_LEFT => GestureDirection::Left,
            ArkUI_GestureDirection_GESTURE_DIRECTION_RIGHT => GestureDirection::Right,
            ArkUI_GestureDirection_GESTURE_DIRECTION_UP => GestureDirection::Up,
            ArkUI_GestureDirection_GESTURE_DIRECTION_DOWN => GestureDirection::Down,
            ArkUI_GestureDirection_GESTURE_DIRECTION_NONE => GestureDirection::None,
            _ => unreachable!("Invalid GestureDirection value"),
        }
    }
}
