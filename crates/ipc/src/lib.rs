//! Safe Rust bindings for OpenHarmony **IPC** (inter-process communication).
//!
//! IPC is the HarmonyOS binder-style transport: a service publishes an
//! [`RemoteStub`], a client holds a [`RemoteProxy`] to it and sends it command
//! words together with a serialized [`Parcel`], and the stub answers in a reply
//! parcel. This crate wraps the native `IPCKit` C API with a safe layer — RAII
//! handles, a typed parcel reader/writer, closure-based request and death
//! callbacks, and `Result`-based error handling.
//!
//! Every symbol wrapped here is available since API 12, so no `api-*` feature is
//! needed to use the crate.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.
//!
//! # Example
//!
//! ```no_run
//! use ohos_ipc_binding as ipc;
//! use ipc::{Parcel, RemoteProxy, RemoteStub};
//!
//! const DESCRIPTOR: &str = "org.example.IEcho";
//! const CODE_ECHO: u32 = 1;
//!
//! // Service side: answer requests until the last peer lets go of the object.
//! let stub = RemoteStub::new(DESCRIPTOR, |code, data, reply| {
//!     if code != CODE_ECHO {
//!         return Err(ipc::IpcError::user(1909001));
//!     }
//!     if data.read_interface_token()? != DESCRIPTOR {
//!         return Err(ipc::IpcError::user(1909002));
//!     }
//!     let message = data.read_string()?;
//!     reply.write_string(&message)
//! })?;
//!
//! // Client side, holding a proxy read out of a parcel.
//! fn echo(proxy: &RemoteProxy, message: &str) -> ipc::Result<String> {
//!     let mut data = Parcel::new()?;
//!     data.write_interface_token(DESCRIPTOR)?;
//!     data.write_string(message)?;
//!     let mut reply = Parcel::new()?;
//!     proxy.send_request(CODE_ECHO, &data, &mut reply)?;
//!     reply.read_string()
//! }
//!
//! drop(stub);
//! # Ok::<(), ipc::IpcError>(())
//! ```

pub use ohos_ipc_sys as sys;

mod alloc;
mod error;
mod parcel;
mod remote;
mod skeleton;

pub use error::{describe, IpcError, Result};
pub use parcel::Parcel;
pub use remote::{DeathRecipient, MessageOption, RemoteProxy, RemoteStub, RequestMode};
pub use skeleton::{
    calling_pid, calling_token_id, calling_uid, first_token_id, is_handling_transaction,
    is_local_calling, join_work_thread, reset_calling_identity, reset_calling_identity_scoped,
    self_token_id, set_calling_identity, set_max_work_thread_num, stop_work_thread,
    CallingIdentity, CallingIdentityGuard,
};
