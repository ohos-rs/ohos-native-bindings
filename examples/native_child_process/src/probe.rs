use std::{
    io::{Read, Write},
    os::fd::{AsFd, AsRawFd},
    os::unix::net::UnixStream,
    sync::{mpsc, Mutex},
    time::{Duration, Instant},
};

use ohos_native_child_process_binding::{
    ChildProcessArgs, ChildProcessArgsRef, ChildProcessConfigs, ChildProcessOptions, IsolationMode,
    NativeChildProcess,
};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const PARAMS: &str = "binding-probe-v1";
pub(crate) const TIMEOUT: Duration = Duration::from_secs(10);
const PROCESS_NAME: &str = "native_child_process_probe";
// This example serializes its probes and owns its explicit native callback.
pub(crate) static PROBE_LOCK: Mutex<()> = Mutex::new(());
static EXIT_SENDER: Mutex<Option<mpsc::SyncSender<(i32, i32)>>> = Mutex::new(None);

pub(crate) extern "C" fn on_exit(pid: i32, signal: i32) {
    let sender = EXIT_SENDER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(sender) = sender.as_ref() {
        let _ = sender.try_send((pid, signal));
    }
}

pub(crate) struct ExitCallback {
    pub(crate) events: mpsc::Receiver<(i32, i32)>,
    registered: bool,
}

impl ExitCallback {
    pub(crate) fn register() -> Result<Self> {
        let (sender, events) = mpsc::sync_channel(128);
        *EXIT_SENDER
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(sender);
        if let Err(error) = NativeChildProcess::register_exit_callback(on_exit) {
            EXIT_SENDER
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .take();
            return Err(error.into());
        }
        Ok(Self {
            events,
            registered: true,
        })
    }

    pub(crate) fn wait(&self, pid: i32) -> Result<i32> {
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let (exited, signal) = self
                .events
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))?;
            if exited == pid {
                return Ok(signal);
            }
            if Instant::now() >= deadline {
                return Err("native child exit callback timed out".into());
            }
        }
    }

    pub(crate) fn close(&mut self) -> Result<()> {
        NativeChildProcess::unregister_exit_callback(on_exit)?;
        self.registered = false;
        Ok(())
    }
}

impl Drop for ExitCallback {
    fn drop(&mut self) {
        if self.registered {
            let _ = NativeChildProcess::unregister_exit_callback(on_exit);
        }
        EXIT_SENDER
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }
}

pub(crate) struct ProbeRunner {
    forced: bool,
    configured: bool,
}

impl ProbeRunner {
    pub(crate) fn new(forced: bool, configured: bool) -> Self {
        Self { forced, configured }
    }

    pub(crate) fn run(&self) -> Result<String> {
        let _probe = PROBE_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        #[cfg(feature = "api-26")]
        if !NativeChildProcess::is_supported() {
            return Err("native child processes are not supported".into());
        }
        let (mut control, child_control) = UnixStream::pair()?;
        let (mut echo, child_echo) = UnixStream::pair()?;
        for stream in [&control, &child_control, &echo, &child_echo] {
            stream.set_read_timeout(Some(TIMEOUT))?;
            stream.set_write_timeout(Some(TIMEOUT))?;
        }
        let mut callback = ExitCallback::register()?;
        let mut args = ChildProcessArgs::new();
        args.set_entry_params(PARAMS)?
            .add_fd("probe.control", child_control.as_fd())?
            .add_fd("probe.echo", child_echo.as_fd())?;
        let entry = "libnative_child_process_example.so:BindingProbeMain";
        let pid = if self.configured {
            let mut configs = ChildProcessConfigs::new()?;
            configs.set_isolation_mode(IsolationMode::Normal)?;
            configs.set_process_name(PROCESS_NAME)?;
            configs.set_isolation_uid(false)?;
            NativeChildProcess::start_with_configs(entry, &args, &mut configs)?
        } else {
            NativeChildProcess::start(entry, &args, ChildProcessOptions::default())?
        };
        drop(args);
        drop(child_control);
        drop(child_echo);
        let result = self.exchange(pid, &mut control, &mut echo, &callback);
        if result.is_err() {
            let _ = NativeChildProcess::kill(pid);
        }
        callback.close()?;
        result
    }

