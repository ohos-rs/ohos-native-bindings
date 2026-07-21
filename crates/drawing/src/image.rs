use std::marker::PhantomData;
use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_Image, OH_Drawing_ImageBuildFromBitmap, OH_Drawing_ImageCreate,
    OH_Drawing_ImageDestroy, OH_Drawing_ImageGetHeight, OH_Drawing_ImageGetWidth,
};

use crate::Bitmap;

#[derive(Debug)]
pub struct Image<'bitmap> {
    raw: NonNull<OH_Drawing_Image>,
    _bitmap: PhantomData<&'bitmap Bitmap>,
}

impl<'bitmap> Image<'bitmap> {
    pub fn from_bitmap(bitmap: &'bitmap Bitmap) -> Option<Self> {
        let raw = unsafe { OH_Drawing_ImageCreate() };
        let raw = NonNull::new(raw)?;
        if unsafe { OH_Drawing_ImageBuildFromBitmap(raw.as_ptr(), bitmap.as_ptr()) } {
            Some(Self {
                raw,
                _bitmap: PhantomData,
            })
        } else {
            unsafe { OH_Drawing_ImageDestroy(raw.as_ptr()) };
            None
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Image {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> i32 {
        unsafe { OH_Drawing_ImageGetWidth(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> i32 {
        unsafe { OH_Drawing_ImageGetHeight(self.raw.as_ptr()) }
    }
}

impl Drop for Image<'_> {
    fn drop(&mut self) {
        unsafe { OH_Drawing_ImageDestroy(self.raw.as_ptr()) };
    }
}
