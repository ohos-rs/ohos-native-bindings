use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_CustomSpanDrawInfo, ArkUI_CustomSpanMeasureInfo, ArkUI_CustomSpanMetrics,
    ArkUI_DrawContext, ArkUI_IntOffset, ArkUI_IntSize, ArkUI_LayoutConstraint,
    ArkUI_NodeCustomEvent, OH_ArkUI_CustomSpanDrawInfo_Create, OH_ArkUI_CustomSpanDrawInfo_Dispose,
    OH_ArkUI_CustomSpanDrawInfo_GetBaseline, OH_ArkUI_CustomSpanDrawInfo_GetLineBottom,
    OH_ArkUI_CustomSpanDrawInfo_GetLineTop, OH_ArkUI_CustomSpanDrawInfo_GetXOffset,
    OH_ArkUI_CustomSpanMeasureInfo_Create, OH_ArkUI_CustomSpanMeasureInfo_Dispose,
    OH_ArkUI_CustomSpanMeasureInfo_GetFontSize, OH_ArkUI_CustomSpanMetrics_Create,
    OH_ArkUI_CustomSpanMetrics_Dispose, OH_ArkUI_CustomSpanMetrics_SetHeight,
    OH_ArkUI_CustomSpanMetrics_SetWidth, OH_ArkUI_DrawContext_GetCanvas,
    OH_ArkUI_DrawContext_GetSize, OH_ArkUI_NodeCustomEvent_GetCustomSpanDrawInfo,
    OH_ArkUI_NodeCustomEvent_GetCustomSpanMeasureInfo,
    OH_ArkUI_NodeCustomEvent_GetDrawContextInDraw, OH_ArkUI_NodeCustomEvent_GetEventTargetId,
    OH_ArkUI_NodeCustomEvent_GetEventType, OH_ArkUI_NodeCustomEvent_GetLayoutConstraintInMeasure,
    OH_ArkUI_NodeCustomEvent_GetNodeHandle, OH_ArkUI_NodeCustomEvent_GetPositionInLayout,
    OH_ArkUI_NodeCustomEvent_GetUserData, OH_ArkUI_NodeCustomEvent_SetCustomSpanMetrics,
};

use crate::{check_arkui_status, ArkUIError, ArkUINode, ArkUIResult};

fn ptr_or_error<T>(ptr: *mut T, func: &'static str) -> ArkUIResult<NonNull<T>> {
    NonNull::new(ptr).ok_or_else(|| {
        ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        )
    })
}

fn non_null_or_panic<T>(ptr: *mut T, name: &'static str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("{name} pointer is null"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntOffset {
    pub x: i32,
    pub y: i32,
}

