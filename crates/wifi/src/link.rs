use ohos_wifi_sys as sys;

use crate::mac::{parse_mac_field, MacAddress};
use crate::util::c_field_to_string;

/// State of the wifi connection of the local station.
///
/// [`ConnState::Unknown`] is the native `OH_WIFI_CONN_UNKNOWN` state, meaning
/// the connection could not be set up; [`ConnState::Other`] carries a state
/// this crate does not know about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnState {
    /// The device is searching for an available access point.
    Scanning,
    /// The connection is being set up.
    Connecting,
    /// The connection is being authenticated.
    Authenticating,
    /// The IP address of the connection is being obtained.
    ObtainingIpAddress,
    /// The connection has been set up.
    Connected,
    /// The connection is being torn down.
    Disconnecting,
    /// The connection has been torn down.
    Disconnected,
    /// The connection is in a special state.
    SpecialConnect,
    /// The connection could not be set up.
    Unknown,
    /// A state value this crate does not know about.
    Other(u32),
}

impl ConnState {
    fn from_raw(raw: sys::OH_WifiConnState) -> Self {
        match raw {
            sys::OH_WifiConnState_OH_WIFI_CONN_SCANNING => ConnState::Scanning,
            sys::OH_WifiConnState_OH_WIFI_CONN_CONNECTING => ConnState::Connecting,
            sys::OH_WifiConnState_OH_WIFI_CONN_AUTHENTICATING => ConnState::Authenticating,
            sys::OH_WifiConnState_OH_WIFI_CONN_OBTAINING_IPADDR => ConnState::ObtainingIpAddress,
            sys::OH_WifiConnState_OH_WIFI_CONN_CONNECTED => ConnState::Connected,
            sys::OH_WifiConnState_OH_WIFI_CONN_DISCONNECTING => ConnState::Disconnecting,
            sys::OH_WifiConnState_OH_WIFI_CONN_DISCONNECTED => ConnState::Disconnected,
            sys::OH_WifiConnState_OH_WIFI_CONN_SPECIAL_CONNECT => ConnState::SpecialConnect,
            sys::OH_WifiConnState_OH_WIFI_CONN_UNKNOWN => ConnState::Unknown,
            other => ConnState::Other(other),
        }
    }
}

/// Channel width the access point currently operates on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelWidth {
    /// 20 MHz.
    Width20Mhz,
    /// 40 MHz.
    Width40Mhz,
    /// 80 MHz.
    Width80Mhz,
    /// 160 MHz.
    Width160Mhz,
    /// 80 + 80 MHz.
    Width80MhzPlus,
    /// The reported width is not valid.
    Invalid,
    /// A width value this crate does not know about.
    Other(u32),
}

impl ChannelWidth {
    fn from_raw(raw: sys::OH_WifiChannelWidth) -> Self {
        match raw {
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_20MHZ => ChannelWidth::Width20Mhz,
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_40MHZ => ChannelWidth::Width40Mhz,
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_80MHZ => ChannelWidth::Width80Mhz,
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_160MHZ => ChannelWidth::Width160Mhz,
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_80MHZ_PLUS => ChannelWidth::Width80MhzPlus,
            sys::OH_WifiChannelWidth_OH_WIFI_WIDTH_INVALID => ChannelWidth::Invalid,
            other => ChannelWidth::Other(other),
        }
    }
}

/// 802.11 standard the connection runs on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WifiStandard {
    /// The standard is not reported.
    Undefined,
    /// 802.11a.
    Ieee80211A,
    /// 802.11b.
    Ieee80211B,
    /// 802.11g.
    Ieee80211G,
    /// 802.11n.
    Ieee80211N,
    /// 802.11ac.
    Ieee80211Ac,
    /// 802.11ax.
    Ieee80211Ax,
    /// 802.11ad.
    Ieee80211Ad,
    /// A standard value this crate does not know about.
    Other(u32),
}

impl WifiStandard {
    fn from_raw(raw: sys::OH_WifiStandard) -> Self {
        match raw {
            sys::OH_WifiStandard_OH_WIFI_STANDARD_UNDEFINED => WifiStandard::Undefined,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11A => WifiStandard::Ieee80211A,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11B => WifiStandard::Ieee80211B,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11G => WifiStandard::Ieee80211G,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11N => WifiStandard::Ieee80211N,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11AC => WifiStandard::Ieee80211Ac,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11AX => WifiStandard::Ieee80211Ax,
            sys::OH_WifiStandard_OH_WIFI_STANDARD_11AD => WifiStandard::Ieee80211Ad,
            other => WifiStandard::Other(other),
        }
    }
}

