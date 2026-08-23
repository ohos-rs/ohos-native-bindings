//! On-device checks for `ohos-usb-serial-binding`.
//!
//! The USB serial DDK needs `ohos.permission.ACCESS_DDK_USB_SERIAL` and a real
//! CDC-ACM device to do anything useful. The functions below are split in two
//! groups: those that hold without any device attached (error-code table, local
//! parameter types, `Ddk` init/release lifetime) and those that need one (open
//! plus the configure/write/read sequence). The second group reports whatever
//! the DDK actually answers instead of pretending a device is there.
//!
//! Everything runs on the calling (UI) thread, so no call is allowed to block
//! for long: read timeouts are kept at 100 ms and `Timeout::Infinite` is never
//! applied.

use napi_derive_ohos::napi;
use ohos_usb_serial_binding::sys;
use ohos_usb_serial_binding::{
    describe, Ddk, FlowControl, Parity, SerialDevice, SerialParams, StopBits, Timeout,
    UsbSerialError,
};

const TAG: &str = "USBSERIAL_TEST";

/// Device ids come from the USB DDK (`OH_Usb_GetDevices`), which this crate does
/// not wrap, so the device-backed checks probe a few low ids instead.
const PROBE_DEVICE_IDS: [u64; 4] = [0, 1, 2, 3];

fn err_text(e: UsbSerialError) -> String {
    format!("code={} desc=\"{}\"", e.code(), describe(e.code()))
}

fn step<T>(name: &str, r: Result<T, UsbSerialError>, out: &mut Vec<String>) -> bool {
    match r {
        Ok(_) => {
            out.push(format!("{name}=Ok"));
            true
        }
        Err(e) => {
            out.push(format!("{name}=Err({})", err_text(e)));
            false
        }
    }
}

