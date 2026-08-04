#![allow(dead_code)]
//! ArkUI Rust bindings and high-level wrappers.
//!
//! This crate is organized by capability modules instead of exposing every
//! symbol at the crate root.

/// Animation types and helpers.
pub mod animate;
/// ArkUI native API wrappers.
pub mod api;
/// Core shared types such as node handles and errors.
pub mod common;
/// Component builders and attribute traits.
pub mod component;
/// Dialog abstractions.
pub mod dialog;
/// Event wrappers.
pub mod event;
/// Gesture builders and gesture data.
pub mod gesture;
/// ArkUI enums and value types.
pub mod r#type;

/// Alias for [`r#type`] with a non-keyword module name.
pub use r#type as types;

/// Re-export of the low-level arkui input binding crate.
pub use ohos_arkui_input_binding as arkui_input_binding;
/// Re-export of the image binding crate.
#[cfg(feature = "image")]
pub use ohos_image_binding as image_binding;
/// Re-export of the image-native binding crate.
#[cfg(feature = "image")]
pub use ohos_image_native_binding as image_native_binding;

pub use common::handle::ArkUIHandle;
pub use component::built_in_component::XComponent;
pub use component::root::RootNode;

// Internal compatibility re-exports for in-crate paths.
pub(crate) use animate::*;
pub(crate) use api::*;
pub(crate) use common::*;
pub(crate) use component::*;
pub(crate) use dialog::*;
pub(crate) use event::*;
pub(crate) use gesture::*;
pub(crate) use r#type::*;
