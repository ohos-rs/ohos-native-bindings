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

/// CPU usage of every thread of the current application process.
///
/// The native linked list is copied out and freed before returning; an empty
/// result means no thread data was available.
pub fn app_thread_cpu_usage() -> Vec<ThreadCpuUsage> {
    // SAFETY: no arguments; a null return means no data was collected.
    let mut head = unsafe { sys::OH_HiDebug_GetAppThreadCpuUsage() };
    let mut usages = Vec::new();
    let mut node = head;
    while !node.is_null() {
        // SAFETY: node is a live list element until the list is freed below.
        let current = unsafe { &*node };
        usages.push(ThreadCpuUsage {
            thread_id: current.threadId,
            cpu_usage: current.cpuUsage,
        });
        node = current.next;
    }
    if !head.is_null() {
        // SAFETY: head comes from OH_HiDebug_GetAppThreadCpuUsage and is freed once.
        unsafe { sys::OH_HiDebug_FreeThreadCpuUsage(&mut head) };
    }
    usages
}
