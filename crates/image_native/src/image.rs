use std::{mem::MaybeUninit, ptr::NonNull};

use crate::{
    common::NativeBufferHandle,
    error::{check_status, ImageNativeResult},
    sys,
};

/// Owned native image wrapper.
pub struct Image {
    raw: NonNull<sys::OH_ImageNative>,
}

impl Image {
    /// Creates a wrapper from a raw image pointer.
    pub fn from_raw(raw: *mut sys::OH_ImageNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw image pointer.
    pub fn as_raw(&self) -> *mut sys::OH_ImageNative {
        self.raw.as_ptr()
    }

    /// Returns image size.
    pub fn size(&self) -> ImageNativeResult<crate::types::ImageSize> {
        let mut size = MaybeUninit::<crate::types::ImageSize>::uninit();
        check_status(unsafe {
            sys::OH_ImageNative_GetImageSize(self.as_raw(), size.as_mut_ptr())
        })?;
        Ok(unsafe { size.assume_init() })
    }

    /// Returns supported component types.
    pub fn component_types(&self) -> ImageNativeResult<Vec<u32>> {
        let mut types = std::ptr::null_mut();
        let mut len = 0;
        check_status(unsafe {
            sys::OH_ImageNative_GetComponentTypes(self.as_raw(), &mut types, &mut len)
        })?;
        if types.is_null() || len == 0 {
            Ok(Vec::new())
        } else {
            Ok(unsafe { std::slice::from_raw_parts(types, len) }.to_vec())
        }
    }

    /// Returns the native buffer of a component type.
    pub fn byte_buffer(
        &self,
        component_type: u32,
    ) -> ImageNativeResult<Option<NativeBufferHandle>> {
        let mut buffer = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageNative_GetByteBuffer(self.as_raw(), component_type, &mut buffer)
        })?;
        Ok(NativeBufferHandle::from_raw(buffer))
    }

    /// Returns buffer size for a component type.
    pub fn buffer_size(&self, component_type: u32) -> ImageNativeResult<usize> {
        let mut size = 0;
        check_status(unsafe {
            sys::OH_ImageNative_GetBufferSize(self.as_raw(), component_type, &mut size)
        })?;
        Ok(size)
    }

    /// Returns row stride for a component type.
    pub fn row_stride(&self, component_type: u32) -> ImageNativeResult<i32> {
        let mut stride = 0;
        check_status(unsafe {
            sys::OH_ImageNative_GetRowStride(self.as_raw(), component_type, &mut stride)
        })?;
        Ok(stride)
    }

    /// Returns pixel stride for a component type.
    pub fn pixel_stride(&self, component_type: u32) -> ImageNativeResult<i32> {
        let mut stride = 0;
        check_status(unsafe {
            sys::OH_ImageNative_GetPixelStride(self.as_raw(), component_type, &mut stride)
        })?;
        Ok(stride)
    }

    /// Returns image timestamp.
    pub fn timestamp(&self) -> ImageNativeResult<i64> {
        let mut timestamp = 0;
        check_status(unsafe { sys::OH_ImageNative_GetTimestamp(self.as_raw(), &mut timestamp) })?;
        Ok(timestamp)
    }

    /// Returns image color-space name.
    #[cfg(feature = "api-23")]
    pub fn color_space(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_ImageNative_GetColorSpace(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    /// Returns image format.
    #[cfg(feature = "api-23")]
    pub fn format(&self) -> ImageNativeResult<crate::types::NativeBufferFormat> {
        let mut format = 0;
        check_status(unsafe { sys::OH_ImageNative_GetFormat(self.as_raw(), &mut format) })?;
        Ok(crate::types::NativeBufferFormat::from(format as i32))
    }

    /// Returns image buffer metadata.
    #[cfg(feature = "api-23")]
    pub fn buffer_data(&self) -> ImageNativeResult<crate::types::ImageBufferData> {
        let mut data = MaybeUninit::<crate::types::ImageBufferData>::uninit();
        check_status(unsafe {
            sys::OH_ImageNative_GetBufferData(self.as_raw(), data.as_mut_ptr())
        })?;
        Ok(unsafe { data.assume_init() })
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImageNative_Release(self.as_raw()) });
    }
}

/// Backward-compatible alias.
pub type NativeImage = Image;
