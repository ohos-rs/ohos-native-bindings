//! Parent-death requires a host-driven test: its test process is terminated.
use crate::{
    interop::InteropProbe,
    ipc::HeldIpcChild,
    probe::{Result, PROBE_LOCK, TIMEOUT},
    suite::require,
};
use ohos_native_child_process_binding::{
    ChildProcessArgs, ChildProcessArgsRef, ChildProcessConfigs, ChildProcessOptions,
    NativeChildProcess,
};
use std::{
    fs,
    path::Path,
    sync::Mutex,
    time::{Duration, Instant},
};

struct ParentChildren {
    fds: Vec<i32>,
    ipc: Vec<HeldIpcChild>,
}
impl Drop for ParentChildren {
    fn drop(&mut self) {
        for &pid in &self.fds {
            let _ = NativeChildProcess::kill(pid);
        }
    }
}
static CHILDREN: Mutex<Option<ParentChildren>> = Mutex::new(None);

pub(crate) struct ParentProbe;
impl ParentProbe {
    pub(crate) fn arm() -> Result<String> {
        let _probe = PROBE_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut held = ParentChildren {
            fds: Vec::new(),
            ipc: Vec::new(),
        };
        let args = ChildProcessArgs::new();
        let entry = "libnative_child_process_example.so:BindingParentHoldMain";
        held.fds.push(NativeChildProcess::start(
            entry,
            &args,
            ChildProcessOptions::default(),
        )?);
        let mut configs = ChildProcessConfigs::new()?;
        configs.set_process_name("binding_parent_fd_configs")?;
        held.fds.push(NativeChildProcess::start_with_configs(
            entry,
            &args,
            &mut configs,
        )?);
        for &pid in &held.fds {
            let marker = InteropProbe::marker(pid)?;
            let deadline = Instant::now() + TIMEOUT;
            while !fs::read_to_string(&marker).is_ok_and(|content| content == "ready") {
                require(
                    Instant::now() < deadline,
                    "parent-exit FD fixture did not reach its entry",
                )?;
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        held.ipc.push(HeldIpcChild::launch(false)?);
        held.ipc.push(HeldIpcChild::launch(true)?);
        let pids: Vec<_> = held
            .fds
            .iter()
            .copied()
            .chain(held.ipc.iter().map(|child| child.pid))
            .collect();
        for &pid in &pids {
            require(
                Path::new(&format!("/proc/{pid}")).exists(),
                "parent-exit child was not alive when armed",
            )?;
        }
        let report = format!("parent_pid={}; child_pids={},{},{},{}; apis=start,start_with_configs,create,create_with_configs; armed=true",std::process::id(),pids[0],pids[1],pids[2],pids[3]);
        *CHILDREN
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(held);
        Ok(report)
    }
    pub(crate) fn hold(_: ChildProcessArgsRef<'_>) -> Result<()> {
        fs::write(InteropProbe::marker(std::process::id() as i32)?, "ready")?;
        std::thread::sleep(Duration::from_secs(60));
        Ok(())
    }
}
