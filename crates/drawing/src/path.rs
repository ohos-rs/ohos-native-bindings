use std::{ffi::CString, ptr::NonNull};

use ohos_native_drawing_sys::{
    OH_Drawing_Path, OH_Drawing_PathAddCircle, OH_Drawing_PathAddRect, OH_Drawing_PathArcTo,
    OH_Drawing_PathBuildFromSvgString, OH_Drawing_PathClose, OH_Drawing_PathContains,
    OH_Drawing_PathCopy, OH_Drawing_PathCreate, OH_Drawing_PathDestroy, OH_Drawing_PathDirection,
    OH_Drawing_PathFillType, OH_Drawing_PathGetLength, OH_Drawing_PathLineTo,
    OH_Drawing_PathMoveTo, OH_Drawing_PathReset, OH_Drawing_PathSetFillType,
};

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

    pub fn as_ptr(&self) -> *mut OH_Drawing_Path {
        self.raw.as_ptr()
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        unsafe { OH_Drawing_PathMoveTo(self.raw.as_ptr(), x, y) };
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        unsafe { OH_Drawing_PathLineTo(self.raw.as_ptr(), x, y) };
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
        direction: OH_Drawing_PathDirection,
    ) {
        unsafe {
            OH_Drawing_PathAddRect(self.raw.as_ptr(), left, top, right, bottom, direction);
        }
    }

    pub fn add_circle(&mut self, x: f32, y: f32, radius: f32, direction: OH_Drawing_PathDirection) {
        unsafe { OH_Drawing_PathAddCircle(self.raw.as_ptr(), x, y, radius, direction) };
    }

    pub fn build_from_svg(&mut self, svg_path: &str) -> bool {
        let svg_path = CString::new(svg_path).expect("svg path contains interior NUL");
        unsafe { OH_Drawing_PathBuildFromSvgString(self.raw.as_ptr(), svg_path.as_ptr()) }
    }

    pub fn set_fill_type(&mut self, fill_type: OH_Drawing_PathFillType) {
        unsafe { OH_Drawing_PathSetFillType(self.raw.as_ptr(), fill_type) };
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
