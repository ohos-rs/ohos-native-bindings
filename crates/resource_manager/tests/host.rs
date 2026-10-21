//! Production bounded-read regressions without an OHOS linker dependency.

#[path = "../src/error.rs"]
mod error;
pub use error::RawFileError;
#[path = "../src/bounded_read.rs"]
mod bounded_read;