fn finish(msg: String) -> String {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// Translate every return code the sys crate exposes, plus two codes that are
/// not part of the enum, and check the table has no gaps.
#[napi]
pub fn test_usb_serial_error_table() -> String {
    let codes: [(&str, u32); 9] = [
        ("SUCCESS", sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_SUCCESS),
        ("NO_PERM", sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_NO_PERM),
        (
            "INVALID_PARAMETER",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_INVALID_PARAMETER,
        ),
        (
            "INVALID_OPERATION",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_INVALID_OPERATION,
        ),
        (
            "INIT_ERROR",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_INIT_ERROR,
        ),
        (
            "SERVICE_ERROR",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_SERVICE_ERROR,
        ),
        (
            "MEMORY_ERROR",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_MEMORY_ERROR,
        ),
        (
            "IO_ERROR",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_IO_ERROR,
        ),
        (
            "DEVICE_NOT_FOUND",
            sys::UsbSerial_DdkRetCode_USB_SERIAL_DDK_DEVICE_NOT_FOUND,
        ),
    ];

    let mut all_named = true;
    let mut parts: Vec<String> = Vec::new();
    for (name, code) in codes {
        let text = describe(code as i32);
        if text == "unknown error" {
            all_named = false;
        }
        parts.push(format!("{name}({code})=\"{text}\""));
    }

    // Codes outside the enum must fall through to the catch-all arm.
    let zero = describe(0);
    let negative = describe(-1);
    let unknown_ok = zero == "unknown error" && negative == "unknown error";

    finish(format!(
        "error_table ok={} entries=[{}] describe(0)=\"{}\" describe(-1)=\"{}\"",
        all_named && unknown_ok,
        parts.join(", "),
        zero,
        negative
    ))
}

/// Round-trip the C-backed enums through their raw representation and check the
/// values against the sys constants.
#[napi]
pub fn test_usb_serial_enum_roundtrip() -> String {
    let flow: [(&str, FlowControl, u32); 3] = [
        (
            "None",
            FlowControl::None,
            sys::UsbSerial_FlowControl_USB_SERIAL_NO_FLOW_CONTROL,
        ),
        (
            "Software",
            FlowControl::Software,
            sys::UsbSerial_FlowControl_USB_SERIAL_SOFTWARE_FLOW_CONTROL,
        ),
        (
            "Hardware",
            FlowControl::Hardware,
            sys::UsbSerial_FlowControl_USB_SERIAL_HARDWARE_FLOW_CONTROL,
        ),
    ];
    let parity: [(&str, Parity, u32); 3] = [
        (
            "None",
            Parity::None,
            sys::UsbSerial_Parity_USB_SERIAL_PARITY_NONE,
        ),
        (
            "Odd",
            Parity::Odd,
            sys::UsbSerial_Parity_USB_SERIAL_PARITY_ODD,
        ),
        (
            "Even",
            Parity::Even,
            sys::UsbSerial_Parity_USB_SERIAL_PARITY_EVEN,
        ),
    ];

    let mut ok = true;
    let mut parts: Vec<String> = Vec::new();

    for (name, value, expected) in flow {
        let raw = u32::from(value);
        let back = FlowControl::from(raw);
        let good = raw == expected && back == value;
        ok &= good;
        parts.push(format!(
            "FlowControl::{name}->{raw}(expect {expected}) back_ok={good}"
        ));
    }
    for (name, value, expected) in parity {
        let raw = u32::from(value);
        let back = Parity::from(raw);
        let good = raw == expected && back == value;
        ok &= good;
        parts.push(format!(
            "Parity::{name}->{raw}(expect {expected}) back_ok={good}"
        ));
    }

    // Out-of-range raws must be rejected instead of panicking in `From`.
    let flow_bad = FlowControl::try_from_raw(99).is_none();
    let parity_bad = Parity::try_from_raw(99).is_none();
    ok &= flow_bad && parity_bad;

    finish(format!(
        "enum_roundtrip ok={ok} {} try_from_raw(99): flow_none={flow_bad} parity_none={parity_bad}",
        parts.join(" ")
    ))
}

/// `SerialParams` builder defaults and overrides, and the `StopBits` variant
/// set. The raw encoding of `nStopBits` is crate-private, so what is checked
/// here is that the enum still carries exactly the two literal counts the Linux
/// adapter accepts: the exhaustive `match` below stops a third variant (such as
/// a re-introduced `OnePointFive`) from compiling unnoticed.
#[napi]
pub fn test_usb_serial_params_builder() -> String {
    let default = SerialParams::new(115_200);
    let default_ok = default.baud_rate == 115_200
        && default.data_bits == 8
        && default.stop_bits == StopBits::One
        && default.parity == Parity::None;

    let custom = SerialParams::new(9_600)
        .data_bits(7)
        .stop_bits(StopBits::Two)
        .parity(Parity::Even);
    let custom_ok = custom.baud_rate == 9_600
        && custom.data_bits == 7
        && custom.stop_bits == StopBits::Two
        && custom.parity == Parity::Even;

    // Setting the same field twice must keep the last value.
    let overridden = SerialParams::new(9_600)
        .stop_bits(StopBits::Two)
        .stop_bits(StopBits::One);
    let override_ok = overridden.stop_bits == StopBits::One;

    let variants: Vec<&str> = [StopBits::One, StopBits::Two]
        .into_iter()
        .map(|s| match s {
            StopBits::One => "One",
            StopBits::Two => "Two",
        })
        .collect();

    let timeouts = format!(
        "{:?}/{:?}/{:?}",
        Timeout::Infinite,
        Timeout::Immediate,
        Timeout::Millis(100)
    );

    finish(format!(
        "params_builder ok={} default={default:?} custom={custom:?} override_ok={override_ok} \
         stop_bits_variants=[{}] timeout_variants={timeouts}",
        default_ok && custom_ok && override_ok,
        variants.join(",")
    ))
}

/// A single `Ddk::init()` and its drop. This is the permission probe: without
/// `ohos.permission.ACCESS_DDK_USB_SERIAL` the DDK answers `NO_PERM` (201).
#[napi]
pub fn test_usb_serial_ddk_init() -> String {
    let msg = match Ddk::init() {
        Ok(ddk) => {
            drop(ddk);
            "init=Ok released=Ok ok=true".to_string()
        }
        Err(e) => format!("init=Err({}) ok=false", err_text(e)),
    };
    finish(format!("ddk_init {msg}"))
}

/// Init/release pairing: five sequential guards, then two overlapping guards
/// (the inner one keeps the DDK up while the outer is dropped). Surviving this
/// without a crash is the evidence that `Drop` releases exactly once.
#[napi]
pub fn test_usb_serial_ddk_raii_cycles() -> String {
    let mut results: Vec<String> = Vec::new();
    let mut ok = true;

    for round in 0..5u32 {
        match Ddk::init() {
            Ok(ddk) => {
                drop(ddk);
                results.push(format!("#{round}=Ok"));
            }
            Err(e) => {
                ok = false;
                results.push(format!("#{round}=Err({})", err_text(e)));
            }
        }
    }

    let nested = match (Ddk::init(), Ddk::init()) {
        (Ok(first), Ok(second)) => {
            // Drop out of order: the DDK must stay up until the last guard goes.
            drop(first);
            drop(second);
            match Ddk::init() {
                Ok(again) => {
                    drop(again);
                    "nested=Ok reinit_after_nested=Ok".to_string()
                }
                Err(e) => {
                    ok = false;
                    format!("nested=Ok reinit_after_nested=Err({})", err_text(e))
                }
            }
        }
        (Ok(_), Err(e)) | (Err(e), _) => {
            ok = false;
            format!("nested=Err({})", err_text(e))
        }
    };

    finish(format!(
        "ddk_raii_cycles ok={ok} cycles=[{}] {nested}",
        results.join(", ")
    ))
}

/// `open()` on ids that cannot exist. Expected outcome is an error carrying a
/// readable code, never a panic and never a handle.
#[napi]
pub fn test_usb_serial_open_invalid() -> String {
    let ddk = match Ddk::init() {
        Ok(ddk) => ddk,
        Err(e) => {
            return finish(format!(
                "open_invalid ok=false init=Err({}) (open not reached)",
                err_text(e)
            ))
        }
    };

    let attempts: [(&str, u64, u8); 3] = [
        ("device_id=u64::MAX,iface=0", u64::MAX, 0),
        ("device_id=u64::MAX,iface=255", u64::MAX, 255),
        ("device_id=0,iface=255", 0, 255),
    ];

    let mut ok = true;
    let mut parts: Vec<String> = Vec::new();
    for (label, device_id, interface) in attempts {
        match ddk.open(device_id, interface) {
            Ok(dev) => {
                // Unexpected, but report it truthfully and release the handle.
                ok = false;
                drop(dev);
                parts.push(format!("{label}=Ok(unexpected handle)"));
            }
            Err(e) => parts.push(format!("{label}=Err({})", err_text(e))),
        }
    }

    finish(format!("open_invalid ok={ok} {}", parts.join(" ")))
}

/// Full configure/write/read sequence against a real device, if one answers.
/// With nothing attached every probe fails and the reported codes are the
/// result. Read timeout is capped at 100 ms so the UI thread is never held.
#[napi]
pub fn test_usb_serial_device_session() -> String {
    let ddk = match Ddk::init() {
        Ok(ddk) => ddk,
        Err(e) => {
            return finish(format!(
                "device_session ok=false skipped=true init=Err({})",
                err_text(e)
            ))
        }
    };

    let mut probes: Vec<String> = Vec::new();
    let mut opened: Option<(u64, SerialDevice)> = None;
    for device_id in PROBE_DEVICE_IDS {
        match ddk.open(device_id, 0) {
            Ok(dev) => {
                probes.push(format!("id={device_id}=Ok"));
                opened = Some((device_id, dev));
                break;
            }
            Err(e) => probes.push(format!("id={device_id}=Err({})", err_text(e))),
        }
    }

    let Some((device_id, mut dev)) = opened else {
        return finish(format!(
            "device_session ok=false skipped=true reason=\"no device answered open()\" probes=[{}]",
            probes.join(", ")
        ));
    };

    let mut out: Vec<String> = Vec::new();
    let mut ok = true;
    ok &= step(
        "set_params(115200 8N1)",
        dev.set_params(SerialParams::new(115_200)),
        &mut out,
    );
    ok &= step("set_baud_rate(9600)", dev.set_baud_rate(9_600), &mut out);
    // Baud/param changes reset timeout and flow control, so they come after.
    ok &= step(
        "set_timeout(100ms)",
        dev.set_timeout(Timeout::Millis(100)),
        &mut out,
    );
    ok &= step(
        "set_flow_control(None)",
        dev.set_flow_control(FlowControl::None),
        &mut out,
    );
    ok &= step("flush_input", dev.flush_input(), &mut out);
    ok &= step("flush_output", dev.flush_output(), &mut out);

    match dev.write(b"AT\r\n") {
        Ok(n) => out.push(format!("write=Ok({n} bytes)")),
        Err(e) => {
            ok = false;
            out.push(format!("write=Err({})", err_text(e)));
        }
    }
    ok &= step("flush", dev.flush(), &mut out);

    let mut buf = [0u8; 64];
    match dev.read(&mut buf) {
        Ok(n) => out.push(format!("read=Ok({n} bytes, first={:?})", &buf[..n.min(16)])),
        Err(e) => {
            ok = false;
            out.push(format!("read=Err({})", err_text(e)));
        }
    }

    // Empty slices are short-circuited by the wrapper, not sent to the DDK.
    match dev.write(&[]) {
        Ok(n) => out.push(format!("write_empty=Ok({n})")),
        Err(e) => {
            ok = false;
            out.push(format!("write_empty=Err({})", err_text(e)));
        }
    }

    match dev.close() {
        Ok(()) => out.push("close=Ok".to_string()),
        Err(e) => {
            ok = false;
            out.push(format!("close=Err({})", err_text(e)));
        }
    }

    finish(format!(
        "device_session ok={ok} skipped=false device_id={device_id} probes=[{}] {}",
        probes.join(", "),
        out.join(" ")
    ))
}
