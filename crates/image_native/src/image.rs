use std::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::Deref,
    ptr::NonNull,
};

use crate::{
    error::{check_status, ImageNativeResult},
    sys,
};
use ohos_native_buffer_binding::NativeBufferRef;

/// Owned native image wrapper.
pub struct Image {
    raw: NonNull<sys::OH_ImageNative>,
}

/// Borrowed native image owned by another ImageKit object.
///
/// Unlike [`Image`], this wrapper does not call `OH_ImageNative_Release` when
/// dropped. It is intended for APIs such as `OH_PhotoNative_GetMainImage`,
/// where the containing native object retains ownership of the image.
pub struct ImageRef<'a> {
    image: ManuallyDrop<Image>,
    _owner: PhantomData<&'a sys::OH_ImageNative>,
}

impl<'a> ImageRef<'a> {
    /// Creates a borrowed wrapper from a raw image pointer.
    ///
    /// # Safety
    ///
    /// `raw` must remain valid for `'a`, and its owner must not be released
    /// while the returned wrapper is alive.
    pub unsafe fn from_raw(raw: *mut sys::OH_ImageNative) -> Option<Self> {
        Image::from_raw(raw).map(|image| Self {
            image: ManuallyDrop::new(image),
            _owner: PhantomData,
        })
    }
}

impl Deref for ImageRef<'_> {
    type Target = Image;

    fn deref(&self) -> &Self::Target {
        &self.image
    }
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
        let mut len = 0;
        // The native API uses a two-call protocol. Its first `types` argument
        // must itself be null so ImageKit only reports the required length.
        // Passing `&mut null_mut()` is not equivalent: some device versions
        // treat a non-null outer pointer as writable storage and dereference
        // the null inner pointer.
        check_status(unsafe {
            sys::OH_ImageNative_GetComponentTypes(self.as_raw(), std::ptr::null_mut(), &mut len)
        })?;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut components = vec![0; len];
        let mut types = components.as_mut_ptr();
        check_status(unsafe {
            sys::OH_ImageNative_GetComponentTypes(self.as_raw(), &mut types, &mut len)
        })?;
        if types.is_null() || len == 0 {
            return Ok(Vec::new());
        }

        // ImageKit normally writes into `components`, as documented. Copying
        // from the returned pointer also handles implementations that replace
        // it with native-owned storage on the second call.
        Ok(unsafe { std::slice::from_raw_parts(types, len) }.to_vec())
    }

    /// Returns the native buffer of a component type.
    pub fn byte_buffer(
        &self,
        component_type: u32,
    ) -> ImageNativeResult<Option<NativeBufferRef<'_>>> {
        let mut buffer = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageNative_GetByteBuffer(self.as_raw(), component_type, &mut buffer)
        })?;
        let byte_len = self.buffer_size(component_type)?;
        // SAFETY: ImageKit owns the returned buffer for `self` and reports its
        // readable component length through GetBufferSize.
        Ok(unsafe { NativeBufferRef::from_raw_parts(buffer, byte_len) })
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
