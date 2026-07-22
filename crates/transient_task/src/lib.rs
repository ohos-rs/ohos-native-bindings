//! Safe Rust bindings for OpenHarmony **transient tasks**
//! (`transient_task_api.h`), the short background grace period an application
//! asks for when it is about to be suspended.
//!
//! An application that still has a brief piece of work to finish requests a
//! delayed transition to the suspended state; the system grants a duration of
//! its own choosing and calls back shortly before it runs out. The request must
//! be cancelled when the work is done, which here is the job of the
//! [`SuspendDelay`] guard.
//!
//! # Requirements
//!
//! - **Feature `api-13`.** The whole native API is `@since 13`, so with default
//!   features this crate exposes nothing but [`sys`]. The application info
//!   query is `@since 20` and sits behind `api-20`.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.

pub use ohos_transient_task_sys as sys;

#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-20")]
mod info;
#[cfg(feature = "api-13")]
mod task;

#[cfg(feature = "api-13")]
pub use error::{describe, Result, TransientTaskError};
#[cfg(feature = "api-20")]
pub use info::{transient_task_info, TransientTaskInfo, MAX_TRANSIENT_TASKS};
#[cfg(feature = "api-13")]
pub use task::{DelaySuspendInfo, ExpiredCallback, SuspendDelay};
