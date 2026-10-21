//! Component definitions and common component attribute traits.

#[cfg(all(feature = "accessibility", feature = "api-23"))]
pub mod accessibility;
pub mod attribute;
pub mod built_in_component;
#[cfg(feature = "napi")]
pub mod content_root;
#[cfg(feature = "napi")]
pub mod native_attachment;
pub mod root;

pub(crate) use attribute::*;
#[allow(unused_imports)]
pub(crate) use built_in_component::*;
#[allow(unused_imports)]
pub(crate) use root::*;
