use std::sync::Arc;

use ohos_camera_sys::*;
use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CameraPoint {
    pub x: f64,
    pub y: f64,
}

impl CameraPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CameraFloatRange {
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CameraFrameRateRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_FlashMode, "Camera_FlashMode_FLASH_MODE_")]
pub enum CameraFlashMode {
    #[default]
    #[suffix("CLOSE")]
    Off,
    #[suffix("OPEN")]
    On,
    Auto,
    #[suffix("ALWAYS_OPEN")]
    AlwaysOn,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_TorchMode, "Camera_TorchMode_")]
pub enum CameraTorchMode {
    #[default]
    Off,
    On,
    Auto,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_ExposureMode, "Camera_ExposureMode_EXPOSURE_MODE_")]
pub enum CameraExposureMode {
    Locked,
    Auto,
    #[default]
    ContinuousAuto,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_FocusMode, "Camera_FocusMode_FOCUS_MODE_")]
pub enum CameraFocusMode {
    Manual,
    #[default]
    ContinuousAuto,
    Auto,
    Locked,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(
    Camera_VideoStabilizationMode,
    "Camera_VideoStabilizationMode_STABILIZATION_MODE_"
)]
pub enum CameraStabilizationMode {
    #[default]
    Off,
    Low,
    Middle,
    High,
    Auto,
}

#[cfg_attr(feature = "api-20", derive(EnumFrom))]
#[cfg_attr(
    feature = "api-20",
    config(
        Camera_WhiteBalanceMode,
        "Camera_WhiteBalanceMode_CAMERA_WHITE_BALANCE_MODE_"
    )
)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CameraWhiteBalanceMode {
    #[default]
    Auto,
    Cloudy,
    Incandescent,
    Fluorescent,
    Daylight,
    Manual,
    Locked,
}

#[cfg_attr(feature = "api-14", derive(EnumFrom))]
#[cfg_attr(
    feature = "api-14",
    config(Camera_QualityPrioritization, "Camera_QualityPrioritization_")
)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CameraQualityPriority {
    #[default]
    HighQuality,
    PowerBalance,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_QualityLevel, "Camera_QualityLevel_QUALITY_LEVEL_")]
pub enum CameraPhotoQuality {
    #[default]
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_ImageRotation, "")]
pub enum CameraImageRotation {
    #[default]
    #[alias("Camera_ImageRotation_IAMGE_ROTATION_0")]
    Degrees0,
    #[alias("Camera_ImageRotation_IAMGE_ROTATION_90")]
    Degrees90,
    #[alias("Camera_ImageRotation_IAMGE_ROTATION_180")]
    Degrees180,
    #[alias("Camera_ImageRotation_IAMGE_ROTATION_270")]
    Degrees270,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CameraLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CameraCaptureOptions {
    pub quality: CameraPhotoQuality,
    pub rotation: CameraImageRotation,
    pub mirror: bool,
    pub location: Option<CameraLocation>,
}

/// Live values and supported controls for one configured capture session.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraControls {
    pub has_flash: bool,
    pub flash_mode: Option<CameraFlashMode>,
    pub supported_flash_modes: Arc<[CameraFlashMode]>,
    pub torch_supported: bool,
    pub torch_mode: CameraTorchMode,
    pub zoom: f32,
    pub zoom_range: CameraFloatRange,
    pub exposure_mode: Option<CameraExposureMode>,
    pub supported_exposure_modes: Arc<[CameraExposureMode]>,
    pub exposure_bias: f32,
    pub exposure_bias_range: CameraFloatRange,
    pub exposure_value: f32,
    pub metering_point: CameraPoint,
    pub focus_mode: Option<CameraFocusMode>,
    pub supported_focus_modes: Arc<[CameraFocusMode]>,
    pub focus_point: CameraPoint,
    pub stabilization_mode: Option<CameraStabilizationMode>,
    pub supported_stabilization_modes: Arc<[CameraStabilizationMode]>,
    pub white_balance_mode: Option<CameraWhiteBalanceMode>,
    pub supported_white_balance_modes: Arc<[CameraWhiteBalanceMode]>,
    pub white_balance_temperature: i32,
    pub white_balance_temperature_range: (i32, i32),
    pub macro_supported: bool,
    pub macro_enabled: bool,
    pub auto_device_switch_supported: bool,
    pub auto_device_switch_enabled: bool,
    pub quality_priority: CameraQualityPriority,
    pub focal_length: f32,
    pub frame_rate: CameraFrameRateRange,
    pub supported_frame_rates: Arc<[CameraFrameRateRange]>,
    pub active_color_space: u32,
    pub supported_color_spaces: Arc<[u32]>,
    pub mirror_supported: bool,
}

