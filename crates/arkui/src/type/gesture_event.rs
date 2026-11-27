//! Module type::gesture_event wrappers and related types.

use bitflags::bitflags;
use ohos_arkui_sys::{
    ArkUI_GestureEventActionType, ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END,
    ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE,
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT
                as ArkUI_GestureEventActionType;
        }
        if value.contains(GestureEventAction::Update) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE
                as ArkUI_GestureEventActionType;
        }
        if value.contains(GestureEventAction::End) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END
                as ArkUI_GestureEventActionType;
        }
        if value.contains(GestureEventAction::Cancel) {
            result |= ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL
                as ArkUI_GestureEventActionType;
        }
        result
    }
}

impl From<ArkUI_GestureEventActionType> for GestureEventAction {
    #[allow(clippy::unnecessary_cast)]
    fn from(value: ArkUI_GestureEventActionType) -> Self {
        let raw = value as u32;
        let mut action = GestureEventAction::empty();
        if raw & (ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT as u32) != 0 {
            action |= GestureEventAction::Accept;
        }
        if raw & (ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE as u32) != 0 {
            action |= GestureEventAction::Update;
        }
        if raw & (ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_END as u32) != 0 {
            action |= GestureEventAction::End;
        }
        if raw & (ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_CANCEL as u32) != 0 {
            action |= GestureEventAction::Cancel;
        }
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_combined_and_unknown_gesture_action_masks_without_panicking() {
        assert_eq!(
            GestureEventAction::from(0 as ArkUI_GestureEventActionType),
            GestureEventAction::empty()
        );
        let combined = (ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_ACCEPT
            | ArkUI_GestureEventActionType_GESTURE_EVENT_ACTION_UPDATE)
            as ArkUI_GestureEventActionType;
        assert_eq!(
            GestureEventAction::from(combined),
            GestureEventAction::Accept | GestureEventAction::Update
        );
        assert_eq!(
            GestureEventAction::from(0x8000_0001u32 as ArkUI_GestureEventActionType),
            GestureEventAction::Accept
        );
    }
}
