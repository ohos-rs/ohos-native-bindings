# ohos-bluetooth-binding

This crate is a binding for the bluetooth module in OpenHarmony.

The native `oh_bluetooth.h` header is a small one: it "provide[s] functions for
querying the status of bluetooth switch" and nothing else. Scanning, pairing,
GATT and the profiles are only available from the ArkTS side, so this crate
wraps the one query the NDK offers with a safe layer.

## Install

```shell
cargo add ohos-bluetooth-binding --features api-13
```

## Usage

```rust
use ohos_bluetooth_binding as bluetooth;

let state = bluetooth::switch_state()?;
println!("bluetooth is {state}");

if state.is_le_available() {
    // start LE work
}
```

## Coverage

The native API arrived in API 13, so the whole safe layer sits behind the
`api-13` feature. Without it the crate exposes only `bluetooth::sys`.

| Feature | Adds |
|---|---|
| `api-13` | `switch_state`, `is_enabled`, `SwitchState`, `BluetoothError`, `describe` |

## Notes

- `switch_state` is a plain query: it neither turns bluetooth on nor requires a
  permission to be granted first.
- `SwitchState` mirrors `Bluetooth_SwitchState` and carries the helpers
  `is_on`, `is_le_available` and `is_transitioning`; LE-only mode counts as
  usable for LE work but not as fully on.
- A state value the system reports but this crate does not know, for instance
  one added by a newer system version, surfaces as
  `BluetoothError::UnknownState` rather than being silently folded into an
  existing variant.
- Native result codes are wrapped in `BluetoothError::Native`; `describe` maps a
  raw code to a short description.

## License

MIT OR Apache-2.0
