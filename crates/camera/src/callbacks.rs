use std::sync::mpsc::Sender;
use std::sync::Mutex;

use ohos_camera_sys as sys;

use crate::image::{CameraImageReader, NativePhoto};
use crate::{CameraError, CameraEvent, CameraFocusState, CameraResult, CameraSize};

#[derive(Clone)]
struct EventRoute {
    sender: Sender<CameraEvent>,
    input_key: usize,
    session_key: usize,
    photo_key: Option<usize>,
    photo_size: Option<CameraSize>,
}

enum CameraInstanceState {
    Available,
    Reserved,
    Active(EventRoute),
}

static CAMERA_INSTANCE: Mutex<CameraInstanceState> = Mutex::new(CameraInstanceState::Available);

/// Unique process-wide lease for the device camera.
///
/// CameraKit callbacks do not carry user data. Restricting the binding to one
/// active device instance gives every callback one unambiguous owner and also
/// mirrors CameraKit's exclusive device-open behavior.
pub(crate) struct CameraInstanceLease {
    active: bool,
}

impl CameraInstanceLease {
    pub(crate) fn acquire() -> CameraResult<Self> {
        let mut state = Self::state();
        match &*state {
            CameraInstanceState::Available => {
                *state = CameraInstanceState::Reserved;
                Ok(Self { active: true })
            }
            CameraInstanceState::Reserved | CameraInstanceState::Active(_) => {
                Err(CameraError::invalid_state(
                    "CameraSession::open",
                    "only one CameraSession may own the device camera at a time",
                ))
            }
        }
    }

    pub(crate) fn activate(
        &mut self,
        sender: Sender<CameraEvent>,
        input_key: usize,
        session_key: usize,
        photo_key: Option<usize>,
        photo_size: Option<CameraSize>,
    ) -> CameraResult<()> {
        let mut state = Self::state();
        if !matches!(*state, CameraInstanceState::Reserved) {
            return Err(CameraError::invalid_state(
                "CameraSession::open",
                "camera instance lease is not reserved",
            ));
        }
        *state = CameraInstanceState::Active(EventRoute {
            sender,
            input_key,
            session_key,
            photo_key,
            photo_size,
        });
        Ok(())
    }

    fn state() -> std::sync::MutexGuard<'static, CameraInstanceState> {
        CAMERA_INSTANCE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Drop for CameraInstanceLease {
    fn drop(&mut self) {
        if self.active {
            *Self::state() = CameraInstanceState::Available;
            self.active = false;
        }
    }
}

/// Owns the unavoidable CameraKit callback trampolines.
pub(crate) struct CameraCallbackRouter;

impl CameraCallbackRouter {
    pub(crate) fn input_error_callback() -> sys::OH_CameraInput_OnError {
        Some(Self::on_input_error)
    }

    pub(crate) fn focus_callback() -> sys::OH_CaptureSession_OnFocusStateChange {
        Some(Self::on_focus_state_change)
    }

    pub(crate) fn session_error_callback() -> sys::OH_CaptureSession_OnError {
        Some(Self::on_session_error)
    }

    pub(crate) fn photo_error_callback() -> sys::OH_PhotoOutput_OnError {
        Some(Self::on_photo_error)
    }

    pub(crate) fn photo_available_callback() -> sys::OH_PhotoOutput_PhotoAvailable {
        Some(Self::on_photo_available)
    }

    fn route() -> Option<EventRoute> {
        let state = CAMERA_INSTANCE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match &*state {
            CameraInstanceState::Active(route) => Some(route.clone()),
            CameraInstanceState::Available | CameraInstanceState::Reserved => None,
        }
    }

    fn emit(route: &EventRoute, event: CameraEvent) {
        let _ = route.sender.send(event);
    }

    fn emit_error(route: &EventRoute, operation: &'static str, code: u32) {
        Self::emit(
            route,
            CameraEvent::Error(CameraError::native(operation, code)),
        );
    }

    unsafe extern "C" fn on_input_error(
        input: *const sys::Camera_Input,
        code: sys::Camera_ErrorCode,
    ) {
        let Some(route) = Self::route().filter(|route| route.input_key == input as usize) else {
            return;
        };
        Self::emit_error(&route, "CameraInput callback", code);
    }

    unsafe extern "C" fn on_session_error(
        session: *mut sys::Camera_CaptureSession,
        code: sys::Camera_ErrorCode,
    ) {
        let Some(route) = Self::route().filter(|route| route.session_key == session as usize)
        else {
            return;
        };
        Self::emit_error(&route, "CaptureSession callback", code);
    }

    unsafe extern "C" fn on_focus_state_change(
        session: *mut sys::Camera_CaptureSession,
        state: sys::Camera_FocusState,
    ) {
        let Some(route) = Self::route().filter(|route| route.session_key == session as usize)
        else {
            return;
        };
        let state = CameraFocusState::try_from_raw(state).unwrap_or_default();
        Self::emit(&route, CameraEvent::FocusState(state));
    }

    unsafe extern "C" fn on_photo_error(
        output: *mut sys::Camera_PhotoOutput,
        code: sys::Camera_ErrorCode,
    ) {
        let Some(route) = Self::route().filter(|route| route.photo_key == Some(output as usize))
        else {
            return;
        };
        Self::emit_error(&route, "PhotoOutput callback", code);
    }

    unsafe extern "C" fn on_photo_available(
        output: *mut sys::Camera_PhotoOutput,
        photo: *mut sys::OH_PhotoNative,
    ) {
        // SAFETY: CameraKit transfers one unique owner to this callback.
        let Some(photo) = (unsafe { NativePhoto::from_callback(photo) }) else {
            return;
        };
        let Some(route) = Self::route().filter(|route| route.photo_key == Some(output as usize))
        else {
            return;
        };
        let Some(photo_size) = route.photo_size else {
            Self::emit_error(
                &route,
                "OH_PhotoOutput_PhotoAvailable",
                sys::Camera_ErrorCode_CAMERA_INVALID_ARGUMENT,
            );
            return;
        };
        match CameraImageReader::copy_photo(photo, photo_size) {
            Ok(photo) => Self::emit(&route, CameraEvent::Photo(photo)),
            Err(error) => Self::emit(&route, CameraEvent::Error(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_instance_lease_is_exclusive_and_reusable() {
        let first = CameraInstanceLease::acquire().expect("first lease");
        assert!(CameraInstanceLease::acquire().is_err());
        drop(first);
        assert!(CameraInstanceLease::acquire().is_ok());
    }
}
