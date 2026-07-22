use once_cell::sync::Lazy;

use crate::SysConfig;

/// USB DDK (`@library libusb_ndk.z.so`, `SystemCapability.Driver.USB.Extension`).
///
/// The Base DDK headers (`ddk/*.h`, `@library libddk_base.z.so`) are folded into this
/// crate on purpose: `OH_Usb_SendPipeRequestWithAshmem` takes a `DDK_Ashmem *`, and the
/// only way to obtain one is `OH_DDK_CreateAshmem`. Generating Base DDK as a separate sys
/// crate would produce a second, incompatible `DDK_Ashmem` type, making that API unusable.
pub const USB: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-usb-sys",
    headers: vec![
        "usb/usb_ddk_api.h",
        "usb/usb_ddk_types.h",
        "ddk/ddk_api.h",
        "ddk/ddk_types.h",
    ],
    white_list: vec!["OH_Usb_.*", "OH_DDK_.*", "Usb.*", "USB_DDK_.*", "DDK_.*"],
    block_list: vec![],
    dynamic_library: vec!["usb_ndk.z", "ddk_base.z"],
    extra: "",
});

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
