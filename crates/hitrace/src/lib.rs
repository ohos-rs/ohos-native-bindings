//! Safe Rust bindings for OpenHarmony **HiTrace**.
//!
//! The native `hitrace/trace.h` API covers two unrelated things, and this crate
//! keeps them apart:
//!
//! * **HiTraceMeter** — trace slices and counters consumed by the system trace
//!   tooling. Slices are RAII: [`start_trace`] and [`start_async_trace`] return
//!   guards that close the slice when dropped.
//! * **HiTraceChain** — a chain id propagated across threads, processes and
//!   devices. [`TraceId`] is a plain `Copy` value; [`begin_chain`] installs one
//!   in thread-local storage until the returned guard is dropped.
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! # Example
//!
//! ```no_run
//! use ohos_hitrace_binding as hitrace;
//!
//! let _chain = hitrace::begin_chain("sync_photos", hitrace::TraceFlags::INCLUDE_ASYNC)?;
//!
//! {
//!     let _slice = hitrace::start_trace("read_index")?;
//!     // ... traced work ...
//! }
//!
//! let upload = hitrace::start_async_trace("upload", 1)?;
//! hitrace::count_trace("pending_uploads", 1)?;
//! drop(upload);
//! # Ok::<(), hitrace::HiTraceError>(())
//! ```

pub use ohos_hitrace_sys as sys;

mod chain;
mod error;
mod meter;

pub use chain::*;
pub use error::{HiTraceError, Result};
pub use meter::*;
