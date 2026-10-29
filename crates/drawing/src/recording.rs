//! Owned native recording commands (API 13). Recording canvases never escape their owner.
use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_RecordCmd, OH_Drawing_RecordCmdDestroy, OH_Drawing_RecordCmdUtils,
    OH_Drawing_RecordCmdUtilsBeginRecording, OH_Drawing_RecordCmdUtilsCreate,
    OH_Drawing_RecordCmdUtilsDestroy, OH_Drawing_RecordCmdUtilsFinishRecording,
};

use crate::{check_error, Canvas, DrawingError, Result};

pub struct Recording {
    canvas: Canvas,
    raw: NonNull<OH_Drawing_RecordCmdUtils>,
}
impl Recording {
    pub fn new(width: i32, height: i32) -> Result<Self> {
        if width < 0 || height < 0 {
            return Err(DrawingError::invalid_parameter());
        }
        // SAFETY: creates a fresh owner with no aliased recording session.
        let raw = NonNull::new(unsafe { OH_Drawing_RecordCmdUtilsCreate() })
            .ok_or_else(DrawingError::from_last_error)?;
        let mut canvas = std::ptr::null_mut();
        // SAFETY: raw is live; native initializes the output canvas on success.
        let result = check_error(unsafe {
            OH_Drawing_RecordCmdUtilsBeginRecording(raw.as_ptr(), width, height, &mut canvas)
        });
        let canvas = match result
            .and_then(|()| NonNull::new(canvas).ok_or_else(DrawingError::from_last_error))
        {
            Ok(canvas) => canvas,
            Err(error) => {
                // SAFETY: creation succeeded and no Rust owner was constructed.
                unsafe { OH_Drawing_RecordCmdUtilsDestroy(raw.as_ptr()) };
                return Err(error);
            }
        };
        Ok(Self {
            // SAFETY: the recording owns this canvas; callers only borrow it.
            canvas: unsafe { Canvas::from_raw_borrowed(canvas.cast()) },
            raw,
        })
    }
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }
    pub fn finish(self) -> Result<RecordCmd> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: consumes the live recording, preventing later use of its canvas.
        check_error(unsafe {
            OH_Drawing_RecordCmdUtilsFinishRecording(self.raw.as_ptr(), &mut raw)
        })?;
        Ok(RecordCmd {
            raw: NonNull::new(raw).ok_or_else(DrawingError::from_last_error)?,
        })
    }
}
impl Drop for Recording {
    fn drop(&mut self) {
        // SAFETY: exactly one owner releases the session and its borrowed canvas.
        unsafe { OH_Drawing_RecordCmdUtilsDestroy(self.raw.as_ptr()) };
    }
}

/// An owned completed recording. Use on one thread at a time.
pub struct RecordCmd {
    raw: NonNull<OH_Drawing_RecordCmd>,
}
impl RecordCmd {
    /// Replays into a native recording canvas (API 13). For recording within
    /// another completed recording, use `draw_nested` on API 19+.
    pub fn draw(&mut self, canvas: &Canvas) -> Result<()> {
        // SAFETY: both owners are live; mutable access prevents concurrent replay.
        check_error(unsafe {
            ohos_native_drawing_sys::OH_Drawing_CanvasDrawRecordCmd(
                canvas.as_ptr(),
                self.raw.as_ptr(),
            )
        })
    }

    /// Replays into a native recording canvas, supporting nesting (API 19).
    #[cfg(feature = "api-19")]
    pub fn draw_nested(&mut self, canvas: &Canvas) -> Result<()> {
        // SAFETY: both owners are live and the mutable borrow prevents concurrent replay.
        check_error(unsafe {
            ohos_native_drawing_sys::OH_Drawing_CanvasDrawRecordCmdNesting(
                canvas.as_ptr(),
                self.raw.as_ptr(),
            )
        })
    }
}
impl Drop for RecordCmd {
    fn drop(&mut self) {
        // SAFETY: exactly one owner releases the completed recording.
        unsafe { OH_Drawing_RecordCmdDestroy(self.raw.as_ptr()) };
    }
}

#[cfg(all(test, not(target_env = "ohos")))]
mod tests {
    use super::*;
    use ohos_native_drawing_sys::OH_Drawing_Canvas;
    use std::sync::{
        atomic::{AtomicU32, Ordering},
        Mutex,
    };

