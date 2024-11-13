use bitflags::bitflags;
use ohos_arkui_sys::{
    ArkUI_GestureEventActionType, ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE,
};

bitflags! {
    #[derive(PartialEq)]
    pub struct GestureEventAction: u32 {
        const Accept = 1;
        const Update = 2;
        const End = 4;
        const Cancel = 8;
    }
}

impl From<GestureEventAction> for ArkUI_GestureEventActionType {
    fn from(value: GestureEventAction) -> Self {
        let mut result = 0;

        if value.contains(GestureEventAction::Accept) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT as u32;
        }
        if value.contains(GestureEventAction::Update) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE as u32;
        }
        if value.contains(GestureEventAction::End) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END as u32;
        }
        if value.contains(GestureEventAction::Cancel) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL as u32;
        }
        result as ArkUI_GestureEventActionType
    }
}

impl From<ArkUI_GestureEventActionType> for GestureEventAction {
    fn from(value: ArkUI_GestureEventActionType) -> Self {
        match value {
            #![allow(non_upper_case_globals)]
            ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT => GestureEventAction::Accept,
            ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE => GestureEventAction::Update,
            ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END => GestureEventAction::End,
            ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL => GestureEventAction::Cancel,
            _ => unreachable!("Invalid ArkUI_GestureEventActionType"),
        }
    }
}
