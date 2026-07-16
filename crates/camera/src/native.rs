//! RAII owners for CameraKit resources.
//!
//! All raw CameraKit calls live in this module. Higher layers only receive
//! initialized handles and owned copies of discovery data.

use std::ffi::CStr;
use std::ptr::NonNull;

use ohos_camera_sys as sys;

use crate::{CameraCaptureOptions, CameraError, CameraResult};

pub(crate) fn check(code: sys::Camera_ErrorCode, operation: &'static str) -> CameraResult<()> {
    if code == sys::Camera_ErrorCode_CAMERA_OK {
        Ok(())
    } else {
        Err(CameraError::native(operation, code))
    }
}

fn created<T>(raw: *mut T, operation: &'static str) -> CameraResult<NonNull<T>> {
    NonNull::new(raw).ok_or_else(|| {
        CameraError::invalid_state(operation, "CameraKit returned a null native handle")
    })
}

pub(crate) struct CameraManager {
    raw: NonNull<sys::Camera_Manager>,
}

impl CameraManager {
    pub(crate) fn new() -> CameraResult<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: `raw` is a valid output pointer and ownership is transferred
        // to the returned manager on success.
        check(
            unsafe { sys::OH_Camera_GetCameraManager(&mut raw) },
            "OH_Camera_GetCameraManager",
        )?;
        Ok(Self {
            raw: created(raw, "OH_Camera_GetCameraManager")?,
        })
    }

    pub(crate) fn supported_cameras(&self) -> CameraResult<SupportedCameras<'_>> {
        let mut cameras = std::ptr::null_mut();
        let mut count = 0;
        // SAFETY: manager is live and both output pointers are initialized.
        check(
            unsafe {
                sys::OH_CameraManager_GetSupportedCameras(
                    self.raw.as_ptr(),
                    &mut cameras,
                    &mut count,
                )
            },
            "OH_CameraManager_GetSupportedCameras",
        )?;
        Ok(SupportedCameras {
            manager: self,
            raw: NonNull::new(cameras),
            count,
        })
    }

    pub(crate) fn output_capability(
        &self,
        camera: &sys::Camera_Device,
    ) -> CameraResult<OutputCapability<'_>> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: manager and the selected camera description are live for the
        // synchronous discovery call.
        check(
            unsafe {
                sys::OH_CameraManager_GetSupportedCameraOutputCapabilityWithSceneMode(
                    self.raw.as_ptr(),
                    camera,
                    sys::Camera_SceneMode_NORMAL_PHOTO,
                    &mut raw,
                )
            },
            "OH_CameraManager_GetSupportedCameraOutputCapabilityWithSceneMode",
        )?;
        Ok(OutputCapability {
            manager: self,
            raw: created(
                raw,
                "OH_CameraManager_GetSupportedCameraOutputCapabilityWithSceneMode",
            )?,
        })
    }

    pub(crate) fn create_input(&self, camera: &sys::Camera_Device) -> CameraResult<CameraInput> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: manager and camera description are live and `raw` is an
        // initialized output pointer.
        check(
            unsafe { sys::OH_CameraManager_CreateCameraInput(self.raw.as_ptr(), camera, &mut raw) },
            "OH_CameraManager_CreateCameraInput",
        )?;
        CameraInput::new(created(raw, "OH_CameraManager_CreateCameraInput")?)
    }

    pub(crate) fn create_capture_session(&self) -> CameraResult<CaptureSession> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: manager is live and `raw` is an initialized output pointer.
        check(
            unsafe { sys::OH_CameraManager_CreateCaptureSession(self.raw.as_ptr(), &mut raw) },
            "OH_CameraManager_CreateCaptureSession",
        )?;
        Ok(CaptureSession::new(created(
            raw,
            "OH_CameraManager_CreateCaptureSession",
        )?))
    }

    pub(crate) fn create_preview_output(
        &self,
        profile: &sys::Camera_Profile,
        surface_id: &CStr,
        operation: &'static str,
    ) -> CameraResult<PreviewOutput> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: all borrowed arguments remain live for this synchronous call.
        check(
            unsafe {
                sys::OH_CameraManager_CreatePreviewOutput(
                    self.raw.as_ptr(),
                    profile,
                    surface_id.as_ptr(),
                    &mut raw,
                )
            },
            operation,
        )?;
        Ok(PreviewOutput {
            raw: created(raw, operation)?,
        })
    }

    pub(crate) fn create_photo_output(
        &self,
        profile: &sys::Camera_Profile,
    ) -> CameraResult<PhotoOutput> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: manager and profile are live and `raw` is initialized.
        check(
            unsafe {
                sys::OH_CameraManager_CreatePhotoOutputWithoutSurface(
                    self.raw.as_ptr(),
                    profile,
                    &mut raw,
                )
            },
            "OH_CameraManager_CreatePhotoOutputWithoutSurface",
        )?;
        Ok(PhotoOutput {
            raw: created(raw, "OH_CameraManager_CreatePhotoOutputWithoutSurface")?,
            callbacks: None,
            photo_available_registered: false,
        })
    }

    pub(crate) fn torch_supported(&self) -> bool {
        let mut supported = false;
        // SAFETY: manager is live and the output pointer is initialized.
        let _ =
            unsafe { sys::OH_CameraManager_IsTorchSupported(self.raw.as_ptr(), &mut supported) };
        supported
    }

    pub(crate) fn set_torch_mode(&self, mode: sys::Camera_TorchMode) -> CameraResult<()> {
        // SAFETY: manager is live for the duration of this call.
        check(
            unsafe { sys::OH_CameraManager_SetTorchMode(self.raw.as_ptr(), mode) },
            "OH_CameraManager_SetTorchMode",
        )
    }
}

