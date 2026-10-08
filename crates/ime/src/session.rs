use std::sync::{Mutex, MutexGuard, TryLockError};

use ohos_input_method_sys::InputMethod_ErrorCode_IME_ERR_IMCLIENT;

use crate::{ImeError, ImeResult, TextEditor};

/// The NDK has one process-wide input proxy. A successful attach or detach is
/// the documented boundary after which an older editor may be destroyed.
pub(crate) struct NativeSessions {
    pub(crate) active_id: u64,
    pub(crate) generation: u64,
}

static SESSIONS: Mutex<NativeSessions> = Mutex::new(NativeSessions {
    active_id: 0,
    generation: 0,
});
static RETIRED_EDITORS: Mutex<Vec<RetiredEditor>> = Mutex::new(Vec::new());

struct RetiredEditor(TextEditor);

// SAFETY: a retired editor's native callback table is no longer mutated. Its
// Rust callbacks are unregistered before transfer. Only a serialized successful
// NDK attach/detach destroys it, after native ownership of older editors ends.
unsafe impl Send for RetiredEditor {}

impl NativeSessions {
    pub(crate) fn acquire(operation: &'static str) -> ImeResult<MutexGuard<'static, Self>> {
        // Native calls can synchronously invoke application callbacks. Do not
        // deadlock if one reenters the binding or waits on a competing thread.
        match SESSIONS.try_lock() {
            Ok(guard) => Ok(guard),
            Err(TryLockError::Poisoned(error)) => Ok(error.into_inner()),
            Err(TryLockError::WouldBlock) => Err(ImeError::new(
                operation,
                InputMethod_ErrorCode_IME_ERR_IMCLIENT,
            )),
        }
    }

    pub(crate) fn complete_transition(&mut self, active_id: u64) {
        self.active_id = active_id;
        self.generation += 1;
        let retired = std::mem::take(
            &mut *RETIRED_EDITORS
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        );
        for RetiredEditor(editor) in retired {
            drop(editor);
        }
    }

    /// Failed native cleanup is not a lifetime boundary. Keep the small native
    /// callback allocation alive until a later successful attach/detach. The
    /// static pool also survives the originating Rust thread's termination.
    pub(crate) fn retire(editor: TextEditor) {
        RETIRED_EDITORS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(RetiredEditor(editor));
    }
}
