//! Safe Rust bindings for the OpenHarmony **native color space manager**.
//!
//! A color space manager instance describes a color space, either a standard
//! one selected by [`ColorSpaceName`] or a custom one described by its
//! primaries and gamma. Once created, the instance reports its color space
//! name, white point and gamma; graphics APIs take it to tag surfaces and
//! images with a color space.
//!
//! This crate wraps the native `native_color_space_manager.h` C API with a safe
//! layer: the instance is an owned handle released on drop, the color space
//! enum is a Rust enum, and the documented failure sentinels of the native
//! getters become [`ColorSpaceError`].
//!
//! The whole API was introduced in API 13, so it sits behind the `api-13`
//! feature. See [`ColorSpaceManager`] for a usage example.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.

pub use ohos_native_color_space_manager_sys as sys;

#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-13")]
mod manager;
#[cfg(feature = "api-13")]
mod name;

#[cfg(feature = "api-13")]
pub use error::{ColorSpaceError, Result};
#[cfg(feature = "api-13")]
pub use manager::{Chromaticity, ColorSpaceManager, ColorSpacePrimaries};
#[cfg(feature = "api-13")]
pub use name::ColorSpaceName;