impl Drop for CameraManager {
    fn drop(&mut self) {
        // SAFETY: this handle uniquely owns one manager instance.
        let _ = unsafe { sys::OH_Camera_DeleteCameraManager(self.raw.as_ptr()) };
    }
}

pub(crate) struct SupportedCameras<'a> {
    manager: &'a CameraManager,
    raw: Option<NonNull<sys::Camera_Device>>,
    count: u32,
}

impl SupportedCameras<'_> {
    pub(crate) fn devices(&self) -> &[sys::Camera_Device] {
        let Some(raw) = self.raw else {
            return &[];
        };
        // SAFETY: CameraKit owns `count` initialized entries until Drop.
        unsafe { std::slice::from_raw_parts(raw.as_ptr(), self.count as usize) }
    }
}

impl Drop for SupportedCameras<'_> {
    fn drop(&mut self) {
        if let Some(raw) = self.raw.take() {
            // SAFETY: paired with GetSupportedCameras on the same manager.
            let _ = unsafe {
                sys::OH_CameraManager_DeleteSupportedCameras(
                    self.manager.raw.as_ptr(),
                    raw.as_ptr(),
                    self.count,
                )
            };
        }
    }
}

pub(crate) struct OutputCapability<'a> {
    manager: &'a CameraManager,
    raw: NonNull<sys::Camera_OutputCapability>,
}

impl OutputCapability<'_> {
    pub(crate) fn preview_profiles(&self) -> Vec<sys::Camera_Profile> {
        let capability = self.value();
        Self::copy_profiles(capability.previewProfiles, capability.previewProfilesSize)
    }

    pub(crate) fn photo_profiles(&self) -> Vec<sys::Camera_Profile> {
        let capability = self.value();
        Self::copy_profiles(capability.photoProfiles, capability.photoProfilesSize)
    }

    fn value(&self) -> &sys::Camera_OutputCapability {
        // SAFETY: this owner keeps the capability live until Drop.
        unsafe { self.raw.as_ref() }
    }

    fn copy_profiles(
        profiles: *mut *mut sys::Camera_Profile,
        count: u32,
    ) -> Vec<sys::Camera_Profile> {
        if profiles.is_null() || count == 0 {
            return Vec::new();
        }
        // SAFETY: CameraKit reports `count` profile pointers owned by the live
        // output capability. Values are copied before returning.
        unsafe { std::slice::from_raw_parts(profiles, count as usize) }
            .iter()
            .filter_map(|profile| unsafe { profile.as_ref().copied() })
            .collect()
    }
}

