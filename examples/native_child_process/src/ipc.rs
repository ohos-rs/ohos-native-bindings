//! IPCKit is used only by this example to exercise the binding's IPC API.
//! Declarations follow SDK26 IPCKit/ipc_cparcel.h and ipc_cremote_object.h.
use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::{self, NonNull},
    sync::{mpsc, Condvar, Mutex},
    thread::ThreadId,
    time::{Duration, Instant},
};

use ohos_native_child_process_binding::{
    sys, ChildProcessConfigs, IsolationMode, NativeChildProcess,
};

use crate::{
    probe::{ExitCallback, Result, TIMEOUT},
    suite::{require, SuiteChild, LIBRARY, MARKER},
};

#[repr(C)]
pub struct RemoteStub {
    _opaque: [u8; 0],
}
#[repr(C)]
struct ParcelRaw {
    _opaque: [u8; 0],
}
#[repr(C)]
struct DeathRecipient {
    _opaque: [u8; 0],
}

#[link(name = "ipc_capi")]
unsafe extern "C" {
    fn OH_IPCRemoteStub_Create(
        descriptor: *const c_char,
        request: Option<
            unsafe extern "C" fn(u32, *const ParcelRaw, *mut ParcelRaw, *mut c_void) -> i32,
        >,
        destroy: Option<unsafe extern "C" fn(*mut c_void)>,
        user_data: *mut c_void,
    ) -> *mut RemoteStub;
    fn OH_IPCRemoteStub_Destroy(stub: *mut RemoteStub);
    fn OH_IPCRemoteProxy_Destroy(proxy: *mut sys::OHIPCRemoteProxy);
    fn OH_IPCRemoteProxy_SendRequest(
        proxy: *const sys::OHIPCRemoteProxy,
        code: u32,
        data: *const ParcelRaw,
        reply: *mut ParcelRaw,
        option: *const c_void,
    ) -> i32;
    fn OH_IPCRemoteProxy_IsRemoteDead(proxy: *const sys::OHIPCRemoteProxy) -> i32;
    fn OH_IPCDeathRecipient_Create(
        death: Option<unsafe extern "C" fn(*mut c_void)>,
        destroy: Option<unsafe extern "C" fn(*mut c_void)>,
        user_data: *mut c_void,
    ) -> *mut DeathRecipient;
    fn OH_IPCDeathRecipient_Destroy(recipient: *mut DeathRecipient);
    fn OH_IPCRemoteProxy_AddDeathRecipient(
        proxy: *mut sys::OHIPCRemoteProxy,
        recipient: *mut DeathRecipient,
    ) -> i32;
    fn OH_IPCRemoteProxy_RemoveDeathRecipient(
        proxy: *mut sys::OHIPCRemoteProxy,
        recipient: *mut DeathRecipient,
    ) -> i32;
    fn OH_IPCParcel_Create() -> *mut ParcelRaw;
    fn OH_IPCParcel_Destroy(parcel: *mut ParcelRaw);
    fn OH_IPCParcel_WriteString(parcel: *mut ParcelRaw, value: *const c_char) -> i32;
    fn OH_IPCParcel_ReadString(parcel: *const ParcelRaw) -> *const c_char;
}

fn ipc_check(code: i32) -> Result<()> {
    require(code == 0, &format!("IPCKit failed: {code}"))
}

struct Parcel(NonNull<ParcelRaw>);
impl Parcel {
    fn new() -> Result<Self> {
        Ok(Self(
            NonNull::new(unsafe { OH_IPCParcel_Create() }).ok_or("IPC parcel allocation failed")?,
        ))
    }
    fn write(&mut self, value: &str) -> Result<()> {
        let value = CString::new(value)?;
        ipc_check(unsafe { OH_IPCParcel_WriteString(self.0.as_ptr(), value.as_ptr()) })
    }
    fn read(&self) -> Result<String> {
        let raw = unsafe { OH_IPCParcel_ReadString(self.0.as_ptr()) };
        require(!raw.is_null(), "IPC reply string is null")?;
        Ok(unsafe { CStr::from_ptr(raw) }.to_str()?.into())
    }
}
impl Drop for Parcel {
    fn drop(&mut self) {
        unsafe { OH_IPCParcel_Destroy(self.0.as_ptr()) }
    }
}

