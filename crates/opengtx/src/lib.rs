use std::{ptr::NonNull, sync::RwLock};

use hms_opengtx_sys::*;

mod error;
mod types;

pub use error::*;
pub use types::*;

pub use hms_opengtx_sys as sys;

type DeviceInfoCallback = Box<dyn Fn(TempLevel) + Send + Sync + 'static>;

static DEVICE_INFO_CALLBACK: RwLock<Option<DeviceInfoCallback>> = RwLock::new(None);

fn set_device_info_callback(callback: Option<DeviceInfoCallback>) {
    let mut guard = DEVICE_INFO_CALLBACK
        .write()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *guard = callback;
}

unsafe extern "C" fn device_info_trampoline(temp_level: OpenGTX_TempLevel) {
    let Some(level) = TempLevel::try_from_raw(temp_level) else {
        return;
    };

    let guard = match DEVICE_INFO_CALLBACK.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = guard.as_ref() {
        callback(level);
    }
}

pub struct OpenGtxContext {
    raw: NonNull<OpenGTX_Context>,
}

impl OpenGtxContext {
    pub fn new() -> OpenGtxResult<Self> {
        let context = unsafe { HMS_OpenGTX_CreateContext(Some(device_info_trampoline)) };
        let Some(raw) = NonNull::new(context) else {
            return Err(OpenGtxError::CreateContextFailed);
        };

        Ok(Self { raw })
    }
    pub fn with_temp_callback<F>(callback: F) -> OpenGtxResult<Self>
    where
        F: Fn(TempLevel) + Send + Sync + 'static,
    {
        Self::set_temp_callback(callback);
        Self::new()
    }

    pub fn set_temp_callback<F>(callback: F)
    where
        F: Fn(TempLevel) + Send + Sync + 'static,
    {
        set_device_info_callback(Some(Box::new(callback)));
    }

    pub fn clear_temp_callback() {
        set_device_info_callback(None);
    }

    pub fn raw(&self) -> *mut OpenGTX_Context {
        self.raw.as_ptr()
    }

    pub fn set_configuration(&mut self, config: &ConfigDescription) -> OpenGtxResult<()> {
        let (raw_config, _package_name, _app_version, _engine_version) = config.to_raw()?;
        let status = unsafe { HMS_OpenGTX_SetConfiguration(self.raw.as_ptr(), &raw_config) };
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
        let (raw_info, _description) = game_scene_info.to_raw()?;
        let status = unsafe { HMS_OpenGTX_DispatchGameSceneInfo(self.raw.as_ptr(), &raw_info) };
        check_status(status)
    }

    pub fn dispatch_network_info(&mut self, network_info: &NetworkInfo) -> OpenGtxResult<()> {
        let (raw_info, _network_server_ip) = network_info.to_raw()?;
        let status = unsafe { HMS_OpenGTX_DispatchNetworkInfo(self.raw.as_ptr(), &raw_info) };
        check_status(status)
    }
}

unsafe impl Send for OpenGtxContext {}

impl Drop for OpenGtxContext {
    fn drop(&mut self) {
        let mut raw = self.raw.as_ptr();
        let _status = unsafe { HMS_OpenGTX_DestroyContext(&mut raw) };

        #[cfg(debug_assertions)]
        debug_assert_eq!(_status, OpenGTX_ErrorCode_OPENGTX_SUCCESS);
    }
}
