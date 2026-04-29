//! Module api::node_custom_event::event wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_sys::{
    ArkUI_NodeCustomEvent, OH_ArkUI_NodeCustomEvent_GetCustomSpanDrawInfo,
    OH_ArkUI_NodeCustomEvent_GetCustomSpanMeasureInfo,
    OH_ArkUI_NodeCustomEvent_GetDrawContextInDraw, OH_ArkUI_NodeCustomEvent_GetEventTargetId,
    OH_ArkUI_NodeCustomEvent_GetEventType, OH_ArkUI_NodeCustomEvent_GetLayoutConstraintInMeasure,
    OH_ArkUI_NodeCustomEvent_GetNodeHandle, OH_ArkUI_NodeCustomEvent_GetPositionInLayout,
    OH_ArkUI_NodeCustomEvent_GetUserData, OH_ArkUI_NodeCustomEvent_SetCustomSpanMetrics,
};

use crate::{check_arkui_status, ArkUINode, ArkUIResult};

use super::{
    CustomSpanDrawInfo, CustomSpanMeasureInfo, CustomSpanMetrics, DrawContext, IntOffset,
    LayoutConstraintHandle,
};

/// Wrapper for node custom-event payload delivered by ArkUI.
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

    /// Returns layout constraint info for measure callbacks.
    pub fn layout_constraint_in_measure(&self) -> Option<LayoutConstraintHandle> {
        let constraint =
            unsafe { OH_ArkUI_NodeCustomEvent_GetLayoutConstraintInMeasure(self.raw()) };
        LayoutConstraintHandle::from_raw(constraint)
    }

    /// Returns measured/layout position offset.
    pub fn position_in_layout(&self) -> IntOffset {
        unsafe { OH_ArkUI_NodeCustomEvent_GetPositionInLayout(self.raw()) }.into()
    }

    /// Returns drawing context for draw-related callbacks.
    pub fn draw_context_in_draw(&self) -> Option<DrawContext> {
        let draw_context = unsafe { OH_ArkUI_NodeCustomEvent_GetDrawContextInDraw(self.raw()) };
        DrawContext::from_raw(draw_context)
    }

    /// Returns target id of the event source.
    pub fn event_target_id(&self) -> i32 {
        unsafe { OH_ArkUI_NodeCustomEvent_GetEventTargetId(self.raw()) }
    }

    /// Returns user data attached by ArkUI callback registration.
    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        let user_data = unsafe { OH_ArkUI_NodeCustomEvent_GetUserData(self.raw()) };
        NonNull::new(user_data)
    }

    /// Returns associated node handle.
    pub fn node_handle(&self) -> Option<ArkUINode> {
        let node = unsafe { OH_ArkUI_NodeCustomEvent_GetNodeHandle(self.raw()) };
        ArkUINode::from_raw_handle(node)
    }

    /// Returns custom event category.
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
