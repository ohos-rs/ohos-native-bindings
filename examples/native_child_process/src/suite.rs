//! Device fixtures for the public binding. Native failures keep their raw codes.
use std::{
    ffi::CStr,
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::fd::{AsFd, AsRawFd},
    os::unix::net::UnixStream,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    time::Instant,
};

use ohos_native_child_process_binding::{
    ChildProcessArgs, ChildProcessArgsRef, ChildProcessConfigs, ChildProcessOptions, IsolationMode,
    NativeChildProcess, NativeChildProcessError,
};

use crate::probe::{on_exit, ExitCallback, Frame, Result, PROBE_LOCK, TIMEOUT};

pub(crate) const LIBRARY: &str = "libnative_child_process_example.so";
pub(crate) const ENTRY: &str = "libnative_child_process_example.so:BindingSuiteMain";
pub(crate) const MARKER: &str = "/data/storage/el2/base/haps/entry/files/binding_sandbox_probe";
const EMPTY_MARKER: &str = "/data/storage/el2/base/haps/entry/files/binding_empty_probe";
static SECOND_SENDER: Mutex<Option<mpsc::SyncSender<(i32, i32)>>> = Mutex::new(None);

pub(crate) fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

pub(crate) fn raw_error<T>(
    result: ohos_native_child_process_binding::Result<T>,
    expected: u32,
) -> Result<()> {
    match result {
        Err(NativeChildProcessError::InternalError(code)) if code == expected => Ok(()),
        Err(error) => Err(format!("expected native error {expected}, got {error}").into()),
        Ok(_) => Err(format!("expected native error {expected}, got success").into()),
    }
}

fn invalid_string<T>(result: ohos_native_child_process_binding::Result<T>) -> Result<()> {
    match result {
        Err(error @ NativeChildProcessError::InvalidString(_)) => {
            require(
                std::error::Error::source(&error).is_some(),
                "NUL error lost its source",
            )?;
            require(!error.to_string().is_empty(), "NUL error has no diagnostic")
        }
        _ => Err("interior NUL was not rejected as InvalidString".into()),
    }
}

/// Holds a child until its transport and process lifetime have been observed.
struct FdChild {
    pid: i32,
    control: UnixStream,
    peers: Vec<UnixStream>,
    completed: bool,
    params: String,
}

impl FdChild {
    fn launch(
        params: &str,
        count: usize,
        mode: IsolationMode,
        configs: Option<&mut ChildProcessConfigs>,
    ) -> Result<Self> {
        let mut parents = Vec::new();
        let mut children = Vec::new();
        for _ in 0..count {
            let (parent, child) = UnixStream::pair()?;
            for stream in [&parent, &child] {
                stream.set_read_timeout(Some(TIMEOUT))?;
                stream.set_write_timeout(Some(TIMEOUT))?;
            }
            parents.push(parent);
            children.push(child);
        }
        let mut args = ChildProcessArgs::default();
        let temporary_params = String::from(params);
        args.set_entry_params(&temporary_params)?;
        drop(temporary_params);
        // Reverse insertion verifies native's ordered name lookup, not Vec order.
        for index in (0..count).rev() {
            args.add_fd(&Self::name(index), children[index].as_fd())?;
        }
        require(
            args.entry_params().to_bytes() == params.as_bytes(),
            "owned argument string changed",
        )?;
        require(args.fd_count() == count, "builder FD count changed")?;
        let moved_args = args;
        let pid = match configs {
            Some(configs) => NativeChildProcess::start_with_configs(ENTRY, &moved_args, configs)?,
            None => NativeChildProcess::start(
                ENTRY,
                &moved_args,
                ChildProcessOptions {
                    isolation_mode: mode,
                },
            )?,
        };
        drop(moved_args);
        // FFI and argument Drop must leave every original borrowed FD open.
        for child in &children {
            require(
                unsafe { libc::fcntl(child.as_raw_fd(), libc::F_GETFD) } >= 0,
                "binding closed a borrowed parent FD",
            )?;
        }
        drop(children);
        let control = parents.remove(0);
        Ok(Self {
            pid,
            control,
            peers: parents,
            completed: false,
            params: params.into(),
        })
    }

    fn name(index: usize) -> String {
        if index == 0 {
            "00control".into()
        } else if index == 1 {
            "描述符".into()
        } else {
            format!("fd{index:018}")
        }
    }

