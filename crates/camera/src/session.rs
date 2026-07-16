use std::sync::mpsc::Sender;

use crate::callbacks::CameraInstanceLease;
use crate::device::CameraDevice;
#[cfg(feature = "api-14")]
use crate::CameraQualityPriority;
#[cfg(feature = "api-20")]
use crate::CameraWhiteBalanceMode;
use crate::{
    CameraCapabilities, CameraCaptureOptions, CameraConfiguration, CameraControls, CameraError,
    CameraEvent, CameraExposureMode, CameraFlashMode, CameraFloatRange, CameraFocusMode,
    CameraFrameRateRange, CameraImageRotation, CameraPoint, CameraResult, CameraSessionInfo,
    CameraStabilizationMode, CameraTorchMode,
};

/// Exclusive, running CameraKit session for the device camera.
pub struct CameraSession {
    // The device must release every callback and native resource before the
    // instance lease makes another CameraSession possible.
    device: CameraDevice,
    _instance: CameraInstanceLease,
    info: CameraSessionInfo,
    capabilities: CameraCapabilities,
    controls: CameraControls,
    flash_mode_before_torch: Option<CameraFlashMode>,
}

impl CameraSession {
    pub fn open(
        configuration: CameraConfiguration,
        events: Sender<CameraEvent>,
    ) -> CameraResult<Self> {
        if configuration.surface.id == 0 {
            return Err(CameraError::surface(
                "CameraSession::open",
                "surface id must be non-zero",
            ));
        }
        let mut instance = CameraInstanceLease::acquire()?;
        let (device, info, capabilities) =
            CameraDevice::open(configuration, events, &mut instance)?;
        let controls = CameraControls::query(&device);
        Ok(Self {
            device,
            _instance: instance,
            info,
            capabilities,
            controls,
            flash_mode_before_torch: None,
        })
    }

    pub fn info(&self) -> CameraSessionInfo {
        self.info
    }

    pub fn capabilities(&self) -> CameraCapabilities {
        self.capabilities.clone()
    }

    pub fn controls(&self) -> CameraControls {
        self.controls.clone()
    }

    pub fn capture(&self) -> CameraResult<()> {
        self.photo_output("OH_PhotoOutput_Capture")?.capture()
    }

    pub fn capture_with_options(&self, options: CameraCaptureOptions) -> CameraResult<()> {
        self.photo_output("OH_PhotoOutput_Capture_WithCaptureSetting")?
            .capture_with_options(options)
    }

