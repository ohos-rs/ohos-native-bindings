use crate::alloc::with_allocator;
use crate::error::{check, IpcError, Result};
use crate::remote::{RemoteProxy, RemoteStub};
use ohos_ipc_sys as sys;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr::NonNull;

/// An IPC serialized object: the payload of a request or a reply.
///
/// A parcel is a cursor-based buffer. Values are appended with the `write_*`
/// methods and taken back out, in the same order, with the `read_*` methods.
/// Its total size cannot exceed 204,800 bytes.
///
/// The object created by [`Parcel::new`] is destroyed on drop. The parcels a
/// stub request handler receives are owned by the runtime, so those are only
/// borrowed and never destroyed by this crate.
///
/// Parcels are neither `Send` nor `Sync`: the native object carries a read and
/// a write cursor and is not documented as thread-safe.
pub struct Parcel {
    raw: NonNull<sys::OHIPCParcel>,
    owned: bool,
}

impl Parcel {
    /// Create an empty parcel.
    pub fn new() -> Result<Self> {
        // SAFETY: the constructor takes no arguments and returns ownership.
        let raw = unsafe { sys::OH_IPCParcel_Create() };
        NonNull::new(raw)
            .map(|raw| Parcel { raw, owned: true })
            .ok_or(IpcError::Failed("failed to create a parcel"))
    }

    /// Wrap a parcel owned by the runtime, such as the request and reply
    /// objects passed to a stub request handler.
    ///
    /// # Safety
    ///
    /// `raw` must point to a live parcel that outlives the returned value, and
    /// the caller must ensure nothing else uses it concurrently.
    pub(crate) unsafe fn borrowed(raw: *mut sys::OHIPCParcel) -> Option<Self> {
        NonNull::new(raw).map(|raw| Parcel { raw, owned: false })
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::OHIPCParcel {
        self.raw.as_ptr()
    }

    /// Size of the data held by this parcel, in bytes.
    pub fn data_size(&self) -> Result<usize> {
        // SAFETY: `self.raw` is a live parcel.
        count(
            unsafe { sys::OH_IPCParcel_GetDataSize(self.as_ptr()) },
            "data size",
        )
    }

    /// Number of bytes that can still be written to this parcel before its
    /// buffer has to grow.
    ///
    /// This is the free room in the buffer currently allocated, not a fixed
    /// budget: the buffer is allocated lazily, so a parcel that has never been
    /// written to reports zero, and the first write allocates a whole chunk of
    /// which this then reports the remainder. A zero here does not mean the
    /// parcel is full, and writing more than this is not an error as long as
    /// the total size stays within the parcel limit.
    pub fn writable_bytes(&self) -> Result<usize> {
        // SAFETY: `self.raw` is a live parcel.
        count(
            unsafe { sys::OH_IPCParcel_GetWritableBytes(self.as_ptr()) },
            "writable bytes",
        )
    }

    /// Number of bytes that can still be read from this parcel.
    pub fn readable_bytes(&self) -> Result<usize> {
        // SAFETY: `self.raw` is a live parcel.
        count(
            unsafe { sys::OH_IPCParcel_GetReadableBytes(self.as_ptr()) },
            "readable bytes",
        )
    }

    /// Current read cursor.
    pub fn read_position(&self) -> Result<usize> {
        // SAFETY: `self.raw` is a live parcel.
        count(
            unsafe { sys::OH_IPCParcel_GetReadPosition(self.as_ptr()) },
            "read position",
        )
    }

    /// Current write cursor.
    pub fn write_position(&self) -> Result<usize> {
        // SAFETY: `self.raw` is a live parcel.
        count(
            unsafe { sys::OH_IPCParcel_GetWritePosition(self.as_ptr()) },
            "write position",
        )
    }

    /// Move the read cursor, which must land between zero and the current data
    /// size.
    pub fn rewind_read_position(&mut self, position: usize) -> Result<()> {
        let position = position_arg(position)?;
        // SAFETY: `self.raw` is a live parcel; the position is validated by the
        // native side.
        check(unsafe { sys::OH_IPCParcel_RewindReadPosition(self.as_ptr(), position) })
    }

    /// Move the write cursor, which must land between zero and the current data
    /// size.
    pub fn rewind_write_position(&mut self, position: usize) -> Result<()> {
        let position = position_arg(position)?;
        // SAFETY: `self.raw` is a live parcel; the position is validated by the
        // native side.
        check(unsafe { sys::OH_IPCParcel_RewindWritePosition(self.as_ptr(), position) })
    }

    /// Append the contents of another parcel to this one.
    pub fn append(&mut self, other: &Parcel) -> Result<()> {
        // SAFETY: both parcels are live and distinct objects.
        check(unsafe { sys::OH_IPCParcel_Append(self.as_ptr(), other.as_ptr()) })
    }
}

macro_rules! scalar_accessors {
    ($($write:ident, $read:ident, $ty:ty, $write_raw:ident, $read_raw:ident, $what:literal;)*) => {
        impl Parcel {
            $(
                #[doc = concat!("Write ", $what, " to this parcel.")]
                pub fn $write(&mut self, value: $ty) -> Result<()> {
                    // SAFETY: `self.raw` is a live parcel and `value` is passed by value.
                    check(unsafe { sys::$write_raw(self.as_ptr(), value) })
                }

                #[doc = concat!("Read ", $what, " from this parcel, advancing the read cursor.")]
                pub fn $read(&mut self) -> Result<$ty> {
                    let mut value: $ty = Default::default();
                    // SAFETY: `self.raw` is a live parcel and `value` is a live local.
                    check(unsafe { sys::$read_raw(self.as_ptr(), &mut value) })?;
                    Ok(value)
                }
            )*
        }
    };
}

scalar_accessors! {
    write_i8, read_i8, i8, OH_IPCParcel_WriteInt8, OH_IPCParcel_ReadInt8, "an `i8`";
    write_i16, read_i16, i16, OH_IPCParcel_WriteInt16, OH_IPCParcel_ReadInt16, "an `i16`";
    write_i32, read_i32, i32, OH_IPCParcel_WriteInt32, OH_IPCParcel_ReadInt32, "an `i32`";
    write_i64, read_i64, i64, OH_IPCParcel_WriteInt64, OH_IPCParcel_ReadInt64, "an `i64`";
    write_f32, read_f32, f32, OH_IPCParcel_WriteFloat, OH_IPCParcel_ReadFloat, "an `f32`";
    write_f64, read_f64, f64, OH_IPCParcel_WriteDouble, OH_IPCParcel_ReadDouble, "an `f64`";
}

impl Parcel {
    /// Write a string, terminator included.
    ///
    /// The string must not contain an interior NUL byte.
    pub fn write_string(&mut self, value: &str) -> Result<()> {
        let value = CString::new(value)
            .map_err(|_| IpcError::InvalidArgument("the string contains an interior NUL byte"))?;
        self.write_c_string(&value)
    }

