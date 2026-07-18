use std::ptr::NonNull;
use std::slice;

use ohos_native_drawing_sys::{
    OH_Drawing_Bitmap, OH_Drawing_BitmapBuild, OH_Drawing_BitmapCreate, OH_Drawing_BitmapDestroy,
    OH_Drawing_BitmapFormat, OH_Drawing_BitmapGetHeight, OH_Drawing_BitmapGetPixels,
    OH_Drawing_BitmapGetWidth, OH_Drawing_BitmapReadPixels,
};

use crate::{BitmapFormat, ImageInfo};

#[derive(Debug)]
pub struct Bitmap {
    raw: NonNull<OH_Drawing_Bitmap>,
    format: BitmapFormat,
}

impl Bitmap {
    pub fn new(width: u32, height: u32, format: BitmapFormat) -> Self {
        let raw = unsafe { OH_Drawing_BitmapCreate() };
        let bitmap = Self {
            raw: NonNull::new(raw).expect("OH_Drawing_BitmapCreate returned null"),
            format,
        };
        let raw_format = OH_Drawing_BitmapFormat {
            colorFormat: format.color.into(),
            alphaFormat: format.alpha.into(),
        };
        unsafe { OH_Drawing_BitmapBuild(bitmap.raw.as_ptr(), width, height, &raw_format) };
        bitmap
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Bitmap {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> u32 {
        unsafe { OH_Drawing_BitmapGetWidth(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> u32 {
        unsafe { OH_Drawing_BitmapGetHeight(self.raw.as_ptr()) }
    }

    pub fn pixels(&self) -> &[u8] {
        let length = self.byte_length();
        let pixels = unsafe { OH_Drawing_BitmapGetPixels(self.raw.as_ptr()) }.cast::<u8>();
        if pixels.is_null() || length == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(pixels, length) }
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        let length = self.byte_length();
        let pixels = unsafe { OH_Drawing_BitmapGetPixels(self.raw.as_ptr()) }.cast::<u8>();
        if pixels.is_null() || length == 0 {
            return &mut [];
        }
        unsafe { slice::from_raw_parts_mut(pixels, length) }
    }

    fn byte_length(&self) -> usize {
        let bytes_per_pixel = match self.format.color {
            crate::ColorFormat::Alpha8 => 1,
            crate::ColorFormat::Rgb565 | crate::ColorFormat::Argb4444 => 2,
            crate::ColorFormat::Rgba8888 | crate::ColorFormat::Bgra8888 => 4,
            crate::ColorFormat::Unknown => 0,
        };
        self.width() as usize * self.height() as usize * bytes_per_pixel
    }

    pub fn read_pixels(
        &self,
        info: ImageInfo,
        destination: &mut [u8],
        row_bytes: usize,
        source_x: i32,
        source_y: i32,
    ) -> bool {
        let required = row_bytes.saturating_mul(info.height.max(0) as usize);
        if destination.len() < required {
            return false;
        }
        let raw_info = info.into_raw();
        unsafe {
            OH_Drawing_BitmapReadPixels(
                self.raw.as_ptr(),
                &raw_info,
                destination.as_mut_ptr().cast(),
                row_bytes,
                source_x,
                source_y,
            )
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { OH_Drawing_BitmapDestroy(self.raw.as_ptr()) };
    }
}
