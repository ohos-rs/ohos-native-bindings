use std::ptr::NonNull;

use crate::{
    common::{non_null, output_string, ImageString},
    error::{check_status, ImageNativeResult},
    pixel_map::PixelMap,
    sys,
    types::{ImageRegion, ImageSize, PixelFormat, RawFileDescriptor},
};

#[cfg(feature = "api-20")]
use crate::common::collect_mime_types;

#[cfg(feature = "api-22")]
const CREATE_PIXELMAP_ERROR: sys::Image_ErrorCode =
    sys::Image_ErrorCode_IMAGE_CREATE_PIXELMAP_FAILED;
#[cfg(not(feature = "api-22"))]
const CREATE_PIXELMAP_ERROR: sys::Image_ErrorCode = sys::Image_ErrorCode_IMAGE_ALLOC_FAILED;

/// Image source information.
pub struct ImageSourceInfo {
    raw: NonNull<sys::OH_ImageSource_Info>,
}

impl ImageSourceInfo {
    /// Creates a new image-source info object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSourceInfo_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ImageSource_Info {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_ImageSourceInfo_GetWidth(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn height(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_ImageSourceInfo_GetHeight(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn dynamic_range(&self) -> ImageNativeResult<bool> {
        let mut value = false;
        check_status(unsafe {
            sys::OH_ImageSourceInfo_GetDynamicRange(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-20")]
    pub fn mime_type(&self) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe { sys::OH_ImageSourceInfo_GetMimeType(self.as_raw(), value) })
    }
}

impl Drop for ImageSourceInfo {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImageSourceInfo_Release(self.as_raw()) });
    }
}

/// Decoding options for image sources.
pub struct DecodingOptions {
    raw: NonNull<sys::OH_DecodingOptions>,
}