impl Drop for OutputCapability<'_> {
    fn drop(&mut self) {
        // SAFETY: paired with capability discovery on the same manager.
        let _ = unsafe {
            sys::OH_CameraManager_DeleteSupportedCameraOutputCapability(
                self.manager.raw.as_ptr(),
                self.raw.as_ptr(),
            )
        };
    }
}

pub(crate) struct CameraInput {
    raw: NonNull<sys::Camera_Input>,
    opened: bool,
    callbacks: Option<Box<sys::CameraInput_Callbacks>>,
}

impl CameraInput {
    fn new(raw: NonNull<sys::Camera_Input>) -> CameraResult<Self> {
        let mut input = Self {
            raw,
            opened: false,
            callbacks: None,
        };
        // SAFETY: the handle was just created and is uniquely owned.
        check(
            unsafe { sys::OH_CameraInput_Open(input.raw.as_ptr()) },
            "OH_CameraInput_Open",
        )?;
        input.opened = true;
        Ok(input)
    }

    pub(crate) fn register_error_callback(
        &mut self,
        callback: sys::OH_CameraInput_OnError,
    ) -> CameraResult<()> {
        let mut callbacks = Box::new(sys::CameraInput_Callbacks { onError: callback });
        // SAFETY: the callback table stays boxed until it is unregistered.
        check(
            unsafe { sys::OH_CameraInput_RegisterCallback(self.raw.as_ptr(), callbacks.as_mut()) },
            "OH_CameraInput_RegisterCallback",
        )?;
        self.callbacks = Some(callbacks);
        Ok(())
    }

    pub(crate) fn raw_key(&self) -> usize {
        self.raw.as_ptr() as usize
    }
}

impl Drop for CameraInput {
    fn drop(&mut self) {
        if let Some(callbacks) = self.callbacks.as_deref_mut() {
            // SAFETY: the same live handle and callback table were registered.
            let _ = unsafe { sys::OH_CameraInput_UnregisterCallback(self.raw.as_ptr(), callbacks) };
        }
        if self.opened {
            // SAFETY: this handle owns the open input.
            let _ = unsafe { sys::OH_CameraInput_Close(self.raw.as_ptr()) };
        }
        // SAFETY: this handle uniquely owns the input.
        let _ = unsafe { sys::OH_CameraInput_Release(self.raw.as_ptr()) };
    }
}

pub(crate) struct CaptureSession {
    raw: NonNull<sys::Camera_CaptureSession>,
    started: bool,
    callbacks: Option<Box<sys::CaptureSession_Callbacks>>,
}

impl CaptureSession {
    fn new(raw: NonNull<sys::Camera_CaptureSession>) -> Self {
        Self {
            raw,
            started: false,
            callbacks: None,
        }
    }

    pub(crate) fn register_callbacks(
        &mut self,
        focus: sys::OH_CaptureSession_OnFocusStateChange,
        error: sys::OH_CaptureSession_OnError,
    ) -> CameraResult<()> {
        let mut callbacks = Box::new(sys::CaptureSession_Callbacks {
            onFocusStateChange: focus,
            onError: error,
        });
        // SAFETY: the callback table remains boxed until Drop unregisters it.
        check(
            unsafe {
                sys::OH_CaptureSession_RegisterCallback(self.raw.as_ptr(), callbacks.as_mut())
            },
            "OH_CaptureSession_RegisterCallback",
        )?;
        self.callbacks = Some(callbacks);
        Ok(())
    }

