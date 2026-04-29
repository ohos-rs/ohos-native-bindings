use std::ptr::NonNull;

#[cfg(any(feature = "api-13", feature = "api-22"))]
use std::ffi::CStr;

use napi_sys_ohos::{napi_env, napi_value};

use crate::{
    common::{
        maybe_non_null, napi_value_or_error, non_null, NativeBufferHandle, PixelMapNativeHandle,
    },
    error::{check_status, ImageNativeResult},
    sys,
    types::{
        HdrMetadataKey, ImageRegion, PixelFormat, PixelMapAntiAliasingLevel,
        PixelMapHdrMetadataValue,
    },
};

#[cfg(feature = "api-13")]
use crate::common::NativeColorSpaceManagerHandle;
#[cfg(feature = "api-20")]
use crate::types::AllocatorMode;
#[cfg(feature = "api-22")]
use crate::types::{ImagePositionArea, ImageScale};

#[cfg(feature = "api-22")]
const CREATE_PIXELMAP_ERROR: sys::Image_ErrorCode =
    sys::Image_ErrorCode_IMAGE_CREATE_PIXELMAP_FAILED;
#[cfg(not(feature = "api-22"))]
const CREATE_PIXELMAP_ERROR: sys::Image_ErrorCode = sys::Image_ErrorCode_IMAGE_ALLOC_FAILED;

/// Pixel-map initialization options.
pub struct PixelMapInitializationOptions {
    raw: NonNull<sys::OH_Pixelmap_InitializationOptions>,
}

impl PixelMapInitializationOptions {
    /// Creates a new options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelmapInitializationOptions_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_Pixelmap_InitializationOptions {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetWidth(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_width(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetWidth(self.as_raw(), value)
        })
    }

    pub fn height(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetHeight(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_height(&mut self, value: u32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetHeight(self.as_raw(), value)
        })
    }

    pub fn pixel_format(&self) -> ImageNativeResult<PixelFormat> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetPixelFormat(self.as_raw(), &mut value)
        })?;
        Ok(PixelFormat::from_i32_or_unknown(value))
    }

    pub fn set_pixel_format(&mut self, value: PixelFormat) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetPixelFormat(self.as_raw(), value.into())
        })
    }

    pub fn src_pixel_format(&self) -> ImageNativeResult<PixelFormat> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetSrcPixelFormat(self.as_raw(), &mut value)
        })?;
        Ok(PixelFormat::from_i32_or_unknown(value))
    }

    pub fn set_src_pixel_format(&mut self, value: PixelFormat) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetSrcPixelFormat(self.as_raw(), value.into())
        })
    }

    pub fn row_stride(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetRowStride(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_row_stride(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetRowStride(self.as_raw(), value)
        })
    }

    pub fn alpha_type(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetAlphaType(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_alpha_type(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetAlphaType(self.as_raw(), value)
        })
    }

    #[cfg(feature = "api-18")]
    pub fn editable(&self) -> ImageNativeResult<bool> {
        let mut value = false;
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_GetEditable(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    #[cfg(feature = "api-18")]
    pub fn set_editable(&mut self, value: bool) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapInitializationOptions_SetEditable(self.as_raw(), value)
        })
    }
}

impl Drop for PixelMapInitializationOptions {
    fn drop(&mut self) {
        let _ =
            check_status(unsafe { sys::OH_PixelmapInitializationOptions_Release(self.as_raw()) });
    }
}

/// Pixel-map info wrapper.
pub struct PixelMapImageInfo {
    raw: NonNull<sys::OH_Pixelmap_ImageInfo>,
}

