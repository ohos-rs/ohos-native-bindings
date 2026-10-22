//! Pure host regressions for callback-owned event decoding.
#![allow(non_snake_case, non_camel_case_types, dead_code)]

extern crate self as ohos_arkui_sys;

use std::{
    ffi::c_void,
    sync::atomic::{AtomicUsize, Ordering},
};

pub struct NodeCustomEvent;
pub struct ArkUI_LayoutConstraint;
pub struct ArkUI_DrawContext;
pub struct ArkUI_IntOffset {
    pub x: i32,
    pub y: i32,
}
pub struct ArkUI_IntSize {
    pub width: i32,
    pub height: i32,
}
static RELEASED: AtomicUsize = AtomicUsize::new(0);

unsafe fn OH_ArkUI_LayoutConstraint_GetMaxWidth(_: *mut ArkUI_LayoutConstraint) -> i32 {
    320
}
unsafe fn OH_ArkUI_LayoutConstraint_GetMaxHeight(_: *mut ArkUI_LayoutConstraint) -> i32 {
    240
}
unsafe fn OH_ArkUI_LayoutConstraint_Dispose(raw: *mut ArkUI_LayoutConstraint) -> *mut c_void {
    // SAFETY: the test transfers exactly one Box into each production guard.
    unsafe {
        drop(Box::from_raw(raw));
    }
    RELEASED.fetch_add(1, Ordering::SeqCst);
    std::ptr::null_mut()
}
unsafe fn OH_ArkUI_DrawContext_GetCanvas(_: *mut ArkUI_DrawContext) -> *mut c_void {
    std::ptr::null_mut()
}
unsafe fn OH_ArkUI_DrawContext_GetSize(_: *mut ArkUI_DrawContext) -> ArkUI_IntSize {
    ArkUI_IntSize {
        width: 0,
        height: 0,
    }
}

#[path = "../src/api/node_custom_event/geometry.rs"]
mod geometry;

#[path = "../src/event/mixed_string.rs"]
mod mixed_string;

#[test]
fn constraint_allocations_retire_once_on_return_and_unwind() {
    let event = NodeCustomEvent;
    let before = RELEASED.load(Ordering::SeqCst);
    {
        let raw = Box::into_raw(Box::new(ArkUI_LayoutConstraint));
        let guard = geometry::LayoutConstraintHandle::from_raw(raw, &event).unwrap();
        assert_eq!((guard.max_width(), guard.max_height()), (320, 240));
    }
    assert_eq!(RELEASED.load(Ordering::SeqCst), before + 1);
    let _ = std::panic::catch_unwind(|| {
        let raw = Box::into_raw(Box::new(ArkUI_LayoutConstraint));
        let _guard = geometry::LayoutConstraintHandle::from_raw(raw, &event).unwrap();
        panic!("consumer failed");
    });
    assert_eq!(RELEASED.load(Ordering::SeqCst), before + 2);
}
