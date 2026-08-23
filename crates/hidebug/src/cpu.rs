use ohos_hidebug_sys as sys;

/// CPU usage of one application thread.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThreadCpuUsage {
    pub thread_id: u32,
    pub cpu_usage: f64,
}

/// CPU usage of the whole system. Zero when it could not be obtained.
pub fn system_cpu_usage() -> f64 {
    // SAFETY: no arguments, no out parameters.
    unsafe { sys::OH_HiDebug_GetSystemCpuUsage() }
}

/// CPU usage of the current application process. Zero when the usage is too
/// low to measure or could not be obtained.
pub fn app_cpu_usage() -> f64 {
    // SAFETY: no arguments, no out parameters.
    unsafe { sys::OH_HiDebug_GetAppCpuUsage() }
}

/// Owns the linked list `OH_HiDebug_GetAppThreadCpuUsage` allocates and frees
/// it on drop, so the list is released on every path out of the read below,
/// including an unwind.
struct ThreadCpuUsageList(*mut sys::HiDebug_ThreadCpuUsage);

impl ThreadCpuUsageList {
    /// Collect the list; a null head means no data was collected.
    fn collect() -> Self {
        // SAFETY: no arguments; a null return means no data was collected.
        ThreadCpuUsageList(unsafe { sys::OH_HiDebug_GetAppThreadCpuUsage() })
    }

    fn iter(&self) -> impl Iterator<Item = ThreadCpuUsage> + '_ {
        let mut node = self.0;
        std::iter::from_fn(move || {
            // SAFETY: every node is a live list element until the list is
            // freed, which only happens in `Drop`.
            let current = unsafe { node.as_ref()? };
            node = current.next;
            Some(ThreadCpuUsage {
                thread_id: current.threadId,
                cpu_usage: current.cpuUsage,
            })
        })
    }
}

impl Drop for ThreadCpuUsageList {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        // SAFETY: the head comes from OH_HiDebug_GetAppThreadCpuUsage and is
        // freed exactly once, here.
        unsafe { sys::OH_HiDebug_FreeThreadCpuUsage(&mut self.0) };
    }
}

/// CPU usage of every thread of the current application process.
///
/// The native linked list is copied out and freed before returning; an empty
/// result means no thread data was available.
pub fn app_thread_cpu_usage() -> Vec<ThreadCpuUsage> {
    ThreadCpuUsageList::collect().iter().collect()
}