    pub(crate) fn configure(
        &mut self,
        input: &CameraInput,
        preview: &PreviewOutput,
        photo: Option<&PhotoOutput>,
        frame: Option<&PreviewOutput>,
    ) -> CameraResult<()> {
        // SAFETY: every handle is live and remains owned by the configured
        // camera for at least the capture-session lifetime.
        check(
            unsafe { sys::OH_CaptureSession_BeginConfig(self.raw.as_ptr()) },
            "OH_CaptureSession_BeginConfig",
        )?;
        check(
            unsafe { sys::OH_CaptureSession_AddInput(self.raw.as_ptr(), input.raw.as_ptr()) },
            "OH_CaptureSession_AddInput",
        )?;
        check(
            unsafe {
                sys::OH_CaptureSession_AddPreviewOutput(self.raw.as_ptr(), preview.raw.as_ptr())
            },
            "OH_CaptureSession_AddPreviewOutput",
        )?;
        if let Some(photo) = photo {
            check(
                unsafe {
                    sys::OH_CaptureSession_AddPhotoOutput(self.raw.as_ptr(), photo.raw.as_ptr())
                },
                "OH_CaptureSession_AddPhotoOutput",
            )?;
        }
        if let Some(frame) = frame {
            check(
                unsafe {
                    sys::OH_CaptureSession_AddPreviewOutput(self.raw.as_ptr(), frame.raw.as_ptr())
                },
                "OH_CaptureSession_AddPreviewOutput(analysis)",
            )?;
        }
        check(
            unsafe { sys::OH_CaptureSession_CommitConfig(self.raw.as_ptr()) },
            "OH_CaptureSession_CommitConfig",
        )
    }

    pub(crate) fn start(&mut self) -> CameraResult<()> {
        // SAFETY: this configured session is live and uniquely owned.
        check(
            unsafe { sys::OH_CaptureSession_Start(self.raw.as_ptr()) },
            "OH_CaptureSession_Start",
        )?;
        self.started = true;
        Ok(())
    }

    pub(crate) fn raw_key(&self) -> usize {
        self.raw.as_ptr() as usize
    }

