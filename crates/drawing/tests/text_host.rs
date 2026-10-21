//! Production text-array ownership and legacy UTF-8 preparation.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code
)]
extern crate self as ohos_native_drawing_sys;
use std::sync::Arc;
pub struct OH_Drawing_TextBox {
    values: Vec<([f32; 4], bool)>,
    lifetime: Arc<()>,
    fail: bool,
}
const OH_Drawing_TextDirection_TEXT_DIRECTION_RTL: u32 = 0;
unsafe fn OH_Drawing_GetSizeOfTextBox(p: *mut OH_Drawing_TextBox) -> usize {
    unsafe { (*p).values.len() }
}
unsafe fn coordinate(p: *mut OH_Drawing_TextBox, index: i32, axis: usize) -> f32 {
    unsafe {
        assert!(!(*p).fail, "getter failed");
        (&(*p).values)[index as usize].0[axis]
    }
}
unsafe fn OH_Drawing_GetLeftFromTextBox(p: *mut OH_Drawing_TextBox, i: i32) -> f32 {
    unsafe { coordinate(p, i, 0) }
}
unsafe fn OH_Drawing_GetTopFromTextBox(p: *mut OH_Drawing_TextBox, i: i32) -> f32 {
    unsafe { coordinate(p, i, 1) }
}
unsafe fn OH_Drawing_GetRightFromTextBox(p: *mut OH_Drawing_TextBox, i: i32) -> f32 {
    unsafe { coordinate(p, i, 2) }
}
unsafe fn OH_Drawing_GetBottomFromTextBox(p: *mut OH_Drawing_TextBox, i: i32) -> f32 {
    unsafe { coordinate(p, i, 3) }
}
unsafe fn OH_Drawing_GetTextDirectionFromTextBox(p: *mut OH_Drawing_TextBox, i: i32) -> i32 {
    unsafe {
        if (&(*p).values)[i as usize].1 {
            0
        } else {
            1
        }
    }
}
unsafe fn OH_Drawing_TypographyDestroyTextBox(p: *mut OH_Drawing_TextBox) {
    unsafe { drop(Box::from_raw(p)) }
}
#[path = "../src/text_boxes.rs"]
mod text_boxes;
#[path = "../src/text_encoding.rs"]
mod text_encoding;
const OH_Drawing_TextDecoration_TEXT_DECORATION_NONE: u32 = 0;
const OH_Drawing_TextDecoration_TEXT_DECORATION_UNDERLINE: u32 = 1;
const OH_Drawing_TextDecoration_TEXT_DECORATION_OVERLINE: u32 = 2;
const OH_Drawing_TextDecoration_TEXT_DECORATION_LINE_THROUGH: u32 = 4;
#[path = "../src/text_decoration.rs"]
mod text_decoration;
#[test]
fn snapshots_outlive_native_storage_and_preserve_visual_direction() {
    let marker = Arc::new(());
    let weak = Arc::downgrade(&marker);
    let raw = Box::into_raw(Box::new(OH_Drawing_TextBox {
        values: vec![([1., 2., 3., 4.], false), ([5., 6., 7., 8.], true)],
        lifetime: marker,
        fail: false,
    }));
    let ranges = text_boxes::TextBoxes::collect(raw);
    assert!(weak.upgrade().is_none());
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[1].bounds, [5., 6., 7., 8.]);
    assert!(!ranges[0].rtl);
    assert!(ranges[1].rtl);
    assert!(text_boxes::TextBoxes::collect(std::ptr::null_mut()).is_empty());
}
#[test]
fn array_guard_releases_storage_if_copying_unwinds() {
    let marker = Arc::new(());
    let weak = Arc::downgrade(&marker);
    let raw = Box::into_raw(Box::new(OH_Drawing_TextBox {
        values: vec![([1.; 4], false)],
        lifetime: marker,
        fail: true,
    }));
    assert!(std::panic::catch_unwind(|| text_boxes::TextBoxes::collect(raw)).is_err());
    assert!(weak.upgrade().is_none());
}
#[test]
fn legacy_text_preserves_utf8_and_rejects_nuls_without_replacement() {
    for text in ["", "a😀界"] {
        let native = text_encoding::nul_terminated(text).unwrap();
        assert_eq!(native.to_str().unwrap(), text);
        assert_eq!(native.as_bytes_with_nul().last(), Some(&0));
    }
    for text in ["\0", "a\0😀\0界"] {
        assert!(text_encoding::nul_terminated(text).is_err());
    }
}

#[test]
fn all_native_decoration_bits_are_caller_combinable() {
    use text_decoration::TextDecoration as D;
    assert_eq!(D::NONE.bits(), 0);
    assert_eq!((D::UNDERLINE | D::OVERLINE | D::LINE_THROUGH).bits(), 7);
    let mut decoration = D::OVERLINE;
    decoration |= D::LINE_THROUGH;
    assert_eq!(decoration.bits(), 6);
}
