//! Module api::node_custom_event::geometry wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_sys::{
    ArkUI_DrawContext, ArkUI_IntOffset, ArkUI_IntSize, ArkUI_LayoutConstraint,
    OH_ArkUI_DrawContext_GetCanvas, OH_ArkUI_DrawContext_GetSize,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Integer offset used by node custom-event geometry APIs.
pub struct IntOffset {
    pub x: i32,
    pub y: i32,
}

impl From<ArkUI_IntOffset> for IntOffset {
    fn from(value: ArkUI_IntOffset) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<IntOffset> for ArkUI_IntOffset {
    fn from(value: IntOffset) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Integer size used by node custom-event geometry APIs.
pub struct IntSize {
    pub width: i32,
    pub height: i32,
}

impl From<ArkUI_IntSize> for IntSize {
    fn from(value: ArkUI_IntSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<IntSize> for ArkUI_IntSize {
    fn from(value: IntSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Borrowed wrapper for `ArkUI_LayoutConstraint`.
pub struct LayoutConstraintHandle {
    raw: NonNull<ArkUI_LayoutConstraint>,
}

impl LayoutConstraintHandle {
    pub(crate) fn from_raw(raw: *mut ArkUI_LayoutConstraint) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_LayoutConstraint {
        self.raw.as_ptr()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Borrowed wrapper for `ArkUI_DrawContext`.
pub struct DrawContext {
    raw: NonNull<ArkUI_DrawContext>,
}

impl DrawContext {
    pub(crate) fn from_raw(raw: *mut ArkUI_DrawContext) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DrawContext {
        self.raw.as_ptr()
    }

    /// Returns canvas handle exposed by ArkUI, if available.
    pub fn canvas(&self) -> Option<NonNull<c_void>> {
        let canvas = unsafe { OH_ArkUI_DrawContext_GetCanvas(self.raw()) };
        NonNull::new(canvas)
    }

    /// Returns draw target size.
    pub fn size(&self) -> IntSize {
        let size = unsafe { OH_ArkUI_DrawContext_GetSize(self.raw()) };
        size.into()
    }
}
