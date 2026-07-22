# ohos-battery-info-binding

This crate is a binding for the battery info module in OpenHarmony.

The native `ohbattery_info.h` module exposes the current battery level and the
power source the device is plugged into, plus the keys carried by the
battery-related common events. This crate wraps that C API with a safe layer.

## Install

```shell
cargo add ohos-battery-info-binding --features api-13
```

## Usage

```rust
use ohos_battery_info_binding as battery;

let level = battery::capacity()?;
let source = battery::plugged_type()?;
println!("battery at {level}%, {source}, charging: {}", source.is_plugged());
```

## Coverage

The whole native module was introduced in API 13, so both functions sit behind
the `api-13` feature.

| Feature | Adds |
|---|---|
| _(none)_ | `COMMON_EVENT_KEY_CAPACITY`, `COMMON_EVENT_KEY_CHARGE_STATE`, `COMMON_EVENT_KEY_PLUGGED_TYPE` |
| `api-13` | `capacity`, `plugged_type`, `PluggedType` |

## Notes

- `capacity` returns a `u8` percentage; the header promises 0..=100, and a value
  outside that range is surfaced as `BatteryInfoError::CapacityOutOfRange`
  instead of being passed on.
- `plugged_type` maps the native `PLUGGED_TYPE_BUTT` ("type is unknown") to
  `PluggedType::Unknown`, which is a normal result rather than an error; only a
  value outside the enum yields `BatteryInfoError::UnknownPluggedType`.
- `PluggedType::is_plugged` reports whether an external power source is
  attached; `Unknown` does not count as attached.
- The common-event keys are `&CStr` constants so they can be handed to the
  common-event APIs directly; use `CStr::to_str` for the plain `&str` form.
- The module has no native error-code family and no resources to release: both
  calls are stateless reads and nothing needs to be freed by the caller.

## License

MIT OR Apache-2.0
