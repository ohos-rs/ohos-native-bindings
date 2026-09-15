//! AbilityKit native child-process example with two named descriptors.

mod interop;
mod ipc;
mod parent;
mod probe;
mod suite;

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::AsyncTask, Env, Result, Task};
use ohos_native_child_process_binding::ChildProcessArgsRef;

pub struct ProbeTask {
    forced: bool,
    configured: bool,
}
impl Task for ProbeTask {
    type Output = String;
    type JsValue = String;
    fn compute(&mut self) -> Result<Self::Output> {
        probe::ProbeRunner::new(self.forced, self.configured)
            .run()
            .map_err(|error| napi_ohos::Error::from_reason(error.to_string()))
    }
    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}
/// Runs all blocking platform/socket work on a libuv worker, not ArkUI.
#[napi(ts_return_type = "Promise<string>")]
pub fn run_probe(forced: bool) -> AsyncTask<ProbeTask> {
    AsyncTask::new(ProbeTask {
        forced,
        configured: false,
    })
}

/// Exercises native configs, including the assigned child process name.
#[napi(ts_return_type = "Promise<string>")]
pub fn run_configured_probe(forced: bool) -> AsyncTask<ProbeTask> {
    AsyncTask::new(ProbeTask {
        forced,
        configured: true,
    })
}
fn child_main(
    args: ChildProcessArgsRef<'_>,
) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    probe::ChildProbe::run(args)
}
ohos_native_child_process_binding::native_child_entry!(BindingProbeMain, child_main);

pub struct BindingTask {
    scenario: String,
}
impl Task for BindingTask {
    type Output = String;
    type JsValue = String;
    fn compute(&mut self) -> Result<String> {
        suite::BindingSuite::new(self.scenario.clone())
            .run()
            .map_err(|error| napi_ohos::Error::from_reason(error.to_string()))
    }
    fn resolve(&mut self, _: Env, output: String) -> Result<String> {
        Ok(output)
    }
}

/// Runs a binding scenario on a native worker with real child processes.
#[napi(ts_return_type = "Promise<string>")]
pub fn run_binding_probe(scenario: String) -> AsyncTask<BindingTask> {
    AsyncTask::new(BindingTask { scenario })
}

ohos_native_child_process_binding::native_child_entry!(BindingSuiteMain, suite::SuiteChild::run);
ohos_native_child_process_binding::native_child_entry!(BindingEmptyMain, suite::SuiteChild::empty);
ohos_native_child_process_binding::native_child_entry!(
    BindingHoldMain,
    interop::InteropProbe::native_hold
);

pub struct InteropTask {
    pid: i32,
    kind: String,
}
impl Task for InteropTask {
    type Output = String;
    type JsValue = String;
    fn compute(&mut self) -> Result<String> {
        interop::InteropProbe::new(self.pid, self.kind.clone())
            .run()
            .map_err(|error| napi_ohos::Error::from_reason(error.to_string()))
    }
    fn resolve(&mut self, _: Env, output: String) -> Result<String> {
        Ok(output)
    }
}
/// Observes and terminates an ArkTS-started child using the Rust binding.
#[napi(ts_return_type = "Promise<string>")]
pub fn run_interop_probe(pid: i32, kind: String) -> AsyncTask<InteropTask> {
    AsyncTask::new(InteropTask { pid, kind })
}

#[unsafe(no_mangle)]
pub extern "C" fn NativeChildProcess_OnConnect() -> *mut ipc::RemoteStub {
    match std::panic::catch_unwind(ipc::IpcChild::connect) {
        Ok(stub) => stub,
        Err(payload) => {
            std::mem::forget(payload);
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn NativeChildProcess_MainProc() {
    if let Err(payload) = std::panic::catch_unwind(ipc::IpcChild::main) {
        std::mem::forget(payload);
    }
}
pub struct ParentTask;
impl Task for ParentTask {
    type Output = String;
    type JsValue = String;
    fn compute(&mut self) -> Result<String> {
        parent::ParentProbe::arm().map_err(|error| napi_ohos::Error::from_reason(error.to_string()))
    }
    fn resolve(&mut self, _: Env, output: String) -> Result<String> {
        Ok(output)
    }
}
/// Arms four live children; the E2E host then terminates their parent.
#[napi(ts_return_type = "Promise<string>")]
pub fn arm_parent_exit_probe() -> AsyncTask<ParentTask> {
    AsyncTask::new(ParentTask)
}
ohos_native_child_process_binding::native_child_entry!(
    BindingParentHoldMain,
    parent::ParentProbe::hold
);