impl DecodingOptions {
    /// Creates a new decoding options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_DecodingOptions_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_DecodingOptions {
        self.raw.as_ptr()
    }

    pub fn pixel_format(&self) -> ImageNativeResult<PixelFormat> {
        let mut value = 0;
        check_status(unsafe { sys::OH_DecodingOptions_GetPixelFormat(self.as_raw(), &mut value) })?;
        Ok(PixelFormat::from_i32_or_unknown(value))
    }

    pub fn set_pixel_format(&mut self, value: crate::types::PixelFormat) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_DecodingOptions_SetPixelFormat(self.as_raw(), value.into()) })
    }

    pub fn index(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_DecodingOptions_GetIndex(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_index(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_DecodingOptions_SetIndex(self.as_raw(), value) })
    }

    pub fn rotate(&self) -> ImageNativeResult<f32> {
        let mut value = 0.0;
        check_status(unsafe { sys::OH_DecodingOptions_GetRotate(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_rotate(&mut self, value: f32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_DecodingOptions_SetRotate(self.as_raw(), value) })
    }

    pub fn desired_size(&self) -> ImageNativeResult<ImageSize> {
        let mut value = ImageSize {
            width: 0,
            height: 0,
        };
        check_status(unsafe { sys::OH_DecodingOptions_GetDesiredSize(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_desired_size(&mut self, value: ImageSize) -> ImageNativeResult<()> {
        let mut value = value;
        check_status(unsafe { sys::OH_DecodingOptions_SetDesiredSize(self.as_raw(), &mut value) })
    }

    pub fn desired_region(&self) -> ImageNativeResult<ImageRegion> {
        let mut value = ImageRegion {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        check_status(unsafe {
            sys::OH_DecodingOptions_GetDesiredRegion(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_desired_region(&mut self, value: ImageRegion) -> ImageNativeResult<()> {
        let mut value = value;
        check_status(unsafe { sys::OH_DecodingOptions_SetDesiredRegion(self.as_raw(), &mut value) })
    }

    pub fn desired_dynamic_range(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_DecodingOptions_GetDesiredDynamicRange(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_desired_dynamic_range(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_DecodingOptions_SetDesiredDynamicRange(self.as_raw(), value)
        })
    }

    #[cfg(feature = "api-18")]
    pub fn crop_and_scale_strategy(&self) -> ImageNativeResult<u32> {
        let mut value = 0i32;
        check_status(unsafe {
            sys::OH_DecodingOptions_GetCropAndScaleStrategy(self.as_raw(), &mut value)
        })?;
        Ok(value as u32)
    }

    #[cfg(feature = "api-18")]
    pub fn set_crop_and_scale_strategy(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_DecodingOptions_SetCropAndScaleStrategy(self.as_raw(), value as i32)
        })
    }

    #[cfg(feature = "api-20")]
    pub fn desired_color_space(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_DecodingOptions_GetDesiredColorSpace(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-20")]
    pub fn set_desired_color_space(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_DecodingOptions_SetDesiredColorSpace(self.as_raw(), value) })
    }

    #[cfg(feature = "api-19")]
    pub fn crop_region(&self) -> ImageNativeResult<ImageRegion> {
        let mut value = ImageRegion {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
        check_status(unsafe { sys::OH_DecodingOptions_GetCropRegion(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    #[cfg(feature = "api-19")]
    pub fn set_crop_region(&mut self, value: ImageRegion) -> ImageNativeResult<()> {
        let mut value = value;
        check_status(unsafe { sys::OH_DecodingOptions_SetCropRegion(self.as_raw(), &mut value) })
    }
}

impl Drop for DecodingOptions {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_DecodingOptions_Release(self.as_raw()) });
    }
}

/// Picture decoding options.
#[cfg(feature = "api-13")]
pub struct DecodingOptionsForPicture {
    raw: NonNull<sys::OH_DecodingOptionsForPicture>,
}

#[cfg(feature = "api-13")]
impl DecodingOptionsForPicture {
    /// Creates a new picture decoding options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_DecodingOptionsForPicture_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_DecodingOptionsForPicture {
        self.raw.as_ptr()
    }

    pub fn desired_auxiliary_pictures(
        &self,
    ) -> ImageNativeResult<Vec<crate::types::AuxiliaryPictureType>> {
        let mut raw = std::ptr::null_mut();
        let mut len = 0;
        check_status(unsafe {
            sys::OH_DecodingOptionsForPicture_GetDesiredAuxiliaryPictures(
                self.as_raw(),
                &mut raw,
                &mut len,
            )
        })?;
        if raw.is_null() || len == 0 {
            Ok(Vec::new())
        } else {
            Ok(unsafe { std::slice::from_raw_parts(raw, len) }
                .iter()
                .filter_map(|value| crate::types::AuxiliaryPictureType::from_raw(*value))
                .collect())
        }
    }

    pub fn set_desired_auxiliary_pictures(
        &mut self,
        values: &[crate::types::AuxiliaryPictureType],
    ) -> ImageNativeResult<()> {
        let mut raw_values: Vec<_> = values.iter().copied().map(Into::into).collect();
        check_status(unsafe {
            sys::OH_DecodingOptionsForPicture_SetDesiredAuxiliaryPictures(
                self.as_raw(),
                raw_values.as_mut_ptr(),
                raw_values.len(),
            )
        })
    }
}

#[cfg(feature = "api-13")]
impl Drop for DecodingOptionsForPicture {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_DecodingOptionsForPicture_Release(self.as_raw()) });
    }
}

/// Owned native image source wrapper.
pub struct ImageSource {
    raw: NonNull<sys::OH_ImageSourceNative>,
}

impl ImageSource {
    /// Creates a wrapper from a raw image-source pointer.
    pub fn from_raw(raw: *mut sys::OH_ImageSourceNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ImageSourceNative {
        self.raw.as_ptr()
    }

    /// Creates an image source from a URI.
    pub fn create_from_uri(uri: &std::ffi::CStr) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreateFromUri(
                uri.as_ptr().cast_mut(),
                uri.to_bytes().len(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Creates an image source from a file descriptor.
    pub fn create_from_fd(fd: i32) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSourceNative_CreateFromFd(fd, &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Creates an image source from in-memory data.
    pub fn create_from_data(data: &mut [u8]) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreateFromData(data.as_mut_ptr(), data.len(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Creates an image source from in-memory data using a user buffer.
    #[cfg(feature = "api-20")]
    pub fn create_from_data_with_user_buffer(data: &mut [u8]) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreateFromDataWithUserBuffer(
                data.as_mut_ptr(),
                data.len(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Creates an image source from a raw file descriptor.
    pub fn create_from_raw_file(raw_file: &mut RawFileDescriptor) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSourceNative_CreateFromRawFile(raw_file, &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Decodes a pixel-map from the image source.
    pub fn create_pixelmap(&self, options: &mut DecodingOptions) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreatePixelmap(self.as_raw(), options.as_raw(), &mut raw)
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(PixelMap::from_non_null)
    }

    /// Decodes a pixel-map with an allocator type.
    #[cfg(feature = "api-15")]
    pub fn create_pixelmap_using_allocator(
        &self,
        options: &mut DecodingOptions,
        allocator: crate::types::AllocatorType,
    ) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreatePixelmapUsingAllocator(
                self.as_raw(),
                options.as_raw(),
                allocator.into(),
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(PixelMap::from_non_null)
    }

    /// Decodes all pixel-maps from the image source.
    pub fn create_pixelmap_list(
        &self,
        options: &mut DecodingOptions,
    ) -> ImageNativeResult<Vec<PixelMap>> {
        let count = self.frame_count()? as usize;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut raws = vec![std::ptr::null_mut(); count];
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreatePixelmapList(
                self.as_raw(),
                options.as_raw(),
                raws.as_mut_ptr(),
                raws.len(),
            )
        })?;
        Ok(raws.into_iter().filter_map(PixelMap::from_raw).collect())
    }

    /// Decodes a picture from the image source.
    #[cfg(feature = "api-13")]
    pub fn create_picture(
        &self,
        options: &mut DecodingOptionsForPicture,
    ) -> ImageNativeResult<crate::picture::Picture> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreatePicture(self.as_raw(), options.as_raw(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_DECODE_FAILED)
            .map(|raw| crate::picture::Picture { raw })
    }

    /// Decodes a picture by index.
    #[cfg(feature = "api-20")]
    pub fn create_picture_at_index(
        &self,
        index: u32,
    ) -> ImageNativeResult<crate::picture::Picture> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSourceNative_CreatePictureAtIndex(self.as_raw(), index, &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_DECODE_FAILED)
            .map(|raw| crate::picture::Picture { raw })
    }

    /// Returns the frame delay list.
    pub fn delay_time_list(&self) -> ImageNativeResult<Vec<i32>> {
        let count = self.frame_count()? as usize;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut values = vec![0; count];
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetDelayTimeList(
                self.as_raw(),
                values.as_mut_ptr(),
                values.len(),
            )
        })?;
        Ok(values)
    }

    /// Returns image info for the given frame.
    pub fn image_info(&self, index: i32) -> ImageNativeResult<ImageSourceInfo> {
        let info = ImageSourceInfo::new()?;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImageInfo(self.as_raw(), index, info.as_raw())
        })?;
        Ok(info)
    }

    /// Returns an image property.
    pub fn get_image_property(&self, key: &mut ImageString) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe {
            sys::OH_ImageSourceNative_GetImageProperty(self.as_raw(), key.as_mut_ptr(), value)
        })
    }

    /// Returns an image property with trailing null.
    #[cfg(feature = "api-19")]
    pub fn get_image_property_with_null(
        &self,
        key: &mut ImageString,
    ) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyWithNull(
                self.as_raw(),
                key.as_mut_ptr(),
                value,
            )
        })
    }

    /// Modifies an image property.
    pub fn modify_image_property(
        &mut self,
        key: &mut ImageString,
        value: &mut ImageString,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImageProperty(
                self.as_raw(),
                key.as_mut_ptr(),
                value.as_mut_ptr(),
            )
        })
    }

    /// Returns the frame count.
    pub fn frame_count(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetFrameCount(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    /// Returns supported input formats.
    #[cfg(feature = "api-20")]
    pub fn supported_formats() -> ImageNativeResult<Vec<String>> {
        let mut raw = std::ptr::null_mut();
        let mut len = 0;
        check_status(unsafe { sys::OH_ImageSourceNative_GetSupportedFormats(&mut raw, &mut len) })?;
        Ok(collect_mime_types(raw, len))
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_short(&self, key: &mut ImageString) -> ImageNativeResult<u16> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyShort(
                self.as_raw(),
                key.as_mut_ptr(),
                &mut value,
            )
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_long(&self, key: &mut ImageString) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyLong(
                self.as_raw(),
                key.as_mut_ptr(),
                &mut value,
            )
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_double(&self, key: &mut ImageString) -> ImageNativeResult<f64> {
        let mut value = 0.0;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyDouble(
                self.as_raw(),
                key.as_mut_ptr(),
                &mut value,
            )
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-23")]
    pub fn image_property_array_size(&self, key: &mut ImageString) -> ImageNativeResult<usize> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyArraySize(
                self.as_raw(),
                key.as_mut_ptr(),
                &mut value,
            )
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_string(&self, key: &mut ImageString) -> ImageNativeResult<String> {
        let size = self.image_property_array_size(key)?;
        let mut value = vec![0u8; size];
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyString(
                self.as_raw(),
                key.as_mut_ptr(),
                value.as_mut_ptr().cast(),
                value.len(),
            )
        })?;
        Ok(String::from_utf8_lossy(&value).into_owned())
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_int_array(
        &self,
        key: &mut ImageString,
    ) -> ImageNativeResult<Vec<i32>> {
        let size = self.image_property_array_size(key)?;
        let mut values = vec![0; size];
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyIntArray(
                self.as_raw(),
                key.as_mut_ptr(),
                values.as_mut_ptr(),
                values.len(),
            )
        })?;
        Ok(values)
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_double_array(
        &self,
        key: &mut ImageString,
    ) -> ImageNativeResult<Vec<f64>> {
        let size = self.image_property_array_size(key)?;
        let mut values = vec![0.0; size];
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyDoubleArray(
                self.as_raw(),
                key.as_mut_ptr(),
                values.as_mut_ptr(),
                values.len(),
            )
        })?;
        Ok(values)
    }

    #[cfg(feature = "api-23")]
    pub fn get_image_property_blob(&self, key: &mut ImageString) -> ImageNativeResult<Vec<u8>> {
        let size = self.image_property_array_size(key)?;
        let mut value = vec![0u8; size];
        check_status(unsafe {
            sys::OH_ImageSourceNative_GetImagePropertyBlob(
                self.as_raw(),
                key.as_mut_ptr(),
                value.as_mut_ptr().cast(),
                value.len(),
            )
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_short(
        &mut self,
        key: &mut ImageString,
        value: u16,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyShort(
                self.as_raw(),
                key.as_mut_ptr(),
                value,
            )
        })
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_long(
        &mut self,
        key: &mut ImageString,
        value: u32,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyLong(
                self.as_raw(),
                key.as_mut_ptr(),
                value,
            )
        })
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_double(
        &mut self,
        key: &mut ImageString,
        value: f64,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyDouble(
                self.as_raw(),
                key.as_mut_ptr(),
                value,
            )
        })
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_int_array(
        &mut self,
        key: &mut ImageString,
        values: &[i32],
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyIntArray(
                self.as_raw(),
                key.as_mut_ptr(),
                values.as_ptr().cast_mut(),
                values.len(),
            )
        })
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_double_array(
        &mut self,
        key: &mut ImageString,
        values: &[f64],
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyDoubleArray(
                self.as_raw(),
                key.as_mut_ptr(),
                values.as_ptr().cast_mut(),
                values.len(),
            )
        })
    }

    #[cfg(feature = "api-23")]
    pub fn modify_image_property_blob(
        &mut self,
        key: &mut ImageString,
        values: &[u8],
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImageSourceNative_ModifyImagePropertyBlob(
                self.as_raw(),
                key.as_mut_ptr(),
                values.as_ptr().cast::<std::ffi::c_void>().cast_mut(),
                values.len(),
            )
        })
    }
}

impl Drop for ImageSource {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImageSourceNative_Release(self.as_raw()) });
    }
}
