//! Bindings-owned normal extended-child two-FD probe, not a shell simulation.

#[cfg(target_env = "ohos")]
mod probe;
#[cfg(target_env = "ohos")]
pub use exports::{run_probe, ProbeTask};

#[cfg(target_env = "ohos")]
mod exports {
    use napi_derive_ohos::napi;
    use napi_ohos::{bindgen_prelude::AsyncTask, Env, Result, Task};
    use ohos_native_child_process_binding::{ChildLaunchArgs, NativeChildProcessError};

    pub struct ProbeTask {
        forced: bool,
    }
    impl Task for ProbeTask {
        type Output = String;
        type JsValue = String;
        fn compute(&mut self) -> Result<Self::Output> {
            super::probe::ProbeRunner::new()
                .run(self.forced)
                .map_err(|error| napi_ohos::Error::from_reason(error.to_string()))
        }
        fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
            Ok(output)
        }
    }
    /// Runs all blocking platform/socket work on a libuv worker, not ArkUI.
    #[napi(ts_return_type = "Promise<string>")]
    pub fn run_probe(forced: bool) -> AsyncTask<ProbeTask> {
        AsyncTask::new(ProbeTask { forced })
    }
    fn child_main(args: ChildLaunchArgs<'_>) -> std::result::Result<(), NativeChildProcessError> {
        super::probe::ChildProbe::run(args)
    }
    ohos_native_child_process_binding::native_child_entry!(BindingProbeMain, child_main);
}
