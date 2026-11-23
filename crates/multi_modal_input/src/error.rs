use std::fmt;

use ohos_multi_modal_input_sys as sys;

/// Errors returned by OpenHarmony multimodal InputKit native APIs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputError {
    PermissionDenied,
    NotSystemApplication,
    ParameterError,
    DeviceNotSupported,
    ServiceException,
    RepeatInterceptor,
    #[cfg(feature = "api-14")]
    OccupiedBySystem,
    #[cfg(feature = "api-14")]
    OccupiedByOther,
    #[cfg(feature = "api-15")]
    KeyboardDeviceNotExist,
    #[cfg(feature = "api-20")]
    AppNotFocused,
    #[cfg(feature = "api-20")]
    DeviceNoPointer,
    #[cfg(feature = "api-22")]
    InvalidWindowId,
    AllocationFailed,
    Other(u32),
}

impl InputError {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(code: sys::Input_Result) -> Self {
        match code {
            sys::Input_Result_INPUT_PERMISSION_DENIED => Self::PermissionDenied,
            sys::Input_Result_INPUT_NOT_SYSTEM_APPLICATION => Self::NotSystemApplication,
            sys::Input_Result_INPUT_PARAMETER_ERROR => Self::ParameterError,
            sys::Input_Result_INPUT_DEVICE_NOT_SUPPORTED => Self::DeviceNotSupported,
            sys::Input_Result_INPUT_SERVICE_EXCEPTION => Self::ServiceException,
            sys::Input_Result_INPUT_REPEAT_INTERCEPTOR => Self::RepeatInterceptor,
            #[cfg(feature = "api-14")]
            sys::Input_Result_INPUT_OCCUPIED_BY_SYSTEM => Self::OccupiedBySystem,
            #[cfg(feature = "api-14")]
            sys::Input_Result_INPUT_OCCUPIED_BY_OTHER => Self::OccupiedByOther,
            #[cfg(feature = "api-15")]
            sys::Input_Result_INPUT_KEYBOARD_DEVICE_NOT_EXIST => Self::KeyboardDeviceNotExist,
            #[cfg(feature = "api-20")]
            sys::Input_Result_INPUT_APP_NOT_FOCUSED => Self::AppNotFocused,
            #[cfg(feature = "api-20")]
            sys::Input_Result_INPUT_DEVICE_NO_POINTER => Self::DeviceNoPointer,
            #[cfg(feature = "api-22")]
            sys::Input_Result_INPUT_INVALID_WINDOWID => Self::InvalidWindowId,
            other => Self::Other(other),
        }
    }

    pub const fn code(self) -> Option<u32> {
        match self {
            Self::PermissionDenied => Some(sys::Input_Result_INPUT_PERMISSION_DENIED),
            Self::NotSystemApplication => Some(sys::Input_Result_INPUT_NOT_SYSTEM_APPLICATION),
            Self::ParameterError => Some(sys::Input_Result_INPUT_PARAMETER_ERROR),
            Self::DeviceNotSupported => Some(sys::Input_Result_INPUT_DEVICE_NOT_SUPPORTED),
            Self::ServiceException => Some(sys::Input_Result_INPUT_SERVICE_EXCEPTION),
            Self::RepeatInterceptor => Some(sys::Input_Result_INPUT_REPEAT_INTERCEPTOR),
            #[cfg(feature = "api-14")]
            Self::OccupiedBySystem => Some(sys::Input_Result_INPUT_OCCUPIED_BY_SYSTEM),
            #[cfg(feature = "api-14")]
            Self::OccupiedByOther => Some(sys::Input_Result_INPUT_OCCUPIED_BY_OTHER),
            #[cfg(feature = "api-15")]
            Self::KeyboardDeviceNotExist => Some(sys::Input_Result_INPUT_KEYBOARD_DEVICE_NOT_EXIST),
            #[cfg(feature = "api-20")]
            Self::AppNotFocused => Some(sys::Input_Result_INPUT_APP_NOT_FOCUSED),
            #[cfg(feature = "api-20")]
            Self::DeviceNoPointer => Some(sys::Input_Result_INPUT_DEVICE_NO_POINTER),
            #[cfg(feature = "api-22")]
            Self::InvalidWindowId => Some(sys::Input_Result_INPUT_INVALID_WINDOWID),
            Self::AllocationFailed => None,
            Self::Other(code) => Some(code),
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PermissionDenied => f.write_str("input permission denied"),
            Self::NotSystemApplication => f.write_str("input API requires system application"),
            Self::ParameterError => f.write_str("invalid input parameter"),
            Self::DeviceNotSupported => f.write_str("input API not supported on this device"),
            Self::ServiceException => f.write_str("multimodal input service exception"),
            Self::RepeatInterceptor => f.write_str("input interceptor already registered"),
            #[cfg(feature = "api-14")]
            Self::OccupiedBySystem => f.write_str("hotkey is occupied by the system"),
            #[cfg(feature = "api-14")]
            Self::OccupiedByOther => f.write_str("hotkey is occupied by another application"),
            #[cfg(feature = "api-15")]
            Self::KeyboardDeviceNotExist => f.write_str("keyboard device does not exist"),
            #[cfg(feature = "api-20")]
            Self::AppNotFocused => f.write_str("application is not focused"),
            #[cfg(feature = "api-20")]
            Self::DeviceNoPointer => f.write_str("no pointer device is available"),
            #[cfg(feature = "api-22")]
            Self::InvalidWindowId => f.write_str("invalid window id"),
            Self::AllocationFailed => f.write_str("native input object allocation failed"),
            Self::Other(code) => write!(f, "multimodal input error code {code}"),
        }
    }
}

impl std::error::Error for InputError {}

pub type Result<T> = std::result::Result<T, InputError>;

#[allow(non_upper_case_globals)]
pub(crate) fn check_status(code: sys::Input_Result) -> Result<()> {
    if code == sys::Input_Result_INPUT_SUCCESS {
        Ok(())
    } else {
        Err(InputError::from_raw(code))
    }
}
