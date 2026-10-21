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
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::ParamInvalid,
                        "dialog creation returned null",
                    ))
                } else {
                    Ok(ret)
                }
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

    pub(crate) fn remove_content(&self, dialog: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        // SAFETY: the caller owns this live controller on its UI thread.
        unsafe {
            let function = (*self.raw()).removeContent.ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "dialog removeContent is unavailable",
                )
            })?;
            check_arkui_status!(function(dialog))
        }
    }
    pub(crate) fn custom_style(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        enabled: bool,
    ) -> ArkUIResult<()> {
        // SAFETY: the caller owns this live controller on its UI thread.
        unsafe {
            let function = (*self.raw()).enableCustomStyle.ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "dialog custom style is unavailable",
                )
            })?;
            check_arkui_status!(function(dialog, enabled))
        }
    }
    pub(crate) fn custom_animation(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        enabled: bool,
    ) -> ArkUIResult<()> {
        // SAFETY: the caller owns this live controller on its UI thread.
        unsafe {
            let function = (*self.raw()).enableCustomAnimation.ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "dialog custom animation is unavailable",
                )
            })?;
            check_arkui_status!(function(dialog, enabled))
        }
    }
    pub(crate) fn register_dismiss(
        &self,
        dialog: ArkUI_NativeDialogHandle,
        data: *mut c_void,
    ) -> ArkUIResult<()> {
        // SAFETY: Dialog retains the stable context until controller disposal.
        unsafe {
            let function = (*self.raw())
                .registerOnWillDismissWithUserData
                .ok_or_else(|| {
                    ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "dialog dismiss callback is unavailable",
                    )
                })?;
            check_arkui_status!(function(dialog, data, Some(dialog_dismiss_callback)))
        }
    }
}

impl Default for ArkUINativeDialogAPI1 {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "C" fn dialog_dismiss_callback(event: *mut ArkUI_DialogDismissEvent) {
    if event.is_null() {
        return;
    }
    let user_data = OH_ArkUI_DialogDismissEvent_GetUserData(event);

    if user_data.is_null() {
        return;
    }
    let pointer = user_data as *const RefCell<InnerDialogDismissData>;
    // SAFETY: registration retains this Rc allocation until native disposal.
    // Keep a callback-owned reference so reentrant disposal cannot free it.
    let user_data_rc = unsafe {
        Rc::increment_strong_count(pointer);
        Rc::from_raw(pointer)
    };
    let Ok(data) = user_data_rc.try_borrow() else {
        return;
    };
    if let Some(callback) = data.callback.clone() {
        drop(data);
        let reason = OH_ArkUI_DialogDismissEvent_GetDismissReason(event) as u32;
        let blocked =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(reason.into())))
                .unwrap_or(true);
        OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss(event, blocked);
        return;
    }

    let handle = data.dismiss_handle;
    let custom_data = data.data;
    let _custom_owner = data.data_owner.clone();
    drop(data);
    if let Some(handle) = handle {
        let reason = OH_ArkUI_DialogDismissEvent_GetDismissReason(event) as u32;
        let ret = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            handle(DialogDismissData {
                dismiss_reason: reason.into(),
                data: custom_data,
            })
        }))
        .unwrap_or(Some(true));
        if let Some(block) = ret {
            if block {
                OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss(event, block);
            }
        }
    }
}
