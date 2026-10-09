use std::ptr::NonNull;

use ohos_web_sys::{
    ArkWeb_HttpBodyStream, OH_ArkWebHttpBodyStream_GetSize, OH_ArkWebHttpBodyStream_GetUserData,
    OH_ArkWebHttpBodyStream_Init, OH_ArkWebHttpBodyStream_IsChunked, OH_ArkWebHttpBodyStream_IsEof,
    OH_ArkWebHttpBodyStream_IsInMemory, OH_ArkWebHttpBodyStream_Read,
    OH_ArkWebHttpBodyStream_SetReadCallback, OH_ArkWebHttpBodyStream_SetUserData,
    OH_ArkWebResourceRequest_DestroyHttpBodyStream,
};

use crate::HttpBodyStreamError;

/// An owned request body stream, confined to the ArkWeb I/O thread.
///
/// Reads consume the stream, so only one operation can use it at a time.
/// [`Self::read`] returns ownership with the bytes; [`Self::read_to_end`] closes
/// it on completion. Both initialize lazily, without rewinding subsequent reads.
/// Callbacks may run inline and must own their captures (`'static`).
///
/// Once submitted, an operation owns the stream and buffer until its callback.
/// If native code never completes it, those resources must remain allocated:
/// ArkWeb provides no cancellation acknowledgement for outstanding writes.
pub struct HttpBodyStream {
    raw: NonNull<ArkWeb_HttpBodyStream>,
    initialized: bool,
}

impl HttpBodyStream {
    /// Take exclusive ownership of a native body stream.
    ///
    /// # Safety
    ///
    /// `raw` must be null or a live stream returned by
    /// `OH_ArkWebResourceRequest_GetHttpBodyStream`, with no operations in flight.
    /// The caller must be on the ArkWeb I/O thread. No other owner may use,
    /// destroy, or change callbacks/userData on this stream after this call.
    pub unsafe fn new(raw: *mut ArkWeb_HttpBodyStream) -> Result<Self, HttpBodyStreamError> {
        Ok(Self {
            raw: NonNull::new(raw).ok_or(HttpBodyStreamError::NullStream)?,
            initialized: false,
        })
    }

    /// Initialize without reading, returning the stream for metadata queries.
    /// Already initialized streams are returned without another native Init.
    /// All errors are delivered once through the callback, possibly inline.
    pub fn initialize<F>(self, callback: F)
    where
        F: FnOnce(Result<Self, HttpBodyStreamError>) + 'static,
    {
        self.read(0, move |result| callback(result.map(|(stream, _)| stream)));
    }

    pub fn is_chunked(&self) -> Result<bool, HttpBodyStreamError> {
        self.ensure_initialized()?;
        Ok(unsafe { OH_ArkWebHttpBodyStream_IsChunked(self.raw.as_ptr()) })
    }

    pub fn is_eof(&self) -> Result<bool, HttpBodyStreamError> {
        self.ensure_initialized()?;
        Ok(unsafe { OH_ArkWebHttpBodyStream_IsEof(self.raw.as_ptr()) })
    }

    pub fn is_in_memory(&self) -> Result<bool, HttpBodyStreamError> {
        self.ensure_initialized()?;
        Ok(unsafe { OH_ArkWebHttpBodyStream_IsInMemory(self.raw.as_ptr()) })
    }

    /// Total body size after initialization: `None` for unknown-length chunked
    /// bodies, `Some(0)` for known empty bodies. Before initialization, errors.
    pub fn size(&self) -> Result<Option<u64>, HttpBodyStreamError> {
        if self.is_chunked()? {
            Ok(None)
        } else {
            Ok(Some(unsafe {
                OH_ArkWebHttpBodyStream_GetSize(self.raw.as_ptr())
            }))
        }
    }

    /// Read at most `size` bytes, returning the stream at its new position.
    /// A zero-sized read initializes but never calls native Read.
    /// On error the stream is closed. The callback is the sole result channel.
    ///
    /// Ownership prevents overlapping reads at compile time:
    /// ```compile_fail,E0382
    /// # use ohos_web_binding::HttpBodyStream;
    /// fn overlap(stream: HttpBodyStream) {
    ///     stream.read(1, |_| {});
    ///     stream.read(1, |_| {}); // stream has moved into the first operation
    /// }
    /// ```
    pub fn read<F>(self, size: usize, callback: F)
    where
        F: FnOnce(Result<(Self, Vec<u8>), HttpBodyStreamError>) + 'static,
    {
        BodyReader::start(self, size, false, Box::new(callback));
    }

    /// Collect the remaining body through EOF and close the stream. Handles
    /// short reads and unknown lengths; errors also close the stream.
    pub fn read_to_end<F>(self, callback: F)
    where
        F: FnOnce(Result<Vec<u8>, HttpBodyStreamError>) + 'static,
    {
        BodyReader::start(
            self,
            64 * 1024,
            true,
            Box::new(move |result| {
                callback(result.map(|(_, bytes)| bytes));
            }),
        );
    }

    fn ensure_initialized(&self) -> Result<(), HttpBodyStreamError> {
        self.initialized
            .then_some(())
            .ok_or(HttpBodyStreamError::NotInitialized)
    }
}

impl Drop for HttpBodyStream {
    fn drop(&mut self) {
        // A submitted operation owns this stream, so no native write is pending.
        unsafe { OH_ArkWebResourceRequest_DestroyHttpBodyStream(self.raw.as_ptr()) };
    }
}