    /// Write an already NUL-terminated string, avoiding the copy
    /// [`Parcel::write_string`] makes.
    pub fn write_c_string(&mut self, value: &CStr) -> Result<()> {
        // SAFETY: `self.raw` is a live parcel and `value` is NUL-terminated and
        // outlives the call, which copies it into the parcel.
        check(unsafe { sys::OH_IPCParcel_WriteString(self.as_ptr(), value.as_ptr()) })
    }

    /// Read a string, advancing the read cursor.
    ///
    /// Bytes that are not valid UTF-8 are replaced. The native call hands back a
    /// pointer into the parcel's own buffer rather than a fresh allocation, so
    /// the data is copied out here and there is nothing for the caller to
    /// release.
    pub fn read_string(&mut self) -> Result<String> {
        // SAFETY: `self.raw` is a live parcel.
        let raw = unsafe { sys::OH_IPCParcel_ReadString(self.as_ptr()) };
        if raw.is_null() {
            return Err(IpcError::Failed("failed to read a string"));
        }
        // SAFETY: on success the native side returns a NUL-terminated string
        // that stays valid at least until the parcel is written to again; it is
        // copied before this borrow ends.
        Ok(unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned())
    }

    /// Write a byte buffer.
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<()> {
        let len = i32::try_from(buffer.len())
            .map_err(|_| IpcError::InvalidArgument("the buffer is longer than i32::MAX"))?;
        // SAFETY: `self.raw` is a live parcel and `buffer` is valid for `len`
        // bytes for the duration of the call, which copies from it.
        check(unsafe { sys::OH_IPCParcel_WriteBuffer(self.as_ptr(), buffer.as_ptr(), len) })
    }

    /// Read exactly `len` bytes, advancing the read cursor.
    ///
    /// Fails if `len` exceeds the readable size, so a short buffer can never be
    /// over-read. As with [`Parcel::read_string`], the native call returns a
    /// pointer into the parcel and the bytes are copied out here.
    pub fn read_buffer(&mut self, len: usize) -> Result<Vec<u8>> {
        if len == 0 {
            return Ok(Vec::new());
        }
        let requested = i32::try_from(len)
            .map_err(|_| IpcError::InvalidArgument("the length is larger than i32::MAX"))?;
        // SAFETY: `self.raw` is a live parcel.
        let raw = unsafe { sys::OH_IPCParcel_ReadBuffer(self.as_ptr(), requested) };
        if raw.is_null() {
            return Err(IpcError::Failed("failed to read a buffer"));
        }
        // SAFETY: on success the native side guarantees `len` readable bytes at
        // `raw`, which are copied before this borrow ends.
        Ok(unsafe { std::slice::from_raw_parts(raw, len) }.to_vec())
    }