    fn read<T: Default>(
        &self,
        getter: unsafe extern "C" fn(
            *mut sys::Camera_CaptureSession,
            *mut T,
        ) -> sys::Camera_ErrorCode,
    ) -> Option<T> {
        let mut value = T::default();
        // SAFETY: session is live and `value` is an initialized output.
        (unsafe { getter(self.raw.as_ptr(), &mut value) } == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some(value)
    }

    fn supports<T: Copy>(
        &self,
        value: T,
        query: unsafe extern "C" fn(
            *mut sys::Camera_CaptureSession,
            T,
            *mut bool,
        ) -> sys::Camera_ErrorCode,
    ) -> bool {
        let mut supported = false;
        // SAFETY: session is live and the output pointer is initialized.
        (unsafe { query(self.raw.as_ptr(), value, &mut supported) }
            == sys::Camera_ErrorCode_CAMERA_OK)
            && supported
    }

    pub(crate) fn has_flash(&self) -> bool {
        self.read(sys::OH_CaptureSession_HasFlash).unwrap_or(false)
    }

    pub(crate) fn supports_flash_mode(&self, mode: sys::Camera_FlashMode) -> bool {
        self.supports(mode, sys::OH_CaptureSession_IsFlashModeSupported)
    }

    pub(crate) fn flash_mode(&self) -> Option<sys::Camera_FlashMode> {
        self.read(sys::OH_CaptureSession_GetFlashMode)
    }

    pub(crate) fn zoom_range(&self) -> Option<(f32, f32)> {
        let (mut min, mut max) = (0.0, 0.0);
        // SAFETY: session is live and both outputs are initialized.
        (unsafe { sys::OH_CaptureSession_GetZoomRatioRange(self.raw.as_ptr(), &mut min, &mut max) }
            == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some((min, max))
    }

    pub(crate) fn zoom(&self) -> Option<f32> {
        self.read(sys::OH_CaptureSession_GetZoomRatio)
    }

    pub(crate) fn supports_exposure_mode(&self, mode: sys::Camera_ExposureMode) -> bool {
        self.supports(mode, sys::OH_CaptureSession_IsExposureModeSupported)
    }

    pub(crate) fn exposure_mode(&self) -> Option<sys::Camera_ExposureMode> {
        self.read(sys::OH_CaptureSession_GetExposureMode)
    }

    pub(crate) fn exposure_bias_range(&self) -> Option<(f32, f32, f32)> {
        let (mut min, mut max, mut step) = (0.0, 0.0, 0.0);
        // SAFETY: session is live and all outputs are initialized.
        (unsafe {
            sys::OH_CaptureSession_GetExposureBiasRange(
                self.raw.as_ptr(),
                &mut min,
                &mut max,
                &mut step,
            )
        } == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some((min, max, step))
    }

    pub(crate) fn exposure_bias(&self) -> Option<f32> {
        self.read(sys::OH_CaptureSession_GetExposureBias)
    }

    pub(crate) fn exposure_value(&self) -> Option<f32> {
        self.read(sys::OH_CaptureSession_GetExposureValue)
    }

    pub(crate) fn metering_point(&self) -> Option<sys::Camera_Point> {
        let mut value = sys::Camera_Point { x: 0.0, y: 0.0 };
        // SAFETY: session is live and `value` is initialized.
        (unsafe { sys::OH_CaptureSession_GetMeteringPoint(self.raw.as_ptr(), &mut value) }
            == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some(value)
    }

    pub(crate) fn supports_focus_mode(&self, mode: sys::Camera_FocusMode) -> bool {
        self.supports(mode, sys::OH_CaptureSession_IsFocusModeSupported)
    }

    pub(crate) fn focus_mode(&self) -> Option<sys::Camera_FocusMode> {
        self.read(sys::OH_CaptureSession_GetFocusMode)
    }

    pub(crate) fn focus_point(&self) -> Option<sys::Camera_Point> {
        let mut value = sys::Camera_Point { x: 0.0, y: 0.0 };
        // SAFETY: session is live and `value` is initialized.
        (unsafe { sys::OH_CaptureSession_GetFocusPoint(self.raw.as_ptr(), &mut value) }
            == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some(value)
    }

    pub(crate) fn supports_stabilization_mode(
        &self,
        mode: sys::Camera_VideoStabilizationMode,
    ) -> bool {
        self.supports(
            mode,
            sys::OH_CaptureSession_IsVideoStabilizationModeSupported,
        )
    }

    pub(crate) fn stabilization_mode(&self) -> Option<sys::Camera_VideoStabilizationMode> {
        self.read(sys::OH_CaptureSession_GetVideoStabilizationMode)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn supports_white_balance_mode(&self, mode: sys::Camera_WhiteBalanceMode) -> bool {
        self.supports(mode, sys::OH_CaptureSession_IsWhiteBalanceModeSupported)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn white_balance_mode(&self) -> Option<sys::Camera_WhiteBalanceMode> {
        self.read(sys::OH_CaptureSession_GetWhiteBalanceMode)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn white_balance_range(&self) -> Option<(i32, i32)> {
        let (mut min, mut max) = (0, 0);
        // SAFETY: session is live and both outputs are initialized.
        (unsafe {
            sys::OH_CaptureSession_GetWhiteBalanceRange(self.raw.as_ptr(), &mut min, &mut max)
        } == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some((min, max))
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn white_balance(&self) -> Option<i32> {
        self.read(sys::OH_CaptureSession_GetWhiteBalance)
    }

    #[cfg(feature = "api-19")]
    pub(crate) fn macro_supported(&self) -> bool {
        self.read(sys::OH_CaptureSession_IsMacroSupported)
            .unwrap_or(false)
    }

    #[cfg(feature = "api-13")]
    pub(crate) fn auto_device_switch_supported(&self) -> bool {
        self.read(sys::OH_CaptureSession_IsAutoDeviceSwitchSupported)
            .unwrap_or(false)
    }

    pub(crate) fn focal_length(&self) -> Option<f32> {
        self.read(sys::OH_CaptureSession_GetFocalLength)
    }

    pub(crate) fn supported_color_spaces(&self) -> Vec<u32> {
        let mut values = std::ptr::null_mut();
        let mut count = 0;
        // SAFETY: session is live and both outputs are initialized.
        let code = unsafe {
            sys::OH_CaptureSession_GetSupportedColorSpaces(
                self.raw.as_ptr(),
                &mut values,
                &mut count,
            )
        };
        if code != sys::Camera_ErrorCode_CAMERA_OK || values.is_null() {
            return Vec::new();
        }
        // SAFETY: CameraKit returned `count` values and keeps them valid until
        // the matching delete call below.
        let result = unsafe { std::slice::from_raw_parts(values, count as usize) }.to_vec();
        // SAFETY: paired with GetSupportedColorSpaces on this session.
        let _ = unsafe { sys::OH_CaptureSession_DeleteColorSpaces(self.raw.as_ptr(), values) };
        result
    }

    pub(crate) fn active_color_space(&self) -> Option<u32> {
        self.read(sys::OH_CaptureSession_GetActiveColorSpace)
    }

    pub(crate) fn set_flash_mode(&self, mode: sys::Camera_FlashMode) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetFlashMode(self.raw.as_ptr(), mode) },
            "OH_CaptureSession_SetFlashMode",
        )
    }

    pub(crate) fn set_zoom(&self, ratio: f32, smooth: bool) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        let code = if smooth {
            unsafe {
                sys::OH_CaptureSession_SetSmoothZoom(
                    self.raw.as_ptr(),
                    ratio,
                    sys::Camera_SmoothZoomMode_NORMAL,
                )
            }
        } else {
            unsafe { sys::OH_CaptureSession_SetZoomRatio(self.raw.as_ptr(), ratio) }
        };
        check(code, "OH_CaptureSession_SetZoomRatio")
    }

    pub(crate) fn set_exposure_mode(&self, mode: sys::Camera_ExposureMode) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetExposureMode(self.raw.as_ptr(), mode) },
            "OH_CaptureSession_SetExposureMode",
        )
    }

    pub(crate) fn set_exposure_bias(&self, bias: f32) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetExposureBias(self.raw.as_ptr(), bias) },
            "OH_CaptureSession_SetExposureBias",
        )
    }

