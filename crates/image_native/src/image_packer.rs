use std::ptr::NonNull;

use crate::{
    common::{non_null, output_string, ImageString},
    error::{check_status, ImageNativeResult},
    pixel_map::PixelMap,
    sys,
};

#[cfg(feature = "api-20")]
use crate::common::collect_mime_types;

/// Packing options.
pub struct PackingOptions {
    raw: NonNull<sys::OH_PackingOptions>,
}

impl PackingOptions {
    /// Creates a new packing options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PackingOptions_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_PackingOptions {
        self.raw.as_ptr()
    }

    pub fn mime_type(&self) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe { sys::OH_PackingOptions_GetMimeType(self.as_raw(), value) })
    }

    #[cfg(feature = "api-19")]
    pub fn mime_type_with_null(&self) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe {
            sys::OH_PackingOptions_GetMimeTypeWithNull(self.as_raw(), value)
        })
    }

    pub fn set_mime_type(&mut self, value: &mut ImageString) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptions_SetMimeType(self.as_raw(), value.as_mut_ptr())
        })
    }

    pub fn quality(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PackingOptions_GetQuality(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_quality(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PackingOptions_SetQuality(self.as_raw(), value) })
    }

    pub fn needs_pack_properties(&self) -> ImageNativeResult<bool> {
        let mut value = false;
        check_status(unsafe {
            sys::OH_PackingOptions_GetNeedsPackProperties(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_needs_pack_properties(&mut self, value: bool) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PackingOptions_SetNeedsPackProperties(self.as_raw(), value) })
    }

    pub fn desired_dynamic_range(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PackingOptions_GetDesiredDynamicRange(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_desired_dynamic_range(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PackingOptions_SetDesiredDynamicRange(self.as_raw(), value) })
    }
}

impl Drop for PackingOptions {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PackingOptions_Release(self.as_raw()) });
    }
}

/// Packing options for image sequences.
#[cfg(feature = "api-18")]
pub struct PackingOptionsForSequence {
    raw: NonNull<sys::OH_PackingOptionsForSequence>,
}

#[cfg(feature = "api-18")]
impl PackingOptionsForSequence {
    /// Creates a new sequence-packing options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PackingOptionsForSequence_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_PackingOptionsForSequence {
        self.raw.as_ptr()
    }

    pub fn set_frame_count(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_SetFrameCount(self.as_raw(), value)
        })
    }

    pub fn frame_count(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_GetFrameCount(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_delay_time_list(&mut self, values: &mut [i32]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_SetDelayTimeList(
                self.as_raw(),
                values.as_mut_ptr(),
                values.len(),
            )
        })
    }

    pub fn delay_time_list(&self, values: &mut [i32]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_GetDelayTimeList(
                self.as_raw(),
                values.as_mut_ptr(),
                values.len(),
            )
        })
    }

    pub fn set_disposal_types(&mut self, values: &mut [u32]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_SetDisposalTypes(
                self.as_raw(),
                values.as_mut_ptr(),
                values.len(),
            )
        })
    }

    pub fn disposal_types(&self, values: &mut [u32]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_GetDisposalTypes(
                self.as_raw(),
                values.as_mut_ptr(),
                values.len(),
            )
        })
    }

    pub fn set_loop_count(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_SetLoopCount(self.as_raw(), value)
        })
    }

    pub fn loop_count(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PackingOptionsForSequence_GetLoopCount(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }
}

#[cfg(feature = "api-18")]
impl Drop for PackingOptionsForSequence {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PackingOptionsForSequence_Release(self.as_raw()) });
    }
}

/// Native image packer.
pub struct ImagePacker {
    raw: NonNull<sys::OH_ImagePackerNative>,
}

impl ImagePacker {
    /// Creates a new image packer.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImagePackerNative_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ImagePackerNative {
        self.raw.as_ptr()
    }

    pub fn pack_to_data_from_image_source(
        &mut self,
        options: &mut PackingOptions,
        image_source: &crate::image_source::ImageSource,
        out: &mut [u8],
    ) -> ImageNativeResult<usize> {
        let mut size = out.len();
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToDataFromImageSource(
                self.as_raw(),
                options.as_raw(),
                image_source.as_raw(),
                out.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    pub fn pack_to_data_from_pixelmap(
        &mut self,
        options: &mut PackingOptions,
        pixelmap: &PixelMap,
        out: &mut [u8],
    ) -> ImageNativeResult<usize> {
        let mut size = out.len();
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToDataFromPixelmap(
                self.as_raw(),
                options.as_raw(),
                pixelmap.as_raw(),
                out.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    #[cfg(feature = "api-13")]
    pub fn pack_to_data_from_picture(
        &mut self,
        options: &mut PackingOptions,
        picture: &crate::picture::Picture,
        out: &mut [u8],
    ) -> ImageNativeResult<usize> {
        let mut size = out.len();
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToDataFromPicture(
                self.as_raw(),
                options.as_raw(),
                picture.as_raw(),
                out.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    #[cfg(feature = "api-18")]
    pub fn pack_to_data_from_pixelmap_sequence(
        &mut self,
        options: &mut PackingOptionsForSequence,
        pixelmaps: &mut [*mut sys::OH_PixelmapNative],
        out: &mut [u8],
    ) -> ImageNativeResult<usize> {
        let mut size = out.len();
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToDataFromPixelmapSequence(
                self.as_raw(),
                options.as_raw(),
                pixelmaps.as_mut_ptr(),
                pixelmaps.len(),
                out.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    pub fn pack_to_file_from_image_source(
        &mut self,
        options: &mut PackingOptions,
        image_source: &crate::image_source::ImageSource,
        fd: i32,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToFileFromImageSource(
                self.as_raw(),
                options.as_raw(),
                image_source.as_raw(),
                fd,
            )
        })
    }

    pub fn pack_to_file_from_pixelmap(
        &mut self,
        options: &mut PackingOptions,
        pixelmap: &PixelMap,
        fd: i32,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToFileFromPixelmap(
                self.as_raw(),
                options.as_raw(),
                pixelmap.as_raw(),
                fd,
            )
        })
    }

    #[cfg(feature = "api-13")]
    pub fn pack_to_file_from_picture(
        &mut self,
        options: &mut PackingOptions,
        picture: &crate::picture::Picture,
        fd: i32,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToFileFromPicture(
                self.as_raw(),
                options.as_raw(),
                picture.as_raw(),
                fd,
            )
        })
    }

    #[cfg(feature = "api-18")]
    pub fn pack_to_file_from_pixelmap_sequence(
        &mut self,
        options: &mut PackingOptionsForSequence,
        pixelmaps: &mut [*mut sys::OH_PixelmapNative],
        fd: i32,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ImagePackerNative_PackToFileFromPixelmapSequence(
                self.as_raw(),
                options.as_raw(),
                pixelmaps.as_mut_ptr(),
                pixelmaps.len(),
                fd,
            )
        })
    }

    #[cfg(feature = "api-20")]
    pub fn supported_formats() -> ImageNativeResult<Vec<String>> {
        let mut raw = std::ptr::null_mut();
        let mut len = 0;
        check_status(unsafe { sys::OH_ImagePackerNative_GetSupportedFormats(&mut raw, &mut len) })?;
        Ok(collect_mime_types(raw, len))
    }
}

impl Drop for ImagePacker {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImagePackerNative_Release(self.as_raw()) });
    }
}
