use std::{
    io::{Read, Write},
    os::fd::AsFd,
    os::unix::net::UnixStream,
    path::Path,
    sync::mpsc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ohos_native_child_process_binding::{
    ChildExitOutcome, ChildFdName, ChildLaunchArgs, ChildProcessArgsBuilder, ChildProcessEntry,
    ChildProcessHandle, ChildProcessOptions, IsolationMode, NativeChildProcessError as Error,
    NativeChildProcessManager,
};

const PARAMS: &str = "binding-probe-v1";
const TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) struct ProbeRunner {
    manager: NativeChildProcessManager,
}

impl ProbeRunner {
    pub(crate) fn new() -> Self {
        Self {
            manager: NativeChildProcessManager::new(),
        }
    }
    pub(crate) fn run(&self, forced: bool) -> Result<String, Error> {
        #[cfg(feature = "api-26")]
        if !self.manager.is_supported()? {
            return Err(Error::NotSupported);
        }
        let (mut control, launch_control) =
            UnixStream::pair().map_err(|error| io_error("control socketpair", error))?;
        let (mut echo, launch_echo) =
            UnixStream::pair().map_err(|error| io_error("echo socketpair", error))?;
        for stream in [&control, &launch_control, &echo, &launch_echo] {
            stream
                .set_read_timeout(Some(TIMEOUT))
                .map_err(|error| io_error("read timeout", error))?;
            stream
                .set_write_timeout(Some(TIMEOUT))
                .map_err(|error| io_error("write timeout", error))?;
        }
        let entry = ChildProcessEntry::new(
            Path::new("libnative_child_process_example.so"),
            "BindingProbeMain",
        )?;
        let args = ChildProcessArgsBuilder::new()
            .entry_params(PARAMS)?
            .named_fd(ChildFdName::new("probe.control")?, launch_control.as_fd())?
            .named_fd(ChildFdName::new("probe.echo")?, launch_echo.as_fd())?;
        let child = self.manager.start(
            &entry,
            args,
            ChildProcessOptions::new(IsolationMode::Normal),
        )?;
        // No parent copy of either child endpoint survives the synchronous call.
        drop(launch_control);
        drop(launch_echo);
        let result = self.exchange(&child, &mut control, &mut echo, forced);
        // Error cleanup is explicit; the identity handle does not kill on Drop.
        if result.is_err() {
            let _ = child.kill();
        }
        result
    }
    fn exchange(
        &self,
        child: &ChildProcessHandle,
        control: &mut UnixStream,
        echo: &mut UnixStream,
        forced: bool,
    ) -> Result<String, Error> {
        let (tx, rx) = mpsc::sync_channel(1);
        let _subscription = child.subscribe_exit(move |event| {
            let _ = tx.try_send(event);
        })?;
        let ready = Frame::read(control)?;
        let announced_pid = ready
            .strip_prefix("Ready:")
            .and_then(|pid| pid.parse::<i32>().ok())
            .ok_or(Error::InvalidChildArguments)?;
        let parent_pid = std::process::id();
        if announced_pid != child.pid().get() || announced_pid as u32 == parent_pid {
            return Err(Error::InvalidChildArguments);
        }
        let nonce = format!(
            "Echo:{}:{}",
            parent_pid,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::Internal)?
                .as_nanos()
        );
        Frame::write(echo, &nonce)?;
        if Frame::read(echo)? != nonce {
            return Err(Error::InvalidChildArguments);
        }
        if forced {
            Frame::write(control, "Hold")?;
            if Frame::read(control)? != "Holding" {
                return Err(Error::InvalidChildArguments);
            }
            child.kill()?;
        } else {
            Frame::write(control, "Shutdown")?;
            if Frame::read(control)? != "Goodbye" {
                return Err(Error::InvalidChildArguments);
            }
        }
        // EOF is mandatory independent transport evidence, not a callback guess.
        if !Frame::eof(control)? {
            return Err(Error::InvalidChildArguments);
        }
        let event = rx.recv_timeout(TIMEOUT).map_err(|_| Error::Timeout)?;
        if event.pid != child.pid() || event.generation != child.generation() {
            return Err(Error::StaleHandle);
        }
        let signal = match event.outcome {
            ChildExitOutcome::Signal(signal) => signal,
            _ => return Err(Error::ObservationAmbiguous),
        };
        Ok(format!("parent_pid={parent_pid}; child_pid={announced_pid}; generation={}; named_fds=2; params={PARAMS}; echo=matched; eof=true; forced={forced}; exit_signal={signal}; mode=normal", child.generation()))
    }
}

pub(crate) struct ChildProbe;

impl ChildProbe {
    pub(crate) fn run(mut args: ChildLaunchArgs<'_>) -> Result<(), Error> {
        if args.entry_params().to_bytes() != PARAMS.as_bytes() || args.fd_count() != 2 {
            return Err(Error::InvalidChildArguments);
        }
        let mut control = UnixStream::from(args.take_fd(&ChildFdName::new("probe.control")?)?);
        let mut echo = UnixStream::from(args.take_fd(&ChildFdName::new("probe.echo")?)?);
        Frame::write(&mut control, &format!("Ready:{}", std::process::id()))?;
        let nonce = Frame::read(&mut echo)?;
        if !nonce.starts_with("Echo:") {
            return Err(Error::InvalidChildArguments);
        }
        Frame::write(&mut echo, &nonce)?;
        match Frame::read(&mut control)?.as_str() {
            "Shutdown" => Frame::write(&mut control, "Goodbye"),
            "Hold" => {
                Frame::write(&mut control, "Holding")?;
                // Wait for the parent's explicit kill or EOF, not a shell child.
                let _ = Frame::read(&mut control)?;
                Err(Error::InvalidChildArguments)
            }
            _ => Err(Error::InvalidChildArguments),
        }
    }
}

/// Tiny bounded line frames for the generic probe, not a product protocol.
struct Frame;
impl Frame {
    fn write(stream: &mut UnixStream, frame: &str) -> Result<(), Error> {
        if frame.len() > 256 || frame.contains('\n') {
            return Err(Error::InvalidChildArguments);
        }
        stream
            .write_all(frame.as_bytes())
            .map_err(|error| io_error("write frame", error))?;
        stream
            .write_all(b"\n")
            .map_err(|error| io_error("write terminator", error))
    }
    fn read(stream: &mut UnixStream) -> Result<String, Error> {
        let mut data = Vec::new();
        loop {
            let mut byte = [0];
            match stream.read(&mut byte) {
                Ok(0) => return Err(Error::InvalidChildArguments),
                Ok(_) if byte[0] == b'\n' => {
                    return String::from_utf8(data).map_err(|_| Error::InvalidChildArguments)
                }
                Ok(_) => {
                    if data.len() == 256 {
                        return Err(Error::InvalidChildArguments);
                    }
                    data.push(byte[0]);
                }
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(io_error("read frame", error)),
            }
        }
    }
    fn eof(stream: &mut UnixStream) -> Result<bool, Error> {
        loop {
            match stream.read(&mut [0]) {
                Ok(count) => return Ok(count == 0),
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(io_error("read EOF", error)),
            }
        }
    }
}

fn io_error(operation: &'static str, error: std::io::Error) -> Error {
    Error::Io {
        operation,
        code: error.raw_os_error().unwrap_or(0),
    }
}