    fn ready(&mut self, mode: IsolationMode, independent_uid: bool) -> Result<String> {
        let info = Frame::read(&mut self.control)?;
        let fields: Vec<_> = info.split('|').collect();
        require(fields.len() == 5, &format!("invalid child info: {info}"))?;
        require(
            fields[0].parse::<i32>()? == self.pid && self.pid as u32 != std::process::id(),
            "wrong child PID",
        )?;
        let parent_uid = unsafe { libc::getuid() };
        let uid = fields[1].parse::<u32>()?;
        require(
            (uid != parent_uid) == independent_uid,
            &format!("unexpected UID: parent={parent_uid}, child={uid}"),
        )?;
        let marker = fs::read_to_string(MARKER)?;
        require(
            (fields[3] == marker) == (mode == IsolationMode::Normal),
            "sandbox sharing does not match isolation mode",
        )?;
        require(
            fields[4].parse::<usize>()? == self.peers.len() + 1,
            "native FD list count mismatch",
        )?;
        require(
            Frame::read(&mut self.control)? == self.params,
            "full entry parameter roundtrip mismatch",
        )?;
        for (index, peer) in self.peers.iter_mut().enumerate() {
            peer.write_all(&[index as u8 + 1])?;
            let mut byte = [0];
            peer.read_exact(&mut byte)?;
            require(
                byte[0] == index as u8 + 1,
                "named FD routed to the wrong stream",
            )?;
        }
        Ok(format!("parent_pid={}; child_pid={}; parent_uid={parent_uid}; child_uid={uid}; process_name={}; sandbox={}; named_fds={}; params_bytes={}", std::process::id(), self.pid, fields[2], if mode == IsolationMode::Normal { "shared" } else { "isolated" }, self.peers.len()+1, self.params.len()))
    }

    fn finish(&mut self, command: &str, callback: Option<&ExitCallback>) -> Result<i32> {
        Frame::write(&mut self.control, command)?;
        require(
            Frame::read(&mut self.control)? == "Goodbye",
            "child did not acknowledge exit",
        )?;
        if command == "kill" {
            NativeChildProcess::kill(self.pid)?;
        }
        require(
            Frame::eof(&mut self.control)?,
            "native entry did not close its transport",
        )?;
        let signal = match callback {
            Some(callback) => callback.wait(self.pid)?,
            None => {
                self.wait_gone()?;
                0
            }
        };
        self.wait_gone()?;
        self.completed = true;
        Ok(signal)
    }

    fn wait_gone(&self) -> Result<()> {
        let deadline = Instant::now() + TIMEOUT;
        while Path::new(&format!("/proc/{}", self.pid)).exists() {
            require(
                Instant::now() < deadline,
                "child remained alive after entry return",
            )?;
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        Ok(())
    }
}

impl Drop for FdChild {
    fn drop(&mut self) {
        if !self.completed {
            let _ = NativeChildProcess::kill(self.pid);
        }
    }
}

pub(crate) struct BindingSuite {
    scenario: String,
}

impl BindingSuite {
    pub(crate) fn new(scenario: String) -> Self {
        Self { scenario }
    }

    pub(crate) fn run(&self) -> Result<String> {
        let _probe = PROBE_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        fs::create_dir_all(Path::new(MARKER).parent().ok_or("no marker parent")?)?;
        fs::write(MARKER, format!("parent-{}", std::process::id()))?;
        let result = match self.scenario.as_str() {
            "support" => Self::support(),
            "boundaries" => Self::boundaries(),
            "callbacks" => Self::callbacks(),
            "empty" => Self::empty(),
            "configs_reuse" => Self::configs_reuse(),
            "concurrent" => Self::concurrent(),
            "kill_stale" => Self::kill_stale(),
            scenario
                if scenario.starts_with("fd_missing_")
                    || scenario.starts_with("fd_configs_missing_") =>
            {
                Self::missing_entry(scenario)
            }
            scenario if scenario.starts_with("ipc_") => crate::ipc::IpcProbe::new(scenario).run(),
            scenario => Self::fd(scenario),
        };
        let _ = fs::remove_file(MARKER);
        result.map(|report| format!("scenario={}; {report}", self.scenario))
    }

