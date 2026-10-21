#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-13")]
mod pasteboard;

#[cfg(feature = "api-13")]
pub use error::*;

#[cfg(feature = "api-13")]
pub use pasteboard::*;
#[cfg(feature = "napi")]
mod clipboard;
#[cfg(feature = "napi")]
pub use clipboard::Clipboard;
