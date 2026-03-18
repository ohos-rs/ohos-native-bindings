use std::{
    ffi::CString,
    os::raw::c_char,
    ptr::{self, NonNull},
};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use crate::{check_arkui_status, ArkUIError, ArkUIResult};

use super::DragPreviewOption;

#[cfg(feature = "api-19")]
pub(crate) fn notify_drag_result(
    request_identify: i32,
    result: crate::DragResult,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_NotifyDragResult(
            request_identify,
            result.into()
        ))
    }
}

#[cfg(feature = "api-19")]
pub(crate) fn notify_drag_end_pending_done(request_identify: i32) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_NotifyDragEndPendingDone(
            request_identify
        ))
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn cancel_data_loading(ui_context: crate::ArkUIContext, key: &str) -> ArkUIResult<()> {
    let key = CString::new(key).map_err(|_| {
        ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            "key contains interior NUL bytes",
        )
    })?;
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_CancelDataLoading(
            ui_context.raw(),
            key.as_ptr()
        ))
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn disable_drop_data_prefetch_on_node(
    node: &crate::ArkUINode,
    disabled: bool,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_DisableDropDataPrefetchOnNode(
            node.raw(),
            disabled
        ))
    }
}

pub(crate) fn set_drag_event_strict_report_with_node(
    node: &crate::ArkUINode,
    enabled: bool,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_SetDragEventStrictReportWithNode(
            node.raw(),
            enabled
        ))
    }
}

pub(crate) fn set_drag_event_strict_report_with_context(
    ui_context: crate::ArkUIContext,
    enabled: bool,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_SetDragEventStrictReportWithContext(
            ui_context.raw(),
            enabled
        ))
    }
}

pub(crate) fn set_node_allowed_drop_data_types(
    node: &crate::ArkUINode,
    types: &[&str],
) -> ArkUIResult<()> {
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

pub(crate) fn disallow_node_any_drop_data_types(node: &crate::ArkUINode) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_DisallowNodeAnyDropDataTypes(node.raw())) }
}

pub(crate) fn allow_node_all_drop_data_types(node: &crate::ArkUINode) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_AllowNodeAllDropDataTypes(node.raw())) }
}

pub(crate) fn set_node_draggable(node: &crate::ArkUINode, enabled: bool) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_SetNodeDraggable(node.raw(), enabled)) }
}

pub(crate) fn set_node_drag_preview(
    node: &crate::ArkUINode,
    preview: NonNull<OH_PixelmapNative>,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_SetNodeDragPreview(node.raw(), preview.as_ptr())) }
}

pub(crate) fn set_node_drag_preview_option(
    node: &crate::ArkUINode,
    option: &DragPreviewOption,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_SetNodeDragPreviewOption(node.raw(), option.raw())) }
}

#[cfg(feature = "api-20")]
pub(crate) fn enable_drop_disallowed_badge(
    ui_context: crate::ArkUIContext,
    enabled: bool,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_EnableDropDisallowedBadge(
            ui_context.raw(),
            enabled
        ))
    }
}
