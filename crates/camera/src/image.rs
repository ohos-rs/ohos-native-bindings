use std::ptr::NonNull;

use ohos_camera_sys as sys;
#[cfg(feature = "api-20")]
use ohos_image_native_binding::Image;
use ohos_image_native_binding::ImageRef;

#[cfg(feature = "api-20")]
use crate::CameraFrame;
use crate::{CameraError, CameraResult, CameraSize, CapturedPhoto};

const JPEG_COMPONENT: u32 = 4;
const MAX_CAPTURE_BYTES: usize = 128 * 1024 * 1024;
#[cfg(feature = "api-20")]
const MAX_FRAME_BYTES: usize = 32 * 1024 * 1024;

/// Copies callback-owned native image data into public owned camera values.
pub(crate) struct CameraImageReader;

impl CameraImageReader {
    pub(crate) fn copy_photo(
        photo: NativePhoto,
        photo_size: CameraSize,
    ) -> CameraResult<CapturedPhoto> {
        let mut raw_image = std::ptr::null_mut();
        // SAFETY: `photo` owns a live callback-transferred photo and the output
        // pointer is initialized.
        super::native::check(
            unsafe { sys::OH_PhotoNative_GetMainImage(photo.raw.as_ptr(), &mut raw_image) },
            "OH_PhotoNative_GetMainImage",
        )?;
        // SAFETY: `photo` keeps the main image alive through this copy.
        let image = unsafe { ImageRef::from_raw(raw_image.cast()) }.ok_or_else(|| {
            CameraError::invalid_state(
                "OH_PhotoNative_GetMainImage",
                "CameraKit returned a null main image",
            )
        })?;
        let timestamp_ns = image
            .timestamp()
            .map_err(|error| CameraError::image("OH_ImageNative_GetTimestamp", error.code))?;
        let components = image
            .component_types()
            .map_err(|error| CameraError::image("OH_ImageNative_GetComponentTypes", error.code))?;
        let component = components
            .iter()
            .copied()
            .find(|component| *component == JPEG_COMPONENT)
            .or_else(|| components.first().copied())
            .ok_or_else(|| {
                CameraError::invalid_state(
                    "OH_ImageNative_GetComponentTypes",
                    "captured image has no components",
                )
            })?;
        let buffer_size = image
            .buffer_size(component)
            .map_err(|error| CameraError::image("OH_ImageNative_GetBufferSize", error.code))?;
        Self::validate_buffer_size(buffer_size, MAX_CAPTURE_BYTES, "captured image")?;
        let mut buffer = image
            .byte_buffer(component)
            .map_err(|error| CameraError::image("OH_ImageNative_GetByteBuffer", error.code))?
            .ok_or_else(|| {
                CameraError::invalid_state(
                    "OH_ImageNative_GetByteBuffer",
                    "captured image has no native buffer",
                )
            })?;
        let mapped = buffer
            .map()
            .map_err(|error| CameraError::image("OH_NativeBuffer_Map", error.code() as u32))?;
        let mut bytes = mapped.bytes().to_vec();
        if bytes.starts_with(&[0xFF, 0xD8]) {
            if let Some(end) = bytes.windows(2).rposition(|pair| pair == [0xFF, 0xD9]) {
                bytes.truncate(end + 2);
            }
        }
        Ok(CapturedPhoto::new(bytes, photo_size, timestamp_ns))
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn copy_frame(image: Image, frame_size: CameraSize) -> CameraResult<CameraFrame> {
        let component = image
            .component_types()
            .map_err(|error| CameraError::image("OH_ImageNative_GetComponentTypes", error.code))?
            .into_iter()
            .next()
            .ok_or_else(|| {
                CameraError::invalid_state(
                    "OH_ImageNative_GetComponentTypes",
                    "analysis frame has no image component",
                )
            })?;
        let buffer_size = image
            .buffer_size(component)
            .map_err(|error| CameraError::image("OH_ImageNative_GetBufferSize", error.code))?;
        Self::validate_buffer_size(buffer_size, MAX_FRAME_BYTES, "analysis frame")?;
        let row_stride = image
            .row_stride(component)
            .map_err(|error| CameraError::image("OH_ImageNative_GetRowStride", error.code))?;
        let row_stride = usize::try_from(row_stride).map_err(|_| {
            CameraError::invalid_state("OH_ImageNative_GetRowStride", "negative row stride")
        })?;
        if row_stride == 0 {
            return Err(CameraError::invalid_state(
                "copy analysis frame",
                "row stride must be non-zero",
            ));
        }
        let width = frame_size.width as usize;
        let height = frame_size.height as usize;
        let required = height
            .saturating_sub(1)
            .saturating_mul(row_stride)
            .saturating_add(width);
        if required > buffer_size || width.saturating_mul(height) > MAX_FRAME_BYTES {
            return Err(CameraError::invalid_state(
                "copy analysis frame",
                "frame strides exceed the mapped buffer",
            ));
        }
        let mut buffer = image
            .byte_buffer(component)
            .map_err(|error| CameraError::image("OH_ImageNative_GetByteBuffer", error.code))?
            .ok_or_else(|| {
                CameraError::invalid_state(
                    "OH_ImageNative_GetByteBuffer",
                    "analysis frame has no native buffer",
                )
            })?;
        let mapped = buffer
            .map()
            .map_err(|error| CameraError::image("OH_NativeBuffer_Map", error.code() as u32))?;
        let source = mapped.bytes();
        let mut luma = Vec::with_capacity(width.saturating_mul(height));
        for y in 0..height {
            let row = y * row_stride;
            // NV21 exposes its byte-per-pixel Y plane in the first rows.
            luma.extend_from_slice(&source[row..row + width]);
        }
        let timestamp_ns = image
            .timestamp()
            .map_err(|error| CameraError::image("OH_ImageNative_GetTimestamp", error.code))?;
        Ok(CameraFrame::new(luma, frame_size, timestamp_ns))
    }

    fn validate_buffer_size(size: usize, max: usize, label: &'static str) -> CameraResult<()> {
        if size == 0 || size > max {
            Err(CameraError::invalid_state(
                "OH_ImageNative_GetBufferSize",
                format!("invalid {label} buffer size: {size}"),
            ))
        } else {
            Ok(())
        }
    }
}

pub(crate) struct NativePhoto {
    raw: NonNull<sys::OH_PhotoNative>,
}

impl NativePhoto {
    /// Take the unique photo owner transferred to a PhotoAvailable callback.
    ///
    /// # Safety
    ///
    /// `raw` must be the callback-transferred owner and must not be wrapped a
    /// second time.
    pub(crate) unsafe fn from_callback(raw: *mut sys::OH_PhotoNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }
}

impl Drop for NativePhoto {
    fn drop(&mut self) {
        // SAFETY: the callback transferred one unique photo owner.
        let _ = unsafe { sys::OH_PhotoNative_Release(self.raw.as_ptr()) };
    }
}
