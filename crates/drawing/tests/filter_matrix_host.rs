//! Production matrix and filter wrappers with statically substituted native effects.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code
)]
extern crate self as ohos_native_drawing_sys;
use std::{
    ptr,
    sync::{Arc, Weak},
};
#[derive(Debug)]
pub struct OH_Drawing_Matrix([f32; 9]);
#[derive(Clone, Copy)]
pub struct OH_Drawing_Point2D {
    x: f32,
    y: f32,
}
#[derive(Clone, Copy)]
pub struct Point(f32, f32);
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
}
pub type Result<T> = std::result::Result<T, i32>;
fn check_error(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(code)
    }
}
unsafe fn OH_Drawing_MatrixCreate() -> *mut OH_Drawing_Matrix {
    Box::into_raw(Box::new(OH_Drawing_Matrix([
        1., 0., 0., 0., 1., 0., 0., 0., 1.,
    ])))
}
unsafe fn OH_Drawing_MatrixDestroy(p: *mut OH_Drawing_Matrix) {
    unsafe { drop(Box::from_raw(p)) }
}
#[allow(clippy::too_many_arguments)]
unsafe fn OH_Drawing_MatrixSetMatrix(
    p: *mut OH_Drawing_Matrix,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
    i: f32,
) {
    unsafe { (*p).0 = [a, b, c, d, e, f, g, h, i] }
}
unsafe fn OH_Drawing_MatrixGetAll(p: *mut OH_Drawing_Matrix, out: *mut f32) -> i32 {
    unsafe { ptr::copy_nonoverlapping((*p).0.as_ptr(), out, 9) };
    0
}
unsafe fn OH_Drawing_MatrixConcat(
    _: *mut OH_Drawing_Matrix,
    _: *mut OH_Drawing_Matrix,
    _: *mut OH_Drawing_Matrix,
) {
    unimplemented!()
}
unsafe fn OH_Drawing_MatrixInvert(_: *mut OH_Drawing_Matrix, _: *mut OH_Drawing_Matrix) -> bool {
    unimplemented!()
}
unsafe fn OH_Drawing_MatrixMapPoints(
    _: *mut OH_Drawing_Matrix,
    _: *const OH_Drawing_Point2D,
    _: *mut OH_Drawing_Point2D,
    _: i32,
) {
    unimplemented!()
}
#[derive(Debug)]
pub struct OH_Drawing_ImageFilter(Arc<()>);
#[derive(Debug)]
pub struct OH_Drawing_Filter(Option<Arc<()>>);
#[derive(Debug)]
pub struct OH_Drawing_ColorFilter;
#[derive(Debug)]
pub struct OH_Drawing_MaskFilter;
const OH_Drawing_BlurType_NORMAL: u32 = 0;
#[derive(Clone, Copy)]
pub enum BlendMode {
    Src,
}
impl From<BlendMode> for u32 {
    fn from(_: BlendMode) -> Self {
        0
    }
}
#[derive(Clone, Copy)]
pub enum TileMode {
    Clamp,
}
impl From<TileMode> for u32 {
    fn from(_: TileMode) -> Self {
        0
    }
}
unsafe fn OH_Drawing_ImageFilterCreateBlur(
    _: f32,
    _: f32,
    _: u32,
    _: *mut OH_Drawing_ImageFilter,
) -> *mut OH_Drawing_ImageFilter {
    Box::into_raw(Box::new(OH_Drawing_ImageFilter(Arc::new(()))))
}
unsafe fn OH_Drawing_ImageFilterDestroy(p: *mut OH_Drawing_ImageFilter) {
    unsafe { drop(Box::from_raw(p)) }
}
unsafe fn OH_Drawing_FilterCreate() -> *mut OH_Drawing_Filter {
    Box::into_raw(Box::new(OH_Drawing_Filter(None)))
}
unsafe fn OH_Drawing_FilterDestroy(p: *mut OH_Drawing_Filter) {
    unsafe { drop(Box::from_raw(p)) }
}
unsafe fn OH_Drawing_FilterSetImageFilter(
    p: *mut OH_Drawing_Filter,
    v: *mut OH_Drawing_ImageFilter,
) {
    unsafe {
        (*p).0 = if v.is_null() {
            None
        } else {
            Some((*v).0.clone())
        }
    }
}
unsafe fn OH_Drawing_FilterSetColorFilter(
    _: *mut OH_Drawing_Filter,
    _: *mut OH_Drawing_ColorFilter,
) {
}
unsafe fn OH_Drawing_FilterSetMaskFilter(_: *mut OH_Drawing_Filter, _: *mut OH_Drawing_MaskFilter) {
}
unsafe fn OH_Drawing_ColorFilterCreateBlendMode(_: u32, _: u32) -> *mut OH_Drawing_ColorFilter {
    Box::into_raw(Box::new(OH_Drawing_ColorFilter))
}
unsafe fn OH_Drawing_ColorFilterCreateMatrix(_: *const f32) -> *mut OH_Drawing_ColorFilter {
    Box::into_raw(Box::new(OH_Drawing_ColorFilter))
}
unsafe fn OH_Drawing_ColorFilterDestroy(p: *mut OH_Drawing_ColorFilter) {
    unsafe { drop(Box::from_raw(p)) }
}
unsafe fn OH_Drawing_MaskFilterCreateBlur(_: u32, _: f32, _: bool) -> *mut OH_Drawing_MaskFilter {
    Box::into_raw(Box::new(OH_Drawing_MaskFilter))
}
unsafe fn OH_Drawing_MaskFilterDestroy(p: *mut OH_Drawing_MaskFilter) {
    unsafe { drop(Box::from_raw(p)) }
}
#[path = "../src/filter.rs"]
mod filter;
#[path = "../src/matrix.rs"]
mod matrix;
#[test]
fn clone_preserves_perspective_after_original_owner_is_dropped() {
    let values = [2., 3., 4., 5., 6., 7., 0.125, 0.25, 1.];
    let clone = {
        let original = matrix::Matrix::from_values(values);
        original.clone()
    };
    assert_eq!(clone.values().unwrap(), values);
    assert_eq!(
        matrix::Matrix::from_affine(1., 2., 3., 4., 5., 6.)
            .values()
            .unwrap(),
        [1., 3., 5., 2., 4., 6., 0., 0., 1.]
    );
}
#[test]
fn native_filter_retains_value_after_image_wrapper_is_dropped_and_clear_releases_it() {
    let mut holder = filter::Filter::new();
    let image = filter::ImageFilter::blur(2., 3., TileMode::Clamp).unwrap();
    holder.set_image_filter(Some(&image));
    // SAFETY: holder owns this mock native filter until the end of this test.
    let weak: Weak<()> = unsafe { Arc::downgrade((*holder.as_ptr()).0.as_ref().unwrap()) };
    drop(image);
    assert!(weak.upgrade().is_some());
    holder.set_image_filter(None);
    assert!(weak.upgrade().is_none());
}
#[test]
fn blur_rejects_invalid_sigmas_before_native_creation() {
    for sigma in [f32::NAN, f32::INFINITY, -1., 0.] {
        assert!(filter::ImageFilter::blur(sigma, 1., TileMode::Clamp).is_none());
        assert!(filter::ImageFilter::blur(1., sigma, TileMode::Clamp).is_none());
    }
    assert!(filter::ColorFilter::blend(0xff00ff00, BlendMode::Src).is_some());
}
