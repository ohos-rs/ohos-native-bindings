//! Safe Rust bindings for OpenHarmony **hidebug** (application self-inspection).
//!
//! hidebug lets an application inspect itself at runtime: CPU and memory usage,
//! in-process trace capture, backtraces with symbolization, crash-context
//! attachments and the resource profiler. This crate wraps the native
//! `hidebug.h` C API with a safe layer.
//!
//! Everything introduced after API 12 sits behind the matching `api-*` feature.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.
//!
//! # Example
//!
//! ```no_run
//! use ohos_hidebug_binding as hidebug;
//!
//! let mem = hidebug::app_native_mem_info();
//! println!("pss={} rss={}", mem.pss, mem.rss);
//!
//! let capture = hidebug::start_app_trace_capture(
//!     hidebug::TraceFlag::MainThread,
//!     hidebug::trace_tag::ARKUI | hidebug::trace_tag::GRAPHICS,
//!     10 * 1024 * 1024,
//! )?;
//! println!("tracing into {}", capture.file_name());
//! capture.stop()?;
//! # Ok::<(), hidebug::HiDebugError>(())
//! ```
//!
//! # Not covered
//!
//! `OH_HiDebug_SetMallocDispatchTable` and its companions replace the
//! process-wide allocator; they cannot be reconciled with Rust's own allocator
//! behind a safe API and are reachable through [`sys`] only.
//!
//! Callbacks other than symbolization take no user-data pointer, so they are
//! exposed as `extern "C" fn` rather than closures.

pub use ohos_hidebug_sys as sys;

mod cpu;
mod error;
mod memory;
mod trace;
mod r#type;

pub mod trace_tag;

#[cfg(feature = "api-20")]
mod backtrace;
#[cfg(feature = "api-23")]
mod crash;
#[cfg(feature = "api-24")]
mod profiler;
#[cfg(feature = "api-22")]
mod sampling;

pub use cpu::*;
pub use error::{describe, HiDebugError, Result};
pub use memory::*;
pub use r#type::*;
pub use trace::*;

#[cfg(feature = "api-20")]
pub use backtrace::*;
#[cfg(feature = "api-23")]
pub use crash::*;
#[cfg(feature = "api-24")]
pub use profiler::*;
#[cfg(feature = "api-22")]
pub use sampling::*;
