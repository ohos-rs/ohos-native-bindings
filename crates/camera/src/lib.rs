//! Safe CameraKit wrappers.
//!
//! The core crate owns CameraKit resources and callbacks without depending on
//! a UI framework. Enable `xcomponent` to attach camera surfaces directly to
//! native ArkUI XComponent nodes.

mod callbacks;
mod controls;
mod device;
mod error;
mod image;
mod model;
mod native;
mod profiles;
mod session;
#[cfg(feature = "xcomponent")]
mod xcomponent;

pub use error::{CameraError, CameraErrorKind, CameraResult};
pub use model::{
    CameraCapabilities, CameraConfiguration, CameraEvent, CameraFocusState, CameraFrame,
    CameraFrameOutputConfiguration, CameraPosition, CameraSessionInfo, CameraSize, CameraSurface,
    CapturedPhoto,
};
pub use session::CameraSession;
#[cfg(feature = "xcomponent")]
pub use xcomponent::{CameraXComponentAttachment, CameraXComponentEvent};

pub use controls::{
    CameraCaptureOptions, CameraControls, CameraExposureMode, CameraFlashMode, CameraFloatRange,
    CameraFocusMode, CameraFrameRateRange, CameraImageRotation, CameraLocation, CameraPhotoQuality,
    CameraPoint, CameraQualityPriority, CameraStabilizationMode, CameraTorchMode,
    CameraWhiteBalanceMode,
};
