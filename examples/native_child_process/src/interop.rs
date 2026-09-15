//! Cross-language fixtures for KillChildProcess's documented process types.
use crate::{
    probe::{ExitCallback, Result, PROBE_LOCK, TIMEOUT},
    suite::{raw_error, require, MARKER},
};
use ohos_native_child_process_binding::NativeChildProcess;
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub(crate) struct InteropProbe {
    pid: i32,
    kind: String,
}
impl InteropProbe {
    pub(crate) fn new(pid: i32, kind: String) -> Self {
        Self { pid, kind }
    }
    pub(crate) fn marker(pid: i32) -> Result<PathBuf> {
        Ok(Path::new(MARKER)
            .parent()
            .ok_or("no files directory")?
            .join(format!("binding_interop_{pid}")))
    }
    pub(crate) fn run(&self) -> Result<String> {
        let _probe = PROBE_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        require(
            self.pid > 0 && self.pid as u32 != std::process::id(),
            "invalid interop fixture PID",
        )?;
        require(
            matches!(self.kind.as_str(), "native" | "ark_spawn" | "self_fork"),
            "invalid interop kind",
        )?;
        let marker = Self::marker(self.pid)?;
        let deadline = Instant::now() + TIMEOUT;
        while !fs::read_to_string(&marker).is_ok_and(|content| content == "ready") {
            require(
                Instant::now() < deadline,
                "interop child did not reach its entry",
            )?;
            std::thread::sleep(Duration::from_millis(10));
        }
        let mut callback = ExitCallback::register()?;
        let code = if self.kind == "self_fork" {
            raw_error(NativeChildProcess::kill(self.pid), 16010010)?;
            require(
                Path::new(&format!("/proc/{}", self.pid)).exists(),
                "rejected SELF_FORK kill terminated the child",
            )?;
            16010010
        } else {
            NativeChildProcess::kill(self.pid)?;
            0
        };
        if self.kind == "native" {
            require(
                callback.wait(self.pid)? == libc::SIGKILL,
                "ArkTS-started Native child did not deliver SIGKILL exit callback",
            )?;
        }
        while Path::new(&format!("/proc/{}", self.pid)).exists() {
            require(Instant::now() < deadline, "interop child did not exit")?;
            std::thread::sleep(Duration::from_millis(10));
        }
        if self.kind == "self_fork" {
            require(
                fs::read_to_string(&marker)? == "returned",
                "SELF_FORK child did not complete its own entry",
            )?;
        } else {
            require(
                fs::read_to_string(&marker)? == "ready",
                "kill allowed the interop child to finish its entry",
            )?;
        }
        if self.kind != "native" {
            require(
                callback
                    .events
                    .recv_timeout(Duration::from_millis(150))
                    .is_err(),
                "ArkTS-only process delivered a Native exit callback",
            )?;
        }
        callback.close()?;
        fs::remove_file(marker)?;
        Ok(format!(
            "kind={}; child_pid={}; kill_code={code}; exited=true; native_exit_callbacks={}",
            self.kind,
            self.pid,
            if self.kind == "native" { 1 } else { 0 }
        ))
    }

    pub(crate) fn native_hold(
        _: ohos_native_child_process_binding::ChildProcessArgsRef<'_>,
    ) -> Result<()> {
        let marker = Self::marker(std::process::id() as i32)?;
        fs::write(&marker, "ready")?;
        std::thread::sleep(Duration::from_secs(3));
        fs::write(marker, "returned")?;
        Ok(())
    }
}
