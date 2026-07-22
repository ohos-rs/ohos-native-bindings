use crate::error::{Result, TeeError};
use crate::types::Direction;
use ohos_tee_client_sys as sys;
use std::fmt;
use std::marker::PhantomData;
use std::os::fd::{AsRawFd, BorrowedFd};
use std::os::raw::c_void;

/// How many parameters an operation carries.
pub const PARAM_COUNT: usize = sys::TEEC_PARAM_NUM as usize;

/// An exclusive borrow of a [`SharedMemory`](crate::SharedMemory) block, ready
/// to be placed in a [`Parameter`].
///
/// Obtained from [`SharedMemory::as_param`](crate::SharedMemory::as_param). The
/// borrow is exclusive because
/// the trusted application may write into the block while the operation runs,
/// so no other view of its content may be alive at the same time. As a
/// consequence one block can occupy at most one parameter slot of an operation.
pub struct SharedMemoryRef<'a> {
    raw: *mut sys::TEEC_SharedMemory,
    borrow: PhantomData<&'a mut [u8]>,
}

impl<'a> SharedMemoryRef<'a> {
    pub(crate) fn new(raw: *mut sys::TEEC_SharedMemory) -> Self {
        SharedMemoryRef {
            raw,
            borrow: PhantomData,
        }
    }

    fn as_ptr(&self) -> *mut sys::TEEC_SharedMemory {
        self.raw
    }
}

impl fmt::Debug for SharedMemoryRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SharedMemoryRef")
    }
}

