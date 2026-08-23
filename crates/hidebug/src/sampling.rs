use crate::error::{check, HiDebugError, Result};
use ohos_hidebug_sys as sys;

/// Callback receiving the sampled stacks as a NUL-terminated string.
pub type ThreadLiteSamplingCallback = unsafe extern "C" fn(stacks: *const std::os::raw::c_char);

/// Sample the stacks of `tids` at `frequency` Hz for `duration` milliseconds.
///
/// The calling thread is blocked until sampling completes, after which
/// `callback` is invoked with the collected stacks.
pub fn request_thread_lite_sampling(
    tids: &[u32],
    frequency: u32,
    duration: u32,
    callback: ThreadLiteSamplingCallback,
) -> Result<()> {
    let size = u32::try_from(tids.len())
        .map_err(|_| HiDebugError::from_code(sys::HiDebug_ErrorCode_HIDEBUG_INVALID_ARGUMENT))?;
    let mut config = sys::HiDebug_ProcessSamplerConfig {
        tids: tids.as_ptr() as *mut u32,
        size,
        frequency,
        duration,
        reserved: 0,
    };
    // SAFETY: the call blocks until sampling finishes, so the borrowed tids
    // outlive hidebug's use of them.
    unsafe {
        check(sys::OH_HiDebug_RequestThreadLiteSampling(
            &mut config,
            Some(callback),
        ))
    }
}
