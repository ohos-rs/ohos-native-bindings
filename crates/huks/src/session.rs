use crate::error::{check, Result};
use crate::key::blob_in;
use crate::param::ParamSet;
use ohos_huks_sys::*;

// Generous fixed buffers for the session handle / auth token returned by init.
const HANDLE_CAP: usize = 64;
const TOKEN_CAP: usize = 64;

/// A three-stage HUKS crypto session: `init` → `update`* → `finish`.
///
/// Used for sign / verify / encrypt / decrypt / mac / derive operations. Create
/// one with [`init_session`].
///
/// Concurrent sessions are limited, so a session dropped without `finish` or
/// `abort` is aborted on a best-effort basis to release the slot.
pub struct Session {
    handle: Vec<u8>,
    token: Vec<u8>,
    done: bool,
}

/// Start a session for the key under `alias` with the given operation parameters
/// (purpose, algorithm, digest, padding, block mode, IV, ...).
pub fn init_session(alias: &[u8], params: &ParamSet) -> Result<Session> {
    let alias = blob_in(alias);
    let mut handle = vec![0u8; HANDLE_CAP];
    let mut token = vec![0u8; TOKEN_CAP];
    let mut handle_blob = OH_Huks_Blob {
        size: handle.len() as u32,
        data: handle.as_mut_ptr(),
    };
    let mut token_blob = OH_Huks_Blob {
        size: token.len() as u32,
        data: token.as_mut_ptr(),
    };
    // SAFETY: handle/token blobs point at owned buffers of the stated size.
    unsafe {
        check(OH_Huks_InitSession(
            &alias,
            params.as_ptr(),
            &mut handle_blob,
            &mut token_blob,
        ))?;
    }
    handle.truncate(handle_blob.size as usize);
    token.truncate(token_blob.size as usize);
    Ok(Session {
        handle,
        token,
        done: false,
    })
}

impl Session {
    fn out_buf(hint: usize) -> Vec<u8> {
        vec![0u8; hint.max(OH_HUKS_MAX_KEY_SIZE as usize)]
    }

    /// The auth token produced by `init`, for key access control flows that
    /// require user authentication. Empty when no authentication is needed.
    pub fn token(&self) -> &[u8] {
        &self.token
    }

    /// Feed a chunk of input; returns any output produced so far.
    pub fn update(&mut self, params: &ParamSet, input: &[u8]) -> Result<Vec<u8>> {
        let handle = blob_in(&self.handle);
        let input_blob = blob_in(input);
        let mut buf = Self::out_buf(input.len() + 64);
        let mut out = OH_Huks_Blob {
            size: buf.len() as u32,
            data: buf.as_mut_ptr(),
        };
        // SAFETY: handle/input are valid; out points at a `buf.len()`-byte buffer.
        unsafe {
            check(OH_Huks_UpdateSession(
                &handle,
                params.as_ptr(),
                &input_blob,
                &mut out,
            ))?;
        }
        buf.truncate(out.size as usize);
        Ok(buf)
    }

    /// Finish the session with a final input chunk; returns the final output
    /// (ciphertext / plaintext / signature / mac). Consumes the session.
    pub fn finish(mut self, params: &ParamSet, input: &[u8]) -> Result<Vec<u8>> {
        self.done = true;
        let handle = blob_in(&self.handle);
        let input_blob = blob_in(input);
        let mut buf = Self::out_buf(input.len() + 64);
        let mut out = OH_Huks_Blob {
            size: buf.len() as u32,
            data: buf.as_mut_ptr(),
        };
        // SAFETY: handle/input are valid; out points at a `buf.len()`-byte buffer.
        unsafe {
            check(OH_Huks_FinishSession(
                &handle,
                params.as_ptr(),
                &input_blob,
                &mut out,
            ))?;
        }
        buf.truncate(out.size as usize);
        Ok(buf)
    }

    /// Abort the session without producing output. Consumes the session.
    pub fn abort(mut self, params: &ParamSet) -> Result<()> {
        self.done = true;
        let handle = blob_in(&self.handle);
        // SAFETY: handle is valid for the call.
        unsafe { check(OH_Huks_AbortSession(&handle, params.as_ptr())) }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.done {
            return;
        }
        if let Ok(params) = ParamSet::empty() {
            let handle = blob_in(&self.handle);
            // SAFETY: handle and the empty param set are valid for the call.
            unsafe { OH_Huks_AbortSession(&handle, params.as_ptr()) };
        }
    }
}
