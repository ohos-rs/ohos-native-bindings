//! Safe Rust bindings for the OpenHarmony **location** module.
//!
//! The native `oh_location.h` C API of LocationKit is a subscription API: the
//! application describes what kind of fixes it wants in a request parameter
//! instance, attaches a reporting callback, starts locating and later stops it
//! again with the same instance. This crate wraps that API with a safe layer —
//! an RAII `RequestConfig` with a builder, an RAII `LocationSession` that
//! accepts a Rust closure, owned `LocationInfo` fixes and `Result` based
//! error handling.
//!
//! The whole surface was introduced in API 13 and therefore sits behind the
//! `api-13` feature; with the default feature set this crate only re-exports
//! the raw bindings as [`sys`], which also covers anything not wrapped here.
//!
//! # Permissions
//!
//! `oh_location.h` declares `@permission ohos.permission.APPROXIMATELY_LOCATION`
//! on `OH_Location_StartLocating` and `OH_Location_StopLocating`; both report
//! `LOCATION_PERMISSION_DENIED` when it is missing. Applications that want
//! precise fixes request `ohos.permission.LOCATION` in addition, which the
//! system only grants together with the approximate one. Querying the switch
//! with `is_locating_enabled` needs no permission.
//!
//! Beyond the permission, a fix also requires the user to have the system
//! location switch on: otherwise starting reports `LOCATION_SWITCH_OFF`, for
//! which `LocationError::is_switch_off` holds.

pub use ohos_location_sys as sys;

#[cfg(feature = "api-13")]
mod config;
#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-13")]
mod info;
#[cfg(feature = "api-13")]
mod session;

#[cfg(feature = "api-13")]
pub use config::{
    LocationPowerConsumptionScene, LocationUseScene, RequestConfig, RequestConfigBuilder,
};
#[cfg(feature = "api-13")]
pub use error::{describe, LocationError, Result};
#[cfg(feature = "api-13")]
pub use info::{LocationInfo, LocationSourceType};
#[cfg(feature = "api-13")]
pub use session::{is_locating_enabled, LocationSession};
