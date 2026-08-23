//! Safe Rust bindings for the OpenHarmony **ability base** want API.
//!
//! A want describes which ability to reach — the bundle, module and ability
//! name — plus the parameters handed to it. The native `want.h` C API creates
//! the want with one call and destroys it with another, stores parameters
//! through key/value setters and reads strings back into caller-provided
//! character arrays. This crate wraps it as an owning [`Want`] that destroys
//! itself on drop, an owned [`Element`] instead of the raw `char*` struct, and
//! getters that manage the output buffer and return `String`.
//!
//! The want API starts at API 15, so everything here sits behind the matching
//! `api-*` feature: [`Want`] and the string, element and file descriptor
//! accessors need `api-15`, the URI and the typed (`i32`, `bool`, `f64`)
//! parameters need `api-17`.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.
//!
//! See [`Want`] for a usage example.

pub use ohos_ability_base_sys as sys;

#[cfg(feature = "api-15")]
mod error;
#[cfg(feature = "api-15")]
mod want;

#[cfg(feature = "api-15")]
pub use error::{describe, AbilityBaseError, Result};
#[cfg(feature = "api-15")]
pub use want::{Element, Want};
