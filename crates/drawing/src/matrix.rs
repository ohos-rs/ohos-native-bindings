use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_Matrix, OH_Drawing_MatrixConcat, OH_Drawing_MatrixCreate, OH_Drawing_MatrixDestroy,
    OH_Drawing_MatrixGetAll, OH_Drawing_MatrixInvert, OH_Drawing_MatrixMapPoints,
    OH_Drawing_MatrixSetMatrix, OH_Drawing_Point2D,
};

use crate::{check_error, Point, Result};

#[derive(Debug)]
pub struct Matrix {
    raw: NonNull<OH_Drawing_Matrix>,
}

impl Matrix {
    pub fn new() -> Self {
        Self::from_raw(unsafe { OH_Drawing_MatrixCreate() })
    }

    pub fn from_affine(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        let mut matrix = Self::new();
        matrix.set_affine(a, b, c, d, e, f);
        matrix
    }

    fn from_raw(raw: *mut OH_Drawing_Matrix) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_MatrixCreate returned null"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Matrix {
        self.raw.as_ptr()
    }

    pub fn set_affine(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        unsafe { OH_Drawing_MatrixSetMatrix(self.raw.as_ptr(), a, c, e, b, d, f, 0.0, 0.0, 1.0) };
    }

    pub fn affine_values(&self) -> Result<[f32; 6]> {
        let mut values = [0.0; 9];
        let code = unsafe { OH_Drawing_MatrixGetAll(self.raw.as_ptr(), values.as_mut_ptr()) };
        check_error(code)?;
        Ok([
            values[0], values[3], values[1], values[4], values[2], values[5],
        ])
    }

    pub fn concat(first: &Self, second: &Self) -> Self {
        let result = Self::new();
        unsafe {
            OH_Drawing_MatrixConcat(result.raw.as_ptr(), first.raw.as_ptr(), second.raw.as_ptr())
        };
        result
    }

    pub fn inverted(&self) -> Option<Self> {
        let inverse = Self::new();
        unsafe { OH_Drawing_MatrixInvert(self.raw.as_ptr(), inverse.raw.as_ptr()) }
            .then_some(inverse)
    }

    pub fn map_points(&self, points: &[Point]) -> Vec<Point> {
        let source: Vec<_> = points
            .iter()
            .map(|point| OH_Drawing_Point2D {
                x: point.x(),
                y: point.y(),
            })
            .collect();
        let mut destination = vec![OH_Drawing_Point2D { x: 0.0, y: 0.0 }; source.len()];
        unsafe {
            OH_Drawing_MatrixMapPoints(
                self.raw.as_ptr(),
                source.as_ptr(),
                destination.as_mut_ptr(),
                source.len() as i32,
            )
        };
        destination
            .into_iter()
            .map(|point| Point::new(point.x, point.y))
            .collect()
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        let [a, b, c, d, e, f] = self
            .affine_values()
            .expect("OH_Drawing_MatrixGetAll failed for a live matrix");
        Self::from_affine(a, b, c, d, e, f)
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe { OH_Drawing_MatrixDestroy(self.raw.as_ptr()) };
    }
}
