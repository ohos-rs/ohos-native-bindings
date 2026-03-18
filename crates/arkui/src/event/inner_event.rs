use ohos_arkui_sys::{
    ArkUI_NodeEvent, ArkUI_NumberValue, OH_ArkUI_NodeEvent_GetDragEvent,
    OH_ArkUI_NodeEvent_GetEventType, OH_ArkUI_NodeEvent_GetInputEvent,
    OH_ArkUI_NodeEvent_GetNodeComponentEvent, OH_ArkUI_NodeEvent_GetNumberValue,
    OH_ArkUI_NodeEvent_GetPreDragStatus, OH_ArkUI_NodeEvent_GetStringAsyncEvent,
    OH_ArkUI_NodeEvent_GetStringValue, OH_ArkUI_NodeEvent_GetTargetId,
    OH_ArkUI_NodeEvent_GetUserData, OH_ArkUI_NodeEvent_SetReturnNumberValue,
};
#[cfg(feature = "api-22")]
use std::ffi::CString;
use std::{cell::RefCell, ffi::CStr, os::raw::c_char, os::raw::c_void, ptr::NonNull, rc::Rc};

use crate::NodeEventType;

#[cfg(feature = "api-15")]
use ohos_arkui_sys::OH_ArkUI_NodeEvent_GetTextChangeEvent;
#[cfg(feature = "api-22")]
use ohos_arkui_sys::{
    ArkUI_Rect, ArkUI_TouchTestInfo, ArkUI_TouchTestInfoItem, ArkUI_TouchTestInfoItemArray,
    OH_ArkUI_NodeEvent_GetTouchTestInfo, OH_ArkUI_TouchTestInfoItem_GetChildId,
    OH_ArkUI_TouchTestInfoItem_GetChildRect, OH_ArkUI_TouchTestInfoItem_GetWindowX,
    OH_ArkUI_TouchTestInfoItem_GetWindowY, OH_ArkUI_TouchTestInfoItem_GetX,
    OH_ArkUI_TouchTestInfoItem_GetXRelativeToParent, OH_ArkUI_TouchTestInfoItem_GetY,
    OH_ArkUI_TouchTestInfoItem_GetYRelativeToParent, OH_ArkUI_TouchTestInfo_GetTouchTestInfoList,
    OH_ArkUI_TouchTestInfo_SetTouchResultId, OH_ArkUI_TouchTestInfo_SetTouchResultStrategy,
};

pub struct Event(NonNull<c_void>);

#[cfg(feature = "api-15")]
#[derive(Debug, Clone)]
pub struct TextChangeEventData {
    pub text: String,
    pub extend_text: String,
    pub number: i32,
}

impl Event {
    pub(crate) fn new(event: *mut ArkUI_NodeEvent) -> Self {
        Self(NonNull::new(event.cast()).expect("ArkUI_NodeEvent pointer is null"))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_NodeEvent {
        self.0.as_ptr().cast()
    }

    pub fn event_type(&self) -> NodeEventType {
        let t = unsafe { OH_ArkUI_NodeEvent_GetEventType(self.raw()) };
        NodeEventType::from(t)
    }

    pub fn target_id(&self) -> i32 {
        unsafe { OH_ArkUI_NodeEvent_GetTargetId(self.raw()) }
    }

    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        let user_data = unsafe { OH_ArkUI_NodeEvent_GetUserData(self.raw()) };
        NonNull::new(user_data)
    }

    pub fn input_event(&self) -> Option<ohos_arkui_input_binding::ArkUIInputEvent> {
        let raw_event = unsafe { OH_ArkUI_NodeEvent_GetInputEvent(self.raw()) };
        if raw_event.is_null() {
            None
        } else {
            Some(ohos_arkui_input_binding::ArkUIInputEvent::from_raw(
                raw_event,
            ))
        }
    }

    pub fn node_component_event(&self) -> Option<NonNull<c_void>> {
        let event = unsafe { OH_ArkUI_NodeEvent_GetNodeComponentEvent(self.raw()) };
        NonNull::new(event.cast())
    }

    pub fn drag_event(&self) -> Option<NonNull<c_void>> {
        let event = unsafe { OH_ArkUI_NodeEvent_GetDragEvent(self.raw()) };
        NonNull::new(event.cast())
    }

    #[cfg(feature = "api-22")]
    pub fn touch_test_info(&self) -> Option<TouchTestInfo> {
        let info = unsafe { OH_ArkUI_NodeEvent_GetTouchTestInfo(self.raw()) };
        TouchTestInfo::from_raw(info)
    }

    pub fn pre_drag_status(&self) -> Option<crate::PreDragStatus> {
        let status = unsafe { OH_ArkUI_NodeEvent_GetPreDragStatus(self.raw()) };
        crate::PreDragStatus::try_from_raw(status)
    }

