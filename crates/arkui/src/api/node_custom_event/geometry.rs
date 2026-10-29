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
    /// Allows a child to be smaller than the parent while retaining the maximum
    /// sizes and percentage references of this owned constraint snapshot.
    pub fn loosen(&mut self) {
        // SAFETY: this guard exclusively owns the live constraint allocation.
        unsafe {
            ohos_arkui_sys::OH_ArkUI_LayoutConstraint_SetMinWidth(self.raw(), 0);
            ohos_arkui_sys::OH_ArkUI_LayoutConstraint_SetMinHeight(self.raw(), 0);
        }
    }

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

#[cfg(all(test, not(target_env = "ohos")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug, PartialEq, Eq)]
    struct Constraint {
        min: [i32; 2],
        max: [i32; 2],
        percent: [i32; 2],
    }
    static DISPOSED: AtomicUsize = AtomicUsize::new(0);

    #[no_mangle]
    unsafe extern "C" fn OH_ArkUI_LayoutConstraint_SetMinWidth(
        raw: *mut ArkUI_LayoutConstraint,
        width: i32,
    ) {
        unsafe { (*raw.cast::<Constraint>()).min[0] = width };
    }
    #[no_mangle]
    unsafe extern "C" fn OH_ArkUI_LayoutConstraint_SetMinHeight(
        raw: *mut ArkUI_LayoutConstraint,
        height: i32,
    ) {
        unsafe { (*raw.cast::<Constraint>()).min[1] = height };
    }
    #[no_mangle]
    unsafe extern "C" fn OH_ArkUI_LayoutConstraint_Dispose(raw: *mut ArkUI_LayoutConstraint) {
        drop(unsafe { Box::from_raw(raw.cast::<Constraint>()) });
        DISPOSED.fetch_add(1, Ordering::Relaxed);
    }

    #[test]
    fn loose_child_preserves_available_size_and_percentage_reference() {
        let raw = Box::into_raw(Box::new(Constraint {
            min: [1260, 2506],
            max: [1260, 2506],
            percent: [1260, 2720],
        }));
        let mut constraint = LayoutConstraintHandle {
            raw: NonNull::new(raw.cast()).unwrap(),
            _event: PhantomData,
        };
        constraint.loosen();
        // SAFETY: the live guard owns this mock allocation.
        assert_eq!(
            unsafe { &*raw },
            &Constraint {
                min: [0, 0],
                max: [1260, 2506],
                percent: [1260, 2720],
            }
        );
        drop(constraint);
        assert_eq!(DISPOSED.load(Ordering::Relaxed), 1);
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