    pub(crate) fn set_metering_point(&self, point: sys::Camera_Point) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetMeteringPoint(self.raw.as_ptr(), point) },
            "OH_CaptureSession_SetMeteringPoint",
        )
    }

    pub(crate) fn set_focus_mode(&self, mode: sys::Camera_FocusMode) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetFocusMode(self.raw.as_ptr(), mode) },
            "OH_CaptureSession_SetFocusMode",
        )
    }

    pub(crate) fn set_focus_point(&self, point: sys::Camera_Point) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetFocusPoint(self.raw.as_ptr(), point) },
            "OH_CaptureSession_SetFocusPoint",
        )
    }

    pub(crate) fn set_stabilization_mode(
        &self,
        mode: sys::Camera_VideoStabilizationMode,
    ) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetVideoStabilizationMode(self.raw.as_ptr(), mode) },
            "OH_CaptureSession_SetVideoStabilizationMode",
        )
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn set_white_balance_mode(
        &self,
        mode: sys::Camera_WhiteBalanceMode,
    ) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetWhiteBalanceMode(self.raw.as_ptr(), mode) },
            "OH_CaptureSession_SetWhiteBalanceMode",
        )
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn set_white_balance(&self, value: i32) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetWhiteBalance(self.raw.as_ptr(), value) },
            "OH_CaptureSession_SetWhiteBalance",
        )
    }

    #[cfg(feature = "api-19")]
    pub(crate) fn set_macro_enabled(&self, enabled: bool) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_EnableMacro(self.raw.as_ptr(), enabled) },
            "OH_CaptureSession_EnableMacro",
        )
    }

    #[cfg(feature = "api-13")]
    pub(crate) fn set_auto_device_switch_enabled(&self, enabled: bool) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_EnableAutoDeviceSwitch(self.raw.as_ptr(), enabled) },
            "OH_CaptureSession_EnableAutoDeviceSwitch",
        )
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn set_quality_priority(
        &self,
        priority: sys::Camera_QualityPrioritization,
    ) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetQualityPrioritization(self.raw.as_ptr(), priority) },
            "OH_CaptureSession_SetQualityPrioritization",
        )
    }

    pub(crate) fn set_active_color_space(&self, color_space: u32) -> CameraResult<()> {
        // SAFETY: session is configured, running, and live.
        check(
            unsafe { sys::OH_CaptureSession_SetActiveColorSpace(self.raw.as_ptr(), color_space) },
            "OH_CaptureSession_SetActiveColorSpace",
        )
    }
}

