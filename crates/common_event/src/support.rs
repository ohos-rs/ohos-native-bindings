//! The names of the system common events an application can subscribe to.

use ohos_common_event_sys as sys;

/// Drop the trailing NUL from a bindgen string constant.
const fn name(bytes: &'static [u8]) -> &'static str {
    let (text, _) = bytes.split_at(bytes.len() - 1);
    match std::str::from_utf8(text) {
        Ok(text) => text,
        Err(_) => panic!("common event name is not valid UTF-8"),
    }
}

/// Predefined system common event names.
///
/// Most of them are protected: only the system may publish them, while any
/// application may subscribe to them. The `Since API` note on each name is the
/// version that introduced the event; the names themselves are plain strings
/// and are always available.
pub mod event {
    use super::name;
    use super::sys;

    /// This commonEvent means when the device is shutting down, note: turn off, not sleeping.
    ///
    /// Since API 12.
    pub const SHUTDOWN: &str = name(sys::COMMON_EVENT_SHUTDOWN);

    /// This commonEvent means when the charging state, level and so on about the battery.
    ///
    /// Since API 12.
    pub const BATTERY_CHANGED: &str = name(sys::COMMON_EVENT_BATTERY_CHANGED);

    /// This commonEvent means when the device in low battery state.
    ///
    /// Since API 12.
    pub const BATTERY_LOW: &str = name(sys::COMMON_EVENT_BATTERY_LOW);

    /// This commonEvent means when the battery level is an ok state.
    ///
    /// Since API 12.
    pub const BATTERY_OKAY: &str = name(sys::COMMON_EVENT_BATTERY_OKAY);

    /// This commonEvent means when the other power is connected to the device.
    ///
    /// Since API 12.
    pub const POWER_CONNECTED: &str = name(sys::COMMON_EVENT_POWER_CONNECTED);

    /// This commonEvent means when the other power is removed from the device.
    ///
    /// Since API 12.
    pub const POWER_DISCONNECTED: &str = name(sys::COMMON_EVENT_POWER_DISCONNECTED);

    /// This commonEvent means when the screen is turned off.
    ///
    /// Since API 12.
    pub const SCREEN_OFF: &str = name(sys::COMMON_EVENT_SCREEN_OFF);

    /// This commonEvent means when the device is awakened and interactive.
    ///
    /// Since API 12.
    pub const SCREEN_ON: &str = name(sys::COMMON_EVENT_SCREEN_ON);

    /// This commonEvent means when the device is about to enter the hibernate mode.
    ///
    /// Since API 15.
    pub const ENTER_HIBERNATE: &str = name(sys::COMMON_EVENT_ENTER_HIBERNATE);

    /// This commonEvent means when the device exits the hibernate mode.
    ///
    /// Since API 15.
    pub const EXIT_HIBERNATE: &str = name(sys::COMMON_EVENT_EXIT_HIBERNATE);

    /// This commonEvent means when the thermal state level change
    ///
    /// Since API 12.
    pub const THERMAL_LEVEL_CHANGED: &str = name(sys::COMMON_EVENT_THERMAL_LEVEL_CHANGED);

    /// This commonEvent means when the current time is changed.
    ///
    /// Since API 12.
    pub const TIME_TICK: &str = name(sys::COMMON_EVENT_TIME_TICK);

    /// This commonEvent means when the time is set.
    ///
    /// Since API 12.
    pub const TIME_CHANGED: &str = name(sys::COMMON_EVENT_TIME_CHANGED);

    /// This commonEvent means when the time zone is changed.
    ///
    /// Since API 12.
    pub const TIMEZONE_CHANGED: &str = name(sys::COMMON_EVENT_TIMEZONE_CHANGED);

    /// This commonEvent means when a new application package is installed on the device.
    ///
    /// Since API 12.
    pub const PACKAGE_ADDED: &str = name(sys::COMMON_EVENT_PACKAGE_ADDED);

    /// This commonEvent means when an existing application package is removed from the device.
    ///
    /// Since API 12.
    pub const PACKAGE_REMOVED: &str = name(sys::COMMON_EVENT_PACKAGE_REMOVED);

    /// This commonEvent means when an installed application's add-on package is removed from the device.
    ///
    /// Since API 12.
    pub const BUNDLE_REMOVED: &str = name(sys::COMMON_EVENT_BUNDLE_REMOVED);

    /// This commonEvent means when an existing application package is completely removed from the device.
    ///
    /// Since API 12.
    pub const PACKAGE_FULLY_REMOVED: &str = name(sys::COMMON_EVENT_PACKAGE_FULLY_REMOVED);

    /// This commonEvent means when an existing application package has been changed.
    ///
    /// Since API 12.
    pub const PACKAGE_CHANGED: &str = name(sys::COMMON_EVENT_PACKAGE_CHANGED);

    /// This commonEvent means the user has restarted a package, and all of its processes have been killed.
    ///
    /// Since API 12.
    pub const PACKAGE_RESTARTED: &str = name(sys::COMMON_EVENT_PACKAGE_RESTARTED);

    /// This commonEvent means the user has cleared the package data.
    ///
    /// Since API 12.
    pub const PACKAGE_DATA_CLEARED: &str = name(sys::COMMON_EVENT_PACKAGE_DATA_CLEARED);

    /// This commonEvent means the user has cleared the package cache.
    ///
    /// Since API 12.
    pub const PACKAGE_CACHE_CLEARED: &str = name(sys::COMMON_EVENT_PACKAGE_CACHE_CLEARED);

    /// This commonEvent means the packages have been suspended.
    ///
    /// Since API 12.
    pub const PACKAGES_SUSPENDED: &str = name(sys::COMMON_EVENT_PACKAGES_SUSPENDED);

    /// This commonEvent Sent to a package that has been suspended by the system.
    ///
    /// Since API 12.
    pub const MY_PACKAGE_SUSPENDED: &str = name(sys::COMMON_EVENT_MY_PACKAGE_SUSPENDED);

    /// Sent to a package that has been un-suspended.
    ///
    /// Since API 12.
    pub const MY_PACKAGE_UNSUSPENDED: &str = name(sys::COMMON_EVENT_MY_PACKAGE_UNSUSPENDED);

    /// The current device's locale has changed.
    ///
    /// Since API 12.
    pub const LOCALE_CHANGED: &str = name(sys::COMMON_EVENT_LOCALE_CHANGED);

    /// Indicates low memory condition notification acknowledged by user and package
    /// management should be started.
    ///
    /// Since API 12.
    pub const MANAGE_PACKAGE_STORAGE: &str = name(sys::COMMON_EVENT_MANAGE_PACKAGE_STORAGE);

    /// Remind new user of that the service has been unlocked.
    ///
    /// Since API 12.
    pub const USER_UNLOCKED: &str = name(sys::COMMON_EVENT_USER_UNLOCKED);

    /// Distributed account logout successfully.
    ///
    /// Since API 12.
    pub const DISTRIBUTED_ACCOUNT_LOGOUT: &str = name(sys::COMMON_EVENT_DISTRIBUTED_ACCOUNT_LOGOUT);

    /// Distributed account is invalid.
    ///
    /// Since API 12.
    pub const DISTRIBUTED_ACCOUNT_TOKEN_INVALID: &str =
        name(sys::COMMON_EVENT_DISTRIBUTED_ACCOUNT_TOKEN_INVALID);

    /// Distributed account logs off.
    ///
    /// Since API 12.
    pub const DISTRIBUTED_ACCOUNT_LOGOFF: &str = name(sys::COMMON_EVENT_DISTRIBUTED_ACCOUNT_LOGOFF);

    /// WIFI state.
    ///
    /// Since API 12.
    pub const WIFI_POWER_STATE: &str = name(sys::COMMON_EVENT_WIFI_POWER_STATE);

    /// WIFI scan results.
    ///
    /// Since API 12.
    pub const WIFI_SCAN_FINISHED: &str = name(sys::COMMON_EVENT_WIFI_SCAN_FINISHED);

    /// WIFI RSSI change.
    ///
    /// Since API 12.
    pub const WIFI_RSSI_VALUE: &str = name(sys::COMMON_EVENT_WIFI_RSSI_VALUE);

    /// WIFI connect state.
    ///
    /// Since API 12.
    pub const WIFI_CONN_STATE: &str = name(sys::COMMON_EVENT_WIFI_CONN_STATE);

    /// WIFI hotspot state.
    ///
    /// Since API 12.
    pub const WIFI_HOTSPOT_STATE: &str = name(sys::COMMON_EVENT_WIFI_HOTSPOT_STATE);

    /// WIFI ap sta join.
    ///
    /// Since API 12.
    pub const WIFI_AP_STA_JOIN: &str = name(sys::COMMON_EVENT_WIFI_AP_STA_JOIN);

    /// WIFI ap sta join.
    ///
    /// Since API 12.
    pub const WIFI_AP_STA_LEAVE: &str = name(sys::COMMON_EVENT_WIFI_AP_STA_LEAVE);

    /// Indicates Wi-Fi MpLink state notification acknowledged by binding or unbinding MpLink.
    ///
    /// Since API 12.
    pub const WIFI_MPLINK_STATE_CHANGE: &str = name(sys::COMMON_EVENT_WIFI_MPLINK_STATE_CHANGE);

    /// Indicates Wi-Fi P2P connection state notification acknowledged by connecting or disconnected P2P.
    ///
    /// Since API 12.
    pub const WIFI_P2P_CONN_STATE: &str = name(sys::COMMON_EVENT_WIFI_P2P_CONN_STATE);

    /// Indicates that the Wi-Fi P2P state change.
    ///
    /// Since API 12.
    pub const WIFI_P2P_STATE_CHANGED: &str = name(sys::COMMON_EVENT_WIFI_P2P_STATE_CHANGED);

    /// Indicates that the Wi-Fi P2P peers state change.
    ///
    /// Since API 12.
    pub const WIFI_P2P_PEERS_STATE_CHANGED: &str =
        name(sys::COMMON_EVENT_WIFI_P2P_PEERS_STATE_CHANGED);

    /// Indicates that the Wi-Fi P2P discovery state change.
    ///
    /// Since API 12.
    pub const WIFI_P2P_PEERS_DISCOVERY_STATE_CHANGED: &str =
        name(sys::COMMON_EVENT_WIFI_P2P_PEERS_DISCOVERY_STATE_CHANGED);

    /// Indicates that the Wi-Fi P2P current device state change.
    ///
    /// Since API 12.
    pub const WIFI_P2P_CURRENT_DEVICE_STATE_CHANGED: &str =
        name(sys::COMMON_EVENT_WIFI_P2P_CURRENT_DEVICE_STATE_CHANGED);

    /// Indicates that the Wi-Fi P2P group info is changed.
    ///
    /// Since API 12.
    pub const WIFI_P2P_GROUP_STATE_CHANGED: &str =
        name(sys::COMMON_EVENT_WIFI_P2P_GROUP_STATE_CHANGED);

    /// Nfc state change.
    ///
    /// Since API 12.
    pub const NFC_ACTION_ADAPTER_STATE_CHANGED: &str =
        name(sys::COMMON_EVENT_NFC_ACTION_ADAPTER_STATE_CHANGED);

    /// Nfc field on detected.
    ///
    /// Since API 12.
    pub const NFC_ACTION_RF_FIELD_ON_DETECTED: &str =
        name(sys::COMMON_EVENT_NFC_ACTION_RF_FIELD_ON_DETECTED);

    /// Nfc field off detected.
    ///
    /// Since API 12.
    pub const NFC_ACTION_RF_FIELD_OFF_DETECTED: &str =
        name(sys::COMMON_EVENT_NFC_ACTION_RF_FIELD_OFF_DETECTED);

    /// Sent when stop charging battery.
    ///
    /// Since API 12.
    pub const DISCHARGING: &str = name(sys::COMMON_EVENT_DISCHARGING);

    /// Sent when start charging battery.
    ///
    /// Since API 12.
    pub const CHARGING: &str = name(sys::COMMON_EVENT_CHARGING);

    /// Sent when device's idle mode changed
    ///
    /// Since API 12.
    pub const DEVICE_IDLE_MODE_CHANGED: &str = name(sys::COMMON_EVENT_DEVICE_IDLE_MODE_CHANGED);

    /// Sent when device's charge idle mode changed.
    ///
    /// Since API 12.
    pub const CHARGE_IDLE_MODE_CHANGED: &str = name(sys::COMMON_EVENT_CHARGE_IDLE_MODE_CHANGED);

    /// Sent when device's power save mode changed
    ///
    /// Since API 12.
    pub const POWER_SAVE_MODE_CHANGED: &str = name(sys::COMMON_EVENT_POWER_SAVE_MODE_CHANGED);

    /// The usb state change events.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const USB_STATE: &str = name(sys::COMMON_EVENT_USB_STATE);

    /// The usb port changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const USB_PORT_CHANGED: &str = name(sys::COMMON_EVENT_USB_PORT_CHANGED);

    /// The usb device attached.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const USB_DEVICE_ATTACHED: &str = name(sys::COMMON_EVENT_USB_DEVICE_ATTACHED);

    /// The usb device detached.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const USB_DEVICE_DETACHED: &str = name(sys::COMMON_EVENT_USB_DEVICE_DETACHED);

    /// Indicates the common event Action indicating that the airplane mode status of the device changes.
    /// Users can register this event to listen to the change of the airplane mode status of the device.
    ///
    /// Since API 12.
    pub const AIRPLANE_MODE_CHANGED: &str = name(sys::COMMON_EVENT_AIRPLANE_MODE_CHANGED);

    /// sent by the window manager service when the window mode is split.
    ///
    /// Since API 12.
    pub const SPLIT_SCREEN: &str = name(sys::COMMON_EVENT_SPLIT_SCREEN);

    /// Indicate the result of quick fix apply.
    /// This common event can be triggered only by system.
    ///
    /// Since API 12.
    pub const QUICK_FIX_APPLY_RESULT: &str = name(sys::COMMON_EVENT_QUICK_FIX_APPLY_RESULT);

    /// Indicate the result of quick fix revoke.
    /// This common event can be triggered only by system.
    ///
    /// Since API 12.
    pub const QUICK_FIX_REVOKE_RESULT: &str = name(sys::COMMON_EVENT_QUICK_FIX_REVOKE_RESULT);

    /// Indicate the action of a common event that the user information has been updated.
    /// This common event can be triggered only by system.
    ///
    /// Since API 12.
    pub const USER_INFO_UPDATED: &str = name(sys::COMMON_EVENT_USER_INFO_UPDATED);

    /// Indicates the action of a common event that the phone SIM card state has changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const SIM_STATE_CHANGED: &str = name(sys::COMMON_EVENT_SIM_STATE_CHANGED);

    /// Indicates the action of a common event that the call state has been changed.
    /// To subscribe to this protected common event, your application must have the ohos.permission.GET_TELEPHONY_STATE
    /// permission.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const CALL_STATE_CHANGED: &str = name(sys::COMMON_EVENT_CALL_STATE_CHANGED);

    /// Indicates the action of a common event that the network state has been changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const NETWORK_STATE_CHANGED: &str = name(sys::COMMON_EVENT_NETWORK_STATE_CHANGED);

    /// Indicates the action of a common event that the signal info has been changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const SIGNAL_INFO_CHANGED: &str = name(sys::COMMON_EVENT_SIGNAL_INFO_CHANGED);

    /// This commonEvent means when the screen is unlocked.
    ///
    /// Since API 12.
    pub const SCREEN_UNLOCKED: &str = name(sys::COMMON_EVENT_SCREEN_UNLOCKED);

    /// This commonEvent means when the screen is locked.
    ///
    /// Since API 12.
    pub const SCREEN_LOCKED: &str = name(sys::COMMON_EVENT_SCREEN_LOCKED);

    /// This commonEvent means when the http proxy change.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const HTTP_PROXY_CHANGE: &str = name(sys::COMMON_EVENT_HTTP_PROXY_CHANGE);

    /// This commonEvent means when the network connectivity change.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const CONNECTIVITY_CHANGE: &str = name(sys::COMMON_EVENT_CONNECTIVITY_CHANGE);

    /// This common event means that minors mode is enabled.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const MINORSMODE_ON: &str = name(sys::COMMON_EVENT_MINORSMODE_ON);

    /// This common event means that minors mode is disabled.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 12.
    pub const MINORSMODE_OFF: &str = name(sys::COMMON_EVENT_MINORSMODE_OFF);

    /// This common event means that the managed browser policy is changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 15.
    pub const MANAGED_BROWSER_POLICY_CHANGED: &str =
        name(sys::COMMON_EVENT_MANAGED_BROWSER_POLICY_CHANGED);

    /// This common event means that the open and closed state of the stand associated
    /// with the tablet mode has changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 23.
    pub const TABLET_MODE_CHANGED: &str = name(sys::COMMON_EVENT_TABLET_MODE_CHANGED);

    /// This common event means that the state (open or closed) of the laptop lid has changed.
    /// This is a protected common event that can only be sent by system.
    ///
    /// Since API 23.
    pub const LID_STATE_CHANGED: &str = name(sys::COMMON_EVENT_LID_STATE_CHANGED);
}
