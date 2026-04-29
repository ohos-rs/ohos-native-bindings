//! Module api::dialog wrappers and related types.

use std::{
    cell::{LazyCell, RefCell},
    ffi::CString,
    os::raw::c_void,
    ptr::NonNull,
    rc::Rc,
};

use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_DialogDismissEvent, ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG,
    ArkUI_NativeDialogAPI_1, ArkUI_NativeDialogHandle,
    OH_ArkUI_DialogDismissEvent_GetDismissReason, OH_ArkUI_DialogDismissEvent_GetUserData,
    OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{
    check_arkui_status, Alignment, ArkUIError, ArkUIResult, DialogDismissData,
    InnerDialogDismissData,
};

thread_local! {
    /// ArkUI_NativeNodeAPI_1 struct
    /// Only can be used in main thread
    pub(crate) static ARK_UI_NATIVE_DIALOG_API_1: LazyCell<ArkUINativeDialogAPI1> =
    LazyCell::new(ArkUINativeDialogAPI1::new);
}

pub(crate) struct ArkUINativeDialogAPI1(pub(crate) NonNull<ArkUI_NativeDialogAPI_1>);

impl ArkUINativeDialogAPI1 {
    /// allow us to get the pointer of ArkUI_NativeDialogAPI_1 and use it directly
    pub(crate) fn raw(&self) -> *mut ArkUI_NativeDialogAPI_1 {
        self.0.as_ptr()
    }

    pub(crate) fn new() -> Self {
        let struct_name = CString::new("ArkUI_NativeDialogAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG,
                struct_name.as_ptr().cast(),
            )
        };
        let api = NonNull::new(raw_ptr.cast())
            .unwrap_or_else(|| panic!("ArkUI_NativeDialogAPI_1 is NULL"));
        Self(api)
    }

    pub(crate) fn create(&self) -> ArkUIResult<ArkUI_NativeDialogHandle> {
        unsafe {
            if let Some(create_dialog_controller) = (*self.raw()).create {
                let ret = create_dialog_controller();
                Ok(ret)
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::create is None",
                ))
            }
        }
    }

    pub(crate) fn set_content(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        content: ArkUI_NodeHandle,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_content) = (*self.raw()).setContent {
                check_arkui_status!(set_content(dialog, content))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setContent is None",
                ))
            }
        }
    }

    pub(crate) fn set_content_alignment(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        alignment: Alignment,
        offset_x: f32,
        offset_y: f32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_content_alignment) = (*self.raw()).setContentAlignment {
                check_arkui_status!(set_content_alignment(
                    dialog,
                    alignment.into(),
                    offset_x,
                    offset_y
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setContentAlignment is None",
                ))
            }
        }
    }

    pub(crate) fn set_background_color(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        color: u32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_background_color) = (*self.raw()).setBackgroundColor {
                check_arkui_status!(set_background_color(dialog, color))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setBackgroundColor is None",
                ))
            }
        }
    }

    pub(crate) fn set_corner_radius(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        top_left_radius: f32,
        top_right_radius: f32,
        bottom_left_radius: f32,
        bottom_right_radius: f32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_corner_radius) = (*self.raw()).setCornerRadius {
                check_arkui_status!(set_corner_radius(
                    dialog,
                    top_left_radius,
                    top_right_radius,
                    bottom_left_radius,
                    bottom_right_radius
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setCornerRadius is None",
                ))
            }
        }
    }

    pub(crate) fn set_modal_mode(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        modal_mode: bool,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_modal_mode) = (*self.raw()).setModalMode {
                check_arkui_status!(set_modal_mode(dialog, modal_mode))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setModalMode is None",
                ))
            }
        }
    }

    pub(crate) fn set_auto_cancel(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        auto_cancel: bool,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_auto_cancel) = (*self.raw()).setAutoCancel {
                check_arkui_status!(set_auto_cancel(dialog, auto_cancel))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::setAutoCancel is None",
                ))
            }
        }
    }

    pub(crate) fn show(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        show_in_sub_window: bool,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(show) = (*self.raw()).show {
                check_arkui_status!(show(dialog, show_in_sub_window))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::show is None",
                ))
            }
        }
    }

    pub(crate) fn close(&self, dialog: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        unsafe {
            if let Some(close_dialog) = (*self.raw()).close {
                check_arkui_status!(close_dialog(dialog))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::close is None",
                ))
            }
        }
    }

    pub(crate) fn dispose(&self, dialog: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        unsafe {
            if let Some(dispose_dialog) = (*self.raw()).dispose {
                dispose_dialog(dialog);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::dispose is None",
                ))
            }
        }
    }

    pub(crate) fn register_dismiss(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        data: Rc<RefCell<InnerDialogDismissData>>,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_dismiss_with_data) =
                (*self.raw()).registerOnWillDismissWithUserData
            {
                check_arkui_status!(register_dismiss_with_data(
                    dialog,
                    Box::into_raw(Box::new(data)) as *mut c_void,
                    Some(dialog_dismiss_callback)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::registerOnWillDismissWithUserData is None",
                ))
            }
        }
    }
}

impl Default for ArkUINativeDialogAPI1 {
    fn default() -> Self {
        Self::new()
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
