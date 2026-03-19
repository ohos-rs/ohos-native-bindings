//! Module api::node_utils::types wrappers and related types.

#[cfg(feature = "api-21")]
use std::os::raw::c_void;
use std::ptr::NonNull;

#[cfg(feature = "api-21")]
use ohos_arkui_sys::{ArkUI_NodeEvent, OH_ArkUI_NodeEvent_GetUserData};
use ohos_arkui_sys::{
    ArkUI_SystemFontStyleEvent, OH_ArkUI_SystemFontStyleEvent_GetFontSizeScale,
    OH_ArkUI_SystemFontStyleEvent_GetFontWeightScale,
};

#[cfg(feature = "api-21")]
#[derive(Clone, Copy, Debug)]
/// Borrowed reference to `ArkUI_NodeEvent`.
pub struct NodeEventRef {
    raw: NonNull<ArkUI_NodeEvent>,
}

#[cfg(feature = "api-21")]
impl NodeEventRef {
    pub(super) fn from_raw(raw: *mut ArkUI_NodeEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_NodeEvent {
        self.raw.as_ptr()
    }

    /// Returns user data associated with the event, if present.
    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { OH_ArkUI_NodeEvent_GetUserData(self.raw()) })
    }
}

#[derive(Clone, Copy, Debug)]
/// Borrowed reference to `ArkUI_SystemFontStyleEvent`.
pub struct SystemFontStyleEventRef {
    raw: NonNull<ArkUI_SystemFontStyleEvent>,
}

impl SystemFontStyleEventRef {
    pub(super) fn from_raw(raw: *mut ArkUI_SystemFontStyleEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn from_const_raw(raw: *const ArkUI_SystemFontStyleEvent) -> Option<Self> {
        Self::from_raw(raw.cast_mut())
    }

    fn raw(&self) -> *mut ArkUI_SystemFontStyleEvent {
        self.raw.as_ptr()
    }

    /// Returns the system font size scale.
    pub fn font_size_scale(&self) -> f32 {
        unsafe { OH_ArkUI_SystemFontStyleEvent_GetFontSizeScale(self.raw()) }
    }

    /// Returns the system font weight scale.
    pub fn font_weight_scale(&self) -> f32 {
        unsafe { OH_ArkUI_SystemFontStyleEvent_GetFontWeightScale(self.raw()) }
    }
}
