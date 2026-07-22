use ohos_usb_serial_sys::*;
use std::fmt;

/// Result alias for USB serial operations.
pub type Result<T> = std::result::Result<T, UsbSerialError>;

/// An error returned by a USB serial operation, carrying the raw
/// `UsbSerial_DdkRetCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbSerialError {
    code: i32,
}

impl UsbSerialError {
    pub(crate) fn new(code: i32) -> Self {
        UsbSerialError { code }
    }

    /// The raw return code (`UsbSerial_DdkRetCode`).
    pub fn code(&self) -> i32 {
        self.code
    }
}

impl fmt::Display for UsbSerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "USB serial error {} ({})",
            self.code,
            describe(self.code)
        )
    }
}

impl std::error::Error for UsbSerialError {}

/// Map a raw return code to a short, stable description.
// Every code below is `@since 18`, and this module only exists under `api-18`,
// so no arm needs its own gate.
#[allow(non_upper_case_globals)] // matching bindgen's mixed-case `UsbSerial_DdkRetCode_*` consts
pub fn describe(code: i32) -> &'static str {
    match code as u32 {
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_SUCCESS => "success",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_NO_PERM => "permission denied",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_INVALID_PARAMETER => "invalid parameter",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_INVALID_OPERATION => "invalid operation",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_INIT_ERROR => "ddk not initialized",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_SERVICE_ERROR => "ddk service communication failed",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_MEMORY_ERROR => "memory error",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_IO_ERROR => "i/o error",
        UsbSerial_DdkRetCode_USB_SERIAL_DDK_DEVICE_NOT_FOUND => "device or interface not found",
        _ => "unknown error",
    }
}

/// Reject a length that does not fit the `uint32_t` the C API takes.
pub(crate) fn checked_u32(len: usize) -> Result<u32> {
    u32::try_from(len).map_err(|_| {
        UsbSerialError::new(UsbSerial_DdkRetCode_USB_SERIAL_DDK_INVALID_PARAMETER as i32)
    })
}

/// Turn a raw return code into `Result<()>`.
///
/// Success is `USB_SERIAL_DDK_SUCCESS` (31600000), not 0.
pub(crate) fn check(code: i32) -> Result<()> {
    if code == UsbSerial_DdkRetCode_USB_SERIAL_DDK_SUCCESS as i32 {
        Ok(())
    } else {
        Err(UsbSerialError::new(code))
    }
}