    fn support() -> Result<String> {
        #[cfg(feature = "api-26")]
        {
            require(
                NativeChildProcess::is_supported(),
                "API26 support query rejected the 2in1 application",
            )?;
            require(
                NativeChildProcess::current_args().is_none(),
                "parent unexpectedly owns native child arguments",
            )?;
            Ok("supported=true; parent_args=none".into())
        }
        #[cfg(not(feature = "api-26"))]
        Err("API26 E2E requires the api-26 example feature".into())
    }

    fn fd(scenario: &str) -> Result<String> {
        let mode = if scenario.contains("isolated") {
            IsolationMode::Isolated
        } else {
            IsolationMode::Normal
        };
        let uid = scenario.ends_with("uid_true") && mode == IsolationMode::Isolated;
        let network = if scenario.starts_with("network_") {
            Some(NetworkPeer::new()?)
        } else {
            None
        };
        let params = match scenario {
            "network_normal" | "network_isolated" => {
                format!("network:{}", network.as_ref().ok_or("no TCP fixture")?.port)
            }
            "params_empty" => String::new(),
            "params_unicode" => "参数🦀é".into(),
            "params_limit" => "p".repeat(150 * 1024),
            "fds_16"
            | "isolated_options"
            | "configs_defaults"
            | "configs_normal_uid_true"
            | "configs_isolated_uid_false"
            | "configs_isolated_uid_true"
            | "configs_kill" => "owned-parameters".into(),
            "entry_error" => "error".into(),
            "entry_panic" => "panic".into(),
            "entry_payload_panic" => "payload_panic".into(),
            "entry_abort" => "abort".into(),
            "nested" => "nested".into(),
            "query_thread" => "query_thread".into(),
            _ => return Err(format!("unknown binding scenario {scenario}").into()),
        };
        let mut callback = ExitCallback::register()?;
        let mut configs = if scenario.starts_with("configs_") {
            Some(ChildProcessConfigs::new()?)
        } else {
            None
        };
        if let Some(configs) = configs.as_mut() {
            if scenario != "configs_defaults" {
                configs.set_isolation_mode(mode)?;
                configs.set_isolation_uid(scenario.ends_with("uid_true"))?;
                let temporary_name = String::from("binding_config_owned");
                configs.set_process_name(&temporary_name)?;
                drop(temporary_name);
            }
        }
        let mut child = FdChild::launch(
            &params,
            if scenario == "fds_16" { 16 } else { 2 },
            mode,
            configs.as_mut(),
        )?;
        drop(configs);
        let mut report = child.ready(mode, uid)?;
        let command = if scenario == "configs_kill" {
            "kill"
        } else {
            "return"
        };
        if scenario == "nested" {
            require(
                Frame::read(&mut child.control)? == "nested=16010005,16010005,16010005,16010005",
                "child creation was not rejected in all four APIs",
            )?;
            report.push_str("; nested_code=16010005");
        }
        if scenario == "query_thread" {
            let query = Frame::read(&mut child.control)?;
            report.push_str(&format!("; thread_args={query}"));
        }
        if network.is_some() {
            let connected = Frame::read(&mut child.control)?;
            require(
                connected
                    == if mode == IsolationMode::Normal {
                        "connected"
                    } else {
                        "rejected"
                    },
                "network environment does not match isolation mode",
            )?;
            report.push_str(&format!(
                "; network={}",
                if mode == IsolationMode::Normal {
                    "shared"
                } else {
                    "isolated"
                }
            ));
        }
        let signal = child.finish(command, Some(&callback))?;
        require(
            signal
                == if scenario == "entry_abort" {
                    libc::SIGABRT
                } else if command == "kill" {
                    libc::SIGKILL
                } else {
                    0
                },
            "unexpected native child exit signal",
        )?;
        callback.close()?;
        Ok(format!(
            "{report}; exit_signal={signal}; eof=true; borrowed_fds=open"
        ))
    }