struct Proxy(NonNull<sys::OHIPCRemoteProxy>);
// IPCKit proxies support requests from different native threads. Ownership
// transfers from the official startup callback to this example's NAPI worker.
unsafe impl Send for Proxy {}
impl Proxy {
    fn request(&self, code: u32, text: &str) -> Result<String> {
        let mut data = Parcel::new()?;
        let reply = Parcel::new()?;
        data.write(text)?;
        // A null options pointer requests synchronous IPC without a packed ABI.
        ipc_check(unsafe {
            OH_IPCRemoteProxy_SendRequest(
                self.0.as_ptr(),
                code,
                data.0.as_ptr(),
                reply.0.as_ptr(),
                ptr::null(),
            )
        })?;
        reply.read()
    }
    fn wait_dead(&self, death: &DeathWatch<'_>) -> Result<()> {
        death.events.recv_timeout(TIMEOUT)?;
        let deadline = Instant::now() + TIMEOUT;
        while unsafe { OH_IPCRemoteProxy_IsRemoteDead(self.0.as_ptr()) } == 0 {
            require(
                Instant::now() < deadline,
                "IPC child remained alive after MainProc returned",
            )?;
            std::thread::sleep(Duration::from_millis(20));
        }
        Ok(())
    }
}
impl Drop for Proxy {
    fn drop(&mut self) {
        unsafe { OH_IPCRemoteProxy_Destroy(self.0.as_ptr()) }
    }
}

type Startup = (i32, Option<Proxy>, ThreadId);
struct DeathWatch<'a> {
    proxy: &'a Proxy,
    raw: NonNull<DeathRecipient>,
    events: mpsc::Receiver<()>,
}
impl<'a> DeathWatch<'a> {
    fn new(proxy: &'a Proxy) -> Result<Self> {
        let (sender, events) = mpsc::sync_channel(1);
        let user_data = Box::into_raw(Box::new(sender));
        let raw = unsafe {
            OH_IPCDeathRecipient_Create(Some(on_death), Some(on_death_destroy), user_data.cast())
        };
        let Some(raw) = NonNull::new(raw) else {
            unsafe { drop(Box::from_raw(user_data)) };
            return Err("IPC death recipient allocation failed".into());
        };
        let watch = Self { proxy, raw, events };
        ipc_check(unsafe { OH_IPCRemoteProxy_AddDeathRecipient(proxy.0.as_ptr(), raw.as_ptr()) })?;
        Ok(watch)
    }
}
impl Drop for DeathWatch<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ =
                OH_IPCRemoteProxy_RemoveDeathRecipient(self.proxy.0.as_ptr(), self.raw.as_ptr());
            OH_IPCDeathRecipient_Destroy(self.raw.as_ptr());
        }
    }
}
unsafe extern "C" fn on_death(user_data: *mut c_void) {
    let sender = unsafe { &*user_data.cast::<mpsc::SyncSender<()>>() };
    let _ = sender.try_send(());
}
unsafe extern "C" fn on_death_destroy(user_data: *mut c_void) {
    unsafe { drop(Box::from_raw(user_data.cast::<mpsc::SyncSender<()>>())) };
}
static STARTED: Mutex<Option<mpsc::SyncSender<Startup>>> = Mutex::new(None);

pub(crate) unsafe extern "C" fn on_started(code: i32, proxy: *mut sys::OHIPCRemoteProxy) {
    let proxy = NonNull::new(proxy).map(Proxy);
    if let Some(sender) = STARTED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .as_ref()
    {
        // On delivery failure the owning Proxy is dropped and released here.
        let _ = sender.try_send((code, proxy, std::thread::current().id()));
    }
}

struct StartupWaiter {
    events: mpsc::Receiver<Startup>,
}
impl StartupWaiter {
    fn new() -> Self {
        let (sender, events) = mpsc::sync_channel(8);
        *STARTED
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(sender);
        Self { events }
    }
}
impl Drop for StartupWaiter {
    fn drop(&mut self) {
        STARTED
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }
}

