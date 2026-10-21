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
    fn layout_node(&self, phase: crate::NodeCustomEventType) -> ArkUIResult<ArkUINode> {
        if self.event_type() != phase {
            return Err(crate::ArkUIError::new(
                crate::arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                "custom layout operation called in the wrong event phase",
            ));
        }
        // SAFETY: this event owns a live node for this synchronous callback.
        unsafe { self.node_handle() }.ok_or_else(|| {
            crate::ArkUIError::new(
                crate::arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                "custom layout event has no node",
            )
        })
    }

    /// Sets this custom node's measured size in physical pixels.
    /// Returns an error unless called during its measure event.
    pub fn set_measured_size(&self, width: i32, height: i32) -> ArkUIResult<()> {
        let node = self.layout_node(crate::NodeCustomEventType::OnMeasure)?;
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_measured_size(&node, width, height))
    }

    /// Measures a live child with this measure event's constraints.
    /// The child must belong to this custom node on the current UI thread.
    pub fn measure_child(&self, child: &ArkUINode) -> ArkUIResult<()> {
        self.layout_node(crate::NodeCustomEventType::OnMeasure)?;
        let constraint = self.layout_constraint_in_measure().ok_or_else(|| {
            crate::ArkUIError::new(
                crate::arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                "measure event has no layout constraint",
            )
        })?;
        // SAFETY: the guard owns this constraint and releases it after the
        // synchronous measureNode call; native code does not retain it.
        crate::ARK_UI_NATIVE_NODE_API_1
            .with(|api| unsafe { api.measure_node(child, &mut *constraint.raw()) })
    }

    /// Sets this node's position in physical pixels during its layout event.
    pub fn set_layout_position(&self, x: i32, y: i32) -> ArkUIResult<()> {
        let node = self.layout_node(crate::NodeCustomEventType::OnLayout)?;
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_layout_position(&node, x, y))
    }

    /// Lays out a live child in physical pixels during this node's layout event.
    /// The child must belong to this custom node on the current UI thread.
    pub fn layout_child(&self, child: &ArkUINode, x: i32, y: i32) -> ArkUIResult<()> {
        self.layout_node(crate::NodeCustomEventType::OnLayout)?;
        crate::ARK_UI_NATIVE_NODE_API_1.with(|api| api.layout_node(child, x, y))
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_NodeCustomEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_NodeCustomEvent {
        self.raw.as_ptr()
    }

    /// Returns layout constraint info for measure callbacks.
    pub fn layout_constraint_in_measure(&self) -> Option<LayoutConstraintHandle<'_>> {
        if self.event_type() != crate::NodeCustomEventType::OnMeasure {
            return None;
        }
        let constraint =
            unsafe { OH_ArkUI_NodeCustomEvent_GetLayoutConstraintInMeasure(self.raw()) };
        LayoutConstraintHandle::from_raw(constraint, self)
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

    /// Borrows the associated node without acquiring ownership.
    ///
    /// # Safety
    /// Use the returned node only during this callback, unless an independent
    /// owner retains the native node until the borrowed wrapper is discarded.
    pub unsafe fn node_handle(&self) -> Option<ArkUINode> {
        let node = unsafe { OH_ArkUI_NodeCustomEvent_GetNodeHandle(self.raw()) };
        // SAFETY: the caller upholds the native node's external-owner contract.
        unsafe { ArkUINode::from_raw_handle(node) }
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
