//! Module api::drag::event wrappers and related types.

use std::{os::raw::c_char, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use crate::{check_arkui_status, ArkUIError, ArkUIResult, DragResult, DropOperation};

#[derive(Clone, Copy)]
pub(crate) struct DragAndDropInfo {
    raw: NonNull<ArkUI_DragAndDropInfo>,
}

impl DragAndDropInfo {
    pub(crate) unsafe fn from_raw(raw: *mut ArkUI_DragAndDropInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DragAndDropInfo {
        self.raw.as_ptr()
    }

    pub(crate) fn drag_status(&self) -> crate::DragStatus {
        unsafe { OH_ArkUI_DragAndDropInfo_GetDragStatus(self.raw()).into() }
    }

    pub(crate) fn drag_event(&self) -> Option<DragEvent> {
        let event = unsafe { OH_ArkUI_DragAndDropInfo_GetDragEvent(self.raw()) };
        unsafe { DragEvent::from_raw(event) }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DragEvent {
    raw: NonNull<ArkUI_DragEvent>,
}

impl DragEvent {
    pub(crate) unsafe fn from_raw(raw: *mut ArkUI_DragEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DragEvent {
        self.raw.as_ptr()
    }

    pub(crate) fn disable_default_drop_animation(&self, disable: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_DisableDefaultDropAnimation(
                self.raw(),
                disable
            ))
        }
    }

    pub(crate) fn set_suggested_drop_operation(
        &self,
        drop_operation: DropOperation,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_SetSuggestedDropOperation(
                self.raw(),
                drop_operation.into()
            ))
        }
    }

    pub(crate) fn set_drag_result(&self, result: DragResult) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragEvent_SetDragResult(self.raw(), result.into())) }
    }

    pub(crate) fn set_data(&self, data: *mut OH_UdmfData) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragEvent_SetData(self.raw(), data)) }
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn set_data_load_params(
        &self,
        data_load_params: *mut OH_UdmfDataLoadParams,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_SetDataLoadParams(
                self.raw(),
                data_load_params
            ))
        }
    }

    pub(crate) fn get_udmf_data(&self, data: *mut OH_UdmfData) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragEvent_GetUdmfData(self.raw(), data)) }
    }

    pub(crate) fn data_type_count(&self) -> ArkUIResult<i32> {
        let mut count = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetDataTypeCount(self.raw(), &mut count))
        }?;
        Ok(count)
    }

    pub(crate) fn data_types(&self, max_str_len: i32) -> ArkUIResult<Vec<String>> {
        if max_str_len <= 0 {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "max_str_len must be greater than 0",
            ));
        }
        let count = self.data_type_count()?;
        if count <= 0 {
            return Ok(Vec::new());
        }

        let mut buffers = vec![vec![0u8; max_str_len as usize]; count as usize];
        let mut ptrs: Vec<*mut c_char> = buffers
            .iter_mut()
            .map(|buffer| buffer.as_mut_ptr().cast())
            .collect();

        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetDataTypes(
                self.raw(),
                ptrs.as_mut_ptr(),
                count,
                max_str_len
            ))
        }?;

        Ok(buffers
            .into_iter()
            .map(|buffer| {
                let len = buffer.iter().position(|v| *v == 0).unwrap_or(buffer.len());
                String::from_utf8_lossy(&buffer[..len]).into_owned()
            })
            .collect())
    }

    pub(crate) fn drag_result(&self) -> ArkUIResult<DragResult> {
        let mut result = ArkUI_DragResult_ARKUI_DRAG_RESULT_CANCELED;
        unsafe { check_arkui_status!(OH_ArkUI_DragEvent_GetDragResult(self.raw(), &mut result)) }?;
        Ok(result.into())
    }

    pub(crate) fn drop_operation(&self) -> ArkUIResult<DropOperation> {
        let mut operation = ArkUI_DropOperation_ARKUI_DROP_OPERATION_COPY;
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetDropOperation(
                self.raw(),
                &mut operation
            ))
        }?;
        Ok(operation.into())
    }

    pub(crate) fn preview_touch_point_x(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetPreviewTouchPointX(self.raw()) }
    }

    pub(crate) fn preview_touch_point_y(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetPreviewTouchPointY(self.raw()) }
    }

    pub(crate) fn preview_rect_width(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetPreviewRectWidth(self.raw()) }
    }

    pub(crate) fn preview_rect_height(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetPreviewRectHeight(self.raw()) }
    }

    pub(crate) fn touch_point_x_to_window(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointXToWindow(self.raw()) }
    }

    pub(crate) fn touch_point_y_to_window(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointYToWindow(self.raw()) }
    }

    pub(crate) fn touch_point_x_to_display(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointXToDisplay(self.raw()) }
    }

    pub(crate) fn touch_point_y_to_display(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointYToDisplay(self.raw()) }
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn touch_point_x_to_global_display(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointXToGlobalDisplay(self.raw()) }
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn touch_point_y_to_global_display(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetTouchPointYToGlobalDisplay(self.raw()) }
    }

    pub(crate) fn velocity_x(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetVelocityX(self.raw()) }
    }

    pub(crate) fn velocity_y(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetVelocityY(self.raw()) }
    }

    pub(crate) fn velocity(&self) -> f32 {
        unsafe { OH_ArkUI_DragEvent_GetVelocity(self.raw()) }
    }

    pub(crate) fn modifier_key_states(&self) -> ArkUIResult<u64> {
        let mut keys = 0u64;
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetModifierKeyStates(
                self.raw(),
                &mut keys
            ))
        }?;
        Ok(keys)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn display_id(&self) -> ArkUIResult<i32> {
        let mut display_id = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetDisplayId(self.raw(), &mut display_id))
        }?;
        Ok(display_id)
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn start_data_loading(
        &self,
        options: *mut OH_UdmfGetDataParams,
        key_len: u32,
    ) -> ArkUIResult<String> {
        if key_len == 0 {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "key_len must be greater than 0",
            ));
        }
        let mut key = vec![0u8; key_len as usize];
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_StartDataLoading(
                self.raw(),
                options,
                key.as_mut_ptr().cast(),
                key_len
            ))
        }?;
        let len = key.iter().position(|v| *v == 0).unwrap_or(key.len());
        Ok(String::from_utf8_lossy(&key[..len]).into_owned())
    }

    #[cfg(feature = "api-19")]
    pub(crate) fn request_drag_end_pending(&self) -> ArkUIResult<i32> {
        let mut request_identify = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_RequestDragEndPending(
                self.raw(),
                &mut request_identify
            ))
        }?;
        Ok(request_identify)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn drag_source(&self, length: i32) -> ArkUIResult<String> {
        if length <= 0 {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "length must be greater than 0",
            ));
        }
        let mut buffer = vec![0u8; length as usize];
        unsafe {
            check_arkui_status!(OH_ArkUI_DragEvent_GetDragSource(
                self.raw(),
                buffer.as_mut_ptr().cast(),
                length
            ))
        }?;
        let len = buffer.iter().position(|v| *v == 0).unwrap_or(buffer.len());
        Ok(String::from_utf8_lossy(&buffer[..len]).into_owned())
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn is_remote(&self) -> ArkUIResult<bool> {
        let mut is_remote = false;
        unsafe { check_arkui_status!(OH_ArkUI_DragEvent_IsRemote(self.raw(), &mut is_remote)) }?;
        Ok(is_remote)
    }
}
