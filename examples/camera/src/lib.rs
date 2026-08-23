use std::sync::{
    mpsc::{self, Sender},
    Mutex, OnceLock,
};

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, Result};
use ohos_camera_binding::{
    CameraConfiguration, CameraEvent, CameraFlashMode, CameraSession, CameraSurface,
    CameraXComponentAttachment, CameraXComponentEvent,
};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::{NativeXComponent, XComponent, XComponentRaw};

enum Command {
    Surface(CameraXComponentEvent),
    Info(Sender<String>),
    Capture(Sender<String>),
    Flash {
        on: bool,
        reply: Sender<String>,
    },
    /// Explicit lifecycle control for the E2E suite: drop the session and
    /// reopen it against the last seen surface.
    Reopen(Sender<String>),
    Close(Sender<String>),
}

static LAST: Mutex<String> = Mutex::new(String::new());
static COMMANDS: OnceLock<Sender<Command>> = OnceLock::new();

fn set_last(msg: impl Into<String>) {
    let msg = msg.into();
    hilog_info!(format!("camera: {msg}"));
    *LAST.lock().unwrap() = msg;
}

fn commands() -> Result<Sender<Command>> {
    COMMANDS
        .get()
        .cloned()
        .ok_or_else(|| Error::from_reason("camera worker is not running"))
}

fn request(cmd_fn: impl FnOnce(Sender<String>) -> Command) -> Result<String> {
    let (reply_tx, reply_rx) = mpsc::channel();
    commands()?
        .send(cmd_fn(reply_tx))
        .map_err(|_| Error::from_reason("camera worker stopped"))?;
    reply_rx
        .recv()
        .map_err(|_| Error::from_reason("camera worker did not reply"))
}

fn worker(rx: mpsc::Receiver<Command>, events: Sender<CameraEvent>) {
    let mut session: Option<CameraSession> = None;
    let mut last_surface: Option<CameraSurface> = None;
    while let Ok(command) = rx.recv() {
        match command {
            Command::Surface(CameraXComponentEvent::Surface(surface)) => {
                last_surface = Some(surface);
                open_session(surface, events.clone(), &mut session);
            }
            Command::Surface(CameraXComponentEvent::SurfaceLost) => {
                session = None;
                set_last("surface lost");
            }
            Command::Info(reply) => {
                let _ = reply.send(describe_session(session.as_ref()));
            }
            Command::Capture(reply) => {
                let msg = match session.as_ref() {
                    Some(current) => current
                        .capture()
                        .map(|_| "capture requested".to_string())
                        .unwrap_or_else(|e| format!("capture ERR {e}")),
                    None => "camera session is not open".to_string(),
                };
                let _ = reply.send(msg);
            }
            Command::Flash { on, reply } => {
                let mode = if on {
                    CameraFlashMode::On
                } else {
                    CameraFlashMode::Off
                };
                let msg = match session.as_mut() {
                    Some(current) => current
                        .set_flash_mode(mode)
                        .map(|_| format!("flash={mode:?}"))
                        .unwrap_or_else(|e| format!("flash ERR {e}")),
                    None => "camera session is not open".to_string(),
                };
                let _ = reply.send(msg);
            }
            Command::Reopen(reply) => {
                let msg = match last_surface {
                    Some(surface) => {
                        // Drop the old session first: the CameraInstanceLease
                        // inside is what serializes camera access, so a full
                        // close-then-open cycle is the lifecycle under test.
                        session = None;
                        open_session(surface, events.clone(), &mut session);
                        if session.is_some() {
                            "reopened".to_string()
                        } else {
                            "reopen failed".to_string()
                        }
                    }
                    None => "no surface seen yet".to_string(),
                };
                let _ = reply.send(msg);
            }
            Command::Close(reply) => {
                let had = session.is_some();
                session = None;
                set_last("session closed");
                let _ = reply.send(if had {
                    "closed".to_string()
                } else {
                    "was not open".to_string()
                });
            }
        }
    }
}