    fn boundaries() -> Result<String> {
        let file = fs::File::open("/dev/null")?;
        let mut args = ChildProcessArgs::new();
        invalid_string(args.set_entry_params("bad\0params"))?;
        invalid_string(args.add_fd("bad\0name", file.as_fd()))?;
        require(
            args.entry_params() == c"" && args.fd_count() == 0,
            "invalid setter mutated arguments",
        )?;
        let mut configs = ChildProcessConfigs::new()?;
        invalid_string(configs.set_process_name("bad\0name"))?;
        invalid_string(NativeChildProcess::start(
            "bad\0entry",
            &args,
            ChildProcessOptions::default(),
        ))?;
        invalid_string(NativeChildProcess::start_with_configs(
            "bad\0entry",
            &args,
            &mut configs,
        ))?;
        unsafe {
            invalid_string(NativeChildProcess::create(
                "bad\0library",
                crate::ipc::on_started,
            ))?;
            invalid_string(NativeChildProcess::create_with_configs(
                "bad\0library",
                &mut configs,
                crate::ipc::on_started,
            ))?;
        }
        for entry in ["", "no_colon"] {
            raw_error(
                NativeChildProcess::start(entry, &args, ChildProcessOptions::default()),
                401,
            )?;
            raw_error(
                NativeChildProcess::start_with_configs(entry, &args, &mut configs),
                401,
            )?;
        }
        for library in ["", "../libbad.so"] {
            unsafe {
                raw_error(
                    NativeChildProcess::create(library, crate::ipc::on_started),
                    401,
                )?;
                raw_error(
                    NativeChildProcess::create_with_configs(
                        library,
                        &mut configs,
                        crate::ipc::on_started,
                    ),
                    401,
                )?;
            }
        }
        for name in ["".into(), "bad-name".into(), "中文".into(), "n".repeat(65)] {
            raw_error(configs.set_process_name(&name), 401)?;
        }
        configs.set_process_name(&"n".repeat(64))?;
        configs.set_process_name("valid_123")?;
        args.add_fd(&"n".repeat(21), file.as_fd())?;
        raw_error(
            NativeChildProcess::start(ENTRY, &args, ChildProcessOptions::default()),
            401,
        )?;
        raw_error(
            NativeChildProcess::start_with_configs(ENTRY, &args, &mut configs),
            401,
        )?;
        let mut full = ChildProcessArgs::new();
        for index in 0..16 {
            full.add_fd(&format!("fd{index}"), file.as_fd())?;
        }
        require(
            matches!(
                full.add_fd("overflow", file.as_fd()),
                Err(NativeChildProcessError::TooManyFileDescriptors)
            ),
            "17th descriptor not rejected",
        )?;
        require(full.fd_count() == 16, "FD overflow changed the list")?;
        for pid in [-1, 0, i32::MAX, std::process::id() as i32, 1] {
            raw_error(NativeChildProcess::kill(pid), 16010010)?;
        }
        drop(full);
        drop(args);
        require(
            file.metadata().is_ok(),
            "argument failure closed the borrowed file",
        )?;
        Ok("nul_sites=7; invalid_param_code=401; fd_limit=16; overflow=TooManyFileDescriptors; invalid_pid_code=16010010; borrowed_fds=open; name_limit=64".into())
    }

    fn empty() -> Result<String> {
        let _ = fs::remove_file(EMPTY_MARKER);
        let mut callback = ExitCallback::register()?;
        let args = ChildProcessArgs::new();
        require(
            args.fd_count() == 0 && args.entry_params() == c"",
            "empty builder defaults are wrong",
        )?;
        let pid = NativeChildProcess::start(
            "libnative_child_process_example.so:BindingEmptyMain",
            &args,
            ChildProcessOptions::default(),
        )?;
        let signal = callback.wait(pid)?;
        require(signal == 0, "empty child did not exit normally")?;
        require(
            fs::read_to_string(EMPTY_MARKER)? == "empty=true; named_fds=0",
            "empty native entry arguments were not observed",
        )?;
        fs::remove_file(EMPTY_MARKER)?;
        callback.close()?;
        Ok(format!(
            "child_pid={pid}; empty=true; named_fds=0; exit_signal={signal}"
        ))
    }

