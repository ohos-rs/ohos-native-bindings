use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_Point, OH_Drawing_PointCreate, OH_Drawing_PointDestroy, OH_Drawing_PointGetX,
    OH_Drawing_PointGetY, OH_Drawing_PointSet, OH_Drawing_Rect, OH_Drawing_RectCreate,
    OH_Drawing_RectDestroy, OH_Drawing_RectGetBottom, OH_Drawing_RectGetHeight,
    OH_Drawing_RectGetLeft, OH_Drawing_RectGetRight, OH_Drawing_RectGetTop,
    OH_Drawing_RectGetWidth, OH_Drawing_RectSetBottom, OH_Drawing_RectSetLeft,
    OH_Drawing_RectSetRight, OH_Drawing_RectSetTop,
};

#[derive(Debug)]
pub struct Point {
    raw: NonNull<OH_Drawing_Point>,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        let raw = unsafe { OH_Drawing_PointCreate(x, y) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_PointCreate returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Point {
        self.raw.as_ptr()
    }

    pub fn set(&mut self, x: f32, y: f32) {
        unsafe { OH_Drawing_PointSet(self.raw.as_ptr(), x, y) };
    }

    pub fn x(&self) -> f32 {
        let mut value = 0.0;
        unsafe { OH_Drawing_PointGetX(self.raw.as_ptr(), &mut value) };
        value
    }

    pub fn y(&self) -> f32 {
        let mut value = 0.0;
        unsafe { OH_Drawing_PointGetY(self.raw.as_ptr(), &mut value) };
        value
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        unsafe { OH_Drawing_PointDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct Rect {
    raw: NonNull<OH_Drawing_Rect>,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        let raw = unsafe { OH_Drawing_RectCreate(left, top, right, bottom) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_RectCreate returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Rect {
        self.raw.as_ptr()
    }

    pub fn left(&self) -> f32 {
        unsafe { OH_Drawing_RectGetLeft(self.raw.as_ptr()) }
    }

    pub fn top(&self) -> f32 {
        unsafe { OH_Drawing_RectGetTop(self.raw.as_ptr()) }
    }

    pub fn right(&self) -> f32 {
        unsafe { OH_Drawing_RectGetRight(self.raw.as_ptr()) }
    }

    pub fn bottom(&self) -> f32 {
        unsafe { OH_Drawing_RectGetBottom(self.raw.as_ptr()) }
    }

    pub fn width(&self) -> f32 {
        unsafe { OH_Drawing_RectGetWidth(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> f32 {
        unsafe { OH_Drawing_RectGetHeight(self.raw.as_ptr()) }
    }

    pub fn set_left(&mut self, left: f32) {
        unsafe { OH_Drawing_RectSetLeft(self.raw.as_ptr(), left) };
    }

    pub fn set_top(&mut self, top: f32) {
        unsafe { OH_Drawing_RectSetTop(self.raw.as_ptr(), top) };
    }

    pub fn set_right(&mut self, right: f32) {
        unsafe { OH_Drawing_RectSetRight(self.raw.as_ptr(), right) };
    }

    pub fn set_bottom(&mut self, bottom: f32) {
        unsafe { OH_Drawing_RectSetBottom(self.raw.as_ptr(), bottom) };
    }
}

impl Drop for Rect {
    fn drop(&mut self) {
        unsafe { OH_Drawing_RectDestroy(self.raw.as_ptr()) };
    }
}
