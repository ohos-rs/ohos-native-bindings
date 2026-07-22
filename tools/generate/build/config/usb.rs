use once_cell::sync::Lazy;

use crate::SysConfig;

/// USB Serial DDK (`@library libusb_serial_ndk.z.so`,
/// `SystemCapability.Driver.UsbSerial.Extension`).
///
/// Every symbol in these headers is `@since 18`, so the whole crate is gated behind
/// `feature = "api-18"`.
pub const USB_SERIAL: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-usb-serial-sys",
    headers: vec![
        "usb_serial/usb_serial_api.h",
        "usb_serial/usb_serial_types.h",
    ],
    white_list: vec!["OH_UsbSerial_.*", "UsbSerial_.*", "USB_SERIAL_.*"],
    block_list: vec![],
    dynamic_library: vec!["usb_serial_ndk.z"],
    extra: "",
});
