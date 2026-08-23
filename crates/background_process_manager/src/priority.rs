use ohos_background_process_manager_sys as sys;

/// Scheduling priority a process can be given while it runs in the background.
///
/// The two levels describe how much work the process still has to do once it
/// left the foreground; the resource schedule service uses that hint to decide
/// how many resources the process keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessPriority {
    /// The process has stopped working and only sits in the background.
    Background,
    /// The process is still working in the background.
    Inactive,
}

impl ProcessPriority {
    /// The raw `BackgroundProcessManager_ProcessPriority` value.
    pub(crate) fn raw(self) -> sys::BackgroundProcessManager_ProcessPriority {
        match self {
            ProcessPriority::Background => {
                sys::BackgroundProcessManager_ProcessPriority_PROCESS_BACKGROUND
            }
            ProcessPriority::Inactive => {
                sys::BackgroundProcessManager_ProcessPriority_PROCESS_INACTIVE
            }
        }
    }
}
