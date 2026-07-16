use std::sync::Arc;

use ohos_camera_sys::*;
use ohos_enum_derive::EnumFrom;

use crate::CameraError;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_Position, "Camera_Position_CAMERA_POSITION_")]
pub enum CameraPosition {
    Front,
    #[default]
    Back,
}

impl CameraPosition {
    pub fn opposite(self) -> Self {
        match self {
            Self::Front => Self::Back,
            Self::Back => Self::Front,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CameraSize {
    pub width: u32,
    pub height: u32,
}

impl CameraSize {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CameraSurface {
    pub id: u64,
    pub size: CameraSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraConfiguration {
    pub position: CameraPosition,
    pub surface: CameraSurface,
    pub preview_size: Option<CameraSize>,
    /// Whether the session should reserve a JPEG photo stream.
    ///
    /// Analysis-only sessions should disable this to avoid asking CameraKit
    /// for an unnecessary third concurrent stream.
    pub enable_photo_output: bool,
    pub photo_size: Option<CameraSize>,
    pub frame_output: Option<CameraFrameOutputConfiguration>,
}

impl Default for CameraConfiguration {
    fn default() -> Self {
        Self {
            position: CameraPosition::default(),
            surface: CameraSurface::default(),
            preview_size: None,
            enable_photo_output: true,
            photo_size: None,
            frame_output: None,
        }
    }
}

/// Optional Y-luma frame stream used by real-time analysis such as scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraFrameOutputConfiguration {
    pub size: Option<CameraSize>,
    pub capacity: u32,
    pub max_frames_per_second: u8,
}

impl Default for CameraFrameOutputConfiguration {
    fn default() -> Self {
        Self {
            size: None,
            capacity: 8,
            max_frames_per_second: 10,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CameraCapabilities {
    pub position: CameraPosition,
    pub preview_sizes: Arc<[CameraSize]>,
    pub photo_sizes: Arc<[CameraSize]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CameraSessionInfo {
    pub position: CameraPosition,
    pub preview_size: CameraSize,
    pub photo_size: Option<CameraSize>,
    pub frame_size: Option<CameraSize>,
}

/// Owned luminance plane copied from an analysis output.
#[derive(Clone, PartialEq, Eq)]
pub struct CameraFrame {
    luma: Arc<[u8]>,
    pub size: CameraSize,
    pub timestamp_ns: i64,
}

impl CameraFrame {
    #[cfg(feature = "api-20")]
    pub(crate) fn new(luma: Vec<u8>, size: CameraSize, timestamp_ns: i64) -> Self {
        Self {
            luma: luma.into(),
            size,
            timestamp_ns,
        }
    }

    pub fn luma(&self) -> &[u8] {
        &self.luma
    }

    pub fn shared_luma(&self) -> Arc<[u8]> {
        self.luma.clone()
    }
}

impl std::fmt::Debug for CameraFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CameraFrame")
            .field("byte_len", &self.luma.len())
            .field("size", &self.size)
            .field("timestamp_ns", &self.timestamp_ns)
            .finish()
    }
}

impl CameraSessionInfo {
    pub const fn supports_photo(self) -> bool {
        self.photo_size.is_some()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, EnumFrom)]
#[config(Camera_FocusState, "Camera_FocusState_FOCUS_STATE_")]
pub enum CameraFocusState {
    #[suffix("SCAN")]
    Scanning,
    Focused,
    #[default]
    Unfocused,
}

#[derive(Clone, PartialEq, Eq)]
pub struct CapturedPhoto {
    bytes: Arc<[u8]>,
    pub size: CameraSize,
    pub timestamp_ns: i64,
}

impl CapturedPhoto {
    pub(crate) fn new(bytes: Vec<u8>, size: CameraSize, timestamp_ns: i64) -> Self {
        Self {
            bytes: bytes.into(),
            size,
            timestamp_ns,
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn shared_bytes(&self) -> Arc<[u8]> {
        self.bytes.clone()
    }

    pub const fn mime_type(&self) -> &'static str {
        "image/jpeg"
    }
}

impl std::fmt::Debug for CapturedPhoto {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CapturedPhoto")
            .field("byte_len", &self.bytes.len())
            .field("size", &self.size)
            .field("timestamp_ns", &self.timestamp_ns)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum CameraEvent {
    Photo(CapturedPhoto),
    Frame(CameraFrame),
    FocusState(CameraFocusState),
    Error(CameraError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_position_uses_generated_sys_conversion() {
        let raw: Camera_Position = CameraPosition::Front.into();
        assert_eq!(raw, Camera_Position_CAMERA_POSITION_FRONT);
        assert_eq!(
            CameraPosition::try_from_raw(Camera_Position_CAMERA_POSITION_BACK),
            Some(CameraPosition::Back)
        );
        assert_eq!(CameraPosition::try_from_raw(u32::MAX), None);
    }

    #[test]
    fn focus_state_alias_maps_scanning_to_native_scan() {
        let raw: Camera_FocusState = CameraFocusState::Scanning.into();
        assert_eq!(raw, Camera_FocusState_FOCUS_STATE_SCAN);
        assert_eq!(
            CameraFocusState::try_from_raw(Camera_FocusState_FOCUS_STATE_UNFOCUSED),
            Some(CameraFocusState::Unfocused)
        );
    }
}