fn open_session(
    surface: CameraSurface,
    events: Sender<CameraEvent>,
    session: &mut Option<CameraSession>,
) {
    let config = CameraConfiguration {
        surface,
        enable_photo_output: true,
        ..Default::default()
    };
    match CameraSession::open(config, events) {
        Ok(opened) => {
            set_last(describe_session(Some(&opened)));
            *session = Some(opened);
        }
        Err(e) => {
            *session = None;
            set_last(format!("open ERR {e}"));
        }
    }
}

fn describe_session(session: Option<&CameraSession>) -> String {
    let Some(session) = session else {
        let last = LAST.lock().unwrap().clone();
        if last.is_empty() {
            return "(waiting for XComponent surface)".to_string();
        }
        return last;
    };
    let info = session.info();
    let caps = session.capabilities();
    let controls = session.controls();
    format!(
        "position={:?} preview={}x{} photo={:?} preview_sizes={} photo_sizes={} flash={:?} torch={:?}",
        info.position,
        info.preview_size.width,
        info.preview_size.height,
        info.photo_size,
        caps.preview_sizes.len(),
        caps.photo_sizes.len(),
        controls.flash_mode,
        controls.torch_mode,
    )
}

#[napi(module_exports)]
pub fn init(exports: Object, env: Env) -> Result<()> {
    // Outside an XComponent host (e.g. imported by a test runner without a
    // native surface) there is no __NATIVE_XCOMPONENT_OBJ__ in exports; skip
    // binding instead of failing module registration.
    let xcomponent = match XComponent::init(env, exports) {
        Ok(xc) => xc,
        Err(e) => {
            hilog_info!("no XComponent surface, skip init: {e}");
            return Ok(());
        }
    };
    let native = NativeXComponent::new(XComponentRaw(xcomponent.raw()));
    let (surface_tx, surface_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();
    let (cmd_tx, cmd_rx) = mpsc::channel();
    let _ = COMMANDS.set(cmd_tx.clone());
    CameraXComponentAttachment::attach(native, surface_tx)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    std::thread::Builder::new()
        .name("camera-demo".into())
        .spawn(move || worker(cmd_rx, event_tx))
        .map_err(|e| Error::from_reason(e.to_string()))?;

    std::thread::Builder::new()
        .name("camera-surface".into())
        .spawn(move || {
            while let Ok(event) = surface_rx.recv() {
                let _ = cmd_tx.send(Command::Surface(event));
            }
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;

    std::thread::Builder::new()
        .name("camera-events".into())
        .spawn(move || {
            while let Ok(event) = event_rx.recv() {
                match event {
                    CameraEvent::Photo(_) => hilog_info!("camera event: photo"),
                    CameraEvent::Frame(frame) => {
                        hilog_info!(format!(
                            "camera event: frame {}x{} ts={}",
                            frame.size.width, frame.size.height, frame.timestamp_ns
                        ));
                    }
                    CameraEvent::FocusState(state) => {
                        hilog_info!(format!("camera event: focus {state:?}"));
                    }
                    CameraEvent::Error(error) => {
                        hilog_info!(format!("camera event: error {error}"));
                    }
                }
            }
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(())
}

#[napi]
pub fn last_result() -> String {
    let last = LAST.lock().unwrap().clone();
    if last.is_empty() {
        "(waiting for XComponent surface)".to_string()
    } else {
        last
    }
}

#[napi]
pub fn session_info() -> Result<String> {
    request(Command::Info)
}

#[napi]
pub fn capture() -> Result<String> {
    request(Command::Capture)
}

#[napi]
pub fn switch_flash(on: bool) -> Result<String> {
    request(|reply| Command::Flash { on, reply })
}

/// Drop the camera session and reopen it against the current surface. The
/// binding's CameraInstanceLease serializes camera access, so this exercises
/// the full close -> reopen lifecycle in one process.
#[napi]
pub fn reopen_session() -> Result<String> {
    request(Command::Reopen)
}

/// Explicitly drop the camera session (the surface stays alive; the next
/// reopen_session brings it back).
#[napi]
pub fn close_session() -> Result<String> {
    request(Command::Close)
}