    /// Write an interface token, used by the stub to verify that the caller
    /// speaks the expected interface.
    pub fn write_interface_token(&mut self, token: &str) -> Result<()> {
        let token = CString::new(token)
            .map_err(|_| IpcError::InvalidArgument("the token contains an interior NUL byte"))?;
        // SAFETY: `self.raw` is a live parcel and `token` is NUL-terminated and
        // outlives the call, which copies it into the parcel.
        check(unsafe { sys::OH_IPCParcel_WriteInterfaceToken(self.as_ptr(), token.as_ptr()) })
    }

    /// Read an interface token, advancing the read cursor.
    ///
    /// Bytes that are not valid UTF-8 are replaced.
    pub fn read_interface_token(&mut self) -> Result<String> {
        let parcel = self.as_ptr();
        let token = with_allocator(
            "failed to read an interface token",
            |out, len, allocator| {
                // SAFETY: `parcel` is live, `out` and `len` are live locals, and the
                // allocator matches the deallocator used on the returned buffer.
                unsafe { sys::OH_IPCParcel_ReadInterfaceToken(parcel, out, len, allocator) }
            },
        )?;
        Ok(token.to_string_lossy().into_owned())
    }

    /// Write a file descriptor.
    ///
    /// The native header does not state whether the descriptor is duplicated,
    /// so the caller keeps owning it and must not close it before the parcel
    /// has been sent.
    pub fn write_file_descriptor(&mut self, fd: i32) -> Result<()> {
        // SAFETY: `self.raw` is a live parcel and `fd` is passed by value.
        check(unsafe { sys::OH_IPCParcel_WriteFileDescriptor(self.as_ptr(), fd) })
    }

    /// Read a file descriptor, advancing the read cursor.
    ///
    /// The native header does not state who closes the descriptor; it is
    /// returned as a plain number and this crate does not close it.
    pub fn read_file_descriptor(&mut self) -> Result<i32> {
        let mut fd: i32 = -1;
        // SAFETY: `self.raw` is a live parcel and `fd` is a live local.
        check(unsafe { sys::OH_IPCParcel_ReadFileDescriptor(self.as_ptr(), &mut fd) })?;
        Ok(fd)
    }

    /// Write a remote stub object, so that the peer can obtain a proxy to it.
    pub fn write_remote_stub(&mut self, stub: &RemoteStub) -> Result<()> {
        // SAFETY: both objects are live for the duration of the call.
        check(unsafe { sys::OH_IPCParcel_WriteRemoteStub(self.as_ptr(), stub.as_ptr()) })
    }

    /// Read a remote stub object, advancing the read cursor.
    pub fn read_remote_stub(&mut self) -> Result<RemoteStub> {
        // SAFETY: `self.raw` is a live parcel.
        let raw = unsafe { sys::OH_IPCParcel_ReadRemoteStub(self.as_ptr()) };
        NonNull::new(raw)
            // SAFETY: the returned object is a fresh handle whose ownership
            // passes to the caller, as for every other object the kit hands out
            // alongside a `_Destroy` function.
            .map(|raw| unsafe { RemoteStub::from_raw(raw) })
            .ok_or(IpcError::Failed("failed to read a remote stub"))
    }

    /// Write a remote proxy object.
    pub fn write_remote_proxy(&mut self, proxy: &RemoteProxy) -> Result<()> {
        // SAFETY: both objects are live for the duration of the call.
        check(unsafe { sys::OH_IPCParcel_WriteRemoteProxy(self.as_ptr(), proxy.as_ptr()) })
    }

    /// Read a remote proxy object, advancing the read cursor.
    pub fn read_remote_proxy(&mut self) -> Result<RemoteProxy> {
        // SAFETY: `self.raw` is a live parcel.
        let raw = unsafe { sys::OH_IPCParcel_ReadRemoteProxy(self.as_ptr()) };
        NonNull::new(raw)
            // SAFETY: the returned object is a fresh handle whose ownership
            // passes to the caller, as for every other object the kit hands out
            // alongside a `_Destroy` function.
            .map(|raw| unsafe { RemoteProxy::from_raw(raw) })
            .ok_or(IpcError::Failed("failed to read a remote proxy"))
    }
}

impl Drop for Parcel {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        // SAFETY: an owned parcel is destroyed exactly once, here, and nothing
        // else holds the pointer.
        unsafe { sys::OH_IPCParcel_Destroy(self.as_ptr()) };
    }
}

impl std::fmt::Debug for Parcel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parcel")
            .field("data_size", &self.data_size().ok())
            .field("owned", &self.owned)
            .finish()
    }
}

/// Turn a native count that reports failure as `-1` into a `usize`.
fn count(value: c_int, what: &'static str) -> Result<usize> {
    match usize::try_from(value) {
        Ok(value) => Ok(value),
        Err(_) => Err(IpcError::Failed(what)),
    }
}

fn position_arg(position: usize) -> Result<u32> {
    u32::try_from(position)
        .map_err(|_| IpcError::InvalidArgument("the position is larger than u32::MAX"))
}