/// One parameter of an [`Operation`].
///
/// `TEEC_Parameter` is a C union whose active member is selected by a 4-bit
/// type code packed into `TEEC_Operation::paramTypes`. Here the union and its
/// type code are a single tagged enum: picking a variant fixes both the payload
/// and the type code, which [`Operation`] packs on its own.
///
/// The temporary memory reference is split by direction because an input-only
/// buffer never needs to be borrowed mutably, while an output buffer always
/// does.
#[derive(Debug)]
pub enum Parameter<'a> {
    /// The slot is unused (`TEEC_NONE`).
    None,
    /// Two 32-bit values passed inline (`TEEC_VALUE_*`).
    Value {
        /// Which way the values flow.
        direction: Direction,
        /// The first value.
        a: u32,
        /// The second value.
        b: u32,
    },
    /// A buffer sent to the trusted application (`TEEC_MEMREF_TEMP_INPUT`).
    TempMemoryInput(&'a [u8]),
    /// A buffer the trusted application fills (`TEEC_MEMREF_TEMP_OUTPUT`).
    TempMemoryOutput(&'a mut [u8]),
    /// A buffer sent to and rewritten by the trusted application
    /// (`TEEC_MEMREF_TEMP_INOUT`).
    TempMemoryInOut(&'a mut [u8]),
    /// A whole registered or allocated shared memory block
    /// (`TEEC_MEMREF_WHOLE`); the direction is the one the block was created
    /// with.
    WholeMemory(SharedMemoryRef<'a>),
    /// A slice of a registered or allocated shared memory block
    /// (`TEEC_MEMREF_PARTIAL_*`).
    PartialMemory {
        /// Which way the data flows.
        direction: Direction,
        /// The block the slice belongs to.
        memory: SharedMemoryRef<'a>,
        /// Offset of the slice within the block, in bytes.
        offset: u32,
        /// Length of the slice, in bytes.
        size: u32,
    },
    /// An ION buffer sent to the trusted application (`TEEC_ION_INPUT`).
    Ion {
        /// The shared ION file descriptor.
        fd: BorrowedFd<'a>,
        /// Size of the ION memory, in bytes.
        size: u32,
    },
    /// An ION scatter-gather list sent to the trusted application
    /// (`TEEC_ION_SGLIST_INPUT`).
    IonScatterGatherList {
        /// The shared ION file descriptor.
        fd: BorrowedFd<'a>,
        /// Size of the ION memory, in bytes.
        size: u32,
    },
}

impl Parameter<'_> {
    /// The 4-bit `TEEC_ParamType` code this variant stands for.
    fn raw_type(&self) -> u32 {
        match self {
            Parameter::None => sys::TEEC_ParamType_TEEC_NONE,
            Parameter::Value { direction, .. } => match direction {
                Direction::Input => sys::TEEC_ParamType_TEEC_VALUE_INPUT,
                Direction::Output => sys::TEEC_ParamType_TEEC_VALUE_OUTPUT,
                Direction::InOut => sys::TEEC_ParamType_TEEC_VALUE_INOUT,
            },
            Parameter::TempMemoryInput(_) => sys::TEEC_ParamType_TEEC_MEMREF_TEMP_INPUT,
            Parameter::TempMemoryOutput(_) => sys::TEEC_ParamType_TEEC_MEMREF_TEMP_OUTPUT,
            Parameter::TempMemoryInOut(_) => sys::TEEC_ParamType_TEEC_MEMREF_TEMP_INOUT,
            Parameter::WholeMemory(_) => sys::TEEC_ParamType_TEEC_MEMREF_WHOLE,
            Parameter::PartialMemory { direction, .. } => match direction {
                Direction::Input => sys::TEEC_ParamType_TEEC_MEMREF_PARTIAL_INPUT,
                Direction::Output => sys::TEEC_ParamType_TEEC_MEMREF_PARTIAL_OUTPUT,
                Direction::InOut => sys::TEEC_ParamType_TEEC_MEMREF_PARTIAL_INOUT,
            },
            Parameter::Ion { .. } => sys::TEEC_ParamType_TEEC_ION_INPUT,
            Parameter::IonScatterGatherList { .. } => sys::TEEC_ParamType_TEEC_ION_SGLIST_INPUT,
        }
    }

    /// Fill the C union member this variant selects.
    fn build_raw(&mut self) -> Result<sys::TEEC_Parameter> {
        // SAFETY: TEEC_Parameter is a plain-old-data C union of pointers and
        // integers, for which an all-zero value is valid.
        let mut raw: sys::TEEC_Parameter = unsafe { std::mem::zeroed() };
        match self {
            Parameter::None => {}
            Parameter::Value { a, b, .. } => raw.value = sys::TEEC_Value { a: *a, b: *b },
            Parameter::TempMemoryInput(buffer) => {
                raw.tmpref = sys::TEEC_TempMemoryReference {
                    buffer: buffer.as_ptr().cast_mut().cast::<c_void>(),
                    size: buffer_len(buffer.len())?,
                }
            }
            Parameter::TempMemoryOutput(buffer) | Parameter::TempMemoryInOut(buffer) => {
                raw.tmpref = sys::TEEC_TempMemoryReference {
                    buffer: buffer.as_mut_ptr().cast::<c_void>(),
                    size: buffer_len(buffer.len())?,
                }
            }
            Parameter::WholeMemory(memory) => {
                // The size and offset of a whole reference are supplied by the
                // implementation from the block itself.
                raw.memref = sys::TEEC_RegisteredMemoryReference {
                    parent: memory.as_ptr(),
                    size: 0,
                    offset: 0,
                }
            }
            Parameter::PartialMemory {
                memory,
                offset,
                size,
                ..
            } => {
                raw.memref = sys::TEEC_RegisteredMemoryReference {
                    parent: memory.as_ptr(),
                    size: *size,
                    offset: *offset,
                }
            }
            Parameter::Ion { fd, size } | Parameter::IonScatterGatherList { fd, size } => {
                raw.ionref = sys::TEEC_IonReference {
                    ion_share_fd: fd.as_raw_fd(),
                    ion_size: *size,
                }
            }
        }
        Ok(raw)
    }
}

fn buffer_len(len: usize) -> Result<u32> {
    u32::try_from(len)
        .map_err(|_| TeeError::invalid_argument("buffer length does not fit in a 32-bit size"))
}

/// The parameters carried by an open-session or invoke-command call.
///
/// An operation holds four parameter slots. The packed `paramTypes` field the
/// native API expects is derived from the slots, so no bit shifting is left to
/// the caller.
///
/// Values written back by the trusted application are read through
/// [`value`](Self::value) and [`output_size`](Self::output_size) after the call
/// returns.
pub struct Operation<'a> {
    params: [Parameter<'a>; PARAM_COUNT],
    outputs: [u32; PARAM_COUNT],
    started: u32,
    raw: Box<sys::TEEC_Operation>,
}

impl Default for Operation<'_> {
    fn default() -> Self {
        Operation::new()
    }
}

impl<'a> Operation<'a> {
    /// An operation with all four slots unused.
    pub fn new() -> Self {
        Operation::with_params([
            Parameter::None,
            Parameter::None,
            Parameter::None,
            Parameter::None,
        ])
    }

    /// An operation with the given four slots.
    pub fn with_params(params: [Parameter<'a>; PARAM_COUNT]) -> Self {
        Operation {
            params,
            outputs: [0; PARAM_COUNT],
            started: 1,
            // SAFETY: TEEC_Operation is a plain-old-data C struct of integers,
            // pointers and a union, for which an all-zero value is valid.
            raw: Box::new(unsafe { std::mem::zeroed() }),
        }
    }

    /// Replace one slot. `index` must be below [`PARAM_COUNT`].
    pub fn set_param(&mut self, index: usize, param: Parameter<'a>) -> Result<()> {
        let slot = self
            .params
            .get_mut(index)
            .ok_or_else(|| TeeError::invalid_argument("parameter index out of range"))?;
        *slot = param;
        self.outputs[index] = 0;
        Ok(())
    }

    /// The parameter in a slot.
    pub fn param(&self, index: usize) -> Option<&Parameter<'a>> {
        self.params.get(index)
    }

    /// The packed `paramTypes` field, computed from the four slots.
    ///
    /// Each slot contributes its 4-bit type code, slot 0 in the lowest nibble.
    pub fn param_types(&self) -> u32 {
        self.params
            .iter()
            .enumerate()
            .fold(0u32, |packed, (index, param)| {
                packed | (param.raw_type() << (4 * index))
            })
    }

    /// The two values of a [`Parameter::Value`] slot, refreshed with whatever
    /// the trusted application wrote back.
    pub fn value(&self, index: usize) -> Option<(u32, u32)> {
        match self.params.get(index)? {
            Parameter::Value { a, b, .. } => Some((*a, *b)),
            _ => None,
        }
    }

    /// The number of bytes the trusted application reported for a memory slot.
    ///
    /// For an output or in-out reference this is the length actually written;
    /// when a call fails with [`ErrorKind::ShortBuffer`](crate::ErrorKind::ShortBuffer)
    /// it is the length that would have been needed. `None` for a slot that
    /// carries no memory reference.
    pub fn output_size(&self, index: usize) -> Option<usize> {
        match self.params.get(index)? {
            Parameter::TempMemoryInput(_)
            | Parameter::TempMemoryOutput(_)
            | Parameter::TempMemoryInOut(_)
            | Parameter::WholeMemory(_)
            | Parameter::PartialMemory { .. } => Some(self.outputs[index] as usize),
            _ => None,
        }
    }

    /// Ask the TEE to cancel the operation.
    ///
    /// This only sends a request; whether it takes effect is up to the TEE and
    /// the trusted application, and the current OpenHarmony implementation
    /// ignores it. An in-flight operation is borrowed exclusively by the call
    /// running it, so a request can only be issued before or after that call.
    pub fn request_cancellation(&mut self) {
        // SAFETY: the boxed operation is a valid, initialised TEEC_Operation.
        unsafe { sys::TEEC_RequestCancellation(&mut *self.raw) };
    }

    /// Fill the owned `TEEC_Operation` from the slots and hand out a pointer to
    /// it. The pointer stays valid while `self` is borrowed, because the
    /// operation lives behind a box.
    pub(crate) fn prepare(&mut self) -> Result<*mut sys::TEEC_Operation> {
        let param_types = self.param_types();
        let mut params: [sys::TEEC_Parameter; PARAM_COUNT] =
            // SAFETY: TEEC_Parameter is plain-old-data; all-zero is valid.
            unsafe { std::mem::zeroed() };
        for (raw, param) in params.iter_mut().zip(self.params.iter_mut()) {
            *raw = param.build_raw()?;
        }
        self.raw.started = self.started;
        self.raw.paramTypes = param_types;
        self.raw.params = params;
        self.raw.session = std::ptr::null_mut();
        self.raw.cancel_flag = false;
        Ok(&mut *self.raw)
    }

    /// Copy back what the trusted application wrote into the slots.
    pub(crate) fn absorb(&mut self) {
        for index in 0..PARAM_COUNT {
            let raw = self.raw.params[index];
            match &mut self.params[index] {
                Parameter::Value { a, b, .. } => {
                    // SAFETY: the slot holds a value parameter, so the value
                    // member is the active one on both sides of the call.
                    let value = unsafe { raw.value };
                    *a = value.a;
                    *b = value.b;
                }
                Parameter::TempMemoryInput(_)
                | Parameter::TempMemoryOutput(_)
                | Parameter::TempMemoryInOut(_) => {
                    // SAFETY: the slot holds a temporary memory reference, so
                    // the tmpref member is the active one.
                    self.outputs[index] = unsafe { raw.tmpref }.size;
                }
                Parameter::WholeMemory(_) | Parameter::PartialMemory { .. } => {
                    // SAFETY: the slot holds a registered memory reference, so
                    // the memref member is the active one.
                    self.outputs[index] = unsafe { raw.memref }.size;
                }
                Parameter::None
                | Parameter::Ion { .. }
                | Parameter::IonScatterGatherList { .. } => {}
            }
        }
    }
}

impl fmt::Debug for Operation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operation")
            .field("param_types", &format_args!("{:#06x}", self.param_types()))
            .field("params", &self.params)
            .field("outputs", &self.outputs)
            .finish()
    }
}