pub(crate) struct IpcProbe<'a> {
    scenario: &'a str,
}
impl<'a> IpcProbe<'a> {
    pub(crate) fn new(scenario: &'a str) -> Self {
        Self { scenario }
    }
    pub(crate) fn run(&self) -> Result<String> {
        let waiter = StartupWaiter::new();
        let caller_thread = std::thread::current().id();
        let isolated = self.scenario.contains("isolated");
        let uid = self.scenario.ends_with("uid_true");
        let configured = self.scenario != "ipc_normal"
            && self.scenario != "ipc_missing"
            && self.scenario != "ipc_kill";
        let library = if self.scenario == "ipc_missing" || self.scenario == "ipc_configs_missing" {
            "libbinding_missing_child.so"
        } else if self.scenario == "ipc_missing_exports" {
            "libipc_capi.so"
        } else {
            LIBRARY
        };
        let expected_error = if self.scenario.contains("missing") {
            16010007
        } else if self.scenario == "ipc_null_connect" {
            16010008
        } else {
            0
        };
        let mut callback = ExitCallback::register()?;
        if configured {
            let mut configs = ChildProcessConfigs::new()?;
            configs.set_isolation_mode(if isolated {
                IsolationMode::Isolated
            } else {
                IsolationMode::Normal
            })?;
            configs.set_isolation_uid(uid)?;
            configs.set_process_name(if self.scenario == "ipc_null_connect" {
                "binding_null_connect"
            } else {
                "binding_ipc_owned"
            })?;
            unsafe { NativeChildProcess::create_with_configs(library, &mut configs, on_started) }?;
        } else {
            unsafe { NativeChildProcess::create(library, on_started) }?;
        }
        let (code, proxy, callback_thread) = waiter.events.recv_timeout(TIMEOUT)?;
        require(
            code == expected_error,
            &format!("unexpected IPC startup result: expected {expected_error}, got {code}"),
        )?;
        require(
            callback_thread != caller_thread,
            "IPC startup callback ran on the calling thread",
        )?;
        if expected_error != 0 {
            require(proxy.is_none(), "failed IPC startup returned a proxy")?;
            callback.close()?;
            return Ok(format!(
                "startup_code={code}; proxy=null; callback_thread=independent"
            ));
        }
        let proxy = proxy.ok_or("successful IPC startup returned a null proxy")?;
        let death = DeathWatch::new(&proxy)?;
        let info = proxy.request(1, "info")?;
        let fields: Vec<_> = info.split('|').collect();
        require(
            fields.len() == 5,
            &format!("invalid IPC child report: {info}"),
        )?;
        let pid = fields[0].parse::<i32>()?;
        require(
            pid > 0 && pid as u32 != std::process::id(),
            "IPC child did not run in another process",
        )?;
        let parent_uid = unsafe { libc::getuid() };
        let child_uid = fields[1].parse::<u32>()?;
        require(
            (parent_uid != child_uid) == (isolated && uid),
            "IPC child UID does not match configs",
        )?;
        if configured {
            require(
                fields[2].ends_with(":binding_ipc_owned"),
                "IPC configs process name was not retained",
            )?;
        }
        let marker = std::fs::read_to_string(MARKER)?;
        require(
            (fields[3] == marker) == !isolated,
            "IPC sandbox sharing does not match configs",
        )?;
        require(
            fields[4] == "none",
            "IPC-only child unexpectedly has FD launch arguments",
        )?;
        let nonce = format!("IPC-🦀-{}-{pid}", std::process::id());
        require(
            proxy.request(2, &nonce)? == nonce,
            "IPC request/reply mismatch",
        )?;
        if self.scenario == "ipc_kill" {
            NativeChildProcess::kill(pid)?;
        } else {
            require(
                proxy.request(3, "shutdown")? == "Goodbye",
                "IPC shutdown was not acknowledged",
            )?;
        }
        proxy.wait_dead(&death)?;
        require(
            waiter
                .events
                .recv_timeout(Duration::from_millis(150))
                .is_err(),
            "IPC startup callback delivered twice",
        )?;
        // The official Native exit callback intentionally excludes Create APIs.
        require(
            callback
                .events
                .recv_timeout(Duration::from_millis(150))
                .is_err(),
            "IPC Create incorrectly triggered a Native FD exit callback",
        )?;
        drop(death);
        drop(proxy);
        callback.close()?;
        Ok(format!("startup_code=0; child_pid={pid}; parent_uid={parent_uid}; child_uid={child_uid}; process_name={}; sandbox={}; callback_thread=independent; ipc=matched; remote_dead=true; proxy=released; fd_exit_callbacks=0", fields[2], if isolated {"isolated"} else {"shared"}))
    }
}

