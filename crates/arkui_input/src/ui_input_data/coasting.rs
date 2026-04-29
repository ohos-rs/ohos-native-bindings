use ohos_arkui_input_sys::{
    ArkUI_CoastingAxisEvent, OH_ArkUI_CoastingAxisEvent_GetDeltaX,
    OH_ArkUI_CoastingAxisEvent_GetDeltaY, OH_ArkUI_CoastingAxisEvent_GetEventTime,
    OH_ArkUI_CoastingAxisEvent_GetPhase, OH_ArkUI_CoastingAxisEvent_SetPropagation,
    OH_ArkUI_UIInputEvent_GetCoastingAxisEvent,
};
use std::ptr::NonNull;

use crate::ui_input_data::check_status;
use crate::{ArkUIInputError, ArkUIInputEvent, UICoastingAxisEventPhase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArkUICoastingAxisEvent {
    event: NonNull<ArkUI_CoastingAxisEvent>,
}

impl ArkUICoastingAxisEvent {
    pub fn raw(&self) -> *mut ArkUI_CoastingAxisEvent {
        self.event.as_ptr()
    }

    pub fn event_time(&self) -> i64 {
        unsafe { OH_ArkUI_CoastingAxisEvent_GetEventTime(self.raw()) }
    }

    pub fn phase(&self) -> UICoastingAxisEventPhase {
        UICoastingAxisEventPhase::from(unsafe { OH_ArkUI_CoastingAxisEvent_GetPhase(self.raw()) })
    }

    pub fn delta_x(&self) -> f32 {
        unsafe { OH_ArkUI_CoastingAxisEvent_GetDeltaX(self.raw()) }
    }

    pub fn delta_y(&self) -> f32 {
        unsafe { OH_ArkUI_CoastingAxisEvent_GetDeltaY(self.raw()) }
    }

    pub fn set_propagation(&self, propagation: bool) -> Result<(), ArkUIInputError> {
        check_status(unsafe { OH_ArkUI_CoastingAxisEvent_SetPropagation(self.raw(), propagation) })
    }
}

impl ArkUIInputEvent {
    pub fn coasting_axis_event(&self) -> Option<ArkUICoastingAxisEvent> {
        let event = unsafe { OH_ArkUI_UIInputEvent_GetCoastingAxisEvent(self.raw_mut()) };
        NonNull::new(event).map(|event| ArkUICoastingAxisEvent { event })
    }
}
