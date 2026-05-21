use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

use hms_opengtx_sys::*;

mod error;
mod types;

pub use error::*;
pub use types::*;

pub use hms_opengtx_sys as sys;

type DeviceInfoCallback = dyn FnMut(TempLevel) + Send + 'static;

static DEVICE_INFO_CALLBACK: LazyLock<Mutex<CallbackState>> =
    LazyLock::new(|| Mutex::new(CallbackState::Empty));

enum CallbackState {
    Empty,
    Calling,
    Callback(Box<DeviceInfoCallback>),
}

pub struct OpenGtxContext {
    raw: NonNull<OpenGTX_Context>,
}

impl OpenGtxContext {
    pub fn new() -> OpenGtxResult<Self> {
        Self::create(None, false)
    }

    pub fn with_device_info_callback<F>(callback: F) -> OpenGtxResult<Self>
    where
        F: FnMut(TempLevel) + Send + 'static,
    {
        Self::set_device_info_callback(callback);
        Self::create(Some(device_info_callback), true)
    }

    pub fn set_device_info_callback<F>(callback: F)
    where
        F: FnMut(TempLevel) + Send + 'static,
    {
        let mut guard = DEVICE_INFO_CALLBACK
            .lock()
            .expect("OpenGTX callback lock poisoned");
        *guard = CallbackState::Callback(Box::new(callback));
    }

    pub fn clear_device_info_callback() {
        let mut guard = DEVICE_INFO_CALLBACK
            .lock()
            .expect("OpenGTX callback lock poisoned");
        *guard = CallbackState::Empty;
    }

    fn create(
        callback: OpenGTX_DeviceInfoCallback,
        clear_callback_on_failure: bool,
    ) -> OpenGtxResult<Self> {
        let context = unsafe { HMS_OpenGTX_CreateContext(callback) };
        let Some(raw) = NonNull::new(context) else {
            if clear_callback_on_failure {
                Self::clear_device_info_callback();
            }
            return Err(OpenGtxError::CreateContextFailed);
        };

        Ok(Self { raw })
    }

    pub fn raw(&self) -> *mut OpenGTX_Context {
        self.raw.as_ptr()
    }

    pub fn set_configuration(&mut self, config: &ConfigDescription) -> OpenGtxResult<()> {
        let raw_config = config.to_raw()?;
        let status =
            unsafe { HMS_OpenGTX_SetConfiguration(self.raw.as_ptr(), raw_config.as_ptr()) };
        check_status(status)
    }

    pub fn activate(&mut self) -> OpenGtxResult<()> {
        let status = unsafe { HMS_OpenGTX_Activate(self.raw.as_ptr()) };
        check_status(status)
    }

    pub fn deactivate(&mut self) -> OpenGtxResult<()> {
        let status = unsafe { HMS_OpenGTX_Deactivate(self.raw.as_ptr()) };
        check_status(status)
    }

    pub fn dispatch_frame_render_info(
        &mut self,
        frame_render_info: FrameRenderInfo,
    ) -> OpenGtxResult<()> {
        let raw_info: OpenGTX_FrameRenderInfo = frame_render_info.into();
        let status = unsafe { HMS_OpenGTX_DispatchFrameRenderInfo(self.raw.as_ptr(), &raw_info) };
        check_status(status)
    }

    pub fn dispatch_game_scene_info(
        &mut self,
        game_scene_info: &GameSceneInfo,
    ) -> OpenGtxResult<()> {
        let raw_info = game_scene_info.to_raw()?;
        let status =
            unsafe { HMS_OpenGTX_DispatchGameSceneInfo(self.raw.as_ptr(), raw_info.as_ptr()) };
        check_status(status)
    }

    pub fn dispatch_network_info(&mut self, network_info: &NetworkInfo) -> OpenGtxResult<()> {
        let raw_info = network_info.to_raw()?;
        let status =
            unsafe { HMS_OpenGTX_DispatchNetworkInfo(self.raw.as_ptr(), raw_info.as_ptr()) };
        check_status(status)
    }
}

unsafe impl Send for OpenGtxContext {}

impl Drop for OpenGtxContext {
    fn drop(&mut self) {
        let mut raw = self.raw.as_ptr();
        let status = unsafe { HMS_OpenGTX_DestroyContext(&mut raw) };

        #[cfg(debug_assertions)]
        debug_assert_eq!(status, OpenGTX_ErrorCode_OPENGTX_SUCCESS);
    }
}

unsafe extern "C" fn device_info_callback(temp_level: OpenGTX_TempLevel) {
    let Some(temp_level) = TempLevel::try_from_raw(temp_level) else {
        return;
    };

    let mut callback = {
        let mut guard = DEVICE_INFO_CALLBACK
            .lock()
            .expect("OpenGTX callback lock poisoned");
        match std::mem::replace(&mut *guard, CallbackState::Calling) {
            CallbackState::Callback(callback) => callback,
            state => {
                *guard = state;
                return;
            }
        }
    };

    let _ = catch_unwind(AssertUnwindSafe(|| callback(temp_level)));

    let mut guard = DEVICE_INFO_CALLBACK
        .lock()
        .expect("OpenGTX callback lock poisoned");
    if matches!(*guard, CallbackState::Calling) {
        *guard = CallbackState::Callback(callback);
    }
}
