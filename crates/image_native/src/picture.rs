#[cfg(any(feature = "api-13", feature = "api-23"))]
use std::ptr::NonNull;

#[cfg(feature = "api-13")]
use crate::{
    common::{non_null, output_string, ImageString},
    error::{check_status, ImageNativeResult},
    pixel_map::PixelMap,
    sys,
    types::{ImageSize, MetadataType, PixelFormat},
};

#[cfg(all(feature = "api-23", not(feature = "api-13")))]
use crate::{
    common::non_null,
    error::{check_status, ImageNativeResult},
    sys,
    types::PixelFormat,
};

/// Picture metadata.
#[cfg(feature = "api-13")]
pub struct PictureMetadata {
    raw: NonNull<sys::OH_PictureMetadata>,
}

#[cfg(feature = "api-13")]
impl PictureMetadata {
    /// Creates a new picture metadata object.
    pub fn new(metadata_type: MetadataType) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PictureMetadata_Create(metadata_type.into(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_PictureMetadata {
        self.raw.as_ptr()
    }

    pub fn get_property(&self, key: &mut ImageString) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe {
            sys::OH_PictureMetadata_GetProperty(self.as_raw(), key.as_mut_ptr(), value)
        })
    }

    pub fn set_property(
        &mut self,
        key: &mut ImageString,
        value: &mut ImageString,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PictureMetadata_SetProperty(self.as_raw(), key.as_mut_ptr(), value.as_mut_ptr())
        })
    }

    #[cfg(feature = "api-19")]
    pub fn get_property_with_null(&self, key: &mut ImageString) -> ImageNativeResult<ImageString> {
        output_string(|value| unsafe {
            sys::OH_PictureMetadata_GetPropertyWithNull(self.as_raw(), key.as_mut_ptr(), value)
        })
    }

    pub fn clone_metadata(&self) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PictureMetadata_Clone(self.as_raw(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }
}

#[cfg(feature = "api-13")]
impl Drop for PictureMetadata {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PictureMetadata_Release(self.as_raw()) });
    }
}

/// Compose options for HDR composition.
#[cfg(feature = "api-23")]
pub struct ComposeOptions {
    raw: NonNull<sys::OH_ComposeOptions>,
}

#[cfg(feature = "api-23")]
impl ComposeOptions {
    /// Creates compose options.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ComposeOptions_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ComposeOptions {
        self.raw.as_ptr()
    }

    pub fn set_desired_pixel_format(&mut self, value: PixelFormat) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_ComposeOptions_SetDesiredPixelFormat(self.as_raw(), value.into())
        })
    }

    pub fn desired_pixel_format(&self) -> ImageNativeResult<crate::types::PixelFormat> {
        let mut value = sys::PIXEL_FORMAT_PIXEL_FORMAT_UNKNOWN;
        check_status(unsafe {
            sys::OH_ComposeOptions_GetDesiredPixelFormat(self.as_raw(), &mut value)
        })?;
        Ok(
            crate::types::PixelFormat::from_raw(value)
                .unwrap_or(crate::types::PixelFormat::Unknown),
        )
    }
}

#[cfg(feature = "api-23")]
impl Drop for ComposeOptions {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ComposeOptions_Release(self.as_raw()) });
    }
}

/// Picture wrapper.
#[cfg(feature = "api-13")]
pub struct Picture {
    pub(crate) raw: NonNull<sys::OH_PictureNative>,
}

#[cfg(feature = "api-13")]
impl Picture {
    /// Creates a picture from a main pixel-map.
    pub fn new(main_pixelmap: &PixelMap) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PictureNative_CreatePicture(main_pixelmap.as_raw(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_PictureNative {
        self.raw.as_ptr()
    }

    pub fn main_pixelmap(&self) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PictureNative_GetMainPixelmap(self.as_raw(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(PixelMap::from_non_null)
    }

    pub fn hdr_composed_pixelmap(&self) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PictureNative_GetHdrComposedPixelmap(self.as_raw(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(PixelMap::from_non_null)
    }

    #[cfg(feature = "api-23")]
    pub fn hdr_composed_pixelmap_with_options(
        &self,
        options: &mut ComposeOptions,
    ) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PictureNative_GetHdrComposedPixelmapWithOptions(
                self.as_raw(),
                options.as_raw(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(PixelMap::from_non_null)
    }

    pub fn gainmap_pixelmap(&self) -> ImageNativeResult<PixelMap> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PictureNative_GetGainmapPixelmap(self.as_raw(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(PixelMap::from_non_null)
    }

    pub fn set_auxiliary_picture(
        &mut self,
        type_: crate::types::AuxiliaryPictureType,
        picture: &mut AuxiliaryPicture,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PictureNative_SetAuxiliaryPicture(self.as_raw(), type_.into(), picture.as_raw())
        })
    }

    pub fn auxiliary_picture(
        &self,
        type_: crate::types::AuxiliaryPictureType,
    ) -> ImageNativeResult<AuxiliaryPicture> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PictureNative_GetAuxiliaryPicture(self.as_raw(), type_.into(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| AuxiliaryPicture { raw })
    }

    pub fn metadata(&self, metadata_type: MetadataType) -> ImageNativeResult<PictureMetadata> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PictureNative_GetMetadata(self.as_raw(), metadata_type.into(), &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| PictureMetadata { raw })
    }

    pub fn set_metadata(
        &mut self,
        metadata_type: MetadataType,
        metadata: &mut PictureMetadata,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PictureNative_SetMetadata(
                self.as_raw(),
                metadata_type.into(),
                metadata.as_raw(),
            )
        })
    }
}

#[cfg(feature = "api-13")]
impl Drop for Picture {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PictureNative_Release(self.as_raw()) });
    }
}

