use std::ptr::NonNull;

use ohos_drawing_sys::{
    OH_Drawing_BlendMode, OH_Drawing_Canvas, OH_Drawing_CanvasAttachBrush,
    OH_Drawing_CanvasAttachPen, OH_Drawing_CanvasClear, OH_Drawing_CanvasCreate,
    OH_Drawing_CanvasDestroy, OH_Drawing_CanvasDetachBrush, OH_Drawing_CanvasDetachPen,
    OH_Drawing_CanvasDrawCircle, OH_Drawing_CanvasDrawColor, OH_Drawing_CanvasDrawLine,
    OH_Drawing_CanvasDrawPath, OH_Drawing_CanvasDrawRect, OH_Drawing_CanvasGetHeight,
    OH_Drawing_CanvasGetSaveCount, OH_Drawing_CanvasGetWidth, OH_Drawing_CanvasRestore,
    OH_Drawing_CanvasSave, OH_Drawing_CanvasScale, OH_Drawing_CanvasTranslate,
};

use crate::{check_error, Brush, Point, Rect, Result};
use crate::{path::Path, pen::Pen};

#[derive(Debug)]
pub struct Canvas {
    raw: NonNull<OH_Drawing_Canvas>,
    owned: bool,
}

impl Canvas {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CanvasCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CanvasCreate returned null"),
            owned: true,
        }
    }

    /// Wraps canvas pointer from external APIs (for example ArkUI draw context).
    ///
    /// # Safety
    /// The caller must ensure:
    /// - `raw` is a valid non-null `OH_Drawing_Canvas` pointer.
    /// - The pointer remains valid for the lifetime of returned `Canvas`.
    /// - The pointer is not destroyed by this wrapper (`owned` is false).
    pub unsafe fn from_raw_borrowed(raw: *mut OH_Drawing_Canvas) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_Canvas is null"),
            owned: false,
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Canvas {
        self.raw.as_ptr()
    }

    pub fn save(&self) {
        unsafe { OH_Drawing_CanvasSave(self.raw.as_ptr()) };
    }

    pub fn restore(&self) {
        unsafe { OH_Drawing_CanvasRestore(self.raw.as_ptr()) };
    }

    pub fn save_count(&self) -> u32 {
        unsafe { OH_Drawing_CanvasGetSaveCount(self.raw.as_ptr()) }
    }

    pub fn clear(&self, color: u32) {
        unsafe { OH_Drawing_CanvasClear(self.raw.as_ptr(), color) };
    }

    pub fn draw_color(&self, color: u32, blend_mode: OH_Drawing_BlendMode) -> Result<()> {
        let code = unsafe { OH_Drawing_CanvasDrawColor(self.raw.as_ptr(), color, blend_mode) };
        check_error(code)
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { OH_Drawing_CanvasDrawLine(self.raw.as_ptr(), x1, y1, x2, y2) };
    }

    pub fn draw_rect(&self, rect: &Rect) {
        unsafe { OH_Drawing_CanvasDrawRect(self.raw.as_ptr(), rect.as_ptr()) };
    }

    pub fn draw_path(&self, path: &Path) {
        unsafe { OH_Drawing_CanvasDrawPath(self.raw.as_ptr(), path.as_ptr()) };
    }

    pub fn draw_circle(&self, center: &Point, radius: f32) {
        unsafe { OH_Drawing_CanvasDrawCircle(self.raw.as_ptr(), center.as_ptr(), radius) };
    }

    pub fn attach_pen(&self, pen: &Pen) {
        unsafe { OH_Drawing_CanvasAttachPen(self.raw.as_ptr(), pen.as_ptr()) };
    }

    pub fn detach_pen(&self) {
        unsafe { OH_Drawing_CanvasDetachPen(self.raw.as_ptr()) };
    }

    pub fn attach_brush(&self, brush: &Brush) {
        unsafe { OH_Drawing_CanvasAttachBrush(self.raw.as_ptr(), brush.as_ptr()) };
    }

    pub fn detach_brush(&self) {
        unsafe { OH_Drawing_CanvasDetachBrush(self.raw.as_ptr()) };
    }

    pub fn translate(&self, dx: f32, dy: f32) {
        unsafe { OH_Drawing_CanvasTranslate(self.raw.as_ptr(), dx, dy) };
    }

    pub fn scale(&self, sx: f32, sy: f32) {
        unsafe { OH_Drawing_CanvasScale(self.raw.as_ptr(), sx, sy) };
    }

    pub fn width(&self) -> i32 {
        unsafe { OH_Drawing_CanvasGetWidth(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { OH_Drawing_CanvasGetHeight(self.raw.as_ptr()) }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        if self.owned {
            unsafe { OH_Drawing_CanvasDestroy(self.raw.as_ptr()) };
        }
    }
}