    fn configs_reuse() -> Result<String> {
        let mut callback = ExitCallback::register()?;
        let mut configs = ChildProcessConfigs::new()?;
        configs.set_isolation_mode(IsolationMode::Isolated)?;
        configs.set_isolation_uid(true)?;
        configs.set_process_name("old_name")?;
        configs.set_isolation_mode(IsolationMode::Normal)?;
        configs.set_isolation_uid(false)?;
        configs.set_process_name(&"n".repeat(64))?;
        raw_error(configs.set_process_name("invalid-name"), 401)?;
        let mut first = FdChild::launch("first", 2, IsolationMode::Normal, Some(&mut configs))?;
        let first_report = first.ready(IsolationMode::Normal, false)?;
        require(
            first_report.contains(&format!(":{};", "n".repeat(64))),
            "invalid setter changed the previous config name",
        )?;
        require(
            first.finish("return", Some(&callback))? == 0,
            "first configs launch failed",
        )?;
        configs.set_process_name("reused_name")?;
        let mut second = FdChild::launch("second", 2, IsolationMode::Normal, Some(&mut configs))?;
        let report = second.ready(IsolationMode::Normal, false)?;
        require(
            report.contains(":reused_name;"),
            "reused config did not replace its owned name",
        )?;
        drop(configs);
        require(
            second.finish("return", Some(&callback))? == 0,
            "second configs launch failed",
        )?;
        callback.close()?;
        Ok(format!(
            "{report}; configs_reused=2; failed_setter=preserved; exit_signal=0"
        ))
    }

    fn concurrent() -> Result<String> {
        let mut callback = ExitCallback::register()?;
        let mut children = Vec::new();
        for index in 0..3 {
            let mut child =
                FdChild::launch(&format!("live-{index}"), 2, IsolationMode::Normal, None)?;
            child.ready(IsolationMode::Normal, false)?;
            require(
                !children
                    .iter()
                    .any(|other: &FdChild| other.pid == child.pid),
                "live children reused a PID",
            )?;
            children.push(child);
        }
        for child in children.iter_mut().rev() {
            require(
                child.finish("return", Some(&callback))? == 0,
                "concurrent child did not exit normally",
            )?;
        }
        callback.close()?;
        Ok("simultaneous_children=3; distinct_pids=true; callbacks=3; eof=true".into())
    }

    fn missing_entry(scenario: &str) -> Result<String> {
        let (mut control, child_control) = UnixStream::pair()?;
        control.set_read_timeout(Some(TIMEOUT))?;
        let mut args = ChildProcessArgs::new();
        args.add_fd("00control", child_control.as_fd())?;
        let entry = if scenario.ends_with("library") {
            "libbinding_missing_child.so:MissingMain"
        } else {
            "libnative_child_process_example.so:BindingMissingMain"
        };
        let mut callback = ExitCallback::register()?;
        let pid = if scenario.contains("configs") {
            NativeChildProcess::start_with_configs(entry, &args, &mut ChildProcessConfigs::new()?)?
        } else {
            NativeChildProcess::start(entry, &args, ChildProcessOptions::default())?
        };
        drop(args);
        drop(child_control);
        // Start acknowledges process creation. dlopen/dlsym happen in the child,
        // unlike Create's asynchronous startup-result callback.
        require(
            Frame::eof(&mut control)?,
            "missing FD entry unexpectedly wrote to its transport",
        )?;
        let signal = callback.wait(pid)?;
        require(
            signal == 0,
            "missing entry cleanup reported an unexpected exit signal",
        )?;
        callback.close()?;
        Ok(format!("child_pid={pid}; launch=acknowledged; entry=not_called; eof=true; exit_signal={signal}"))
    }

    fn kill_stale() -> Result<String> {
        let mut callback = ExitCallback::register()?;
        let mut child = FdChild::launch("stale-pid", 2, IsolationMode::Normal, None)?;
        child.ready(IsolationMode::Normal, false)?;
        require(
            child.finish("return", Some(&callback))? == 0,
            "stale PID fixture did not exit normally",
        )?;
        raw_error(NativeChildProcess::kill(child.pid), 16010010)?;
        callback.close()?;
        Ok("stale_pid_code=16010010; exited=true".into())
    }