impl PixelMapImageInfo {
    /// Creates a new pixel-map info object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelmapImageInfo_Create(&mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_Pixelmap_ImageInfo {
        self.raw.as_ptr()
    }

    pub fn width(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapImageInfo_GetWidth(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn height(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapImageInfo_GetHeight(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    #[cfg(feature = "api-20")]
    pub fn alpha_mode(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapImageInfo_GetAlphaMode(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn row_stride(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapImageInfo_GetRowStride(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn pixel_format(&self) -> ImageNativeResult<PixelFormat> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapImageInfo_GetPixelFormat(self.as_raw(), &mut value)
        })?;
        Ok(PixelFormat::from_i32_or_unknown(value))
    }

    pub fn alpha_type(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapImageInfo_GetAlphaType(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn dynamic_range(&self) -> ImageNativeResult<bool> {
        let mut value = false;
        check_status(unsafe {
            sys::OH_PixelmapImageInfo_GetDynamicRange(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }
}

impl Drop for PixelMapImageInfo {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PixelmapImageInfo_Release(self.as_raw()) });
    }
}

/// Owned native pixel-map wrapper.
pub struct PixelMap {
    raw: NonNull<sys::OH_PixelmapNative>,
}

impl PixelMap {
    /// Creates a wrapper from a raw pointer.
    pub fn from_raw(raw: *mut sys::OH_PixelmapNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn from_non_null(raw: NonNull<sys::OH_PixelmapNative>) -> Self {
        Self { raw }
    }

    /// Returns the wrapped raw pointer.
    pub fn as_raw(&self) -> *mut sys::OH_PixelmapNative {
        self.raw.as_ptr()
    }

    /// Returns a borrowed pixel-map handle.
    pub fn handle(&self) -> PixelMapNativeHandle {
        PixelMapNativeHandle::from_erased_raw(self.raw.as_ptr().cast())
            .expect("native pixel-map pointer should not be null")
    }

    /// Creates a pixel-map from BGRA data.
    pub fn create(
        data: &mut [u8],
        options: &mut PixelMapInitializationOptions,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreatePixelmap(
                data.as_mut_ptr(),
                data.len(),
                options.as_raw(),
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Creates a pixel-map with an explicit allocator mode.
    #[cfg(feature = "api-20")]
    pub fn create_using_allocator(
        data: &mut [u8],
        options: &mut PixelMapInitializationOptions,
        allocator: AllocatorMode,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreatePixelmapUsingAllocator(
                data.as_mut_ptr(),
                data.len(),
                options.as_raw(),
                allocator.into(),
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Parses a native pixel-map from a N-API `PixelMap` object.
    pub fn from_napi(env: napi_env, pixel_map: napi_value) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_ConvertPixelmapNativeFromNapi(env, pixel_map, &mut raw)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER).map(|raw| Self { raw })
    }

    /// Converts the pixel-map into a N-API `PixelMap`.
    pub fn to_napi(&self, env: napi_env) -> ImageNativeResult<napi_value> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_ConvertPixelmapNativeToNapi(env, self.as_raw(), &mut value)
        })?;
        napi_value_or_error(value)
    }

    /// Reads all pixels into the provided buffer.
    pub fn read_pixels(&self, destination: &mut [u8]) -> ImageNativeResult<usize> {
        let mut size = destination.len();
        check_status(unsafe {
            sys::OH_PixelmapNative_ReadPixels(self.as_raw(), destination.as_mut_ptr(), &mut size)
        })?;
        Ok(size)
    }

    /// Writes pixels from the provided buffer.
    pub fn write_pixels(&mut self, source: &[u8]) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_WritePixels(
                self.as_raw(),
                source.as_ptr().cast_mut(),
                source.len(),
            )
        })
    }

    /// Reads a region into the supplied area descriptor.
    #[cfg(feature = "api-22")]
    pub fn read_pixels_from_area(&self, area: &mut ImagePositionArea) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_ReadPixelsFromArea(self.as_raw(), area) })
    }

    /// Writes pixels from the supplied area descriptor.
    #[cfg(feature = "api-22")]
    pub fn write_pixels_to_area(&mut self, area: &mut ImagePositionArea) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_WritePixelsToArea(self.as_raw(), area) })
    }

    /// Reads pixels as ARGB data.
    #[cfg(feature = "api-13")]
    pub fn argb_pixels(&self, destination: &mut [u8]) -> ImageNativeResult<usize> {
        let mut size = destination.len();
        check_status(unsafe {
            sys::OH_PixelmapNative_GetArgbPixels(self.as_raw(), destination.as_mut_ptr(), &mut size)
        })?;
        Ok(size)
    }

    /// Converts the pixel-map to SDR.
    pub fn to_sdr(&mut self) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_ToSdr(self.as_raw()) })
    }

    /// Obtains pixel-map info.
    pub fn image_info(&self) -> ImageNativeResult<PixelMapImageInfo> {
        let info = PixelMapImageInfo::new()?;
        check_status(unsafe { sys::OH_PixelmapNative_GetImageInfo(self.as_raw(), info.as_raw()) })?;
        Ok(info)
    }

    /// Sets opacity.
    pub fn set_opacity(&mut self, rate: f32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Opacity(self.as_raw(), rate) })
    }

    /// Scales the pixel-map.
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Scale(self.as_raw(), scale_x, scale_y) })
    }

    /// Scales the pixel-map with anti-aliasing.
    pub fn scale_with_anti_aliasing(
        &mut self,
        scale_x: f32,
        scale_y: f32,
        level: PixelMapAntiAliasingLevel,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_ScaleWithAntiAliasing(
                self.as_raw(),
                scale_x,
                scale_y,
                level.into(),
            )
        })
    }

    /// Creates a scaled copy.
    #[cfg(feature = "api-18")]
    pub fn create_scaled(&self, scale_x: f32, scale_y: f32) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateScaledPixelMap(self.as_raw(), &mut raw, scale_x, scale_y)
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_ALLOC_FAILED).map(|raw| Self { raw })
    }

    /// Creates a scaled copy with anti-aliasing.
    #[cfg(feature = "api-18")]
    pub fn create_scaled_with_anti_aliasing(
        &self,
        scale_x: f32,
        scale_y: f32,
        level: PixelMapAntiAliasingLevel,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateScaledPixelMapWithAntiAliasing(
                self.as_raw(),
                &mut raw,
                scale_x,
                scale_y,
                level.into(),
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_ALLOC_FAILED).map(|raw| Self { raw })
    }

    /// Translates the pixel-map.
    pub fn translate(&mut self, x: f32, y: f32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Translate(self.as_raw(), x, y) })
    }

    /// Creates an alpha-only copy.
    #[cfg(feature = "api-22")]
    pub fn create_alpha_pixelmap(&self) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateAlphaPixelmap(self.as_raw(), &mut raw)
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Clones the pixel-map.
    #[cfg(feature = "api-22")]
    pub fn clone_pixelmap(&self) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelmapNative_Clone(self.as_raw(), &mut raw) })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_ALLOC_FAILED).map(|raw| Self { raw })
    }

    /// Creates a cropped-and-scaled copy.
    #[cfg(feature = "api-22")]
    pub fn create_cropped_and_scaled(
        &self,
        region: &mut ImageRegion,
        scale: &mut ImageScale,
        level: PixelMapAntiAliasingLevel,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateCroppedAndScaledPixelMap(
                self.as_raw(),
                region,
                scale,
                level.into(),
                &mut raw,
            )
        })?;
        non_null(raw, sys::Image_ErrorCode_IMAGE_ALLOC_FAILED).map(|raw| Self { raw })
    }

    /// Rotates the pixel-map.
    pub fn rotate(&mut self, angle: f32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Rotate(self.as_raw(), angle) })
    }

    /// Flips the pixel-map.
    pub fn flip(&mut self, horizontal: bool, vertical: bool) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Flip(self.as_raw(), horizontal, vertical) })
    }

    /// Crops the pixel-map.
    pub fn crop(&mut self, region: &mut ImageRegion) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_Crop(self.as_raw(), region) })
    }

    /// Converts alpha format into another pixel-map.
    pub fn convert_alpha_format(
        &self,
        destination: &mut PixelMap,
        is_premul: bool,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_ConvertAlphaFormat(
                self.as_raw(),
                destination.as_raw(),
                is_premul,
            )
        })
    }

    /// Creates an empty pixel-map.
    pub fn create_empty(options: &mut PixelMapInitializationOptions) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateEmptyPixelmap(options.as_raw(), &mut raw)
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Creates an empty pixel-map with allocator mode.
    #[cfg(feature = "api-20")]
    pub fn create_empty_using_allocator(
        options: &mut PixelMapInitializationOptions,
        allocator: AllocatorMode,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreateEmptyPixelmapUsingAllocator(
                options.as_raw(),
                allocator.into(),
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Creates a pixel-map from a surface identifier.
    #[cfg(feature = "api-22")]
    pub fn create_from_surface(surface_id: &CStr) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreatePixelmapFromSurface(
                surface_id.as_ptr(),
                surface_id.to_bytes().len(),
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Creates a pixel-map from a surface identifier with optional inverse transformation.
    #[cfg(feature = "api-23")]
    pub fn create_from_surface_with_transformation(
        surface_id: &CStr,
        transform_enabled: bool,
    ) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreatePixelmapFromSurfaceWithTransformation(
                surface_id.as_ptr(),
                surface_id.to_bytes().len(),
                transform_enabled,
                &mut raw,
            )
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Creates a pixel-map from a native buffer.
    #[cfg(feature = "api-22")]
    pub fn create_from_native_buffer(buffer: NativeBufferHandle) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_CreatePixelmapFromNativeBuffer(buffer.as_raw(), &mut raw)
        })?;
        non_null(raw, CREATE_PIXELMAP_ERROR).map(Self::from_non_null)
    }

    /// Returns HDR metadata.
    pub fn metadata(
        &self,
        key: HdrMetadataKey,
    ) -> ImageNativeResult<Option<PixelMapHdrMetadataValue>> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_GetMetadata(self.as_raw(), key.into(), &mut value)
        })?;
        Ok(maybe_non_null(value).map(|value| unsafe { *value.as_ptr() }))
    }

    /// Sets HDR metadata.
    pub fn set_metadata(
        &mut self,
        key: HdrMetadataKey,
        value: &mut PixelMapHdrMetadataValue,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_SetMetadata(self.as_raw(), key.into(), value)
        })
    }

    /// Returns the backing native buffer.
    pub fn native_buffer(&self) -> ImageNativeResult<Option<NativeBufferHandle>> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelmapNative_GetNativeBuffer(self.as_raw(), &mut value) })?;
        Ok(NativeBufferHandle::from_raw(value))
    }

    /// Returns the native color space.
    #[cfg(feature = "api-13")]
    pub fn color_space(&self) -> ImageNativeResult<Option<NativeColorSpaceManagerHandle>> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelmapNative_GetColorSpaceNative(self.as_raw(), &mut value)
        })?;
        Ok(NativeColorSpaceManagerHandle::from_raw(value))
    }

    /// Sets the native color space.
    #[cfg(feature = "api-13")]
    pub fn set_color_space(
        &mut self,
        value: NativeColorSpaceManagerHandle,
    ) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_SetColorSpaceNative(self.as_raw(), value.as_raw())
        })
    }

    /// Sets the memory name.
    #[cfg(feature = "api-13")]
    pub fn set_memory_name(&mut self, name: &CStr, size: &mut usize) -> ImageNativeResult<()> {
        check_status(unsafe {
            sys::OH_PixelmapNative_SetMemoryName(self.as_raw(), name.as_ptr().cast_mut(), size)
        })
    }

    /// Returns byte count.
    #[cfg(feature = "api-18")]
    pub fn byte_count(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapNative_GetByteCount(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    /// Returns allocation byte count.
    #[cfg(feature = "api-18")]
    pub fn allocation_byte_count(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_PixelmapNative_GetAllocationByteCount(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    /// Locks pixel memory and returns the mapped address.
    #[cfg(feature = "api-15")]
    pub fn access_pixels(&self) -> ImageNativeResult<NonNull<std::ffi::c_void>> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelmapNative_AccessPixels(self.as_raw(), &mut value) })?;
        non_null(value, sys::Image_ErrorCode_IMAGE_BAD_PARAMETER)
    }

    /// Unlocks pixel memory.
    #[cfg(feature = "api-15")]
    pub fn unaccess_pixels(&self) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_PixelmapNative_UnaccessPixels(self.as_raw()) })
    }

    /// Returns the pixel-map unique ID.
    #[cfg(feature = "api-22")]
    pub fn unique_id(&self) -> ImageNativeResult<u32> {
        let mut value = 0;
        check_status(unsafe { sys::OH_PixelmapNative_GetUniqueId(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    /// Returns whether the pixel-map has been released.
    #[cfg(feature = "api-22")]
    pub fn is_released(&self) -> ImageNativeResult<bool> {
        let mut value = false;
        check_status(unsafe { sys::OH_PixelmapNative_IsReleased(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    /// Explicitly releases the pixel-map.
    pub fn release(self) -> ImageNativeResult<()> {
        let raw = self.into_raw();
        check_status(unsafe { sys::OH_PixelmapNative_Release(raw) })
    }

    /// Explicitly destroys the pixel-map.
    #[cfg(feature = "api-18")]
    pub fn destroy(self) -> ImageNativeResult<()> {
        let mut raw = self.into_raw();
        check_status(unsafe { sys::OH_PixelmapNative_Destroy(&mut raw) })
    }

    fn into_raw(self) -> *mut sys::OH_PixelmapNative {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for PixelMap {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_PixelmapNative_Release(self.as_raw()) });
    }
}