impl Drop for CaptureSession {
    fn drop(&mut self) {
        if self.started {
            // SAFETY: this handle owns the running session.
            let _ = unsafe { sys::OH_CaptureSession_Stop(self.raw.as_ptr()) };
        }
        if let Some(callbacks) = self.callbacks.as_deref_mut() {
            // SAFETY: the same table was registered on this live session.
            let _ =
                unsafe { sys::OH_CaptureSession_UnregisterCallback(self.raw.as_ptr(), callbacks) };
        }
        // SAFETY: this handle uniquely owns the capture session.
        let _ = unsafe { sys::OH_CaptureSession_Release(self.raw.as_ptr()) };
    }
}

pub(crate) struct PreviewOutput {
    raw: NonNull<sys::Camera_PreviewOutput>,
}

impl PreviewOutput {
    pub(crate) fn supported_frame_rates(&self) -> Vec<sys::Camera_FrameRateRange> {
        let mut values = std::ptr::null_mut();
        let mut count = 0;
        // SAFETY: preview output is live and both outputs are initialized.
        let code = unsafe {
            sys::OH_PreviewOutput_GetSupportedFrameRates(self.raw.as_ptr(), &mut values, &mut count)
        };
        if code != sys::Camera_ErrorCode_CAMERA_OK || values.is_null() {
            return Vec::new();
        }
        // SAFETY: CameraKit returned `count` initialized values.
        let result = unsafe { std::slice::from_raw_parts(values, count as usize) }.to_vec();
        // SAFETY: paired with GetSupportedFrameRates on this output.
        let _ = unsafe { sys::OH_PreviewOutput_DeleteFrameRates(self.raw.as_ptr(), values) };
        result
    }

    pub(crate) fn active_frame_rate(&self) -> Option<sys::Camera_FrameRateRange> {
        let mut value = sys::Camera_FrameRateRange { min: 0, max: 0 };
        // SAFETY: preview output is live and `value` is initialized.
        (unsafe { sys::OH_PreviewOutput_GetActiveFrameRate(self.raw.as_ptr(), &mut value) }
            == sys::Camera_ErrorCode_CAMERA_OK)
            .then_some(value)
    }

    pub(crate) fn set_frame_rate(&self, min: i32, max: i32) -> CameraResult<()> {
        // SAFETY: preview output is live.
        check(
            unsafe { sys::OH_PreviewOutput_SetFrameRate(self.raw.as_ptr(), min, max) },
            "OH_PreviewOutput_SetFrameRate",
        )
    }

    pub(crate) fn set_rotation(&self, rotation: sys::Camera_ImageRotation) -> CameraResult<()> {
        // SAFETY: preview output is live.
        check(
            unsafe { sys::OH_PreviewOutput_SetPreviewRotation(self.raw.as_ptr(), rotation, true) },
            "OH_PreviewOutput_SetPreviewRotation",
        )
    }
}

