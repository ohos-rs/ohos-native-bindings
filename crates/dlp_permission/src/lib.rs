//! Safe Rust bindings for the OpenHarmony **DLP permission** API
//! (Data Protection Kit).
//!
//! Data loss prevention (DLP) files are opened by the system in a dedicated
//! sandbox application. This crate wraps the native `dlp_permission_api.h` C
//! API with a safe layer: a sandbox application can read the permission granted
//! on the file it opened, and a regular application can manage the
//! configuration handed to the sandbox.
//!
//! The whole native API is `@since 14`, so every item of this crate sits behind
//! the `api-14` feature. Without it the crate exposes only the raw bindings,
//! re-exported as [`sys`].
//!
//! # Memory ownership
//!
//! `OH_DLP_GetOriginalFileName` and `OH_DLP_GetSandboxAppConfig` hand out
//! `malloc`-allocated buffers, but the DLP library exports no matching
//! deallocator. This crate copies the content into a `String` and releases the
//! native buffer with the C library `free`, matching the official NDK usage
//! guide. Callers never see a raw buffer.
//!
//! # Example
//!
//! ```no_run
//! use ohos_dlp_permission_binding as dlp;
//!
//! if dlp::is_in_sandbox()? {
//!     let info = dlp::permission_info()?;
//!     if info.actions.contains(dlp::Actions::PRINT) {
//!         println!("printing allowed, access: {:?}", info.access);
//!     }
//! } else {
//!     dlp::set_sandbox_app_config("{\"key\":\"value\"}")?;
//! }
//!
//! let original = dlp::original_file_name("report.docx.dlp")?;
//! println!("original file name: {original}");
//! # Ok::<(), dlp::DlpError>(())
//! ```

pub use ohos_dlp_permission_sys as sys;

#[cfg(feature = "api-14")]
mod api;
#[cfg(feature = "api-14")]
mod error;
#[cfg(feature = "api-14")]
mod types;

#[cfg(feature = "api-14")]
pub use api::{
    clean_sandbox_app_config, is_in_sandbox, original_file_name, permission_info,
    sandbox_app_config, set_sandbox_app_config,
};
#[cfg(feature = "api-14")]
pub use error::{describe, DlpError, Result};
#[cfg(feature = "api-14")]
pub use types::{Actions, FileAccess, PermissionInfo};
