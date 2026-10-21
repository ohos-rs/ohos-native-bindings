//! Module api::node_custom_event::geometry wrappers and related types.

use std::{marker::PhantomData, os::raw::c_void, ptr::NonNull};

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

#[derive(Debug, PartialEq, Eq)]
/// An owned constraint snapshot obtained during a live custom-node event.
///
/// ```compile_fail
/// use ohos_arkui_binding::api::node_custom_event::{LayoutConstraintHandle, NodeCustomEvent};
/// fn escape(event: &NodeCustomEvent) -> LayoutConstraintHandle<'static> {
///     event.layout_constraint_in_measure().unwrap()
/// }
/// ```
pub struct LayoutConstraintHandle<'event> {
    raw: NonNull<ArkUI_LayoutConstraint>,
    _event: PhantomData<&'event super::NodeCustomEvent>,
}

impl<'event> LayoutConstraintHandle<'event> {
    /// Maximum measured width in physical pixels.
    pub fn max_width(&self) -> i32 {
        // SAFETY: this guard owns the live constraint allocation.
        unsafe { ohos_arkui_sys::OH_ArkUI_LayoutConstraint_GetMaxWidth(self.raw()) }
    }

    /// Maximum measured height in physical pixels.
    pub fn max_height(&self) -> i32 {
        // SAFETY: this guard owns the live constraint allocation.
        unsafe { ohos_arkui_sys::OH_ArkUI_LayoutConstraint_GetMaxHeight(self.raw()) }
    }

    pub(crate) fn from_raw(
        raw: *mut ArkUI_LayoutConstraint,
        _event: &'event super::NodeCustomEvent,
    ) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
            _event: PhantomData,
        })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_LayoutConstraint {
        self.raw.as_ptr()
    }
}

impl Drop for LayoutConstraintHandle<'_> {
    fn drop(&mut self) {
        // SAFETY: GetLayoutConstraintInMeasure returns a new allocation and
        // this non-Clone guard is its sole owner.
        unsafe { ohos_arkui_sys::OH_ArkUI_LayoutConstraint_Dispose(self.raw()) };
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