impl Drop for PreviewOutput {
    fn drop(&mut self) {
        // SAFETY: this handle uniquely owns the preview output.
        let _ = unsafe { sys::OH_PreviewOutput_Release(self.raw.as_ptr()) };
    }
}

pub(crate) struct PhotoOutput {
    raw: NonNull<sys::Camera_PhotoOutput>,
    callbacks: Option<Box<sys::PhotoOutput_Callbacks>>,
    photo_available_registered: bool,
}

impl PhotoOutput {
    pub(crate) fn register_callbacks(
        &mut self,
        error: sys::OH_PhotoOutput_OnError,
        photo_available: sys::OH_PhotoOutput_PhotoAvailable,
    ) -> CameraResult<()> {
        let mut callbacks = Box::new(sys::PhotoOutput_Callbacks {
            onFrameStart: None,
            onFrameShutter: None,
            onFrameEnd: None,
            onError: error,
        });
        // SAFETY: callback storage remains boxed until Drop unregisters it.
        check(
            unsafe { sys::OH_PhotoOutput_RegisterCallback(self.raw.as_ptr(), callbacks.as_mut()) },
            "OH_PhotoOutput_RegisterCallback",
        )?;
        self.callbacks = Some(callbacks);
        // SAFETY: the callback function has static lifetime.
        check(
            unsafe {
                sys::OH_PhotoOutput_RegisterPhotoAvailableCallback(
                    self.raw.as_ptr(),
                    photo_available,
                )
            },
            "OH_PhotoOutput_RegisterPhotoAvailableCallback",
        )?;
        self.photo_available_registered = true;
        Ok(())
    }

    pub(crate) fn raw_key(&self) -> usize {
        self.raw.as_ptr() as usize
    }

    pub(crate) fn capture(&self) -> CameraResult<()> {
        // SAFETY: output belongs to a running capture session.
        check(
            unsafe { sys::OH_PhotoOutput_Capture(self.raw.as_ptr()) },
            "OH_PhotoOutput_Capture",
        )
    }

    pub(crate) fn capture_with_options(&self, options: CameraCaptureOptions) -> CameraResult<()> {
        let mut location = options.location.map(|location| sys::Camera_Location {
            latitude: location.latitude,
            longitude: location.longitude,
            altitude: location.altitude,
        });
        let setting = sys::Camera_PhotoCaptureSetting {
            quality: options.quality.into(),
            rotation: options.rotation.into(),
            location: location
                .as_mut()
                .map_or(std::ptr::null_mut(), std::ptr::from_mut),
            mirror: options.mirror,
        };
        // SAFETY: output is live and setting pointers remain valid for this call.
        check(
            unsafe { sys::OH_PhotoOutput_Capture_WithCaptureSetting(self.raw.as_ptr(), setting) },
            "OH_PhotoOutput_Capture_WithCaptureSetting",
        )
    }

    pub(crate) fn mirror_supported(&self) -> bool {
        let mut supported = false;
        // SAFETY: output is live and the output pointer is initialized.
        let _ = unsafe { sys::OH_PhotoOutput_IsMirrorSupported(self.raw.as_ptr(), &mut supported) };
        supported
    }
}

impl Drop for PhotoOutput {
    fn drop(&mut self) {
        if self.photo_available_registered {
            // SAFETY: the same callback was registered on this live output.
            let _ = unsafe {
                sys::OH_PhotoOutput_UnregisterPhotoAvailableCallback(
                    self.raw.as_ptr(),
                    crate::callbacks::CameraCallbackRouter::photo_available_callback(),
                )
            };
        }
        if let Some(callbacks) = self.callbacks.as_deref_mut() {
            // SAFETY: the same callback table was registered on this output.
            let _ = unsafe { sys::OH_PhotoOutput_UnregisterCallback(self.raw.as_ptr(), callbacks) };
        }
        // SAFETY: this handle uniquely owns the photo output.
        let _ = unsafe { sys::OH_PhotoOutput_Release(self.raw.as_ptr()) };
    }
}