/// Feature category the access point is classified into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WifiCategory {
    /// Default category.
    Default,
    /// Wi-Fi 6.
    Wifi6,
    /// Wi-Fi 6 plus.
    Wifi6Plus,
    /// Wi-Fi 7.
    Wifi7,
    /// Wi-Fi 7 plus.
    Wifi7Plus,
    /// A category value this crate does not know about.
    Other(u32),
}

impl WifiCategory {
    fn from_raw(raw: sys::OH_WifiCategory) -> Self {
        match raw {
            sys::OH_WifiCategory_OH_WIFI_CATEGORY_DEFAULT => WifiCategory::Default,
            sys::OH_WifiCategory_OH_WIFI_CATEGORY_WIFI6 => WifiCategory::Wifi6,
            sys::OH_WifiCategory_OH_WIFI_CATEGORY_WIFI6_PLUS => WifiCategory::Wifi6Plus,
            sys::OH_WifiCategory_OH_WIFI_CATEGORY_WIFI7 => WifiCategory::Wifi7,
            sys::OH_WifiCategory_OH_WIFI_CATEGORY_WIFI7_PLUS => WifiCategory::Wifi7Plus,
            other => WifiCategory::Other(other),
        }
    }
}

/// Multi-link operation mode of the connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkType {
    /// Not connected.
    Disconnect,
    /// Default link.
    DefaultLink,
    /// Wi-Fi 7 single link.
    Wifi7SingleLink,
    /// Wi-Fi 7 multi-link single radio.
    Wifi7Mlsr,
    /// Wi-Fi 7 enhanced multi-link single radio.
    Wifi7Emlsr,
    /// Wi-Fi 7 simultaneous transmit and receive.
    Wifi7Str,
    /// Wi-Fi 7 legacy mode, without multi-link operation.
    Wifi7Legacy,
    /// A link type value this crate does not know about.
    Other(i32),
}

impl LinkType {
    fn from_raw(raw: sys::OH_WifiLinkType) -> Self {
        match raw {
            sys::OH_WifiLinkType_OH_WIFI_LINK_DISCONNECT => LinkType::Disconnect,
            sys::OH_WifiLinkType_OH_WIFI_LINK_DEFAULT_LINK => LinkType::DefaultLink,
            sys::OH_WifiLinkType_OH_WIFI_LINK_WIFI7_SINGLE_LINK => LinkType::Wifi7SingleLink,
            sys::OH_WifiLinkType_OH_WIFI_LINK_WIFI7_MLSR => LinkType::Wifi7Mlsr,
            sys::OH_WifiLinkType_OH_WIFI_LINK_WIFI7_EMLSR => LinkType::Wifi7Emlsr,
            sys::OH_WifiLinkType_OH_WIFI_LINK_WIFI7_STR => LinkType::Wifi7Str,
            sys::OH_WifiLinkType_OH_WIFI_LINK_WIFI7_LEGACY => LinkType::Wifi7Legacy,
            other => LinkType::Other(other),
        }
    }
}

/// Frequency band the access point operates on.
///
/// The native header documents the two values as plain integers rather than as
/// an enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Band {
    /// 2.4 GHz, reported as `1`.
    Band24Ghz,
    /// 5 GHz, reported as `2`.
    Band5Ghz,
    /// A band value this crate does not know about.
    Other(i32),
}

impl Band {
    fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Band::Band24Ghz,
            2 => Band::Band5Ghz,
            other => Band::Other(other),
        }
    }
}

/// Kind of MAC address the local station presents to the access point.
///
/// The native header documents the two values as plain integers rather than as
/// an enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacAddressType {
    /// A randomized address, reported as `0`.
    Random,
    /// The address of the device itself, reported as `1`.
    Device,
    /// An address type this crate does not know about.
    Other(i32),
}

impl MacAddressType {
    fn from_raw(raw: i32) -> Self {
        match raw {
            0 => MacAddressType::Random,
            1 => MacAddressType::Device,
            other => MacAddressType::Other(other),
        }
    }
}