type ReadCallback = Box<dyn FnOnce(Result<(HttpBodyStream, Vec<u8>), HttpBodyStreamError>)>;

enum Completion {
    Initialized(i32),
    Read { buffer: *mut u8, count: i32 },
}

/// A single operation owns everything the native callback can access.
struct BodyReader {
    stream: HttpBodyStream,
    callback: ReadCallback,
    buffer: Vec<u8>,
    data: Vec<u8>,
    to_end: bool,
    driving: bool,
    completion: Option<Completion>,
}

impl BodyReader {
    fn start(stream: HttpBodyStream, size: usize, to_end: bool, callback: ReadCallback) {
        if size > i32::MAX as usize {
            drop(stream);
            callback(Err(HttpBodyStreamError::InvalidReadSize(size)));
            return;
        }
        let native = stream.raw.as_ptr();
        let reader = Box::into_raw(Box::new(Self {
            stream,
            callback,
            buffer: vec![0; size],
            data: Vec::new(),
            to_end,
            driving: true,
            completion: None,
        }));
        // Transfer the Box to the pending operation. No Rust reference to it
        // crosses Init/Read: either may reenter through the registered callback.
        unsafe {
            let status = OH_ArkWebHttpBodyStream_SetUserData(native, reader.cast());
            if status != 0 {
                Self::finish(reader, Err(HttpBodyStreamError::SetUserDataFailed(status)));
                return;
            }
            let status = OH_ArkWebHttpBodyStream_SetReadCallback(native, Some(read_callback));
            if status != 0 {
                Self::finish(
                    reader,
                    Err(HttpBodyStreamError::SetReadCallbackFailed(status)),
                );
                return;
            }
            if (*reader).stream.initialized {
                (*reader).completion = Some(Completion::Initialized(0));
            } else {
                let status = OH_ArkWebHttpBodyStream_Init(native, Some(initialized));
                if status != 0 {
                    (*reader).completion = Some(Completion::Initialized(status));
                }
            }
            Self::drive(reader);
        }
    }

    unsafe fn drive(reader: *mut Self) {
        (*reader).driving = true;
        while let Some(completion) = (*reader).completion.take() {
            match (*reader).accept(completion) {
                Ok(false) => {}
                result => {
                    Self::finish(reader, result.map(|_| ()));
                    return; // finish reclaimed the Box; never access it again.
                }
            }
            let native = (*reader).stream.raw.as_ptr();
            let buffer = (*reader).buffer.as_mut_ptr();
            let length = (*reader).buffer.len() as i32;
            // Inline callbacks only record completion. Processing after Read
            // returns prevents stack growth and premature buffer destruction.
            OH_ArkWebHttpBodyStream_Read(native, buffer, length);
        }
        (*reader).driving = false; // A later callback resumes this loop.
    }

    fn accept(&mut self, completion: Completion) -> Result<bool, HttpBodyStreamError> {
        match completion {
            Completion::Initialized(status) => {
                if status != 0 {
                    return Err(HttpBodyStreamError::InitializationFailed(status));
                }
                self.stream.initialized = true;
            }
            Completion::Read { buffer, count } => {
                if count < 0 {
                    return Err(HttpBodyStreamError::ReadFailed(count));
                }
                if count as usize > self.buffer.len() {
                    return Err(HttpBodyStreamError::InvalidReadCount {
                        count,
                        capacity: self.buffer.len(),
                    });
                }
                if buffer != self.buffer.as_mut_ptr() {
                    return Err(HttpBodyStreamError::UnexpectedBuffer);
                }
                if count == 0 && !self.stream.is_eof()? {
                    return Err(HttpBodyStreamError::NoProgress);
                }
                if !self.to_end {
                    self.buffer.truncate(count as usize);
                    return Ok(true);
                }
                self.data.extend_from_slice(&self.buffer[..count as usize]);
            }
        }
        if self.buffer.is_empty() || self.stream.is_eof()? {
            self.buffer.clear();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    unsafe fn complete(native: *const ArkWeb_HttpBodyStream, completion: Completion) {
        let reader = OH_ArkWebHttpBodyStream_GetUserData(native).cast::<Self>();
        (*reader).completion = Some(completion);
        if !(*reader).driving {
            Self::drive(reader);
        }
    }

    unsafe fn finish(reader: *mut Self, result: Result<(), HttpBodyStreamError>) {
        let Self {
            stream,
            callback,
            buffer,
            data,
            to_end,
            ..
        } = *Box::from_raw(reader);
        // The native write is complete. Retire the callback before returning
        // ownership: user code may immediately start another read on this stream.
        // SetUserData(null) is not supported by every ArkWeb version; the next
        // operation replaces it before submitting work, or Drop destroys it.
        OH_ArkWebHttpBodyStream_SetReadCallback(stream.raw.as_ptr(), None);
        callback(result.map(|()| (stream, if to_end { data } else { buffer })));
    }
}

unsafe extern "C" fn initialized(stream: *const ArkWeb_HttpBodyStream, status: i32) {
    BodyReader::complete(stream, Completion::Initialized(status));
}

unsafe extern "C" fn read_callback(
    stream: *const ArkWeb_HttpBodyStream,
    buffer: *mut u8,
    count: i32,
) {
    BodyReader::complete(stream, Completion::Read { buffer, count });
}
