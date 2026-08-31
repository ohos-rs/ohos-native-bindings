//! Safe wrappers for ArkUI's native accessibility provider API.

#[cfg(feature = "api-13")]
mod element;
#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-13")]
mod provider;
#[cfg(feature = "api-13")]
mod types;

#[cfg(feature = "api-13")]
pub use element::*;
#[cfg(feature = "api-13")]
pub use error::*;
#[cfg(feature = "api-13")]
pub use provider::*;
#[cfg(feature = "api-13")]
pub use types::*;

pub use ohos_accessibility_sys as sys;
