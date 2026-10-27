# ohos-device-binding

This crate is a binding for the device info module in OpenHarmony.

## Install

```shell
cargo add ohos-device-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_device_binding as device;

let device_type = device::device_type();
let sdk_api = device::sdk_api_version();
```

## License

MIT OR Apache-2.0