/// Information about the access point the local station is connected to.
///
/// This is the owned form of the native `OH_WifiLinkedInfo` structure: the
/// fixed-size C character fields are decoded into a [`String`] and into
/// [`MacAddress`] values, and the C enumerations into Rust enumerations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkedInfo {
    /// Service set identifier of the access point, decoded from at most 32
    /// bytes. Bytes that are not valid UTF-8 are replaced.
    pub ssid: String,
    /// Basic service set identifier of the access point.
    ///
    /// This is the real BSSID only if the application holds
    /// `ohos.permission.GET_WIFI_PEERS_MAC`; otherwise the wifi service reports
    /// a randomized address. It is `None` when the service reported no address
    /// at all, and [`Some`] with a malformed value never occurs because such a
    /// value is rejected as [`WifiError::MalformedMacAddress`].
    ///
    /// [`WifiError::MalformedMacAddress`]: crate::WifiError::MalformedMacAddress
    pub bssid: Option<MacAddress>,
    /// Received signal strength indicator, in dBm.
    pub rssi: i32,
    /// Frequency band of the access point.
    pub band: Band,
    /// Negotiated link speed, in Mbps.
    pub link_speed_mbps: i32,
    /// Negotiated downlink speed, in Mbps.
    pub rx_link_speed_mbps: i32,
    /// Highest transmit link speed the link supports, in Mbps.
    pub max_supported_tx_link_speed_mbps: i32,
    /// Highest receive link speed the link supports, in Mbps.
    pub max_supported_rx_link_speed_mbps: i32,
    /// Frequency of the access point, in MHz.
    pub frequency_mhz: i32,
    /// Whether the access point hides its SSID.
    pub is_hidden: bool,
    /// Whether data access over this connection is restricted.
    pub is_restricted: bool,
    /// Kind of MAC address the local station uses on this connection.
    pub mac_address_type: MacAddressType,
    /// MAC address of the local station.
    ///
    /// When [`mac_address_type`] is [`MacAddressType::Device`], reading it also
    /// requires `ohos.permission.GET_WIFI_LOCAL_MAC`; without that permission
    /// the wifi service reports no address and this field is `None`.
    ///
    /// [`mac_address_type`]: LinkedInfo::mac_address_type
    pub mac_address: Option<MacAddress>,
    /// IPv4 address of the connection, in the packed form the wifi service
    /// reports it in.
    pub ip_address: u32,
    /// State of the connection.
    pub conn_state: ConnState,
    /// Channel width the access point currently operates on.
    pub channel_width: ChannelWidth,
    /// 802.11 standard the connection runs on.
    pub wifi_standard: WifiStandard,
    /// Feature category the access point is classified into.
    pub supported_wifi_category: WifiCategory,
    /// Whether the access point is part of a HiLink network.
    pub is_hilink_network: bool,
    /// Multi-link operation mode of the connection.
    pub link_type: LinkType,
}

impl LinkedInfo {
    /// Decode a native `OH_WifiLinkedInfo` the wifi service filled in.
    pub(crate) fn from_raw(
        raw: &sys::OH_WifiLinkedInfo,
    ) -> Result<Self, crate::mac::MacAddressParseError> {
        Ok(LinkedInfo {
            ssid: c_field_to_string(&raw.ssid),
            bssid: parse_mac_field(&raw.bssid)?,
            rssi: raw.rssi,
            band: Band::from_raw(raw.band),
            link_speed_mbps: raw.linkSpeed,
            rx_link_speed_mbps: raw.rxLinkSpeed,
            max_supported_tx_link_speed_mbps: raw.maxSupportedTxLinkSpeed,
            max_supported_rx_link_speed_mbps: raw.maxSupportedRxLinkSpeed,
            frequency_mhz: raw.frequency,
            is_hidden: raw.isHidden,
            is_restricted: raw.isRestricted,
            mac_address_type: MacAddressType::from_raw(raw.macType),
            mac_address: parse_mac_field(&raw.macAddress)?,
            ip_address: raw.ipAddress,
            conn_state: ConnState::from_raw(raw.connState),
            channel_width: ChannelWidth::from_raw(raw.channelWidth),
            wifi_standard: WifiStandard::from_raw(raw.wifiStandard),
            supported_wifi_category: WifiCategory::from_raw(raw.supportedWifiCategory),
            is_hilink_network: raw.isHiLinkNetwork,
            link_type: LinkType::from_raw(raw.wifiLinkType),
        })
    }
}