    pub(crate) fn number_value(&self, index: i32) -> Option<ArkUI_NumberValue> {
        let mut value = ArkUI_NumberValue { i32_: 0 };
        let status = unsafe { OH_ArkUI_NodeEvent_GetNumberValue(self.raw(), index, &mut value) };
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
            unsafe { OH_ArkUI_NodeEvent_GetStringValue(self.raw(), index, &mut ptr, &mut size) };
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
        let string_event = unsafe { OH_ArkUI_NodeEvent_GetStringAsyncEvent(self.raw()) };
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
        let event = unsafe { OH_ArkUI_NodeEvent_GetTextChangeEvent(self.raw()) };
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
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.raw(), values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_u32(&self, value: u32) -> bool {
        let mut values = [ArkUI_NumberValue { u32_: value }];
        let status =
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.raw(), values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_f32(&self, value: f32) -> bool {
        let mut values = [ArkUI_NumberValue { f32_: value }];
        let status =
            unsafe { OH_ArkUI_NodeEvent_SetReturnNumberValue(self.raw(), values.as_mut_ptr(), 1) };
        status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32
    }

    pub fn set_return_bool(&self, value: bool) -> bool {
        self.set_return_i32(value as i32)
    }

    pub(crate) fn set_return_values(&self, values: &mut [ArkUI_NumberValue]) -> bool {
        if values.is_empty() {
            return false;
        }
        let status = unsafe {
            OH_ArkUI_NodeEvent_SetReturnNumberValue(
                self.raw(),
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

#[cfg(feature = "api-22")]
fn is_arkui_ok(status: u32) -> bool {
    status == ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR
}

#[cfg(feature = "api-22")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchTestRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[cfg(feature = "api-22")]
impl From<ArkUI_Rect> for TouchTestRect {
    fn from(value: ArkUI_Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug)]
pub struct TouchTestInfo {
    raw: NonNull<ArkUI_TouchTestInfo>,
}

#[cfg(feature = "api-22")]
impl TouchTestInfo {
    pub(crate) fn from_raw(raw: *mut ArkUI_TouchTestInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_TouchTestInfo {
        self.raw.as_ptr()
    }

    pub fn items(&self) -> Vec<TouchTestInfoItem> {
        let mut items: ArkUI_TouchTestInfoItemArray = std::ptr::null_mut();
        let mut size = 0;
        let status = unsafe {
            OH_ArkUI_TouchTestInfo_GetTouchTestInfoList(self.raw(), &mut items, &mut size)
        };
        if !is_arkui_ok(status as u32) || items.is_null() || size <= 0 {
            return Vec::new();
        }
        let items = unsafe { std::slice::from_raw_parts(items, size as usize) };
        items
            .iter()
            .filter_map(|item| TouchTestInfoItem::from_raw(*item))
            .collect()
    }

    pub fn set_touch_result_strategy(&mut self, strategy: crate::TouchTestStrategy) -> bool {
        let status =
            unsafe { OH_ArkUI_TouchTestInfo_SetTouchResultStrategy(self.raw(), strategy.into()) };
        is_arkui_ok(status as u32)
    }

    pub fn set_touch_result_id<T: AsRef<str>>(&mut self, id: T) -> bool {
        let Ok(id) = CString::new(id.as_ref()) else {
            return false;
        };
        let status = unsafe { OH_ArkUI_TouchTestInfo_SetTouchResultId(self.raw(), id.as_ptr()) };
        is_arkui_ok(status as u32)
    }
}

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug)]
pub struct TouchTestInfoItem {
    raw: NonNull<ArkUI_TouchTestInfoItem>,
}

#[cfg(feature = "api-22")]
impl TouchTestInfoItem {
    pub(crate) fn from_raw(raw: *mut ArkUI_TouchTestInfoItem) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *const ArkUI_TouchTestInfoItem {
        self.raw.as_ptr()
    }

    pub fn x(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetX(self.raw()) }
    }

    pub fn y(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetY(self.raw()) }
    }

    pub fn window_x(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetWindowX(self.raw()) }
    }

    pub fn window_y(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetWindowY(self.raw()) }
    }

    pub fn x_relative_to_parent(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetXRelativeToParent(self.raw()) }
    }

    pub fn y_relative_to_parent(&self) -> f32 {
        unsafe { OH_ArkUI_TouchTestInfoItem_GetYRelativeToParent(self.raw()) }
    }

    pub fn child_rect(&self) -> Option<TouchTestRect> {
        let mut child_rect: ArkUI_Rect = unsafe { std::mem::zeroed() };
        let status =
            unsafe { OH_ArkUI_TouchTestInfoItem_GetChildRect(self.raw(), &mut child_rect) };
        if is_arkui_ok(status as u32) {
            Some(child_rect.into())
        } else {
            None
        }
    }

    pub fn child_id(&self) -> Option<String> {
        let mut buffer = vec![0u8; 256];
        let status = unsafe {
            OH_ArkUI_TouchTestInfoItem_GetChildId(
                self.raw(),
                buffer.as_mut_ptr().cast(),
                buffer.len() as i32,
            )
        };
        if !is_arkui_ok(status as u32) {
            return None;
        }
        let len = buffer.iter().position(|v| *v == 0).unwrap_or(buffer.len());
        Some(String::from_utf8_lossy(&buffer[..len]).into_owned())
    }
}
