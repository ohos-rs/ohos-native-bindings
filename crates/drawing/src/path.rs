use std::{ffi::CString, ptr::NonNull};

use ohos_native_drawing_sys::{
    OH_Drawing_Path, OH_Drawing_PathAddArc, OH_Drawing_PathAddCircle, OH_Drawing_PathAddOval,
    OH_Drawing_PathAddPath, OH_Drawing_PathAddRect, OH_Drawing_PathAddRoundRect,
    OH_Drawing_PathArcTo, OH_Drawing_PathBuildFromSvgString, OH_Drawing_PathClose,
    OH_Drawing_PathContains, OH_Drawing_PathCopy, OH_Drawing_PathCreate, OH_Drawing_PathCubicTo,
    OH_Drawing_PathDestroy, OH_Drawing_PathGetLength, OH_Drawing_PathLineTo, OH_Drawing_PathMoveTo,
    OH_Drawing_PathQuadTo, OH_Drawing_PathReset, OH_Drawing_PathSetFillType,
    OH_Drawing_PathTransform,
};

use crate::{Matrix, PathDirection, PathFillType, Rect, RoundRect};

#[derive(Debug)]
pub struct Path {
    raw: NonNull<OH_Drawing_Path>,
}

impl Path {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_PathCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_PathCreate returned null"),
        }
    }

    pub fn clone_path(&self) -> Self {
        let raw = unsafe { OH_Drawing_PathCopy(self.raw.as_ptr()) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_PathCopy returned null"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Path {
        self.raw.as_ptr()
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe { OH_Drawing_PathMoveTo(self.raw.as_ptr(), x, y) };
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe { OH_Drawing_PathLineTo(self.raw.as_ptr(), x, y) };
    }

    pub fn quadratic_curve_to(&mut self, control_x: f32, control_y: f32, x: f32, y: f32) {
        unsafe { OH_Drawing_PathQuadTo(self.raw.as_ptr(), control_x, control_y, x, y) };
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bezier_curve_to(
        &mut self,
        control_x1: f32,
        control_y1: f32,
        control_x2: f32,
        control_y2: f32,
        x: f32,
        y: f32,
    ) {
        unsafe {
            OH_Drawing_PathCubicTo(
                self.raw.as_ptr(),
                control_x1,
                control_y1,
                control_x2,
                control_y2,
                x,
                y,
            )
        };
    }

    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, start_deg: f32, sweep_deg: f32) {
        unsafe { OH_Drawing_PathArcTo(self.raw.as_ptr(), x1, y1, x2, y2, start_deg, sweep_deg) };
    }

    pub fn add_rect(
        &mut self,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        direction: PathDirection,
    ) {
        unsafe {
            OH_Drawing_PathAddRect(
                self.raw.as_ptr(),
                left,
                top,
                right,
                bottom,
                direction.into(),
            );
        }
    }

    pub fn add_circle(&mut self, x: f32, y: f32, radius: f32, direction: PathDirection) {
        unsafe { OH_Drawing_PathAddCircle(self.raw.as_ptr(), x, y, radius, direction.into()) };
    }

    pub fn add_oval(&mut self, rect: &Rect, direction: PathDirection) {
        unsafe { OH_Drawing_PathAddOval(self.raw.as_ptr(), rect.as_ptr(), direction.into()) };
    }

    pub fn add_arc(&mut self, rect: &Rect, start_degrees: f32, sweep_degrees: f32) {
        unsafe {
            OH_Drawing_PathAddArc(
                self.raw.as_ptr(),
                rect.as_ptr(),
                start_degrees,
                sweep_degrees,
            )
        };
    }

    pub fn add_round_rect(&mut self, round_rect: &RoundRect, direction: PathDirection) {
        unsafe {
            OH_Drawing_PathAddRoundRect(self.raw.as_ptr(), round_rect.as_ptr(), direction.into())
        };
    }

    pub fn add_path(&mut self, path: &Self, transform: Option<&Matrix>) {
        let identity;
        let transform = match transform {
            Some(transform) => transform,
            None => {
                identity = Matrix::new();
                &identity
            }
        };
        unsafe { OH_Drawing_PathAddPath(self.raw.as_ptr(), path.as_ptr(), transform.as_ptr()) };
    }

    pub fn transform(&mut self, transform: &Matrix) {
        unsafe { OH_Drawing_PathTransform(self.raw.as_ptr(), transform.as_ptr()) };
    }

    pub fn build_from_svg(&mut self, svg_path: &str) -> bool {
        let svg_path = CString::new(svg_path).expect("svg path contains interior NUL");
        unsafe { OH_Drawing_PathBuildFromSvgString(self.raw.as_ptr(), svg_path.as_ptr()) }
    }

    pub fn set_fill_type(&mut self, fill_type: PathFillType) {
        unsafe { OH_Drawing_PathSetFillType(self.raw.as_ptr(), fill_type.into()) };
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        unsafe { OH_Drawing_PathContains(self.raw.as_ptr(), x, y) }
    }

    pub fn length(&self, force_closed: bool) -> f32 {
        unsafe { OH_Drawing_PathGetLength(self.raw.as_ptr(), force_closed) }
    }

    pub fn close(&mut self) {
        unsafe { OH_Drawing_PathClose(self.raw.as_ptr()) };
    }

    pub fn reset(&mut self) {
        unsafe { OH_Drawing_PathReset(self.raw.as_ptr()) };
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { OH_Drawing_PathDestroy(self.raw.as_ptr()) };
    }
}