    fn callbacks() -> Result<String> {
        let mut callback = ExitCallback::register()?;
        NativeChildProcess::register_exit_callback(on_exit)?;
        let mut second = SecondCallback::register()?;
        let mut child = FdChild::launch("callbacks", 2, IsolationMode::Normal, None)?;
        child.ready(IsolationMode::Normal, false)?;
        require(
            child.finish("return", Some(&callback))? == 0,
            "first callback signal was not zero",
        )?;
        require(
            second.events.recv_timeout(TIMEOUT)? == (child.pid, 0),
            "second registered callback missed the exit",
        )?;
        std::thread::sleep(std::time::Duration::from_millis(150));
        require(
            callback.events.try_recv().is_err(),
            "duplicate callback registration delivered twice",
        )?;
        callback.close()?;
        raw_error(
            NativeChildProcess::unregister_exit_callback(on_exit),
            16010009,
        )?;
        let mut child = FdChild::launch("unregistered", 2, IsolationMode::Normal, None)?;
        child.ready(IsolationMode::Normal, false)?;
        child.finish("return", None)?;
        require(
            second.events.recv_timeout(TIMEOUT)? == (child.pid, 0),
            "remaining callback was removed with the other callback",
        )?;
        require(
            callback
                .events
                .recv_timeout(std::time::Duration::from_millis(150))
                .is_err(),
            "unregistered callback received another child exit",
        )?;
        second.close()?;
        raw_error(
            NativeChildProcess::unregister_exit_callback(on_second_exit),
            16010009,
        )?;
        // Re-register after the native callback set became empty.
        let mut callback = ExitCallback::register()?;
        let mut child = FdChild::launch("registered-again", 2, IsolationMode::Normal, None)?;
        child.ready(IsolationMode::Normal, false)?;
        require(
            child.finish("return", Some(&callback))? == 0,
            "callback re-registration failed",
        )?;
        callback.close()?;
        Ok("deduplicated=true; multiple_callbacks=2; unregistered_delivery=0; missing_callback_code=16010009; reregistered=true".into())
    }
}

extern "C" fn on_second_exit(pid: i32, signal: i32) {
    if let Some(sender) = SECOND_SENDER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .as_ref()
    {
        let _ = sender.try_send((pid, signal));
    }
}

struct SecondCallback {
    events: mpsc::Receiver<(i32, i32)>,
    registered: bool,
}
impl SecondCallback {
    fn register() -> Result<Self> {
        let (sender, events) = mpsc::sync_channel(128);
        *SECOND_SENDER
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(sender);
        NativeChildProcess::register_exit_callback(on_second_exit)?;
        Ok(Self {
            events,
            registered: true,
        })
    }
    fn close(&mut self) -> Result<()> {
        NativeChildProcess::unregister_exit_callback(on_second_exit)?;
        self.registered = false;
        Ok(())
    }
}
impl Drop for SecondCallback {
    fn drop(&mut self) {
        if self.registered {
            let _ = NativeChildProcess::unregister_exit_callback(on_second_exit);
        }
        SECOND_SENDER
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }
}

pub(crate) struct SuiteChild;
impl SuiteChild {
    pub(crate) fn run(args: ChildProcessArgsRef<'_>) -> Result<()> {
        let fds: Vec<_> = args.fds().collect();
        let mut control = UnixStream::from(
            fds.iter()
                .find(|fd| fd.name == c"00control")
                .ok_or("no control FD")?
                .fd
                .try_clone_to_owned()?,
        );
        control.set_read_timeout(Some(TIMEOUT))?;
        control.set_write_timeout(Some(TIMEOUT))?;
        let params = args.entry_params().ok_or("null entry params")?.to_str()?;
        let name = Self::process_name()?;
        let marker = fs::read_to_string(MARKER).unwrap_or_else(|_| "inaccessible".into());
        Frame::write(
            &mut control,
            &format!(
                "{}|{}|{name}|{marker}|{}",
                std::process::id(),
                unsafe { libc::getuid() },
                fds.len()
            ),
        )?;
        Frame::write(&mut control, params)?;
        for index in 1..fds.len() {
            let name = FdChild::name(index);
            let fd = fds
                .iter()
                .find(|fd| fd.name.to_bytes() == name.as_bytes())
                .ok_or("lost named FD")?;
            let mut stream = UnixStream::from(fd.fd.try_clone_to_owned()?);
            let mut byte = [0];
            stream.read_exact(&mut byte)?;
            stream.write_all(&byte)?;
            drop(stream);
            require(
                unsafe { libc::fcntl(fd.fd.as_raw_fd(), libc::F_GETFD) } >= 0,
                "borrowed child FD was adopted or closed",
            )?;
        }
        if params == "nested" {
            let args = ChildProcessArgs::new();
            let mut configs = ChildProcessConfigs::new()?;
            raw_error(
                NativeChildProcess::start(ENTRY, &args, ChildProcessOptions::default()),
                16010005,
            )?;
            raw_error(
                NativeChildProcess::start_with_configs(ENTRY, &args, &mut configs),
                16010005,
            )?;
            unsafe {
                raw_error(
                    NativeChildProcess::create(LIBRARY, crate::ipc::on_started),
                    16010005,
                )?;
                raw_error(
                    NativeChildProcess::create_with_configs(
                        LIBRARY,
                        &mut configs,
                        crate::ipc::on_started,
                    ),
                    16010005,
                )?;
            }
            Frame::write(&mut control, "nested=16010005,16010005,16010005,16010005")?;
        }
        if params == "query_thread" {
            let expected_params = params.to_owned();
            let expected_fds: Vec<_> = fds
                .iter()
                .map(|fd| (fd.name.to_bytes().to_vec(), fd.fd.as_raw_fd()))
                .collect();
            let result = std::thread::spawn(move || match NativeChildProcess::current_args() {
                None => "missing",
                Some(raw) => {
                    let view = unsafe { ChildProcessArgsRef::from_raw(raw.as_ref()) };
                    if view.entry_params().map(CStr::to_bytes) == Some(expected_params.as_bytes())
                        && view
                            .fds()
                            .map(|fd| (fd.name.to_bytes().to_vec(), fd.fd.as_raw_fd()))
                            .eq(expected_fds)
                    {
                        "matched"
                    } else {
                        "mismatch"
                    }
                }
            })
            .join()
            .map_err(|_| "current-args thread panicked")?;
            Frame::write(&mut control, result)?;
        }
        if let Some(port) = params.strip_prefix("network:") {
            let address: SocketAddr = format!("127.0.0.1:{port}").parse()?;
            let connected =
                match TcpStream::connect_timeout(&address, std::time::Duration::from_secs(1)) {
                    Err(_) => false,
                    Ok(mut stream) => {
                        stream.set_read_timeout(Some(TIMEOUT))?;
                        stream.set_write_timeout(Some(TIMEOUT))?;
                        stream.write_all(b"native-network")?;
                        let mut echo = [0; 14];
                        stream.read_exact(&mut echo)?;
                        require(&echo == b"native-network", "TCP network echo mismatch")?;
                        true
                    }
                };
            Frame::write(
                &mut control,
                if connected { "connected" } else { "rejected" },
            )?;
        }
        let command = Frame::read(&mut control)?;
        Frame::write(&mut control, "Goodbye")?;
        if command == "kill" {
            let _ = Frame::read(&mut control)?;
            return Err("child was not killed".into());
        }
        require(command == "return", "unexpected exit command")?;
        match params {
            "error" => Err("intentional entry error".into()),
            "panic" => panic!("intentional entry panic"),
            "payload_panic" => std::panic::panic_any(PanickingPayload),
            "abort" => std::process::abort(),
            _ => Ok(()),
        }
    }

