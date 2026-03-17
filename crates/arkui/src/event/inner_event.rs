use ohos_arkui_sys::{
    ArkUI_NodeEvent, ArkUI_NumberValue, OH_ArkUI_NodeEvent_GetEventType,
    OH_ArkUI_NodeEvent_GetInputEvent, OH_ArkUI_NodeEvent_GetNumberValue,
    OH_ArkUI_NodeEvent_GetPreDragStatus, OH_ArkUI_NodeEvent_GetStringAsyncEvent,
    OH_ArkUI_NodeEvent_GetStringValue, OH_ArkUI_NodeEvent_GetTargetId,
    OH_ArkUI_NodeEvent_SetReturnNumberValue,
};
use std::{cell::RefCell, ffi::CStr, os::raw::c_char, rc::Rc};

use crate::NodeEventType;

#[cfg(feature = "api-15")]
use ohos_arkui_sys::OH_ArkUI_NodeEvent_GetTextChangeEvent;

pub struct Event(*mut ArkUI_NodeEvent);

#[cfg(feature = "api-15")]
#[derive(Debug, Clone)]
pub struct TextChangeEventData {
    pub text: String,
    pub extend_text: String,
    pub number: i32,
}

impl Event {
    pub fn new(event: *mut ArkUI_NodeEvent) -> Self {
        Self(event)
    }

    pub fn raw(&self) -> *mut ArkUI_NodeEvent {
        self.0
    }

    pub fn event_type(&self) -> NodeEventType {
        let t = unsafe { OH_ArkUI_NodeEvent_GetEventType(self.0) };
        NodeEventType::from(t)
    }

    pub fn target_id(&self) -> i32 {
        unsafe { OH_ArkUI_NodeEvent_GetTargetId(self.0) }
    }

    pub fn input_event(&self) -> Option<ohos_arkui_input_binding::ArkUIInputEvent> {
        let raw_event = unsafe { OH_ArkUI_NodeEvent_GetInputEvent(self.0) };
        if raw_event.is_null() {
            None
        } else {
            Some(ohos_arkui_input_binding::ArkUIInputEvent::from_raw(
                raw_event,
            ))
        }
    }

    pub fn pre_drag_status(&self) -> Option<crate::PreDragStatus> {
        let status = unsafe { OH_ArkUI_NodeEvent_GetPreDragStatus(self.0) };
        crate::PreDragStatus::try_from_raw(status)
    }

    pub fn number_value(&self, index: i32) -> Option<ArkUI_NumberValue> {
        let mut value = ArkUI_NumberValue { i32_: 0 };
        let status = unsafe { OH_ArkUI_NodeEvent_GetNumberValue(self.0, index, &mut value) };
        if status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
        {
            Some(value)
        } else {
            None
        }
    }

    pub fn i32_value(&self, index: i32) -> Option<i32> {
        self.number_value(index).map(|v| unsafe { v.i32_ })
    }

    pub fn u32_value(&self, index: i32) -> Option<u32> {
        self.number_value(index).map(|v| unsafe { v.u32_ })
    }

    pub fn f32_value(&self, index: i32) -> Option<f32> {
        self.number_value(index).map(|v| unsafe { v.f32_ })
    }

    pub fn string_value(&self, index: i32) -> Option<String> {
        let mut ptr: *mut c_char = std::ptr::null_mut();
        let mut size = 0;
        let status =
            unsafe { OH_ArkUI_NodeEvent_GetStringValue(self.0, index, &mut ptr, &mut size) };
        if status != ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
            || ptr.is_null()
            || size <= 0
        {
            return None;
        }
        let bytes = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), size as usize) };
        let bytes = if bytes.last() == Some(&0) {
            &bytes[..bytes.len() - 1]
        } else {
            bytes
        };
        Some(String::from_utf8_lossy(bytes).into_owned())
    }

    pub fn async_string(&self) -> Option<String> {
        let string_event = unsafe { OH_ArkUI_NodeEvent_GetStringAsyncEvent(self.0) };
        if string_event.is_null() {
            return None;
        }
        let ptr = unsafe { (*string_event).pStr };
        if ptr.is_null() {
            return None;
        }
        Some(
            unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn text_change(&self) -> Option<TextChangeEventData> {
        let event = unsafe { OH_ArkUI_NodeEvent_GetTextChangeEvent(self.0) };
        if event.is_null() {
            return None;
        }
        let text_ptr = unsafe { (*event).pStr };
        let extend_text_ptr = unsafe { (*event).pExtendStr };
        let text = if text_ptr.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(text_ptr) }
                .to_string_lossy()
                .into_owned()
        };
        let extend_text = if extend_text_ptr.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(extend_text_ptr) }
                .to_string_lossy()
                .into_owned()
        };
        Some(TextChangeEventData {
            text,
            extend_text,
            number: unsafe { (*event).number },
        })
    }

    pub fn set_return_i32(&self, value: i32) -> bool {
        let mut values = [ArkUI_NumberValue { i32_: value }];
        let status =
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.0, values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_u32(&self, value: u32) -> bool {
        let mut values = [ArkUI_NumberValue { u32_: value }];
        let status =
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.0, values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_f32(&self, value: f32) -> bool {
        let mut values = [ArkUI_NumberValue { f32_: value }];
        let status =
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.0, values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_bool(&self, value: bool) -> bool {
        self.set_return_i32(value as i32)
    }

    pub fn set_return_values(&self, values: &mut [ArkUI_NumberValue]) -> bool {
        if values.is_empty() {
            return false;
        }
        let status = unsafe {
            OH_ArkUI_NodeEvent_SetReturnNumberValue(
                self.0,
                values.as_mut_ptr(),
                values.len() as i32,
            )
        };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }
}

pub type EventClause = Rc<RefCell<dyn Fn(&Event)>>;

#[derive(Default, Clone)]
pub struct EventHandle {
    pub(crate) callbacks: Vec<(NodeEventType, EventClause)>,
}

impl EventHandle {
    pub(crate) fn set_event_callback(&mut self, event_type: NodeEventType, callback: EventClause) {
        if let Some((_, cb)) = self
            .callbacks
            .iter_mut()
            .find(|(current_event_type, _)| *current_event_type == event_type)
        {
            *cb = callback;
            return;
        }
        self.callbacks.push((event_type, callback));
    }

    pub(crate) fn get_event_callback(&self, event_type: NodeEventType) -> Option<&EventClause> {
        self.callbacks
            .iter()
            .find(|(current_event_type, _)| *current_event_type == event_type)
            .map(|(_, callback)| callback)
    }

    pub(crate) fn has_callback(&self) -> bool {
        !self.callbacks.is_empty()
    }
}
