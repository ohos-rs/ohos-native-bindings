use crate::LogLevel;

use super::arrays::slice_assume_init_ref;
use super::{openharmony_log, uninit_array, LOGGING_MSG_MAX_LEN};
use log::Level;
use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::{fmt, mem, ptr};

/// The purpose of this "writer" is to split logged messages on whitespace when the log message
/// length exceeds the maximum. Without allocations.
pub struct PlatformLogWriter<'a> {
    priority: crate::LogLevel,
    len: usize,
    last_newline_index: usize,
    tag: &'a CStr,
    buffer: [MaybeUninit<u8>; LOGGING_MSG_MAX_LEN + 1],
}

impl PlatformLogWriter<'_> {
    pub fn new_with_priority(priority: crate::LogLevel, tag: &CStr) -> PlatformLogWriter<'_> {
        #[allow(deprecated)] // created an issue #35 for this
        PlatformLogWriter {
            priority,
            len: 0,
            last_newline_index: 0,
            tag,
            buffer: uninit_array(),
        }
    }

    pub fn new(level: Level, tag: &CStr) -> PlatformLogWriter<'_> {
        PlatformLogWriter::new_with_priority(
            match level {
                Level::Warn => LogLevel::LogWarn,
                Level::Info => LogLevel::LogInfo,
                Level::Debug => LogLevel::LogDebug,
                Level::Error => LogLevel::LogError,
                Level::Trace => LogLevel::LogInfo,
            },
            tag,
        )
    }

    /// Flush some bytes to android logger.
    ///
    /// If there is a newline, flush up to it.
    /// If there was no newline, flush all.
    ///
    /// Not guaranteed to flush everything.
    fn temporal_flush(&mut self) {
        let total_len = self.len;

        if total_len == 0 {
            return;
        }

        if self.last_newline_index > 0 {
            let copy_from_index = self.last_newline_index;
            let remaining_chunk_len = total_len - copy_from_index;

            unsafe { self.output_specified_len(copy_from_index) };
            self.copy_bytes_to_start(copy_from_index, remaining_chunk_len);
            self.len = remaining_chunk_len;
        } else {
            unsafe { self.output_specified_len(total_len) };
            self.len = 0;
        }
        self.last_newline_index = 0;
    }

    /// Flush everything remaining to android logger.
    pub fn flush(&mut self) {
        let total_len = self.len;

        if total_len == 0 {
            return;
        }

        unsafe { self.output_specified_len(total_len) };
        self.len = 0;
        self.last_newline_index = 0;
    }

    /// Output buffer up until the \0 which will be placed at `len` position.
    ///
    /// # Safety
    /// The first `len` bytes of `self.buffer` must be initialized.
    unsafe fn output_specified_len(&mut self, len: usize) {
        let mut last_byte = MaybeUninit::new(b'\0');

        mem::swap(
            &mut last_byte,
            self.buffer.get_mut(len).expect("`len` is out of bounds"),
        );

        let initialized = unsafe { slice_assume_init_ref(&self.buffer[..len + 1]) };
        let msg = CStr::from_bytes_with_nul(initialized)
            .expect("Unreachable: nul terminator was placed at `len`");
        openharmony_log(self.priority, self.tag, msg);

        unsafe { *self.buffer.get_unchecked_mut(len) = last_byte };
    }

    /// Copy `len` bytes from `index` position to starting position.
    fn copy_bytes_to_start(&mut self, index: usize, len: usize) {
        let dst = self.buffer.as_mut_ptr();
        let src = unsafe { self.buffer.as_ptr().add(index) };
        unsafe { ptr::copy(src, dst, len) };
    }
}

impl fmt::Write for PlatformLogWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut incoming_bytes = s.as_bytes();

        while !incoming_bytes.is_empty() {
            let len = self.len;

            // write everything possible to buffer and mark last \n
            let new_len = len + incoming_bytes.len();
            let last_newline = self.buffer[len..LOGGING_MSG_MAX_LEN]
                .iter_mut()
                .zip(incoming_bytes)
                .enumerate()
                .fold(None, |acc, (i, (output, input))| {
                    output.write(*input);
                    if *input == b'\n' {
                        Some(i)
                    } else {
                        acc
                    }
                });

            // update last \n index
            if let Some(newline) = last_newline {
                self.last_newline_index = len + newline;
            }

            // calculate how many bytes were written
            let written_len = if new_len <= LOGGING_MSG_MAX_LEN {
                // if the len was not exceeded
                self.len = new_len;
                new_len - len // written len
            } else {
                // if new length was exceeded
                self.len = LOGGING_MSG_MAX_LEN;
                self.temporal_flush();

                LOGGING_MSG_MAX_LEN - len // written len
            };

            incoming_bytes = &incoming_bytes[written_len..];
        }

        Ok(())
    }
}
