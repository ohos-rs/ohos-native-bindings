//! Module api::drag::action wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
#[cfg(feature = "udmf")]
use ohos_arkui_sys::OH_ArkUI_DragAction_SetData;
#[cfg(feature = "image")]
use ohos_arkui_sys::OH_ArkUI_DragAction_SetPixelMaps;
use ohos_arkui_sys::{
    ArkUI_DragAction, ArkUI_DragAndDropInfo, OH_ArkUI_CreateDragActionWithContext,
    OH_ArkUI_CreateDragActionWithNode, OH_ArkUI_DragAction_Dispose,
    OH_ArkUI_DragAction_RegisterStatusListener, OH_ArkUI_DragAction_SetDragPreviewOption,
    OH_ArkUI_DragAction_SetPointerId, OH_ArkUI_DragAction_SetTouchPointX,
    OH_ArkUI_DragAction_SetTouchPointY, OH_ArkUI_DragAction_UnregisterStatusListener,
    OH_ArkUI_StartDrag,
};
#[cfg(feature = "image")]
use ohos_image_native_binding::PixelMapNativeHandle;
#[cfg(feature = "udmf")]
use ohos_udmf_binding::UdmfData;

#[cfg(all(feature = "api-20", feature = "udmf"))]
use ohos_udmf_binding::UdmfDataLoadParams;

use crate::{check_arkui_status, ArkUIError, ArkUIHandle, ArkUIResult};

use super::{DragAndDropInfo, DragPreviewOption};

struct DragStatusListenerCallbackContext {
    callback: Box<dyn Fn(&DragAndDropInfo)>,
}

pub(crate) struct DragAction {
    raw: NonNull<ArkUI_DragAction>,
    status_listener: Option<NonNull<DragStatusListenerCallbackContext>>,
}

impl DragAction {
    pub(crate) fn new_with_node(node: &crate::ArkUINode) -> ArkUIResult<Self> {
        let action = unsafe { OH_ArkUI_CreateDragActionWithNode(node.raw()) };
        let action = NonNull::new(action).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateDragActionWithNode returned null",
            )
        })?;
        Ok(Self::from_non_null(action))
    }

    pub(crate) fn new_with_context(ui_context: crate::ArkUIContext) -> ArkUIResult<Self> {
        let action = unsafe { OH_ArkUI_CreateDragActionWithContext(ui_context.raw()) };
        let action = NonNull::new(action).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateDragActionWithContext returned null",
            )
        })?;
        Ok(Self::from_non_null(action))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DragAction {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_DragAction) -> ArkUIResult<Self> {
        let raw = NonNull::new(raw).ok_or_else(|| {
            ArkUIError::new(ArkUIErrorCode::ParamInvalid, "ArkUI_DragAction is null")
        })?;
        Ok(Self::from_non_null(raw))
    }

    pub(crate) fn from_non_null(raw: NonNull<ArkUI_DragAction>) -> Self {
        Self {
            raw,
            status_listener: None,
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_DragAction {
        self.raw.as_ptr()
    }

    pub(crate) fn dispose(mut self) {
        self.unregister_status_listener();
        unsafe { OH_ArkUI_DragAction_Dispose(self.raw()) }
    }

    pub(crate) fn set_pointer_id(&mut self, pointer: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragAction_SetPointerId(self.raw(), pointer)) }
    }

    #[cfg(feature = "image")]
    pub(crate) fn set_pixel_maps(
        &mut self,
        pixelmap_array: &[PixelMapNativeHandle],
    ) -> ArkUIResult<()> {
        let mut raw_pixelmap_array: Vec<*mut _> = pixelmap_array
            .iter()
            .map(|pixel_map| pixel_map.as_raw().cast())
            .collect();
        unsafe {
            check_arkui_status!(OH_ArkUI_DragAction_SetPixelMaps(
                self.raw(),
                raw_pixelmap_array.as_mut_ptr(),
                raw_pixelmap_array.len() as i32
            ))
        }
    }

    pub(crate) fn set_touch_point_x(&mut self, x: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragAction_SetTouchPointX(self.raw(), x)) }
    }

    pub(crate) fn set_touch_point_y(&mut self, y: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragAction_SetTouchPointY(self.raw(), y)) }
    }

    #[cfg(feature = "udmf")]
    pub(crate) fn set_data(&mut self, data: &UdmfData) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DragAction_SetData(self.raw(), data.raw().as_ptr())) }
    }

    #[cfg(all(feature = "api-20", feature = "udmf"))]
    pub(crate) fn set_data_load_params(
        &mut self,
        data_load_params: &UdmfDataLoadParams,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_DragAction_SetDataLoadParams(
                self.raw(),
                data_load_params.as_raw()
            ))
        }
    }

    pub(crate) fn set_drag_preview_option(
        &mut self,
        option: &DragPreviewOption,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragAction_SetDragPreviewOption(
                self.raw(),
                option.raw()
            ))
        }
    }

    pub(crate) fn register_status_listener<T: Fn(&DragAndDropInfo) + 'static>(
        &mut self,
        listener: T,
    ) -> ArkUIResult<()> {
        self.unregister_status_listener();
        let listener = NonNull::new(Box::into_raw(Box::new(DragStatusListenerCallbackContext {
            callback: Box::new(listener),
        })))
        .expect("DragStatusListenerCallbackContext should not be null");

        let result = unsafe {
            check_arkui_status!(OH_ArkUI_DragAction_RegisterStatusListener(
                self.raw(),
                listener.as_ptr().cast(),
                Some(drag_action_status_listener_callback_trampoline)
            ))
        };

        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(listener.as_ptr()));
            }
            return Err(err);
        }

        self.status_listener = Some(listener);
        Ok(())
    }

    pub(crate) fn unregister_status_listener(&mut self) {
        unsafe { OH_ArkUI_DragAction_UnregisterStatusListener(self.raw()) }
        if let Some(listener) = self.status_listener.take() {
            unsafe {
                drop(Box::from_raw(listener.as_ptr()));
            }
        }
    }

    pub(crate) fn start_drag(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_StartDrag(self.raw())) }
    }
}

impl ArkUIHandle {
    pub(crate) fn create_drag_action_with_node(
        &self,
        node: &crate::ArkUINode,
    ) -> ArkUIResult<DragAction> {
        let _ = self.raw();
        DragAction::new_with_node(node)
    }

    pub(crate) fn create_drag_action_with_context(
        &self,
        ui_context: crate::ArkUIContext,
    ) -> ArkUIResult<DragAction> {
        let _ = self.raw();
        DragAction::new_with_context(ui_context)
    }
}

unsafe extern "C" fn drag_action_status_listener_callback_trampoline(
    drag_and_drop_info: *mut ArkUI_DragAndDropInfo,
    user_data: *mut c_void,
) {
    let Some(callback) = NonNull::new(user_data.cast::<DragStatusListenerCallbackContext>()) else {
        return;
    };
    let Some(drag_info) = (unsafe { DragAndDropInfo::from_raw(drag_and_drop_info) }) else {
        return;
    };
    let callback = unsafe { callback.as_ref() };
    (callback.callback)(&drag_info);
}
