use std::{
    ffi::{c_void, CString},
    ptr::NonNull,
};

use ohos_native_drawing_sys::OH_Drawing_CanvasDrawSingleCharacter;
#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::OH_Drawing_CanvasDrawSingleCharacterWithFeatures;
use ohos_native_drawing_sys::{
    OH_Drawing_Canvas, OH_Drawing_CanvasAttachBrush, OH_Drawing_CanvasAttachPen,
    OH_Drawing_CanvasBind, OH_Drawing_CanvasClear, OH_Drawing_CanvasClipPath,
    OH_Drawing_CanvasClipRect, OH_Drawing_CanvasConcatMatrix, OH_Drawing_CanvasCreate,
    OH_Drawing_CanvasDestroy, OH_Drawing_CanvasDetachBrush, OH_Drawing_CanvasDetachPen,
    OH_Drawing_CanvasDrawBitmap, OH_Drawing_CanvasDrawBitmapRect, OH_Drawing_CanvasDrawCircle,
    OH_Drawing_CanvasDrawColor, OH_Drawing_CanvasDrawImageRectWithSrc, OH_Drawing_CanvasDrawLine,
    OH_Drawing_CanvasDrawOval, OH_Drawing_CanvasDrawPath, OH_Drawing_CanvasDrawRect,
    OH_Drawing_CanvasDrawRoundRect, OH_Drawing_CanvasDrawTextBlob, OH_Drawing_CanvasGetHeight,
    OH_Drawing_CanvasGetSaveCount, OH_Drawing_CanvasGetTotalMatrix, OH_Drawing_CanvasGetWidth,
    OH_Drawing_CanvasReadPixels, OH_Drawing_CanvasResetMatrix, OH_Drawing_CanvasRestore,
    OH_Drawing_CanvasRotate, OH_Drawing_CanvasSave, OH_Drawing_CanvasScale,
    OH_Drawing_CanvasSetMatrix, OH_Drawing_CanvasSkew, OH_Drawing_CanvasTranslate,
    OH_Drawing_SrcRectConstraint_STRICT_SRC_RECT_CONSTRAINT,
};

#[cfg(feature = "api-20")]
use crate::FontFeatures;
use crate::{
    check_error, Bitmap, BlendMode, Brush, ClipOperation, Font, Image, ImageInfo, Matrix, Path,
    Pen, Point, Rect, Result, RoundRect, SamplingOptions, TextBlob,
};

#[derive(Debug)]
pub struct Canvas {
    raw: NonNull<OH_Drawing_Canvas>,
    owned: bool,
    bitmap: Option<Bitmap>,
}