/// Auxiliary picture wrapper.
#[cfg(feature = "api-13")]
pub struct AuxiliaryPicture {
    raw: NonNull<sys::OH_AuxiliaryPictureNative>,
}

#[cfg(feature = "api-13")]
impl AuxiliaryPicture {
    /// Creates an auxiliary picture.
    pub fn new(
        data: &mut [u8],
        size: &mut ImageSize,
        type_: crate::types::AuxiliaryPictureType,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_Create(
                data.as_mut_ptr(),
                data.len(),
                size,
                type_.into(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_AuxiliaryPictureNative {
        self.raw.as_ptr()
    }

    pub fn write_pixels(&mut self, source: &[u8]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_WritePixels(
                self.as_raw(),
                source.as_ptr().cast_mut(),
                source.len(),
            )
        })
    }

    pub fn read_pixels(&self, destination: &mut [u8]) -> ImageNativeResult<usize> {
        let mut size = destination.len();
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_ReadPixels(
                self.as_raw(),
                destination.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    pub fn type_(&self) -> ImageNativeResult<crate::types::AuxiliaryPictureType> {
        let mut value = 0;
        check_status(unsafe { sys::OH_AuxiliaryPictureNative_GetType(self.as_raw(), &mut value) })?;
        Ok(crate::types::AuxiliaryPictureType::from_raw(value)
            .unwrap_or(crate::types::AuxiliaryPictureType::Gainmap))
    }

    pub fn info(&self) -> ImageNativeResult<AuxiliaryPictureInfo> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_AuxiliaryPictureNative_GetInfo(self.as_raw(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER)
            .map(|raw| AuxiliaryPictureInfo { raw })
    }

    pub fn set_info(&mut self, info: &mut AuxiliaryPictureInfo) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_SetInfo(self.as_raw(), info.as_raw())
        })
    }

    pub fn metadata(&self, metadata_type: MetadataType) -> ImageNativeResult<PictureMetadata> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_GetMetadata(
                self.as_raw(),
                metadata_type.into(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| PictureMetadata { raw })
    }

    pub fn set_metadata(
        &mut self,
        metadata_type: MetadataType,
        metadata: &mut PictureMetadata,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_AuxiliaryPictureNative_SetMetadata(
                self.as_raw(),
                metadata_type.into(),
                metadata.as_raw(),
            )
        })
    }
}

#[cfg(feature = "api-13")]
impl Drop for AuxiliaryPicture {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_AuxiliaryPictureNative_Release(self.as_raw()) });
    }
}

/// Auxiliary picture info.
#[cfg(feature = "api-13")]
pub struct AuxiliaryPictureInfo {
    raw: NonNull<sys::OH_AuxiliaryPictureInfo>,
}

#[cfg(feature = "api-13")]
impl AuxiliaryPictureInfo {
    /// Creates auxiliary picture info.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_AuxiliaryPictureInfo {
        self.raw.as_ptr()
    }

    pub fn type_(&self) -> ImageNativeResult<crate::types::AuxiliaryPictureType> {
        let mut value = 0;
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_GetType(self.as_raw(), &mut value) })?;
        Ok(crate::types::AuxiliaryPictureType::from_raw(value)
            .unwrap_or(crate::types::AuxiliaryPictureType::Gainmap))
    }

    pub fn set_type(&mut self, value: crate::types::AuxiliaryPictureType) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_SetType(self.as_raw(), value.into()) })
    }

    pub fn size(&self) -> ImageNativeResult<ImageSize> {
        let mut value = ImageSize {
            width: 0,
            height: 0,
        };
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_GetSize(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_size(&mut self, value: &mut ImageSize) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_SetSize(self.as_raw(), value) })
    }

    pub fn row_stride(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_AuxiliaryPictureInfo_GetRowStride(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_row_stride(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_AuxiliaryPictureInfo_SetRowStride(self.as_raw(), value) })
    }

    pub fn pixel_format(&self) -> ImageNativeResult<PixelFormat> {
        let mut value = sys::PIXEL_FORMAT_PIXEL_FORMAT_UNKNOWN;
        check_status(unsafe {
            sys::OH_AuxiliaryPictureInfo_GetPixelFormat(self.as_raw(), &mut value)
        })?;
        Ok(PixelFormat::from_raw(value).unwrap_or(PixelFormat::Unknown))
    }

    pub fn set_pixel_format(&mut self, value: PixelFormat) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_AuxiliaryPictureInfo_SetPixelFormat(self.as_raw(), value.into())
        })
    }
}

#[cfg(feature = "api-13")]
impl Drop for AuxiliaryPictureInfo {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_AuxiliaryPictureInfo_Release(self.as_raw()) });
    }
}
