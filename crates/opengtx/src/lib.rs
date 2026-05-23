use std::ptr::NonNull;

use hms_opengtx_sys::*;

mod error;
mod types;

pub use error::*;
pub use types::*;

pub use hms_opengtx_sys as sys;

pub struct OpenGtxContext {
    raw: NonNull<OpenGTX_Context>,
}

impl OpenGtxContext {
    pub fn new() -> OpenGtxResult<Self> {
        let context = unsafe { HMS_OpenGTX_CreateContext() };
        let Some(raw) = NonNull::new(context) else {
            return Err(OpenGtxError::CreateContextFailed);
        };

        Ok(Self { raw })
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