struct Stub(NonNull<RemoteStub>);
unsafe impl Send for Stub {}
impl Drop for Stub {
    fn drop(&mut self) {
        unsafe { OH_IPCRemoteStub_Destroy(self.0.as_ptr()) }
    }
}
static STUB: Mutex<Option<Stub>> = Mutex::new(None);
static STOP: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());

pub(crate) struct IpcChild;
pub(crate) struct HeldIpcChild {
    pub(crate) pid: i32,
    _proxy: Proxy,
}
impl HeldIpcChild {
    pub(crate) fn launch(configured: bool) -> Result<Self> {
        let waiter = StartupWaiter::new();
        if configured {
            let mut configs = ChildProcessConfigs::new()?;
            configs.set_process_name("binding_parent_ipc_configs")?;
            unsafe { NativeChildProcess::create_with_configs(LIBRARY, &mut configs, on_started) }?;
        } else {
            unsafe { NativeChildProcess::create(LIBRARY, on_started) }?;
        }
        let (code, proxy, _) = waiter.events.recv_timeout(TIMEOUT)?;
        require(code == 0, "parent-exit IPC fixture failed to start")?;
        let proxy = proxy.ok_or("parent-exit IPC fixture returned no proxy")?;
        let info = proxy.request(1, "info")?;
        let pid = info.split('|').next().ok_or("no held IPC PID")?.parse()?;
        Ok(Self { pid, _proxy: proxy })
    }
}
impl Drop for HeldIpcChild {
    fn drop(&mut self) {
        let _ = NativeChildProcess::kill(self.pid);
    }
}
impl IpcChild {
    pub(crate) fn connect() -> *mut RemoteStub {
        if SuiteChild::process_name().is_ok_and(|name| name.ends_with(":binding_null_connect")) {
            return ptr::null_mut();
        }
        let raw = unsafe {
            OH_IPCRemoteStub_Create(
                c"ohos.rs.binding.child.probe".as_ptr(),
                Some(on_request),
                None,
                ptr::null_mut(),
            )
        };
        if let Some(raw) = NonNull::new(raw) {
            *STUB
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Stub(raw));
        }
        raw
    }

    pub(crate) fn main() {
        let stop = STOP
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let _ = STOP.1.wait_timeout_while(stop, TIMEOUT, |stop| !*stop);
        // Let the shutdown request's reply leave its binder callback first.
        std::thread::sleep(Duration::from_millis(100));
        STUB.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }

    unsafe fn request(code: u32, data: *const ParcelRaw, reply: *mut ParcelRaw) -> Result<()> {
        let result = match code {
            1 => format!(
                "{}|{}|{}|{}|{}",
                std::process::id(),
                unsafe { libc::getuid() },
                SuiteChild::process_name()?,
                std::fs::read_to_string(MARKER).unwrap_or_else(|_| "inaccessible".into()),
                if NativeChildProcess::current_args().is_none() {
                    "none"
                } else {
                    "present"
                }
            ),
            2 => {
                let raw = unsafe { OH_IPCParcel_ReadString(data) };
                require(!raw.is_null(), "IPC request string is null")?;
                unsafe { CStr::from_ptr(raw) }.to_str()?.into()
            }
            3 => "Goodbye".into(),
            _ => return Err("unexpected IPC request code".into()),
        };
        let result = CString::new(result)?;
        ipc_check(unsafe { OH_IPCParcel_WriteString(reply, result.as_ptr()) })?;
        if code == 3 {
            *STOP
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner) = true;
            STOP.1.notify_all();
        }
        Ok(())
    }
}

unsafe extern "C" fn on_request(
    code: u32,
    data: *const ParcelRaw,
    reply: *mut ParcelRaw,
    _: *mut c_void,
) -> i32 {
    match std::panic::catch_unwind(|| unsafe { IpcChild::request(code, data, reply) }) {
        Ok(Ok(())) => 0,
        Ok(Err(_)) => 1909001,
        Err(payload) => {
            std::mem::forget(payload);
            1909001
        }
    }
}