    static CALLS: Mutex<Vec<&str>> = Mutex::new(Vec::new());
    static FAIL: AtomicU32 = AtomicU32::new(0);
    const ERROR: u32 = 401;
    fn handle<T>() -> *mut T {
        Box::into_raw(Box::new(0u8)).cast()
    }

    #[no_mangle]
    extern "C" fn OH_Drawing_RecordCmdUtilsCreate() -> *mut OH_Drawing_RecordCmdUtils {
        CALLS.lock().unwrap().push("create");
        handle()
    }
    #[no_mangle]
    unsafe extern "C" fn OH_Drawing_RecordCmdUtilsBeginRecording(
        raw: *mut OH_Drawing_RecordCmdUtils,
        _: i32,
        _: i32,
        canvas: *mut *mut OH_Drawing_Canvas,
    ) -> u32 {
        CALLS.lock().unwrap().push("begin");
        if FAIL.load(Ordering::Relaxed) == 1 {
            return ERROR;
        }
        unsafe {
            *canvas = raw.cast();
        }
        0
    }
    #[no_mangle]
    unsafe extern "C" fn OH_Drawing_RecordCmdUtilsFinishRecording(
        _: *mut OH_Drawing_RecordCmdUtils,
        command: *mut *mut OH_Drawing_RecordCmd,
    ) -> u32 {
        CALLS.lock().unwrap().push("finish");
        if FAIL.load(Ordering::Relaxed) == 2 {
            return ERROR;
        }
        unsafe {
            *command = handle();
        }
        0
    }
    #[no_mangle]
    unsafe extern "C" fn OH_Drawing_RecordCmdUtilsDestroy(
        raw: *mut OH_Drawing_RecordCmdUtils,
    ) -> u32 {
        CALLS.lock().unwrap().push("destroy session");
        unsafe {
            drop(Box::from_raw(raw.cast::<u8>()));
        }
        0
    }
    #[no_mangle]
    unsafe extern "C" fn OH_Drawing_RecordCmdDestroy(raw: *mut OH_Drawing_RecordCmd) -> u32 {
        CALLS.lock().unwrap().push("destroy commands");
        unsafe {
            drop(Box::from_raw(raw.cast::<u8>()));
        }
        0
    }
    #[no_mangle]
    extern "C" fn OH_Drawing_CanvasDestroy(_: *mut OH_Drawing_Canvas) {
        CALLS.lock().unwrap().push("destroy borrowed canvas");
    }
    #[no_mangle]
    extern "C" fn OH_Drawing_BitmapDestroy(_: *mut ohos_native_drawing_sys::OH_Drawing_Bitmap) {
        CALLS.lock().unwrap().push("destroy bitmap");
    }
    #[no_mangle]
    extern "C" fn OH_Drawing_ErrorCodeGet() -> u32 {
        ERROR
    }

    #[test]
    fn recording_releases_each_native_owner_once_on_success_abort_and_errors() {
        assert!(Recording::new(-1, 10).is_err());
        assert!(CALLS.lock().unwrap().is_empty());
        let commands = Recording::new(10, 20).unwrap().finish().unwrap();
        assert_eq!(
            *CALLS.lock().unwrap(),
            ["create", "begin", "finish", "destroy session"]
        );
        drop(commands);
        assert_eq!(CALLS.lock().unwrap().pop(), Some("destroy commands"));
        CALLS.lock().unwrap().clear();
        drop(Recording::new(10, 20).unwrap());
        assert_eq!(
            *CALLS.lock().unwrap(),
            ["create", "begin", "destroy session"]
        );
        CALLS.lock().unwrap().clear();
        FAIL.store(1, Ordering::Relaxed);
        assert!(Recording::new(10, 20).is_err());
        assert_eq!(
            *CALLS.lock().unwrap(),
            ["create", "begin", "destroy session"]
        );
        CALLS.lock().unwrap().clear();
        FAIL.store(2, Ordering::Relaxed);
        assert!(Recording::new(10, 20).unwrap().finish().is_err());
        assert_eq!(
            *CALLS.lock().unwrap(),
            ["create", "begin", "finish", "destroy session"]
        );
        FAIL.store(0, Ordering::Relaxed);
    }
}