impl Canvas {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CanvasCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CanvasCreate returned null"),
            owned: true,
            bitmap: None,
        }
    }

    pub fn with_bitmap(bitmap: Bitmap) -> Self {
        let canvas = Self::new();
        unsafe { OH_Drawing_CanvasBind(canvas.raw.as_ptr(), bitmap.as_ptr()) };
        // The canvas is dropped before this field because fields are dropped in
        // declaration order, so the native canvas cannot outlive its bitmap.
        canvas.with_owned_bitmap(bitmap)
    }

    fn with_owned_bitmap(mut self, bitmap: Bitmap) -> Self {
        self.bitmap = Some(bitmap);
        self
    }

    pub fn bitmap(&self) -> Option<&Bitmap> {
        self.bitmap.as_ref()
    }

    pub fn bitmap_mut(&mut self) -> Option<&mut Bitmap> {
        self.bitmap.as_mut()
    }

    /// Wraps canvas pointer from external APIs (for example ArkUI draw context).
    ///
    /// # Safety
    /// The caller must ensure:
    /// - `raw` points to a valid `OH_Drawing_Canvas`.
    /// - The pointer remains valid for the lifetime of returned `Canvas`.
    /// - The pointer is not destroyed by this wrapper (`owned` is false).
    pub unsafe fn from_raw_borrowed(raw: NonNull<c_void>) -> Self {
        Self {
            raw: raw.cast(),
            owned: false,
            bitmap: None,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Canvas {
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

    pub fn draw_color(&self, color: u32, blend_mode: BlendMode) -> Result<()> {
        let code =
            unsafe { OH_Drawing_CanvasDrawColor(self.raw.as_ptr(), color, blend_mode.into()) };
        check_error(code)
    }

    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        unsafe { OH_Drawing_CanvasDrawLine(self.raw.as_ptr(), x1, y1, x2, y2) };
    }

    pub fn draw_rect(&self, rect: &Rect) {
        unsafe { OH_Drawing_CanvasDrawRect(self.raw.as_ptr(), rect.as_ptr()) };
    }

    pub fn draw_oval(&self, rect: &Rect) {
        unsafe { OH_Drawing_CanvasDrawOval(self.raw.as_ptr(), rect.as_ptr()) };
    }

    pub fn draw_round_rect(&self, round_rect: &RoundRect) {
        unsafe { OH_Drawing_CanvasDrawRoundRect(self.raw.as_ptr(), round_rect.as_ptr()) };
    }

    pub fn draw_path(&self, path: &Path) {
        unsafe { OH_Drawing_CanvasDrawPath(self.raw.as_ptr(), path.as_ptr()) };
    }

    pub fn draw_circle(&self, center: &Point, radius: f32) {
        unsafe { OH_Drawing_CanvasDrawCircle(self.raw.as_ptr(), center.as_ptr(), radius) };
    }

    pub fn draw_text_blob(&self, text: &TextBlob, x: f32, y: f32) {
        unsafe { OH_Drawing_CanvasDrawTextBlob(self.raw.as_ptr(), text.as_ptr(), x, y) };
    }

    pub fn draw_single_character(
        &self,
        character: &str,
        font: &Font,
        x: f32,
        y: f32,
    ) -> Result<()> {
        let character =
            CString::new(character).map_err(|_| crate::DrawingError::invalid_parameter())?;
        let code = unsafe {
            OH_Drawing_CanvasDrawSingleCharacter(
                self.raw.as_ptr(),
                character.as_ptr(),
                font.as_ptr(),
                x,
                y,
            )
        };
        check_error(code)
    }

    #[cfg(feature = "api-20")]
    pub fn draw_single_character_with_features(
        &self,
        character: &str,
        font: &Font,
        x: f32,
        y: f32,
        features: &FontFeatures,
    ) -> Result<()> {
        let character =
            CString::new(character).map_err(|_| crate::DrawingError::invalid_parameter())?;
        let code = unsafe {
            OH_Drawing_CanvasDrawSingleCharacterWithFeatures(
                self.raw.as_ptr(),
                character.as_ptr(),
                font.as_ptr(),
                x,
                y,
                features.as_ptr(),
            )
        };
        check_error(code)
    }

    pub fn draw_bitmap(&self, bitmap: &Bitmap, left: f32, top: f32) {
        unsafe { OH_Drawing_CanvasDrawBitmap(self.raw.as_ptr(), bitmap.as_ptr(), left, top) };
    }

    pub fn draw_bitmap_rect(
        &self,
        bitmap: &Bitmap,
        source: Option<&Rect>,
        destination: &Rect,
        sampling: &SamplingOptions,
    ) {
        unsafe {
            OH_Drawing_CanvasDrawBitmapRect(
                self.raw.as_ptr(),
                bitmap.as_ptr(),
                source.map_or(std::ptr::null(), |rect| rect.as_ptr().cast_const()),
                destination.as_ptr(),
                sampling.as_ptr(),
            )
        };
    }

    pub fn draw_image_rect(
        &self,
        image: &Image<'_>,
        source: &Rect,
        destination: &Rect,
        sampling: &SamplingOptions,
    ) {
        unsafe {
            OH_Drawing_CanvasDrawImageRectWithSrc(
                self.raw.as_ptr(),
                image.as_ptr(),
                source.as_ptr(),
                destination.as_ptr(),
                sampling.as_ptr(),
                OH_Drawing_SrcRectConstraint_STRICT_SRC_RECT_CONSTRAINT,
            )
        };
    }

    pub fn clip_rect(&self, rect: &Rect, operation: ClipOperation, anti_alias: bool) {
        unsafe {
            OH_Drawing_CanvasClipRect(
                self.raw.as_ptr(),
                rect.as_ptr(),
                operation.into(),
                anti_alias,
            )
        };
    }

    pub fn clip_path(&self, path: &Path, operation: ClipOperation, anti_alias: bool) {
        unsafe {
            OH_Drawing_CanvasClipPath(
                self.raw.as_ptr(),
                path.as_ptr(),
                operation.into(),
                anti_alias,
            )
        };
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

    pub fn rotate(&self, radians: f32) {
        unsafe { OH_Drawing_CanvasRotate(self.raw.as_ptr(), radians.to_degrees(), 0.0, 0.0) };
    }

    pub fn rotate_degrees_around(&self, degrees: f32, pivot_x: f32, pivot_y: f32) {
        unsafe { OH_Drawing_CanvasRotate(self.raw.as_ptr(), degrees, pivot_x, pivot_y) };
    }

    pub fn skew(&self, sx: f32, sy: f32) {
        unsafe { OH_Drawing_CanvasSkew(self.raw.as_ptr(), sx, sy) };
    }

    pub fn concat(&self, matrix: &Matrix) {
        unsafe { OH_Drawing_CanvasConcatMatrix(self.raw.as_ptr(), matrix.as_ptr()) };
    }

    pub fn set_matrix(&self, matrix: &Matrix) {
        unsafe { OH_Drawing_CanvasSetMatrix(self.raw.as_ptr(), matrix.as_ptr()) };
    }

    pub fn reset_matrix(&self) {
        unsafe { OH_Drawing_CanvasResetMatrix(self.raw.as_ptr()) };
    }

    pub fn total_matrix(&self) -> Matrix {
        let matrix = Matrix::new();
        unsafe { OH_Drawing_CanvasGetTotalMatrix(self.raw.as_ptr(), matrix.as_ptr()) };
        matrix
    }

    pub fn read_pixels(
        &self,
        info: ImageInfo,
        destination: &mut [u8],
        row_bytes: u32,
        source_x: i32,
        source_y: i32,
    ) -> bool {
        let required = row_bytes as usize * info.height.max(0) as usize;
        if destination.len() < required {
            return false;
        }
        let mut raw_info = info.into_raw();
        unsafe {
            OH_Drawing_CanvasReadPixels(
                self.raw.as_ptr(),
                &mut raw_info,
                destination.as_mut_ptr().cast(),
                row_bytes,
                source_x,
                source_y,
            )
        }
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
