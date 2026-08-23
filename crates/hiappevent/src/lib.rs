//! Safe Rust bindings for OpenHarmony **HiAppEvent** (application event logging).
//!
//! HiAppEvent records application events into a local event file: custom events
//! written by the application, and OS events such as crashes, freezes and jank.
//! An application can subscribe to them with a [`Watcher`], tune how the OS
//! collects them with an `EventConfig` (api-15), and upload them through a
//! `Processor` (api-18).
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! # Example
//!
//! ```no_run
//! use ohos_hiappevent_binding as hiappevent;
//! use hiappevent::{event, param, AppEventType, EventParams};
//!
//! let params = EventParams::builder()
//!     .add(param::USER_ID, 1234_i64)
//!     .add("page", "home")
//!     .build()?;
//!
//! hiappevent::write("my_domain", event::USER_LOGIN, AppEventType::Behavior, &params)?;
//! # Ok::<(), hiappevent::HiAppEventError>(())
//! ```
//!
//! # Callbacks
//!
//! The watcher callbacks are plain `extern "C" fn`s. The NDK hands them no user
//! data, so a closure would have nowhere to keep its captures; the raw arguments
//! can be turned into owned values with [`app_event_groups`] and
//! [`taken_events`].

pub use ohos_hiappevent_sys as sys;

#[cfg(feature = "api-15")]
mod config;
mod constant;
mod error;
mod params;
#[cfg(feature = "api-18")]
mod processor;
mod r#type;
mod watcher;
mod write;

use std::ffi::c_int;

pub use constant::{config_item, domain, event, param};
pub use error::{describe, HiAppEventError, Result};
pub use params::{EventParams, EventParamsBuilder, EventValue, IntoEventValue};
#[cfg(feature = "api-18")]
pub use processor::{remove_processor, Processor};
pub use r#type::AppEventType;
pub use watcher::{
    app_event_groups, taken_events, AppEventGroup, AppEventInfo, OnReceive, OnTake, OnTrigger,
    Watcher,
};
pub use write::{clear_data, configure, set_logging_disabled, set_max_storage, write};

#[cfg(feature = "api-15")]
pub use config::EventConfig;

/// Array lengths cross the boundary as a C `int`; the assert catches a
/// truncating cast in debug builds.
pub(crate) fn array_len(len: usize) -> c_int {
    debug_assert!(
        len <= c_int::MAX as usize,
        "array too long for a C int length"
    );
    len as c_int
}
