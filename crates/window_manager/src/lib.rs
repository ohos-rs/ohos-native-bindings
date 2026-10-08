mod error;
#[cfg(feature = "api-26")]
mod frame_metrics;
mod manager;
mod types;
mod window;

#[cfg(feature = "api-24")]
mod density;

#[cfg(feature = "api-24")]
pub use density::DensityInfo;
#[cfg(feature = "api-24")]
pub use density::{DensityInfoChangeCallback, DensityInfoRef, RawDensityInfo};
pub use error::{Error, Result};
#[cfg(feature = "api-26")]
pub use frame_metrics::{FrameMetrics, FrameMetricsMeasuredCallback, RawFrameMetrics};
pub use manager::WindowManager;
#[cfg(feature = "api-21")]
pub use manager::{MainWindowSnapshotCallback, RawPixelMap, WindowSnapshotConfig};
#[cfg(feature = "api-15")]
pub use ohos_image_native_binding::PixelMapNativeHandle;
pub use ohos_native_window_manager_sys::Input_KeyEvent;
#[cfg(feature = "api-15")]
pub use ohos_native_window_manager_sys::{Input_MouseEvent, Input_TouchEvent};
#[cfg(feature = "api-21")]
pub use types::MainWindowInfo;
#[cfg(feature = "api-15")]
pub use types::{AvoidArea, AvoidAreaType, Rect, WindowProperties, WindowType};
pub use window::{KeyEventFilter, Window};
#[cfg(feature = "api-15")]
pub use window::{MouseEventFilter, TouchEventFilter};
