# ohos-location-binding

This crate is a binding for the location module in OpenHarmony.

The native `oh_location.h` C API of LocationKit is a subscription API: the
application describes what kind of fixes it wants in a request parameter
instance, attaches a reporting callback, starts locating and later stops it
again with the same instance. This crate wraps that C API with a safe layer.

## Install

```shell
cargo add ohos-location-binding --features api-13
```

## Usage

```rust
use ohos_location_binding as location;
use location::{LocationSession, LocationUseScene, RequestConfig};

if !location::is_locating_enabled()? {
    // guide the user to the system location switch
}

let config = RequestConfig::builder()
    .use_scene(LocationUseScene::DailyLifeService)
    .interval(5)
    .build()?;

let session = LocationSession::start(config, |fix| {
    println!("{} {} +-{}m", fix.latitude, fix.longitude, fix.accuracy);
})?;

// Stops on drop as well.
session.stop()?;
```

## Coverage

The whole module was introduced in API 13, so everything sits behind the
`api-13` feature. With the default feature set the crate only re-exports the
raw bindings as `location::sys`.

| Item | Wraps |
|---|---|
| `RequestConfig`, `RequestConfigBuilder` | `OH_Location_CreateRequestConfig` / `OH_Location_DestroyRequestConfig` / `OH_LocationRequestConfig_Set{UseScene,PowerConsumptionScene,Interval}` |
| `LocationSession` | `OH_LocationRequestConfig_SetCallback`, `OH_Location_StartLocating`, `OH_Location_StopLocating` |
| `LocationInfo` | `OH_LocationInfo_GetBasicInfo`, `OH_LocationInfo_GetAdditionalInfo` |
| `is_locating_enabled` | `OH_Location_IsLocatingEnabled` |
| `LocationUseScene`, `LocationPowerConsumptionScene`, `LocationSourceType` | the `Location_*` enums |
| `LocationError`, `describe` | `Location_ResultCode` |

## Permissions

`oh_location.h` declares `@permission ohos.permission.APPROXIMATELY_LOCATION` on
`OH_Location_StartLocating` and `OH_Location_StopLocating`; without it both
report `LOCATION_PERMISSION_DENIED` (201), for which
`LocationError::is_permission_denied` holds. Applications that want precise
fixes request `ohos.permission.LOCATION` in addition, which the system only
grants together with the approximate one. `is_locating_enabled` needs no
permission.

A fix further requires the user to have the system location switch on;
otherwise starting reports `LOCATION_SWITCH_OFF` (3301100), for which
`LocationError::is_switch_off` holds.

## Notes

- Both native handle pairs are RAII: `RequestConfig` destroys its instance on
  drop, and `LocationSession` stops locating on drop before the instance it owns
  is destroyed. The session owns the request parameters because the native API
  requires stopping to pass the very instance that started the subscription.
- The reporting closure lives in a process-wide registry keyed by an integer,
  and that integer — not a pointer into Rust memory — is what is handed to the
  native `userData` parameter. A report can therefore never dereference a freed
  closure, no matter when the service delivers it. The registry entry is dropped
  after `OH_Location_StopLocating` returns, and a report that is already running
  holds its own reference, so stopping never frees a closure in use.
- The callback runs on a thread owned by the location service, hence the
  `Send + Sync + 'static` bound. It should return quickly: the native location
  instance is recycled as soon as it does, which is why `LocationInfo` is a copy
  rather than a view.
- `LocationInfo::additional_info` is the JSON string the service reports
  alongside the fix, read into a 512 byte buffer (the native documentation
  recommends at least 256); it is `None` when the service reported none.
- The use scenario takes precedence over the power consumption scenario: once
  `use_scene` is set, `power_consumption_scene` is ignored. With neither set the
  service behaves as if `LocationUseScene::DailyLifeService` had been requested.

## License

MIT OR Apache-2.0