    pub(crate) fn process_name() -> Result<String> {
        let bytes = fs::read("/proc/self/cmdline")?;
        Ok(std::str::from_utf8(bytes.split(|byte| *byte == 0).next().ok_or("no cmdline")?)?.into())
    }

    pub(crate) fn empty(args: ChildProcessArgsRef<'_>) -> Result<()> {
        require(
            args.entry_params() == Some(c"") && args.fds().count() == 0,
            "empty native arguments are invalid",
        )?;
        fs::write(EMPTY_MARKER, "empty=true; named_fds=0")?;
        Ok(())
    }
}

struct PanickingPayload;
impl Drop for PanickingPayload {
    fn drop(&mut self) {
        panic!("panic payload destructor must not run across native ABI")
    }
}

struct NetworkPeer {
    port: u16,
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<Result<()>>>,
}
impl NetworkPeer {
    fn new() -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        listener.set_nonblocking(true)?;
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let worker = std::thread::spawn(move || {
            let deadline = Instant::now() + TIMEOUT;
            while !worker_stop.load(Ordering::Acquire) && Instant::now() < deadline {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_read_timeout(Some(TIMEOUT))?;
                        stream.set_write_timeout(Some(TIMEOUT))?;
                        let mut bytes = [0; 14];
                        stream.read_exact(&mut bytes)?;
                        stream.write_all(&bytes)?;
                        return Ok(());
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(10))
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Ok(())
        });
        Ok(Self {
            port,
            stop,
            worker: Some(worker),
        })
    }
}
impl Drop for NetworkPeer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
