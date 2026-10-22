//! Native GL texture ownership; EGL and GLES state belong to the caller.
use std::{marker::PhantomData, rc::Rc};

use ohos_native_image_sys::{
    OH_NativeImage_Create, OH_NativeImage_GetTransformMatrixV2, OH_NativeImage_UpdateSurfaceImage,
};

#[cfg(feature = "api-13")]
use crate::BufferUsage;
use crate::{check, ConsumerSurface, Error, FrameAvailableListener, NATIVE_ERROR_NO_BUFFER};

/// A NativeImage attached to a caller-owned external GL texture.
///
/// This owner is confined to its creating thread. It does not create an EGL
/// context, load GLES, convert pixels or select buffer budgets.
pub struct TextureImage {
    surface: ConsumerSurface,
    _thread: PhantomData<Rc<()>>,
}

impl TextureImage {
    /// Creates an image attached to the caller's texture (native API 9).
    ///
    /// # Safety
    /// The texture must belong to the current EGL context and remain valid until
    /// this image is destroyed. The same context must be current on this thread
    /// for every image update and when this owner is dropped. The caller must
    /// retain the producer and its window for any outstanding producer work.
    pub unsafe fn new(texture: u32, target: u32) -> Result<Self, Error> {
        // SAFETY: the caller supplies the current-context and texture lifetime contract.
        let raw = std::ptr::NonNull::new(unsafe { OH_NativeImage_Create(texture, target) }).ok_or(
            Error {
                operation: "create texture",
                code: -1,
            },
        )?;
        Ok(Self {
            surface: ConsumerSurface {
                raw,
                listening: false,
                _not_sync: PhantomData,
            },
            _thread: PhantomData,
        })
    }

    /// Returns the producer surface identity (native API 11).
    pub fn surface_id(&mut self) -> Result<u64, Error> {
        self.surface.surface_id()
    }

    /// Sets future producer-buffer dimensions (API 13+).
    #[cfg(feature = "api-13")]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        self.surface.resize(width, height)
    }

    /// Sets the exact future producer-buffer usage flags (API 13+).
    #[cfg(feature = "api-13")]
    pub fn set_usage(&mut self, usage: BufferUsage) -> Result<(), Error> {
        self.surface.set_usage(usage)
    }

    /// Updates the attached texture after a frame-available notification.
    /// The constructor's EGL context must be current.
    pub fn update(&mut self) -> Result<bool, Error> {
        // SAFETY: unique ownership serializes the call; new's caller retains the context.
        let status = unsafe { OH_NativeImage_UpdateSurfaceImage(self.surface.raw.as_ptr()) };
        if status == NATIVE_ERROR_NO_BUFFER {
            return Ok(false);
        }
        check("update texture", status)?;
        Ok(true)
    }

    /// Returns the SDK producer transform from the latest successful update (API 12).
    pub fn transform(&self) -> Result<[f32; 16], Error> {
        let mut matrix = [0.0; 16];
        // SAFETY: the live image and writable 4x4 matrix cover this synchronous call.
        check("texture transform", unsafe {
            OH_NativeImage_GetTransformMatrixV2(self.surface.raw.as_ptr(), matrix.as_mut_ptr())
        })?;
        Ok(matrix)
    }

    /// Registers a caller-owned native listener (API 11).
    ///
    /// # Safety
    /// The callback/context contract is identical to
    /// [`ConsumerSurface::set_frame_available_listener`]. Unregistration and Drop
    /// do not drain callbacks; never free a context that a late callback may use.
    pub unsafe fn set_frame_available_listener(
        &mut self,
        listener: FrameAvailableListener,
    ) -> Result<(), Error> {
        // SAFETY: forwarded caller contract retains and synchronizes the context.
        unsafe { self.surface.set_frame_available_listener(listener) }
    }

    /// Clears the listener without asserting native callback quiescence.
    pub fn unset_frame_available_listener(&mut self) -> Result<(), Error> {
        self.surface.unset_frame_available_listener()
    }
}
