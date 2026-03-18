use std::ptr::NonNull;

use ohos_arkui_sys::{
    ArkUI_DragPreviewOption, OH_ArkUI_CreateDragPreviewOption, OH_ArkUI_DragPreviewOption_Dispose,
    OH_ArkUI_DragPreviewOption_SetBadgeNumber,
    OH_ArkUI_DragPreviewOption_SetDefaultAnimationBeforeLiftingEnabled,
    OH_ArkUI_DragPreviewOption_SetDefaultRadiusEnabled,
    OH_ArkUI_DragPreviewOption_SetDefaultShadowEnabled,
    OH_ArkUI_DragPreviewOption_SetNumberBadgeEnabled, OH_ArkUI_DragPreviewOption_SetScaleMode,
};

use crate::{check_arkui_status, ArkUIResult};

use super::helper::non_null_or_error;

pub(crate) struct DragPreviewOption {
    raw: NonNull<ArkUI_DragPreviewOption>,
}

impl DragPreviewOption {
    pub(crate) fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_CreateDragPreviewOption() };
        non_null_or_error(option, "OH_ArkUI_CreateDragPreviewOption").map(Self::from_non_null)
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_DragPreviewOption) -> ArkUIResult<Self> {
        non_null_or_error(raw, "ArkUI_DragPreviewOption").map(Self::from_non_null)
    }

    pub(crate) fn from_non_null(raw: NonNull<ArkUI_DragPreviewOption>) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DragPreviewOption {
        self.raw.as_ptr()
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_DragPreviewOption {
        self.raw.as_ptr()
    }

    pub(crate) fn dispose(self) {
        unsafe { OH_ArkUI_DragPreviewOption_Dispose(self.raw()) }
    }

    pub(crate) fn set_scale_mode(
        &self,
        scale_mode: crate::DragPreviewScaleMode,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragPreviewOption_SetScaleMode(
                self.raw(),
                scale_mode.into()
            ))
        }
    }

    pub(crate) fn set_default_shadow_enabled(&self, enabled: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragPreviewOption_SetDefaultShadowEnabled(
                self.raw(),
                enabled
            ))
        }
    }

    pub(crate) fn set_default_radius_enabled(&self, enabled: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragPreviewOption_SetDefaultRadiusEnabled(
                self.raw(),
                enabled
            ))
        }
    }

    pub(crate) fn set_number_badge_enabled(&self, enabled: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragPreviewOption_SetNumberBadgeEnabled(
                self.raw(),
                enabled
            ))
        }
    }

    pub(crate) fn set_badge_number(&self, forced_number: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DragPreviewOption_SetBadgeNumber(
                self.raw(),
                forced_number
            ))
        }
    }

    pub(crate) fn set_default_animation_before_lifting_enabled(
        &self,
        enabled: bool,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                OH_ArkUI_DragPreviewOption_SetDefaultAnimationBeforeLiftingEnabled(
                    self.raw(),
                    enabled
                )
            )
        }
    }
}
