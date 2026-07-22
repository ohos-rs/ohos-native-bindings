//! Safe Rust bindings for the OpenHarmony **native effect** filters.
//!
//! An [`Filter`] is created from a source pixel map, effects are added to it in
//! order, and the result is rendered into a new pixel map. The native filter
//! object is released when the [`Filter`] is dropped.
//!
//! Pixel maps belong to the image bindings, not to this crate. They cross the
//! boundary as [`PixelMapHandle`], a borrowed pointer view without `Drop`: the
//! source pixel map stays owned by the caller, and the pixel map returned by
//! [`Filter::effect_pixel_map`] is a newly created object the caller then owns
//! and has to release through the image bindings.
//!
//! Everything introduced after API 12 sits behind the matching `api-*` feature;
//! the tile-mode blur needs `api-14`.
//!
//! The raw bindings are re-exported as [`sys`].
//!
//! # Example
//!
//! ```no_run
//! use ohos_native_effect_binding::{ColorMatrix, Filter, PixelMapHandle};
//! # use std::ffi::c_void;
//! # let source: *mut c_void = std::ptr::null_mut();
//!
//! // `source` is a native pixel map pointer owned by the caller.
//! let handle = unsafe { PixelMapHandle::from_raw(source) }.expect("null pixel map");
//!
//! let mut filter = Filter::create(handle)?;
//! filter.blur(10.0)?;
//! filter.brighten(0.5)?;
//! filter.set_color_matrix(&ColorMatrix::IDENTITY)?;
//!
//! // The result is a new pixel map; releasing it is up to the caller.
//! let result = filter.effect_pixel_map()?;
//! println!("effect pixel map at {:?}", result.as_raw());
//! # Ok::<(), ohos_native_effect_binding::EffectError>(())
//! ```

pub use ohos_native_effect_sys as sys;

mod error;
mod filter;
mod types;

pub use error::{describe, EffectError, Result};
pub use filter::Filter;
#[cfg(feature = "api-14")]
pub use types::TileMode;
pub use types::{ColorMatrix, PixelMapHandle, COLOR_MATRIX_LEN};
