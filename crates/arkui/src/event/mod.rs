//! Event wrappers used by ArkUI callbacks.

pub mod inner_event;
#[cfg(feature = "api-14")]
mod key_event;

#[cfg(feature = "api-14")]
pub use key_event::{KeyCode, KeyEvent, KeyEventType, KeyIntention, KeySource};

pub(crate) use inner_event::*;
