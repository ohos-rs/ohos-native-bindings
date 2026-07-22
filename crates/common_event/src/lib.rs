//! Safe Rust bindings for the OpenHarmony **common event** service.
//!
//! Common events are the system-wide notifications an application can listen
//! to — the screen turning on, a package being installed, the battery level
//! changing — and, since API 18, publish itself. An application describes what
//! it wants in a [`SubscribeInfo`], creates a [`Subscriber`] for it and starts
//! it with [`subscribe`](Subscriber::subscribe); each event arrives as an
//! [`RcvData`] in the [`ReceiveHandler`].
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! # Example
//!
//! ```no_run
//! use ohos_common_event_binding as common_event;
//! use common_event::{event, RcvData, ReceiveHandler, SubscribeInfo, Subscriber};
//!
//! struct OnPower;
//!
//! impl ReceiveHandler for OnPower {
//!     fn on_receive(data: &RcvData<'_>) {
//!         let name = data.event().unwrap_or_default();
//!         let level = match data.parameters() {
//!             Some(parameters) => parameters.int("soc", -1).unwrap_or(-1),
//!             None => -1,
//!         };
//!         println!("{name}: {level}");
//!     }
//! }
//!
//! let mut info = SubscribeInfo::new(&[event::BATTERY_CHANGED, event::POWER_CONNECTED])?;
//! info.set_publisher_bundle_name("com.example.publisher")?;
//!
//! let mut subscriber = Subscriber::new::<OnPower>(&info)?;
//! subscriber.subscribe()?;
//! // Dropping the subscriber unsubscribes.
//! # Ok::<(), common_event::CommonEventError>(())
//! ```
//!
//! # Ownership
//!
//! [`SubscribeInfo`], [`Subscriber`], `Parameters` and `PublishInfo` own a
//! native handle and release it on drop; a `Subscriber` unsubscribes first, so
//! that the service stops dispatching into the callback before the handle goes
//! away.
//!
//! [`RcvData`] and [`ParametersRef`] are the opposite: the service allocates
//! them just before it calls the handler and frees them as soon as the handler
//! returns, so they are borrowed views with no `Drop`, and their lifetime keeps
//! them from escaping the callback. Values to be kept must be copied out.
//!
//! # Callbacks
//!
//! The native receive callback is a bare C function pointer with no user-data
//! argument, which leaves a closure nowhere to keep its captures. The handler
//! is a type implementing [`ReceiveHandler`] instead, and [`Subscriber::new`]
//! instantiates one trampoline per handler type.

pub use ohos_common_event_sys as sys;

mod error;
mod info;
mod parameters;
#[cfg(feature = "api-18")]
mod publish;
mod subscriber;
mod support;

pub use error::{describe, CommonEventError, Result};
pub use info::SubscribeInfo;
pub use parameters::ParametersRef;
pub use subscriber::{RcvData, ReceiveHandler, Subscriber};
pub use support::event;

#[cfg(feature = "api-18")]
pub use parameters::Parameters;
#[cfg(feature = "api-18")]
pub use publish::{publish, publish_with_info, PublishInfo};
