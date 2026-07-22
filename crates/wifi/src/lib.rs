//! Safe Rust bindings for OpenHarmony **wifi**.
//!
//! The native `oh_wifi.h` API lets an application query the wifi state of the
//! device it runs on: whether the wifi switch is on, the MAC address of the
//! device, and the details of the access point the local station is connected
//! to. It is a read-only API; joining or leaving a network is not part of it.
//! This crate wraps that C API with a safe layer.
//!
//! Every item sits behind an `api-*` feature, because the native API starts at
//! API 13:
//!
//! | Feature | Adds |
//! |---|---|
//! | `api-13` | [`is_wifi_enabled`], [`WifiError`], [`describe`] |
//! | `api-21` | [`device_mac_address`], [`MacAddress`] |
//! | `api-24` | [`linked_info`], [`LinkedInfo`] and its enumerations |
//!
//! # Permissions
//!
//! Most of the API is permission-guarded; a missing permission surfaces as
//! [`WifiError::PermissionDenied`].
//!
//! - [`is_wifi_enabled`] declares no permission.
//! - [`device_mac_address`] requires `ohos.permission.GET_WIFI_LOCAL_MAC` and
//!   `ohos.permission.GET_WIFI_INFO`.
//! - [`linked_info`] requires `ohos.permission.GET_WIFI_INFO`. Two of its
//!   fields are refined by further permissions instead of failing the call:
//!   [`LinkedInfo::bssid`] is the real BSSID only with
//!   `ohos.permission.GET_WIFI_PEERS_MAC` and a randomized address otherwise,
//!   and [`LinkedInfo::mac_address`] is reported only with
//!   `ohos.permission.GET_WIFI_LOCAL_MAC` when the station uses the device
//!   address.
//!
//! The raw bindings are re-exported as [`sys`].

pub use ohos_wifi_sys as sys;

#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-24")]
mod link;
#[cfg(feature = "api-21")]
mod mac;
#[cfg(feature = "api-21")]
mod util;

#[cfg(feature = "api-13")]
pub use error::{describe, Result, WifiError};
#[cfg(feature = "api-24")]
pub use link::{
    Band, ChannelWidth, ConnState, LinkType, LinkedInfo, MacAddressType, WifiCategory, WifiStandard,
};
#[cfg(feature = "api-21")]
pub use mac::{MacAddress, MacAddressParseError};

/// Whether the wifi switch of the device is turned on.
///
/// # Example
///
/// ```no_run
/// use ohos_wifi_binding as wifi;
///
/// if wifi::is_wifi_enabled()? {
///     // the radio is on, a connection may be up
/// }
/// # Ok::<(), wifi::WifiError>(())
/// ```
#[cfg(feature = "api-13")]
pub fn is_wifi_enabled() -> Result<bool> {
    let mut enabled = false;
    error::check(unsafe { sys::OH_Wifi_IsWifiEnabled(&mut enabled) })?;
    Ok(enabled)
}

/// The MAC address of the device.
///
/// Requires `ohos.permission.GET_WIFI_LOCAL_MAC` and
/// `ohos.permission.GET_WIFI_INFO`; without them the call fails with
/// [`WifiError::PermissionDenied`]. It also fails with
/// [`WifiError::StaDisabled`] while the wifi station mode is off.
///
/// # Example
///
/// ```no_run
/// use ohos_wifi_binding as wifi;
///
/// let address = wifi::device_mac_address()?;
/// println!("{address}");
/// println!("{:?}", address.octets());
/// # Ok::<(), wifi::WifiError>(())
/// ```
#[cfg(feature = "api-21")]
pub fn device_mac_address() -> Result<MacAddress> {
    const BUFFER_LEN: usize = sys::WIFI_MAC_LEN as usize;

    let mut buffer: [std::os::raw::c_char; BUFFER_LEN] = [0; BUFFER_LEN];
    let mut buffer_len: std::os::raw::c_uint = sys::WIFI_MAC_LEN;
    error::check(unsafe {
        sys::OH_Wifi_GetDeviceMacAddress(buffer.as_mut_ptr(), &mut buffer_len)
    })?;
    match mac::parse_mac_field(&buffer) {
        Ok(Some(address)) => Ok(address),
        Ok(None) | Err(_) => Err(WifiError::MalformedMacAddress),
    }
}

/// Information about the access point the local station is connected to.
///
/// Requires `ohos.permission.GET_WIFI_INFO`; without it the call fails with
/// [`WifiError::PermissionDenied`]. Fields the caller is not permitted to see
/// are withheld by the wifi service rather than failing the call, see
/// [`LinkedInfo::bssid`] and [`LinkedInfo::mac_address`].
///
/// # Example
///
/// ```no_run
/// use ohos_wifi_binding as wifi;
/// use ohos_wifi_binding::ConnState;
///
/// let info = wifi::linked_info()?;
/// if info.conn_state == ConnState::Connected {
///     println!("{} at {} dBm", info.ssid, info.rssi);
/// }
/// # Ok::<(), wifi::WifiError>(())
/// ```
#[cfg(feature = "api-24")]
pub fn linked_info() -> Result<LinkedInfo> {
    // The wifi service fills the structure in; zeroing it keeps every field at
    // a valid value should the call fail on the way.
    let mut info = std::mem::MaybeUninit::<sys::OH_WifiLinkedInfo>::zeroed();
    error::check(unsafe { sys::OH_Wifi_GetLinkedInfo(info.as_mut_ptr()) })?;
    let info = unsafe { info.assume_init() };
    LinkedInfo::from_raw(&info).map_err(|_| WifiError::MalformedMacAddress)
}