impl CameraControls {
    pub(crate) fn query(device: &crate::device::CameraDevice) -> Self {
        let session = device.session();
        let mut controls = Self {
            has_flash: session.has_flash(),
            ..Self::default()
        };
        controls.supported_flash_modes = [
            CameraFlashMode::Off,
            CameraFlashMode::On,
            CameraFlashMode::Auto,
            CameraFlashMode::AlwaysOn,
        ]
        .into_iter()
        .filter(|mode| session.supports_flash_mode((*mode).into()))
        .collect::<Vec<_>>()
        .into();
        controls.flash_mode = session.flash_mode().and_then(CameraFlashMode::try_from_raw);
        controls.torch_supported = device.manager().torch_supported()
            || controls
                .supported_flash_modes
                .contains(&CameraFlashMode::AlwaysOn);
        if let Some((min, max)) = session.zoom_range() {
            controls.zoom_range = CameraFloatRange {
                min,
                max,
                step: 0.0,
            };
        }
        controls.zoom = session.zoom().unwrap_or_default();

        controls.supported_exposure_modes = [
            CameraExposureMode::Locked,
            CameraExposureMode::Auto,
            CameraExposureMode::ContinuousAuto,
        ]
        .into_iter()
        .filter(|mode| session.supports_exposure_mode((*mode).into()))
        .collect::<Vec<_>>()
        .into();
        controls.exposure_mode = session
            .exposure_mode()
            .and_then(CameraExposureMode::try_from_raw);
        if let Some((min, max, step)) = session.exposure_bias_range() {
            controls.exposure_bias_range = CameraFloatRange { min, max, step };
        }
        controls.exposure_bias = session.exposure_bias().unwrap_or_default();
        controls.exposure_value = session.exposure_value().unwrap_or_default();
        controls.metering_point = session
            .metering_point()
            .map(Self::camera_point)
            .unwrap_or_default();

        controls.supported_focus_modes = [
            CameraFocusMode::Manual,
            CameraFocusMode::ContinuousAuto,
            CameraFocusMode::Auto,
            CameraFocusMode::Locked,
        ]
        .into_iter()
        .filter(|mode| session.supports_focus_mode((*mode).into()))
        .collect::<Vec<_>>()
        .into();
        controls.focus_mode = session.focus_mode().and_then(CameraFocusMode::try_from_raw);
        controls.focus_point = session
            .focus_point()
            .map(Self::camera_point)
            .unwrap_or_default();

        controls.supported_stabilization_modes = [
            CameraStabilizationMode::Off,
            CameraStabilizationMode::Low,
            CameraStabilizationMode::Middle,
            CameraStabilizationMode::High,
            CameraStabilizationMode::Auto,
        ]
        .into_iter()
        .filter(|mode| session.supports_stabilization_mode((*mode).into()))
        .collect::<Vec<_>>()
        .into();
        controls.stabilization_mode = session
            .stabilization_mode()
            .and_then(CameraStabilizationMode::try_from_raw);

        #[cfg(feature = "api-20")]
        {
            controls.supported_white_balance_modes = [
                CameraWhiteBalanceMode::Auto,
                CameraWhiteBalanceMode::Cloudy,
                CameraWhiteBalanceMode::Incandescent,
                CameraWhiteBalanceMode::Fluorescent,
                CameraWhiteBalanceMode::Daylight,
                CameraWhiteBalanceMode::Manual,
                CameraWhiteBalanceMode::Locked,
            ]
            .into_iter()
            .filter(|mode| session.supports_white_balance_mode((*mode).into()))
            .collect::<Vec<_>>()
            .into();
            controls.white_balance_mode = session
                .white_balance_mode()
                .and_then(CameraWhiteBalanceMode::try_from_raw);
            controls.white_balance_temperature_range =
                session.white_balance_range().unwrap_or_default();
            controls.white_balance_temperature = session.white_balance().unwrap_or_default();
        }

        #[cfg(feature = "api-19")]
        {
            controls.macro_supported = session.macro_supported();
        }
        #[cfg(feature = "api-13")]
        {
            controls.auto_device_switch_supported = session.auto_device_switch_supported();
        }
        controls.focal_length = session.focal_length().unwrap_or_default();
        controls.supported_frame_rates = device
            .preview_output()
            .supported_frame_rates()
            .into_iter()
            .map(|range| CameraFrameRateRange {
                min: range.min,
                max: range.max,
            })
            .collect::<Vec<_>>()
            .into();
        if let Some(range) = device.preview_output().active_frame_rate() {
            controls.frame_rate = CameraFrameRateRange {
                min: range.min,
                max: range.max,
            };
        }
        controls.supported_color_spaces = session.supported_color_spaces().into();
        controls.active_color_space = session.active_color_space().unwrap_or_default();
        controls.mirror_supported = device
            .photo_output()
            .is_some_and(crate::native::PhotoOutput::mirror_supported);
        controls
    }

