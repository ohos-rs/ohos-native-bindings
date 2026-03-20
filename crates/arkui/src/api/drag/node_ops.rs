//! Module api::drag::node_ops wrappers and related types.

use std::{ffi::CString, os::raw::c_char, ptr};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;
use ohos_image_native_binding::PixelMapNativeHandle;

use crate::{check_arkui_status, ArkUIError, ArkUIHandle, ArkUINode, ArkUIResult};

use super::DragPreviewOption;

impl ArkUIHandle {
    #[cfg(feature = "api-19")]
    pub(crate) fn notify_drag_result(
        request_identify: i32,
        result: crate::DragResult,
    ) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NotifyDragResult(request_identify, result.into())) }
    }

    #[cfg(feature = "api-19")]
    pub(crate) fn notify_drag_end_pending_done(request_identify: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NotifyDragEndPendingDone(request_identify)) }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn cancel_data_loading(&self, node: &ArkUINode, key: &str) -> ArkUIResult<()> {
        let _ = self.raw();
        let key = CString::new(key).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "key contains interior NUL bytes",
            )
        })?;
        let context = self.context_by_node(node)?;
        unsafe { check_arkui_status!(OH_ArkUI_CancelDataLoading(context, key.as_ptr())) }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn disable_drop_data_prefetch_on_node(
        &self,
        node: &ArkUINode,
        disabled: bool,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_DisableDropDataPrefetchOnNode(node.raw(), disabled)) }
    }

    pub(crate) fn set_drag_event_strict_report_with_node(
        &self,
        node: &ArkUINode,
        enabled: bool,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_SetDragEventStrictReportWithNode(
                node.raw(),
                enabled
            ))
        }
    }

    pub(crate) fn set_drag_event_strict_report_with_context(
        &self,
        node: &ArkUINode,
        enabled: bool,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let context = self.context_by_node(node)?;
        unsafe {
            check_arkui_status!(OH_ArkUI_SetDragEventStrictReportWithContext(
                context, enabled
            ))
        }
    }

    pub(crate) fn set_node_allowed_drop_data_types(
        &self,
        node: &ArkUINode,
        types: &[&str],
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let mut owned = Vec::with_capacity(types.len());
        for ty in types {
            owned.push(CString::new(*ty).map_err(|_| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "drop data type contains interior NUL bytes",
                )
            })?);
        }
        let mut ptrs: Vec<*const c_char> = owned.iter().map(|item| item.as_ptr()).collect();
        let ptr = if ptrs.is_empty() {
            ptr::null_mut()
        } else {
            ptrs.as_mut_ptr()
        };
        unsafe {
            check_arkui_status!(OH_ArkUI_SetNodeAllowedDropDataTypes(
                node.raw(),
                ptr,
                ptrs.len() as i32
            ))
        }
    }

    pub(crate) fn disallow_node_any_drop_data_types(&self, node: &ArkUINode) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_DisallowNodeAnyDropDataTypes(node.raw())) }
    }

    pub(crate) fn allow_node_all_drop_data_types(&self, node: &ArkUINode) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_AllowNodeAllDropDataTypes(node.raw())) }
    }

    pub(crate) fn set_node_draggable(&self, node: &ArkUINode, enabled: bool) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_SetNodeDraggable(node.raw(), enabled)) }
    }

    pub(crate) fn set_node_drag_preview(
        &self,
        node: &ArkUINode,
        preview: PixelMapNativeHandle,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_SetNodeDragPreview(
                node.raw(),
                preview.as_raw().cast()
            ))
        }
    }

    pub(crate) fn set_node_drag_preview_option(
        &self,
        node: &ArkUINode,
        option: &DragPreviewOption,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_SetNodeDragPreviewOption(node.raw(), option.raw())) }
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn enable_drop_disallowed_badge(
        &self,
        node: &ArkUINode,
        enabled: bool,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let context = self.context_by_node(node)?;
        unsafe { check_arkui_status!(OH_ArkUI_EnableDropDisallowedBadge(context, enabled)) }
    }
}