    pub fn set_flash_mode(&mut self, mode: CameraFlashMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_flash_modes.contains(&mode),
            "OH_CaptureSession_SetFlashMode",
            "flash mode is not supported",
        )?;
        self.device.session().set_flash_mode(mode.into())?;
        self.controls.flash_mode = Some(mode);
        self.controls.torch_mode = if mode == CameraFlashMode::AlwaysOn {
            CameraTorchMode::On
        } else {
            CameraTorchMode::Off
        };
        self.flash_mode_before_torch = None;
        Ok(())
    }

    pub fn set_torch_mode(&mut self, mode: CameraTorchMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.torch_supported,
            "OH_CameraManager_SetTorchMode",
            "torch is not supported",
        )?;
        let session_flash_mode = match mode {
            CameraTorchMode::On => Some(CameraFlashMode::AlwaysOn),
            CameraTorchMode::Off => {
                Some(self.flash_mode_before_torch.unwrap_or(CameraFlashMode::Off))
            }
            CameraTorchMode::Auto => None,
        };
        if let Some(flash_mode) = session_flash_mode
            .filter(|flash_mode| self.controls.supported_flash_modes.contains(flash_mode))
        {
            if mode == CameraTorchMode::On && self.controls.torch_mode != CameraTorchMode::On {
                self.flash_mode_before_torch = self.controls.flash_mode;
            }
            self.device.session().set_flash_mode(flash_mode.into())?;
            self.controls.flash_mode = Some(flash_mode);
            self.controls.torch_mode = mode;
            if mode == CameraTorchMode::Off {
                self.flash_mode_before_torch = None;
            }
            return Ok(());
        }
        self.device.manager().set_torch_mode(mode.into())?;
        self.controls.torch_mode = mode;
        Ok(())
    }

    pub fn set_zoom(&mut self, ratio: f32, smooth: bool) -> CameraResult<()> {
        Self::ensure_in_range(
            ratio,
            self.controls.zoom_range,
            "OH_CaptureSession_SetZoomRatio",
        )?;
        self.device.session().set_zoom(ratio, smooth)?;
        self.controls.zoom = ratio;
        Ok(())
    }

    pub fn set_exposure_mode(&mut self, mode: CameraExposureMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_exposure_modes.contains(&mode),
            "OH_CaptureSession_SetExposureMode",
            "exposure mode is not supported",
        )?;
        self.device.session().set_exposure_mode(mode.into())?;
        self.controls.exposure_mode = Some(mode);
        Ok(())
    }

    pub fn set_exposure_bias(&mut self, bias: f32) -> CameraResult<()> {
        Self::ensure_in_range(
            bias,
            self.controls.exposure_bias_range,
            "OH_CaptureSession_SetExposureBias",
        )?;
        self.device.session().set_exposure_bias(bias)?;
        self.controls.exposure_bias = bias;
        Ok(())
    }

    pub fn set_metering_point(&mut self, point: CameraPoint) -> CameraResult<()> {
        Self::validate_point(point, "OH_CaptureSession_SetMeteringPoint")?;
        self.device
            .session()
            .set_metering_point(CameraControls::native_point(point))?;
        self.controls.metering_point = point;
        Ok(())
    }

    pub fn set_focus_mode(&mut self, mode: CameraFocusMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_focus_modes.contains(&mode),
            "OH_CaptureSession_SetFocusMode",
            "focus mode is not supported",
        )?;
        self.device.session().set_focus_mode(mode.into())?;
        self.controls.focus_mode = Some(mode);
        Ok(())
    }

    pub fn set_focus_point(&mut self, point: CameraPoint) -> CameraResult<()> {
        Self::validate_point(point, "OH_CaptureSession_SetFocusPoint")?;
        self.device
            .session()
            .set_focus_point(CameraControls::native_point(point))?;
        self.controls.focus_point = point;
        Ok(())
    }

    pub fn set_stabilization_mode(&mut self, mode: CameraStabilizationMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_stabilization_modes.contains(&mode),
            "OH_CaptureSession_SetVideoStabilizationMode",
            "stabilization mode is not supported",
        )?;
        self.device.session().set_stabilization_mode(mode.into())?;
        self.controls.stabilization_mode = Some(mode);
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub fn set_white_balance_mode(&mut self, mode: CameraWhiteBalanceMode) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_white_balance_modes.contains(&mode),
            "OH_CaptureSession_SetWhiteBalanceMode",
            "white balance mode is not supported",
        )?;
        self.device.session().set_white_balance_mode(mode.into())?;
        self.controls.white_balance_mode = Some(mode);
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub fn set_white_balance_temperature(&mut self, temperature: i32) -> CameraResult<()> {
        let (min, max) = self.controls.white_balance_temperature_range;
        Self::ensure_supported(
            temperature >= min && temperature <= max,
            "OH_CaptureSession_SetWhiteBalance",
            "white balance temperature is outside the supported range",
        )?;
        self.device.session().set_white_balance(temperature)?;
        self.controls.white_balance_temperature = temperature;
        Ok(())
    }

    #[cfg(feature = "api-19")]
    pub fn set_macro_enabled(&mut self, enabled: bool) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.macro_supported,
            "OH_CaptureSession_EnableMacro",
            "macro mode is not supported",
        )?;
        self.device.session().set_macro_enabled(enabled)?;
        self.controls.macro_enabled = enabled;
        Ok(())
    }

    #[cfg(feature = "api-13")]
    pub fn set_auto_device_switch_enabled(&mut self, enabled: bool) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.auto_device_switch_supported,
            "OH_CaptureSession_EnableAutoDeviceSwitch",
            "automatic device switching is not supported",
        )?;
        self.device
            .session()
            .set_auto_device_switch_enabled(enabled)?;
        self.controls.auto_device_switch_enabled = enabled;
        Ok(())
    }

    #[cfg(feature = "api-14")]
    pub fn set_quality_priority(&mut self, priority: CameraQualityPriority) -> CameraResult<()> {
        self.device
            .session()
            .set_quality_priority(priority.into())?;
        self.controls.quality_priority = priority;
        Ok(())
    }

    pub fn set_frame_rate(&mut self, range: CameraFrameRateRange) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_frame_rates.contains(&range),
            "OH_PreviewOutput_SetFrameRate",
            "frame-rate range is not supported",
        )?;
        self.device
            .preview_output()
            .set_frame_rate(range.min as i32, range.max as i32)?;
        self.controls.frame_rate = range;
        Ok(())
    }

    pub fn set_preview_rotation(&mut self, rotation: CameraImageRotation) -> CameraResult<()> {
        self.device.preview_output().set_rotation(rotation.into())
    }

    pub fn set_color_space(&mut self, color_space: u32) -> CameraResult<()> {
        Self::ensure_supported(
            self.controls.supported_color_spaces.contains(&color_space),
            "OH_CaptureSession_SetActiveColorSpace",
            "color space is not supported",
        )?;
        self.device.session().set_active_color_space(color_space)?;
        self.controls.active_color_space = color_space;
        Ok(())
    }

    fn photo_output(&self, operation: &'static str) -> CameraResult<&crate::native::PhotoOutput> {
        self.device.photo_output().ok_or_else(|| {
            CameraError::unsupported(
                operation,
                "the active camera does not expose JPEG photo capture",
            )
        })
    }

    fn ensure_supported(
        supported: bool,
        operation: &'static str,
        message: &'static str,
    ) -> CameraResult<()> {
        if supported {
            Ok(())
        } else {
            Err(CameraError::unsupported(operation, message))
        }
    }

    fn ensure_in_range(
        value: f32,
        range: CameraFloatRange,
        operation: &'static str,
    ) -> CameraResult<()> {
        if value.is_finite() && value >= range.min && value <= range.max {
            Ok(())
        } else {
            Err(CameraError::invalid_state(
                operation,
                format!(
                    "value {value} is outside supported range {}..={}",
                    range.min, range.max
                ),
            ))
        }
    }

    fn validate_point(point: CameraPoint, operation: &'static str) -> CameraResult<()> {
        if point.x.is_finite()
            && point.y.is_finite()
            && (0.0..=1.0).contains(&point.x)
            && (0.0..=1.0).contains(&point.y)
        {
            Ok(())
        } else {
            Err(CameraError::invalid_state(
                operation,
                "camera point coordinates must be finite values in 0.0..=1.0",
            ))
        }
    }
}
