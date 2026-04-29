use std::{
    collections::HashMap,
    ptr::NonNull,
    sync::{Mutex, OnceLock},
};

use crate::{
    error::{check_status, ImageNativeResult},
    image::Image,
    sys,
    types::ImageSize,
};

#[cfg(feature = "api-20")]
use std::os::raw::c_void;

struct ReceiverCallbackContext {
    callback: Box<dyn FnMut() + Send>,
}

fn on_callback_registry() -> &'static Mutex<HashMap<usize, usize>> {
    static REGISTRY: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Image receiver options.
pub struct ImageReceiverOptions {
    raw: NonNull<sys::OH_ImageReceiverOptions>,
}

impl ImageReceiverOptions {
    /// Creates a new image-receiver options object.
    pub fn new() -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageReceiverOptions_Create(&mut raw) })?;
        NonNull::new(raw)
            .ok_or(crate::ImageNativeError {
                code: sys::Image_ErrorCode_IMAGE_BAD_PARAMETER,
            })
            .map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ImageReceiverOptions {
        self.raw.as_ptr()
    }

    pub fn size(&self) -> ImageNativeResult<ImageSize> {
        let mut value = ImageSize {
            width: 0,
            height: 0,
        };
        check_status(unsafe { sys::OH_ImageReceiverOptions_GetSize(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn set_size(&mut self, value: ImageSize) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_ImageReceiverOptions_SetSize(self.as_raw(), value) })
    }

    pub fn capacity(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageReceiverOptions_GetCapacity(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn set_capacity(&mut self, value: i32) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_ImageReceiverOptions_SetCapacity(self.as_raw(), value) })
    }
}

impl Drop for ImageReceiverOptions {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImageReceiverOptions_Release(self.as_raw()) });
    }
}

/// Native image receiver.
pub struct ImageReceiver {
    raw: NonNull<sys::OH_ImageReceiverNative>,
    #[cfg(feature = "api-20")]
    on_image_arrive_context: Option<NonNull<ReceiverCallbackContext>>,
}

impl ImageReceiver {
    /// Creates a new image receiver.
    pub fn new(options: &mut ImageReceiverOptions) -> ImageNativeResult<Self> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageReceiverNative_Create(options.as_raw(), &mut raw) })?;
        NonNull::new(raw)
            .ok_or(crate::ImageNativeError {
                code: sys::Image_ErrorCode_IMAGE_BAD_PARAMETER,
            })
            .map(|raw| Self {
                raw,
                #[cfg(feature = "api-20")]
                on_image_arrive_context: None,
            })
    }

    pub fn as_raw(&self) -> *mut sys::OH_ImageReceiverNative {
        self.raw.as_ptr()
    }

    pub fn receiving_surface_id(&self) -> ImageNativeResult<u64> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageReceiverNative_GetReceivingSurfaceId(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn read_latest_image(&self) -> ImageNativeResult<Image> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageReceiverNative_ReadLatestImage(self.as_raw(), &mut raw)
        })?;
        Image::from_raw(raw).ok_or(crate::ImageNativeError {
            code: sys::Image_ErrorCode_IMAGE_ALLOC_FAILED,
        })
    }

    pub fn read_next_image(&self) -> ImageNativeResult<Image> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageReceiverNative_ReadNextImage(self.as_raw(), &mut raw)
        })?;
        Image::from_raw(raw).ok_or(crate::ImageNativeError {
            code: sys::Image_ErrorCode_IMAGE_ALLOC_FAILED,
        })
    }

    /// Registers a callback for any incoming image.
    pub fn on<F>(&mut self, callback: F) -> ImageNativeResult<()>
    where
        F: FnMut() + Send + 'static,
    {
        self.off()?;
        let callback = Box::into_raw(Box::new(ReceiverCallbackContext {
            callback: Box::new(callback),
        }));
        check_status(unsafe {
            sys::OH_ImageReceiverNative_On(self.as_raw(), Some(on_trampoline))
        })?;
        let mut registry = match on_callback_registry().lock() {
            Ok(registry) => registry,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = registry.insert(self.as_raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut ReceiverCallbackContext));
            }
        }
        Ok(())
    }

    /// Unregisters the `on` callback.
    pub fn off(&mut self) -> ImageNativeResult<()> {
        check_status(unsafe { sys::OH_ImageReceiverNative_Off(self.as_raw()) })?;
        let mut registry = match on_callback_registry().lock() {
            Ok(registry) => registry,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = registry.remove(&(self.as_raw() as usize)) {
            unsafe {
                drop(Box::from_raw(callback as *mut ReceiverCallbackContext));
            }
        }
        Ok(())
    }

    /// Registers an image-arrive callback.
    #[cfg(feature = "api-20")]
    pub fn on_image_arrive<F>(&mut self, callback: F) -> ImageNativeResult<()>
    where
        F: FnMut() + Send + 'static,
    {
        self.off_image_arrive()?;
        let context = NonNull::new(Box::into_raw(Box::new(ReceiverCallbackContext {
            callback: Box::new(callback),
        })))
        .expect("callback context should not be null");
        check_status(unsafe {
            sys::OH_ImageReceiverNative_OnImageArrive(
                self.as_raw(),
                Some(on_image_arrive_trampoline),
                context.as_ptr().cast(),
            )
        })?;
        self.on_image_arrive_context = Some(context);
        Ok(())
    }

    /// Unregisters the image-arrive callback.
    #[cfg(feature = "api-20")]
    pub fn off_image_arrive(&mut self) -> ImageNativeResult<()> {
        if self.on_image_arrive_context.is_some() {
            check_status(unsafe {
                sys::OH_ImageReceiverNative_OffImageArrive(
                    self.as_raw(),
                    Some(on_image_arrive_trampoline),
                )
            })?;
            if let Some(context) = self.on_image_arrive_context.take() {
                unsafe {
                    drop(Box::from_raw(context.as_ptr()));
                }
            }
        }
        Ok(())
    }

    pub fn size(&self) -> ImageNativeResult<ImageSize> {
        let mut value = ImageSize {
            width: 0,
            height: 0,
        };
        check_status(unsafe { sys::OH_ImageReceiverNative_GetSize(self.as_raw(), &mut value) })?;
        Ok(value)
    }

    pub fn capacity(&self) -> ImageNativeResult<i32> {
        let mut value = 0;
        check_status(unsafe {
            sys::OH_ImageReceiverNative_GetCapacity(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }
}

impl Drop for ImageReceiver {
    fn drop(&mut self) {
        let _ = self.off();
        #[cfg(feature = "api-20")]
        let _ = self.off_image_arrive();
        let _ = check_status(unsafe { sys::OH_ImageReceiverNative_Release(self.as_raw()) });
    }
}

unsafe extern "C" fn on_trampoline(receiver: *mut sys::OH_ImageReceiverNative) {
    let registry = match on_callback_registry().lock() {
        Ok(registry) => registry,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = registry.get(&(receiver as usize)) {
        let callback = unsafe { &mut *(*callback as *mut ReceiverCallbackContext) };
        (callback.callback)();
    }
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_image_arrive_trampoline(
    _receiver: *mut sys::OH_ImageReceiverNative,
    user_data: *mut c_void,
) {
    let Some(context) = NonNull::new(user_data.cast::<ReceiverCallbackContext>()) else {
        return;
    };
    let callback = unsafe { context.as_ptr().as_mut() };
    if let Some(callback) = callback {
        (callback.callback)();
    }
}
