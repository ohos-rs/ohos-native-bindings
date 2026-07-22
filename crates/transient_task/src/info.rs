use crate::error::{check, Result};
use crate::task::DelaySuspendInfo;
use ohos_transient_task_sys as sys;

/// The number of concurrent delay requests an application may hold
/// (`TRANSIENT_TASK_MAX_NUM`), which is also the length of the array the system
/// fills in.
pub const MAX_TRANSIENT_TASKS: usize = sys::TRANSIENT_TASK_MAX_NUM as usize;

/// A snapshot of the transient task state of the calling application, mirroring
/// `TransientTask_TransientTaskInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransientTaskInfo {
    remaining_quota: i32,
    tasks: [DelaySuspendInfo; MAX_TRANSIENT_TASKS],
}

impl TransientTaskInfo {
    /// The remaining quota of delay requests.
    pub fn remaining_quota(&self) -> i32 {
        self.remaining_quota
    }

    /// The whole task array as the system left it.
    ///
    /// The C API reports no count, so the slots it did not fill keep the zeroed
    /// value this crate passes in. Use [`TransientTaskInfo::granted`] to iterate
    /// only over slots that carry a request id.
    pub fn tasks(&self) -> &[DelaySuspendInfo; MAX_TRANSIENT_TASKS] {
        &self.tasks
    }

    /// The task slots that carry a non-zero request id.
    pub fn granted(&self) -> impl Iterator<Item = &DelaySuspendInfo> {
        self.tasks.iter().filter(|task| task.request_id() != 0)
    }
}

/// Obtain the transient task info of the calling application.
///
/// ```no_run
/// let info = ohos_transient_task_binding::transient_task_info()?;
/// println!("quota left: {}", info.remaining_quota());
/// for task in info.granted() {
///     println!("request {} for {:?}", task.request_id(), task.actual_delay());
/// }
/// # Ok::<(), ohos_transient_task_binding::TransientTaskError>(())
/// ```
pub fn transient_task_info() -> Result<TransientTaskInfo> {
    let mut raw = sys::TransientTask_TransientTaskInfo {
        remainingQuota: 0,
        transientTasks: [sys::TransientTask_DelaySuspendInfo {
            requestId: 0,
            actualDelayTime: 0,
        }; MAX_TRANSIENT_TASKS],
    };
    // SAFETY: `raw` is a local the service fills in; its task array already has
    // the length the C API expects.
    check(unsafe { sys::OH_BackgroundTaskManager_GetTransientTaskInfo(&mut raw) })?;
    Ok(TransientTaskInfo {
        remaining_quota: raw.remainingQuota,
        tasks: raw.transientTasks.map(DelaySuspendInfo::from_raw),
    })
}
