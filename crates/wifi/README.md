# ohos-wifi-binding

This crate is a binding for the wifi module in OpenHarmony.

The native `oh_wifi.h` API lets an application query the wifi state of the
device it runs on: whether the wifi switch is on, the MAC address of the device,
and the details of the access point the local station is connected to. It is a
read-only API; joining or leaving a network is not part of it. This crate wraps
that C API with a safe layer.

## Install

```shell
cargo add ohos-wifi-binding --features api-24
```

## Usage

```rust
use ohos_wifi_binding as wifi;
use ohos_wifi_binding::ConnState;

if wifi::is_wifi_enabled()? {
    let info = wifi::linked_info()?;
    if info.conn_state == ConnState::Connected {
        println!("{} at {} dBm", info.ssid, info.rssi);
    }
}

let address = wifi::device_mac_address()?;
println!("{address}");
```

## Coverage

The native API starts at API 13, so every item sits behind an `api-*` feature.

| Feature | Item | Native counterpart |
|---|---|---|
| `api-13` | `is_wifi_enabled` | `OH_Wifi_IsWifiEnabled` |
| `api-13` | `WifiError`, `describe` | `Wifi_ResultCode` |
| `api-21` | `device_mac_address` | `OH_Wifi_GetDeviceMacAddress` |
| `api-21` | `MacAddress`, `MacAddressParseError` | `char[WIFI_MAC_LEN]` fields |
| `api-24` | `linked_info` | `OH_Wifi_GetLinkedInfo` |
| `api-24` | `LinkedInfo` | `OH_WifiLinkedInfo` |
| `api-24` | `ConnState` | `OH_WifiConnState` |
| `api-24` | `ChannelWidth` | `OH_WifiChannelWidth` |
| `api-24` | `WifiStandard` | `OH_WifiStandard` |
| `api-24` | `WifiCategory` | `OH_WifiCategory` |
| `api-24` | `LinkType` | `OH_WifiLinkType` |
| `api-24` | `Band`, `MacAddressType` | `band`, `macType` integer fields |

## Permissions

Most of the API is permission-guarded; a missing permission surfaces as
`WifiError::PermissionDenied`.

| Item | Permission |
|---|---|
| `is_wifi_enabled` | none |
| `device_mac_address` | `ohos.permission.GET_WIFI_LOCAL_MAC` and `ohos.permission.GET_WIFI_INFO` |
| `linked_info` | `ohos.permission.GET_WIFI_INFO` |
| `LinkedInfo::bssid`, real address | `ohos.permission.GET_WIFI_PEERS_MAC` |
| `LinkedInfo::mac_address`, device address | `ohos.permission.GET_WIFI_LOCAL_MAC` |

The two `LinkedInfo` fields are refined by their permission rather than failing
the call: without `ohos.permission.GET_WIFI_PEERS_MAC` the reported BSSID is a
randomized address instead of the real one, and without
`ohos.permission.GET_WIFI_LOCAL_MAC` the device MAC address is withheld, which
this crate reports as `None`. `ohos.permission.GET_WIFI_LOCAL_MAC` is available
to regular applications only on PC and 2-in-1 devices from API 16 on; elsewhere
it stays restricted to system applications.

## Notes

- `LinkedInfo` is the owned form of `OH_WifiLinkedInfo`: the fixed-size C
  character fields become a `String` (SSID, decoded up to the terminating NUL,
  with invalid UTF-8 replaced) and `Option<MacAddress>` values, and the C
  enumerations become Rust enumerations.
- `MacAddress` holds the six decoded octets and renders them back in the
  `AA:BB:CC:DD:EE:FF` form the native API uses. A withheld address is `None`; an
  address that is present but not in that form is reported as
  `WifiError::MalformedMacAddress`.
- Every enumeration carries an `Other` variant with the raw value, so a state,
  width, standard, category or link type added by a later platform release is
  passed through rather than lost.
- `LinkedInfo::ip_address` is kept as the packed `u32` the wifi service reports;
  the native header does not document the octet order, so no conversion is
  applied.
- `Wifi_ResultCode` values this crate does not know about become
  `WifiError::Unknown`. `WIFI_STA_DISABLED` was added in API 21, so under a
  lower feature set it arrives as `WifiError::Unknown(2501001)`.

## License

MIT OR Apache-2.0
