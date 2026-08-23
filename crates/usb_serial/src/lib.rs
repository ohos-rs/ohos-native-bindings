//! Safe Rust bindings for the OpenHarmony **USB Serial DDK**
//! (`usb_serial_api.h`), which drives USB CDC-ACM serial devices from native
//! code: open an interface, configure the line, and read/write bytes.
//!
//! # Requirements
//!
//! - **Feature `api-18`.** The whole native API is `@since 18`, so with default
//!   features this crate exposes nothing but [`sys`].
//! - **Permission `ohos.permission.ACCESS_DDK_USB_SERIAL`.** It is
//!   `system_grant` at `system_basic` level with `provisionEnable`, so there is
//!   no runtime prompt and an ordinary application cannot obtain it — the
//!   signing profile must carry it in its ACL list (driver-package scenarios).
//!   Applications outside that scope should use the ArkTS `usbManager` API,
//!   which is a separate stack with its own per-device user authorization.
//!
//! Device ids come from the USB DDK (`OH_Usb_GetDevices`); this crate does not
//! enumerate devices.
//!
//! # Blocking
//!
//! The native API has no callbacks and no asynchronous variants: every call
//! blocks the calling thread. Drive the device from a worker thread or a
//! TaskPool task, never from the UI thread.
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.

pub use ohos_usb_serial_sys as sys;

#[cfg(feature = "api-18")]
mod device;
#[cfg(feature = "api-18")]
mod error;
#[cfg(feature = "api-18")]
mod r#type;

#[cfg(feature = "api-18")]
pub use device::{Ddk, SerialDevice};
#[cfg(feature = "api-18")]
pub use error::{describe, Result, UsbSerialError};
#[cfg(feature = "api-18")]
pub use r#type::{FlowControl, Parity, SerialParams, StopBits, Timeout};