    fn exchange(
        &self,
        pid: i32,
        control: &mut UnixStream,
        echo: &mut UnixStream,
        callback: &ExitCallback,
    ) -> Result<String> {
        let ready = Frame::read(control)?;
        if let Some(error) = ready.strip_prefix("Error:") {
            return Err(format!("child probe failed: {error}").into());
        }
        let announced_pid = ready
            .strip_prefix("Ready:")
            .and_then(|pid| pid.parse::<i32>().ok());
        let parent_pid = std::process::id();
        if announced_pid != Some(pid) || pid as u32 == parent_pid {
            return Err("child PID does not match the native launch result".into());
        }
        let current_args = Frame::read(control)?;
        let current_args = current_args
            .strip_prefix("CurrentArgs:")
            .ok_or("missing current-argument query result")?;
        if !matches!(current_args, "matched" | "missing" | "mismatch") {
            return Err("invalid current-argument query result".into());
        }
        let name = Frame::read(control)?;
        let process_name = name
            .strip_prefix("Name:")
            .ok_or("missing child process name")?;
        if self.configured && !process_name.ends_with(&format!(":{PROCESS_NAME}")) {
            return Err(format!("unexpected configured child process name: {process_name}").into());
        }
        let nonce = format!("Echo:{parent_pid}:{pid}");
        Frame::write(echo, &nonce)?;
        if Frame::read(echo)? != nonce {
            return Err("child echo mismatch".into());
        }
        if self.forced {
            Frame::write(control, "Hold")?;
            if Frame::read(control)? != "Holding" {
                return Err("child did not acknowledge Hold".into());
            }
            NativeChildProcess::kill(pid)?;
        } else {
            Frame::write(control, "Shutdown")?;
            if Frame::read(control)? != "Goodbye" {
                return Err("child did not acknowledge Shutdown".into());
            }
        }
        if !Frame::eof(control)? {
            return Err("child control transport did not close".into());
        }
        let signal = callback.wait(pid)?;
        let forced = self.forced;
        let configured = self.configured;
        Ok(format!("parent_pid={parent_pid}; child_pid={pid}; named_fds=2; params={PARAMS}; current_args={current_args}; echo=matched; eof=true; forced={forced}; configured={configured}; process_name={process_name}; exit_signal={signal}; mode=normal"))
    }
}

pub(crate) struct ChildProbe;

impl ChildProbe {
    pub(crate) fn run(args: ChildProcessArgsRef<'_>) -> Result<()> {
        let result = Self::exchange(args);
        if let Err(error) = &result {
            if let Some(control) = args.fds().find(|fd| fd.name == c"probe.control") {
                if let Ok(fd) = control.fd.try_clone_to_owned() {
                    let _ = Frame::write(&mut UnixStream::from(fd), &format!("Error:{error}"));
                }
            }
        }
        result
    }

    fn exchange(args: ChildProcessArgsRef<'_>) -> Result<()> {
        if args.entry_params() != Some(c"binding-probe-v1") || args.fds().count() != 2 {
            return Err("invalid child arguments".into());
        }
        let current_args = match NativeChildProcess::current_args() {
            None => "missing",
            Some(current) => {
                // SAFETY: System-owned argument storage is borrowed only
                // within the native entry; no native FD is closed here.
                let current = unsafe { ChildProcessArgsRef::from_raw(current.as_ref()) };
                if current.entry_params() == args.entry_params()
                    && current
                        .fds()
                        .map(|fd| (fd.name, fd.fd.as_raw_fd()))
                        .eq(args.fds().map(|fd| (fd.name, fd.fd.as_raw_fd())))
                {
                    "matched"
                } else {
                    "mismatch"
                }
            }
        };
        let control = args
            .fds()
            .find(|fd| fd.name == c"probe.control")
            .ok_or("missing control FD")?;
        let echo = args
            .fds()
            .find(|fd| fd.name == c"probe.echo")
            .ok_or("missing echo FD")?;
        let mut control = UnixStream::from(control.fd.try_clone_to_owned()?);
        let mut echo = UnixStream::from(echo.fd.try_clone_to_owned()?);
        Frame::write(&mut control, &format!("Ready:{}", std::process::id()))?;
        Frame::write(&mut control, &format!("CurrentArgs:{current_args}"))?;
        let cmdline = std::fs::read("/proc/self/cmdline")?;
        let name = cmdline
            .split(|byte| *byte == 0)
            .next()
            .ok_or("missing process name")?;
        Frame::write(
            &mut control,
            &format!("Name:{}", std::str::from_utf8(name)?),
        )?;
        let nonce = Frame::read(&mut echo)?;
        if !nonce.starts_with("Echo:") {
            return Err("invalid echo frame".into());
        }
        Frame::write(&mut echo, &nonce)?;
        match Frame::read(&mut control)?.as_str() {
            "Shutdown" => Frame::write(&mut control, "Goodbye"),
            "Hold" => {
                Frame::write(&mut control, "Holding")?;
                let _ = Frame::read(&mut control)?;
                Err("Hold ended without forced termination".into())
            }
            _ => Err("invalid control frame".into()),
        }
    }
}

/// Bounded line framing used only by the example's control protocol.
pub(crate) struct Frame;

impl Frame {
    pub(crate) fn write(stream: &mut UnixStream, frame: &str) -> Result<()> {
        if frame.len() > 160 * 1024 || frame.contains('\n') {
            return Err("invalid frame".into());
        }
        stream.write_all(frame.as_bytes())?;
        stream.write_all(b"\n")?;
        Ok(())
    }

    pub(crate) fn read(stream: &mut UnixStream) -> Result<String> {
        let mut data = Vec::new();
        loop {
            let mut byte = [0];
            match stream.read(&mut byte) {
                Ok(0) => return Err("unexpected control EOF".into()),
                Ok(_) if byte[0] == b'\n' => return Ok(String::from_utf8(data)?),
                Ok(_) => {
                    if data.len() == 160 * 1024 {
                        return Err("frame exceeds 160 KiB".into());
                    }
                    data.push(byte[0]);
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error.into()),
            }
        }
    }

    pub(crate) fn eof(stream: &mut UnixStream) -> Result<bool> {
        loop {
            match stream.read(&mut [0]) {
                Ok(count) => return Ok(count == 0),
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error.into()),
            }
        }
    }
}
