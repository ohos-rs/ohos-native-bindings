use ohos_enum_macro::EnumFrom;
use ohos_qos_sys::*;

mod error;
pub use error::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(QoS_Level, "QoS_Level_QOS_")]
pub enum QosLevel {
    Background,
    Utility,
    Default,
    UserInitiated,
    DeadlineRequest,
    UserInteractive,
}

pub fn set_thread_qos(level: QosLevel) -> Result<(), QosError> {
    let ret = unsafe { OH_QoS_SetThreadQoS(level.into()) };
    if ret == 0 {
        Ok(())
    } else {
        Err(QosError::InternalError)
    }
}

pub fn reset_thread_qos() -> Result<(), QosError> {
    let ret = unsafe { OH_QoS_ResetThreadQoS() };
    if ret == 0 {
        Ok(())
    } else {
        Err(QosError::InternalError)
    }
}

pub fn get_thread_qos() -> Result<QosLevel, QosError> {
    let mut raw_level: QoS_Level = 0;
    let ret = unsafe { OH_QoS_GetThreadQoS(&mut raw_level as *mut _) };
    if ret != 0 {
        return Err(QosError::InternalError);
    }
    QosLevel::try_from_raw(raw_level).ok_or(QosError::InvalidValue)
}

#[cfg(feature = "api-20")]
pub type GewuSession = OH_QoS_GewuSession;

#[cfg(feature = "api-20")]
pub type GewuRequest = OH_QoS_GewuRequest;

#[cfg(feature = "api-20")]
pub fn gewu_create_session(attributes: &str) -> Result<GewuSession, GewuError> {
    let c_attributes = std::ffi::CString::new(attributes).map_err(|_| GewuError::InvalidString)?;
    let result = unsafe { OH_QoS_GewuCreateSession(c_attributes.as_ptr()) };
    let error = GewuErrorCode::try_from_raw(result.error).ok_or(GewuError::InvalidCode)?;
    if error == GewuErrorCode::Ok {
        Ok(result.session)
    } else {
        Err(GewuError::Api(error))
    }
}

#[cfg(feature = "api-20")]
pub fn gewu_destroy_session(session: GewuSession) -> Result<(), GewuError> {
    let error = GewuErrorCode::try_from_raw(unsafe { OH_QoS_GewuDestroySession(session) })
        .ok_or(GewuError::InvalidCode)?;
    if error == GewuErrorCode::Ok {
        Ok(())
    } else {
        Err(GewuError::Api(error))
    }
}

#[cfg(feature = "api-20")]
pub fn gewu_abort_request(session: GewuSession, request: GewuRequest) -> Result<(), GewuError> {
    let error = GewuErrorCode::try_from_raw(unsafe { OH_QoS_GewuAbortRequest(session, request) })
        .ok_or(GewuError::InvalidCode)?;
    if error == GewuErrorCode::Ok {
        Ok(())
    } else {
        Err(GewuError::Api(error))
    }
}

#[cfg(feature = "api-20")]
pub fn gewu_submit_request(
    session: GewuSession,
    request: &str,
    callback: OH_QoS_GewuOnResponse,
    context: *mut std::os::raw::c_void,
) -> Result<GewuRequest, GewuError> {
    let c_request = std::ffi::CString::new(request).map_err(|_| GewuError::InvalidString)?;
    let result =
        unsafe { OH_QoS_GewuSubmitRequest(session, c_request.as_ptr(), callback, context) };
    let error = GewuErrorCode::try_from_raw(result.error).ok_or(GewuError::InvalidCode)?;
    if error == GewuErrorCode::Ok {
        Ok(result.request)
    } else {
        Err(GewuError::Api(error))
    }
}
