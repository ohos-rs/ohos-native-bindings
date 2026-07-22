use napi_derive_ohos::napi;
use ohos_background_process_manager_binding as bpm;
use ohos_background_process_manager_binding::ProcessPriority;
use ohos_transient_task_binding as transient;
use ohos_transient_task_binding::SuspendDelay;

const TAG: &str = "BGTASK_TEST";

// The native callback type is `void (*)(void)` and runs on a system thread
// shortly before the granted delay expires. It must never unwind, so the body
// stays empty: the tests below cancel long before the delay runs out.
unsafe extern "C" fn on_expired() {}

// Full request -> inspect -> explicit cancel cycle of the SuspendDelay guard.
#[napi]
pub fn test_transient_request_cycle() -> String {
    let msg = match SuspendDelay::request("bgtasktest request cycle", on_expired) {
        Ok(task) => {
            let request_id = task.request_id();
            let actual_delay_ms = task.actual_delay_time_ms();
            let remaining = match task.remaining_delay_time_ms() {
                Ok(ms) => format!("remaining_ms={ms}"),
                Err(e) => format!("remaining_err({e})"),
            };
            match task.cancel() {
                Ok(()) => format!(
                    "transient request_id={request_id} actual_delay_ms={actual_delay_ms} \
                     {remaining} cancel_ok=true ok=true"
                ),
                Err(e) => format!(
                    "transient request_id={request_id} actual_delay_ms={actual_delay_ms} \
                     {remaining} cancel_ok=false cancel_err({e}) ok=false"
                ),
            }
        }
        Err(e) => format!("transient request err({e}) ok=false"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// The guard cancels on Drop. Request one, let it fall out of scope without
// calling cancel(), and confirm the process survives the implicit cancellation.
#[napi]
pub fn test_transient_drop_cancel() -> String {
    let msg = match SuspendDelay::request("bgtasktest drop cancel", on_expired) {
        Ok(task) => {
            let request_id = task.request_id();
            let actual_delay_ms = task.actual_delay_time_ms();
            drop(task);
            format!(
                "transient drop request_id={request_id} actual_delay_ms={actual_delay_ms} \
                 drop_cancel_ok=true ok=true"
            )
        }
        Err(e) => format!("transient drop request err({e}) drop_cancel_ok=false ok=false"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// transient_task_info() is @since 20 and the device runs API 23. Report the
// quota and the slots that carry a request id.
#[napi]
pub fn test_transient_task_info() -> String {
    let msg = match transient::transient_task_info() {
        Ok(info) => {
            let granted = info.granted().count();
            let ids: Vec<String> = info
                .granted()
                .map(|task| task.request_id().to_string())
                .collect();
            format!(
                "transient_info remaining_quota={} granted={granted} slots={} ids=[{}] ok=true",
                info.remaining_quota(),
                transient::MAX_TRANSIENT_TASKS,
                ids.join(",")
            )
        }
        Err(e) => format!("transient_info err({e}) ok=false"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Lower this very process to background priority, then reset it. The resource
// schedule service may refuse either call; that refusal is a real result and is
// reported as such.
#[napi]
pub fn test_process_priority() -> String {
    let pid = std::process::id() as i32;
    let set = match bpm::set_process_priority(pid, ProcessPriority::Background) {
        Ok(()) => "set_ok=true".to_string(),
        Err(e) => format!("set_ok=false set_err({e}) set_code={}", e.code()),
    };
    let reset = match bpm::reset_process_priority(pid) {
        Ok(()) => "reset_ok=true".to_string(),
        Err(e) => format!("reset_ok=false reset_err({e}) reset_code={}", e.code()),
    };
    let msg = format!("bgpm pid={pid} priority=background {set} {reset}");
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Same pair through the convenience wrappers, which resolve the current pid
// themselves, with Inactive as the hint.
#[napi]
pub fn test_current_process_priority_inactive() -> String {
    let set = match bpm::set_current_process_priority(ProcessPriority::Inactive) {
        Ok(()) => "set_ok=true".to_string(),
        Err(e) => format!("set_ok=false set_err({e}) set_code={}", e.code()),
    };
    let reset = match bpm::reset_current_process_priority() {
        Ok(()) => "reset_ok=true".to_string(),
        Err(e) => format!("reset_ok=false reset_err({e}) reset_code={}", e.code()),
    };
    let msg = format!("bgpm current priority=inactive {set} {reset}");
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