    pub(crate) fn native_point(point: CameraPoint) -> Camera_Point {
        Camera_Point {
            x: point.x,
            y: point.y,
        }
    }

    fn camera_point(point: Camera_Point) -> CameraPoint {
        CameraPoint::new(point.x, point.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_control_enum_conversions_match_camera_sys() {
        let flash: Camera_FlashMode = CameraFlashMode::AlwaysOn.into();
        let torch: Camera_TorchMode = CameraTorchMode::Auto.into();
        let exposure: Camera_ExposureMode = CameraExposureMode::ContinuousAuto.into();
        let focus: Camera_FocusMode = CameraFocusMode::ContinuousAuto.into();
        let stabilization: Camera_VideoStabilizationMode = CameraStabilizationMode::Middle.into();
        let quality: Camera_QualityLevel = CameraPhotoQuality::Medium.into();
        let rotation: Camera_ImageRotation = CameraImageRotation::Degrees270.into();

        assert_eq!(flash, Camera_FlashMode_FLASH_MODE_ALWAYS_OPEN);
        assert_eq!(torch, Camera_TorchMode_AUTO);
        assert_eq!(exposure, Camera_ExposureMode_EXPOSURE_MODE_CONTINUOUS_AUTO);
        assert_eq!(focus, Camera_FocusMode_FOCUS_MODE_CONTINUOUS_AUTO);
        assert_eq!(
            stabilization,
            Camera_VideoStabilizationMode_STABILIZATION_MODE_MIDDLE
        );
        assert_eq!(quality, Camera_QualityLevel_QUALITY_LEVEL_MEDIUM);
        assert_eq!(rotation, Camera_ImageRotation_IAMGE_ROTATION_270);
    }

    #[test]
    fn generated_control_enum_conversion_rejects_unknown_raw_values() {
        assert_eq!(CameraFlashMode::try_from_raw(u32::MAX), None);
        assert_eq!(CameraTorchMode::try_from_raw(u32::MAX), None);
        assert_eq!(CameraExposureMode::try_from_raw(u32::MAX), None);
        assert_eq!(CameraFocusMode::try_from_raw(u32::MAX), None);
        assert_eq!(CameraStabilizationMode::try_from_raw(u32::MAX), None);
        assert_eq!(CameraPhotoQuality::try_from_raw(u32::MAX), None);
        assert_eq!(CameraImageRotation::try_from_raw(u32::MAX), None);
    }

    #[cfg(feature = "api-14")]
    #[test]
    fn quality_priority_conversion_is_api_gated() {
        let raw: Camera_QualityPrioritization = CameraQualityPriority::PowerBalance.into();
        assert_eq!(raw, Camera_QualityPrioritization_POWER_BALANCE);
    }

    #[cfg(feature = "api-20")]
    #[test]
    fn white_balance_conversion_is_api_gated() {
        let raw: Camera_WhiteBalanceMode = CameraWhiteBalanceMode::Daylight.into();
        assert_eq!(
            raw,
            Camera_WhiteBalanceMode_CAMERA_WHITE_BALANCE_MODE_DAYLIGHT
        );
    }
}
