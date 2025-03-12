use std::{
    cell::{LazyCell, RefCell},
    os::raw::c_void,
    ptr::NonNull,
    rc::Rc,
};

use ohos_arkui_sys::{
    ArkUI_DialogDismissEvent, ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG,
    ArkUI_NativeDialogAPI_1, ArkUI_NativeDialogHandle, ArkUI_NodeHandle,
    OH_ArkUI_DialogDismissEvent_GetDismissReason, OH_ArkUI_DialogDismissEvent_GetUserData,
    OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{Alignment, ArkUIError, DialogDismissData, InnerDialogDismissData};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_DIALOG_API_1: LazyCell<ArkUINativeDialogAPI1> = LazyCell::new(|| {
    let api = ArkUINativeDialogAPI1::new();
    api
});

pub struct ArkUINativeDialogAPI1 {
    pub(crate) raw: NonNull<ArkUI_NativeDialogAPI_1>,
}

impl ArkUINativeDialogAPI1 {
    /// allow us to get the pointer of ArkUI_NativeDialogAPI_1 and use it directly
    pub fn raw(&self) -> NonNull<ArkUI_NativeDialogAPI_1> {
        self.raw
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeDialogAPI_1 = std::ptr::null_mut();
        let struct_name = c"ArkUI_NativeDialogAPI_1";
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeDialogAPI_1 is NULL");
        api = raw_ptr.cast();
        Self {
            raw: unsafe { NonNull::new_unchecked(api) },
        }
    }

    pub fn create(&self) -> Result<ArkUI_NativeDialogHandle, ArkUIError> {
        unsafe {
            if let Some(create_dialog_controller) = (*self.raw.as_ptr()).create {
                let ret = create_dialog_controller();
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is: ArkUI_NativeDialogAPI_1::create",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::create is None",
                )))
            }
        }
    }

    pub fn set_content(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        content: ArkUI_NodeHandle,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_content) = (*self.raw.as_ptr()).setContent {
                let ret = set_content(dialog, content);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setContent, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setContent is None",
                )))
            }
        }
    }

    pub fn set_content_alignment(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        alignment: Alignment,
        offset_x: f32,
        offset_y: f32,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_content_alignment) = (*self.raw.as_ptr()).setContentAlignment {
                let ret = set_content_alignment(dialog, alignment.into(), offset_x, offset_y);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setContentAlignment, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setContentAlignment is None",
                )))
            }
        }
    }

    pub fn set_background_color(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        color: u32,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_background_color) = (*self.raw.as_ptr()).setBackgroundColor {
                let ret = set_background_color(dialog, color);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setBackgroundColor, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setBackgroundColor is None",
                )))
            }
        }
    }

    pub fn set_corner_radius(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        top_left_radius: f32,
        top_right_radius: f32,
        bottom_left_radius: f32,
        bottom_right_radius: f32,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_corner_radius) = (*self.raw.as_ptr()).setCornerRadius {
                let ret = set_corner_radius(
                    dialog,
                    top_left_radius,
                    top_right_radius,
                    bottom_left_radius,
                    bottom_right_radius,
                );
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setCornerRadius, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setCornerRadius is None",
                )))
            }
        }
    }

    pub fn set_modal_mode(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        modal_mode: bool,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_modal_mode) = (*self.raw.as_ptr()).setModalMode {
                let ret = set_modal_mode(dialog, modal_mode);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setModalMode, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setModalMode is None",
                )))
            }
        }
    }

    pub fn set_auto_cancel(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        auto_cancel: bool,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_auto_cancel) = (*self.raw.as_ptr()).setAutoCancel {
                let ret = set_auto_cancel(dialog, auto_cancel);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::setAutoCancel, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::setAutoCancel is None",
                )))
            }
        }
    }

    pub fn show(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        show_in_sub_window: bool,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(show) = (*self.raw.as_ptr()).show {
                let ret = show(dialog, show_in_sub_window);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::show, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::show is None",
                )))
            }
        }
    }

    pub fn close(&self, dialog: ArkUI_NativeDialogHandle) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(close_dialog) = (*self.raw.as_ptr()).close {
                let ret = close_dialog(dialog);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::close, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::close is None",
                )))
            }
        }
    }

    pub fn dispose(&self, dialog: ArkUI_NativeDialogHandle) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(dispose_dialog) = (*self.raw.as_ptr()).dispose {
                dispose_dialog(dialog);
                Ok(())
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::dispose is None",
                )))
            }
        }
    }

    pub(crate) fn register_dismiss(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        data: Rc<RefCell<InnerDialogDismissData>>,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(register_dismiss_with_data) =
                (*self.raw.as_ptr()).registerOnWillDismissWithUserData
            {
                let ret = register_dismiss_with_data(
                    dialog,
                    Box::into_raw(Box::new(data)) as *mut c_void,
                    Some(dialog_dismiss_callback),
                );
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeDialogAPI_1::registerOnWillDismissWithUserData, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUI_NativeDialogAPI_1::registerOnWillDismissWithUserData is None",
                )))
            }
        }
    }
}

unsafe extern "C" fn dialog_dismiss_callback(event: *mut ArkUI_DialogDismissEvent) {
    let user_data = OH_ArkUI_DialogDismissEvent_GetUserData(event);

    #[cfg(debug_assertions)]
    assert!(!user_data.is_null(), "user_data is NULL");

    let user_data_rc: &Rc<RefCell<InnerDialogDismissData>> =
        &*(user_data as *const Rc<RefCell<InnerDialogDismissData>>);

    let data = user_data_rc.borrow_mut();

    if let Some(handle) = data.dismiss_handle.as_ref() {
        let reason = OH_ArkUI_DialogDismissEvent_GetDismissReason(event) as u32;

        let ret = handle(DialogDismissData {
            dismiss_reason: reason.into(),
            data: data.data,
        });
        if let Some(block) = ret {
            if block {
                OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss(event, block);
            }
        }
    }
}
