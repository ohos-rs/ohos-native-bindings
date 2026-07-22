# ohos-usb-serial-binding

This crate is a binding for the USB Serial DDK module in OpenHarmony.

The USB Serial DDK drives USB CDC-ACM serial devices from native code: open an
interface, configure the line, and read/write bytes. This crate wraps
`usb_serial_api.h` with a safe layer — reference-counted DDK initialization, an
RAII device handle, `Result`-based error handling, and typed line parameters.

## Requirements

- **Feature `api-18`.** The whole native API is `@since 18`, so with default
  features this crate exposes nothing but the raw `sys` re-export.
- **Permission `ohos.permission.ACCESS_DDK_USB_SERIAL`.** It is `system_grant`
  at `system_basic` level with `provisionEnable`, so there is no runtime prompt
  and an ordinary application cannot obtain it — the signing profile must carry
  it in its ACL list (driver-package scenarios). Applications outside that scope
  should use the ArkTS `usbManager` API, which is a separate stack with its own
  per-device user authorization.

Device ids come from the USB DDK (`OH_Usb_GetDevices`); this crate does not
enumerate devices.

## Install

```shell
cargo add ohos-usb-serial-binding --features api-18
```

## Usage

```rust
use ohos_usb_serial_binding::{Ddk, SerialParams, Timeout};

let ddk = Ddk::init()?;
let mut dev = ddk.open(device_id, 0)?;
dev.set_params(SerialParams::new(115_200))?;
dev.set_timeout(Timeout::Millis(500))?;

dev.write(b"AT\r\n")?;
let mut buf = [0u8; 64];
let n = dev.read(&mut buf)?;
```

Every call blocks the calling thread — the native API has no callbacks and no
asynchronous variants. Drive the device from a worker thread or a TaskPool task,
never from the UI thread. `SerialDevice` is deliberately not `Send`: the DDK does
not specify whether a handle may be used from a thread other than the one that
opened it, so open the device on the thread that will drive it.

## Coverage

- DDK lifetime: `Ddk::init`, reference counted, released when the last guard
  (including any open device) is dropped.
- Device: `Ddk::open` → `SerialDevice::read` / `write` / `close`, with `Drop`
  closing the handle.
- Configuration: `set_baud_rate`, `set_params` (`SerialParams`, `StopBits`,
  `Parity`), `set_timeout` (`Timeout`), `set_flow_control` (`FlowControl`),
  `flush` / `flush_input` / `flush_output`.

`describe` maps a raw `UsbSerial_DdkRetCode` to a short description; note that
success is `31600000`, not `0`.

The raw bindings are re-exported as `ohos_usb_serial_binding::sys` for anything
not yet covered by the safe layer.

## Status

Compile-checked for all three OpenHarmony targets, but **not verified on a
device**: the required ACL permission is unavailable outside a driver package.

## License

MIT OR Apache-2.0
