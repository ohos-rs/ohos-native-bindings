//! Safe Rust bindings for the OpenHarmony **TEE Client API**.
//!
//! The TEE Client API is the GlobalPlatform interface a normal-world client
//! application uses to talk to a trusted application running in the secure
//! world: connect to a TEE, open a session with a trusted application, invoke
//! commands on it, and share memory with it.
//!
//! Its C form is built on three pairs of calls that must be balanced, and
//! unwound in the right order: `TEEC_InitializeContext` / `TEEC_FinalizeContext`,
//! `TEEC_OpenSession` / `TEEC_CloseSession`, and
//! `TEEC_RegisterSharedMemory` or `TEEC_AllocateSharedMemory` /
//! `TEEC_ReleaseSharedMemory`. Every session and every shared memory block is
//! threaded into a list inside its context, so finalising a context that still
//! owns either corrupts those lists.
//!
//! This crate turns that discipline into types:
//!
//! - each pair is an RAII type — [`Context`], [`Session`], [`SharedMemory`] —
//!   that releases its half on drop, so nothing leaks on an early return;
//! - [`Session`] and [`SharedMemory`] borrow their [`Context`], so the borrow
//!   checker rejects a program that would finalise a context while a session or
//!   a shared memory block is still open. The C ordering rule becomes a compile
//!   error rather than a runtime corruption;
//! - a registered [`SharedMemory`] also keeps the borrow of the buffer it was
//!   built from, so that buffer cannot be reused while the TEE can reach it.
//!
//! Operation parameters get the same treatment. In C, `TEEC_Parameter` is a
//! union whose active member is selected by a 4-bit code the caller packs by
//! hand into `TEEC_Operation::paramTypes`. Here [`Parameter`] is a tagged enum:
//! choosing a variant fixes both the payload and the type code, and
//! [`Operation`] derives the packed field on its own.
//!
//! Errors are [`Result`]-based. [`TeeError`] classifies the `TEEC_Result` code
//! as an [`ErrorKind`] and, for the calls that report one, records the
//! [`ReturnOrigin`] identifying the layer that detected the failure.
//!
//! The whole API arrived in API 20, so everything here is behind the `api-20`
//! feature. The raw bindings are re-exported as [`sys`].
//!
//! See [`Context`] for a worked example.

pub use ohos_tee_client_sys as sys;

#[cfg(feature = "api-20")]
mod context;
#[cfg(feature = "api-20")]
mod error;
#[cfg(feature = "api-20")]
mod memory;
#[cfg(feature = "api-20")]
mod param;
#[cfg(feature = "api-20")]
mod types;

#[cfg(feature = "api-20")]
pub use context::{Context, Session};
#[cfg(feature = "api-20")]
pub use error::{ErrorKind, Result, ReturnOrigin, TeeError};
#[cfg(feature = "api-20")]
pub use memory::SharedMemory;
#[cfg(feature = "api-20")]
pub use param::{Operation, Parameter, SharedMemoryRef, PARAM_COUNT};
#[cfg(feature = "api-20")]
pub use types::{Direction, Login, Uuid};
