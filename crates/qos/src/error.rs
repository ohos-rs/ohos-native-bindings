#[cfg(feature = "api-20")]
use ohos_enum_macro::EnumFrom;
#[cfg(feature = "api-20")]
use ohos_qos_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QosError {
    InternalError,
    InvalidValue,
}

#[cfg(feature = "api-20")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(OH_QoS_GewuErrorCode, "OH_QoS_GewuErrorCode_OH_QOS_GEWU_")]
pub enum GewuErrorCode {
    Ok,
    Noperm,
    Nomem,
    Inval,
    Exist,
    Noent,
    Nosys,
    Fault,
}

#[cfg(feature = "api-20")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GewuError {
    InvalidString,
    InvalidCode,
    Api(GewuErrorCode),
}