impl From<ArkUI_IntOffset> for IntOffset {
    fn from(value: ArkUI_IntOffset) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<IntOffset> for ArkUI_IntOffset {
    fn from(value: IntOffset) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntSize {
    pub width: i32,
    pub height: i32,
}

impl From<ArkUI_IntSize> for IntSize {
    fn from(value: ArkUI_IntSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<IntSize> for ArkUI_IntSize {
    fn from(value: IntSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LayoutConstraintHandle {
    raw: NonNull<ArkUI_LayoutConstraint>,
}

impl LayoutConstraintHandle {
    pub(crate) fn from_raw(raw: *mut ArkUI_LayoutConstraint) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_LayoutConstraint {
        self.raw.as_ptr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawContext {
    raw: NonNull<ArkUI_DrawContext>,
}

impl DrawContext {
    pub(crate) fn from_raw(raw: *mut ArkUI_DrawContext) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DrawContext {
        self.raw.as_ptr()
    }

    pub fn canvas(&self) -> Option<NonNull<c_void>> {
        let canvas = unsafe { OH_ArkUI_DrawContext_GetCanvas(self.raw()) };
        NonNull::new(canvas)
    }

    pub fn size(&self) -> IntSize {
        let size = unsafe { OH_ArkUI_DrawContext_GetSize(self.raw()) };
        size.into()
    }
}

pub struct CustomSpanMeasureInfo {
    raw: NonNull<ArkUI_CustomSpanMeasureInfo>,
}

impl CustomSpanMeasureInfo {
    pub fn new() -> ArkUIResult<Self> {
        let info = unsafe { OH_ArkUI_CustomSpanMeasureInfo_Create() };
        ptr_or_error(info, "OH_ArkUI_CustomSpanMeasureInfo_Create").map(Self::from_non_null)
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanMeasureInfo>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanMeasureInfo) -> Self {
        Self::from_non_null(non_null_or_panic(raw, "ArkUI_CustomSpanMeasureInfo"))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanMeasureInfo {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanMeasureInfo_Dispose(self.raw()) }
    }

    pub fn font_size(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanMeasureInfo_GetFontSize(self.raw()) }
    }
}

pub struct CustomSpanMetrics {
    raw: NonNull<ArkUI_CustomSpanMetrics>,
}

impl CustomSpanMetrics {
    pub fn new() -> ArkUIResult<Self> {
        let metrics = unsafe { OH_ArkUI_CustomSpanMetrics_Create() };
        ptr_or_error(metrics, "OH_ArkUI_CustomSpanMetrics_Create").map(Self::from_non_null)
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanMetrics>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanMetrics) -> Self {
        Self::from_non_null(non_null_or_panic(raw, "ArkUI_CustomSpanMetrics"))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanMetrics {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanMetrics_Dispose(self.raw()) }
    }

    pub fn set_width(&mut self, width: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomSpanMetrics_SetWidth(self.raw(), width)) }
    }

    pub fn set_height(&mut self, height: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomSpanMetrics_SetHeight(self.raw(), height)) }
    }
}

pub struct CustomSpanDrawInfo {
    raw: NonNull<ArkUI_CustomSpanDrawInfo>,
}

impl CustomSpanDrawInfo {
    pub fn new() -> ArkUIResult<Self> {
        let info = unsafe { OH_ArkUI_CustomSpanDrawInfo_Create() };
        ptr_or_error(info, "OH_ArkUI_CustomSpanDrawInfo_Create").map(Self::from_non_null)
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanDrawInfo>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanDrawInfo) -> Self {
        Self::from_non_null(non_null_or_panic(raw, "ArkUI_CustomSpanDrawInfo"))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanDrawInfo {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_Dispose(self.raw()) }
    }

    pub fn x_offset(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetXOffset(self.raw()) }
    }

    pub fn line_top(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetLineTop(self.raw()) }
    }

    pub fn line_bottom(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetLineBottom(self.raw()) }
    }

    pub fn baseline(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetBaseline(self.raw()) }
    }
}

pub struct NodeCustomEvent {
    raw: NonNull<ArkUI_NodeCustomEvent>,
}

impl NodeCustomEvent {
    pub(crate) fn from_raw(raw: *mut ArkUI_NodeCustomEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_NodeCustomEvent {
        self.raw.as_ptr()
    }

    pub fn layout_constraint_in_measure(&self) -> Option<LayoutConstraintHandle> {
        let constraint =
            unsafe { OH_ArkUI_NodeCustomEvent_GetLayoutConstraintInMeasure(self.raw()) };
        LayoutConstraintHandle::from_raw(constraint)
    }

    pub fn position_in_layout(&self) -> IntOffset {
        unsafe { OH_ArkUI_NodeCustomEvent_GetPositionInLayout(self.raw()) }.into()
    }

    pub fn draw_context_in_draw(&self) -> Option<DrawContext> {
        let draw_context = unsafe { OH_ArkUI_NodeCustomEvent_GetDrawContextInDraw(self.raw()) };
        DrawContext::from_raw(draw_context)
    }

    pub fn event_target_id(&self) -> i32 {
        unsafe { OH_ArkUI_NodeCustomEvent_GetEventTargetId(self.raw()) }
    }

    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        let user_data = unsafe { OH_ArkUI_NodeCustomEvent_GetUserData(self.raw()) };
        NonNull::new(user_data)
    }

    pub fn node_handle(&self) -> Option<ArkUINode> {
        let node = unsafe { OH_ArkUI_NodeCustomEvent_GetNodeHandle(self.raw()) };
        ArkUINode::from_raw_handle(node)
    }

    pub fn event_type(&self) -> crate::NodeCustomEventType {
        unsafe { OH_ArkUI_NodeCustomEvent_GetEventType(self.raw()).into() }
    }

    pub fn get_custom_span_measure_info(
        &self,
        info: &mut CustomSpanMeasureInfo,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeCustomEvent_GetCustomSpanMeasureInfo(
                self.raw(),
                info.raw()
            ))
        }
    }

    pub fn set_custom_span_metrics(&self, metrics: &mut CustomSpanMetrics) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeCustomEvent_SetCustomSpanMetrics(
                self.raw(),
                metrics.raw()
            ))
        }
    }

    pub fn get_custom_span_draw_info(&self, info: &mut CustomSpanDrawInfo) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeCustomEvent_GetCustomSpanDrawInfo(
                self.raw(),
                info.raw()
            ))
        }
    }
}
