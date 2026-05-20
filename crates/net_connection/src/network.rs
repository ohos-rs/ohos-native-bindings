#![allow(non_upper_case_globals)]

use ohos_net_connection_sys::*;

use crate::{HttpProxy, NetConnectionError, Result};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetHandle {
    pub net_id: i32,
}

impl From<NetConn_NetHandle> for NetHandle {
    fn from(value: NetConn_NetHandle) -> Self {
        Self {
            net_id: value.netId,
        }
    }
}

impl From<NetHandle> for NetConn_NetHandle {
    fn from(value: NetHandle) -> Self {
        NetConn_NetHandle {
            netId: value.net_id,
        }
    }
}

impl NetHandle {
    pub fn connection_properties(&self) -> Result<ConnectionProperties> {
        let mut raw_handle = NetConn_NetHandle::from(*self);
        let mut raw_properties = unsafe { std::mem::zeroed() };
        let code =
            unsafe { OH_NetConn_GetConnectionProperties(&mut raw_handle, &mut raw_properties) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        ConnectionProperties::from_raw(&raw_properties)
    }

    pub fn net_capabilities(&self) -> Result<NetCapabilities> {
        let mut raw_handle = NetConn_NetHandle::from(*self);
        let mut raw_capabilities = unsafe { std::mem::zeroed() };
        let code = unsafe { OH_NetConn_GetNetCapabilities(&mut raw_handle, &mut raw_capabilities) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(NetCapabilities::from(raw_capabilities))
    }

    pub fn bind_socket(&self, socket_fd: i32) -> Result<()> {
        let mut raw_handle = NetConn_NetHandle::from(*self);
        let code = unsafe { OH_NetConn_BindSocket(socket_fd, &mut raw_handle) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetCap {
    Mms,
    NotMetered,
    Internet,
    NotVpn,
    Validated,
    Portal,
    CheckingConnectivity,
    Unknown(u32),
}

impl From<NetConn_NetCap> for NetCap {
    fn from(value: NetConn_NetCap) -> Self {
        match value {
            NetConn_NetCap_NETCONN_NET_CAPABILITY_MMS => Self::Mms,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_NOT_METERED => Self::NotMetered,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_INTERNET => Self::Internet,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_NOT_VPN => Self::NotVpn,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_VALIDATED => Self::Validated,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_PORTAL => Self::Portal,
            NetConn_NetCap_NETCONN_NET_CAPABILITY_CHECKING_CONNECTIVITY => {
                Self::CheckingConnectivity
            }
            value => Self::Unknown(value),
        }
    }
}

impl From<NetCap> for NetConn_NetCap {
    fn from(value: NetCap) -> Self {
        match value {
            NetCap::Mms => NetConn_NetCap_NETCONN_NET_CAPABILITY_MMS,
            NetCap::NotMetered => NetConn_NetCap_NETCONN_NET_CAPABILITY_NOT_METERED,
            NetCap::Internet => NetConn_NetCap_NETCONN_NET_CAPABILITY_INTERNET,
            NetCap::NotVpn => NetConn_NetCap_NETCONN_NET_CAPABILITY_NOT_VPN,
            NetCap::Validated => NetConn_NetCap_NETCONN_NET_CAPABILITY_VALIDATED,
            NetCap::Portal => NetConn_NetCap_NETCONN_NET_CAPABILITY_PORTAL,
            NetCap::CheckingConnectivity => {
                NetConn_NetCap_NETCONN_NET_CAPABILITY_CHECKING_CONNECTIVITY
            }
            NetCap::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetBearerType {
    Cellular,
    Wifi,
    Bluetooth,
    Ethernet,
    Vpn,
    Unknown(u32),
}

impl From<NetConn_NetBearerType> for NetBearerType {
    fn from(value: NetConn_NetBearerType) -> Self {
        match value {
            NetConn_NetBearerType_NETCONN_BEARER_CELLULAR => Self::Cellular,
            NetConn_NetBearerType_NETCONN_BEARER_WIFI => Self::Wifi,
            NetConn_NetBearerType_NETCONN_BEARER_BLUETOOTH => Self::Bluetooth,
            NetConn_NetBearerType_NETCONN_BEARER_ETHERNET => Self::Ethernet,
            NetConn_NetBearerType_NETCONN_BEARER_VPN => Self::Vpn,
            value => Self::Unknown(value),
        }
    }
}

impl From<NetBearerType> for NetConn_NetBearerType {
    fn from(value: NetBearerType) -> Self {
        match value {
            NetBearerType::Cellular => NetConn_NetBearerType_NETCONN_BEARER_CELLULAR,
            NetBearerType::Wifi => NetConn_NetBearerType_NETCONN_BEARER_WIFI,
            NetBearerType::Bluetooth => NetConn_NetBearerType_NETCONN_BEARER_BLUETOOTH,
            NetBearerType::Ethernet => NetConn_NetBearerType_NETCONN_BEARER_ETHERNET,
            NetBearerType::Vpn => NetConn_NetBearerType_NETCONN_BEARER_VPN,
            NetBearerType::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NetCapabilities {
    pub link_up_bandwidth_kbps: u32,
    pub link_down_bandwidth_kbps: u32,
    pub net_caps: Vec<NetCap>,
    pub bearer_types: Vec<NetBearerType>,
}

impl From<NetConn_NetCapabilities> for NetCapabilities {
    fn from(value: NetConn_NetCapabilities) -> Self {
        let net_caps_len = if value.netCapsSize <= 0 {
            0
        } else {
            (value.netCapsSize as usize).min(value.netCaps.len())
        };
        let bearer_types_len = if value.bearerTypesSize <= 0 {
            0
        } else {
            (value.bearerTypesSize as usize).min(value.bearerTypes.len())
        };

        Self {
            link_up_bandwidth_kbps: value.linkUpBandwidthKbps,
            link_down_bandwidth_kbps: value.linkDownBandwidthKbps,
            net_caps: value.netCaps[..net_caps_len]
                .iter()
                .copied()
                .map(NetCap::from)
                .collect(),
            bearer_types: value.bearerTypes[..bearer_types_len]
                .iter()
                .copied()
                .map(NetBearerType::from)
                .collect(),
        }
    }
}

impl NetCapabilities {
    pub(crate) fn to_raw(&self) -> Result<NetConn_NetCapabilities> {
        let mut raw: NetConn_NetCapabilities = unsafe { std::mem::zeroed() };

        if self.net_caps.len() > raw.netCaps.len()
            || self.bearer_types.len() > raw.bearerTypes.len()
        {
            return Err(NetConnectionError::TooManyItems);
        }

        raw.linkUpBandwidthKbps = self.link_up_bandwidth_kbps;
        raw.linkDownBandwidthKbps = self.link_down_bandwidth_kbps;
        raw.netCapsSize = self.net_caps.len() as i32;
        raw.bearerTypesSize = self.bearer_types.len() as i32;

        for (index, item) in self.net_caps.iter().copied().enumerate() {
            raw.netCaps[index] = item.into();
        }

        for (index, item) in self.bearer_types.iter().copied().enumerate() {
            raw.bearerTypes[index] = item.into();
        }

        Ok(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetAddr {
    pub family: u8,
    pub prefix_len: u8,
    pub port: u8,
    pub address: String,
}

impl NetAddr {
    pub(crate) fn from_raw(value: &NetConn_NetAddr) -> Result<Self> {
        let address_len = value
            .address
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.address.len());
        let address = String::from_utf8(
            value.address[..address_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;

        Ok(Self {
            family: value.family,
            prefix_len: value.prefixlen,
            port: value.port,
            address,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub iface: String,
    pub destination: NetAddr,
    pub gateway: NetAddr,
    pub has_gateway: bool,
    pub is_default_route: bool,
}

impl Route {
    pub(crate) fn from_raw(value: &NetConn_Route) -> Result<Self> {
        let iface_len = value
            .iface
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.iface.len());
        let iface = String::from_utf8(
            value.iface[..iface_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;

        Ok(Self {
            iface,
            destination: NetAddr::from_raw(&value.destination)?,
            gateway: NetAddr::from_raw(&value.gateway)?,
            has_gateway: value.hasGateway != 0,
            is_default_route: value.isDefaultRoute != 0,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionProperties {
    pub iface_name: String,
    pub domain: String,
    pub tcp_buffer_sizes: String,
    pub mtu: u16,
    pub net_addr_list: Vec<NetAddr>,
    pub dns_list: Vec<NetAddr>,
    pub route_list: Vec<Route>,
    pub http_proxy: HttpProxy,
}

impl ConnectionProperties {
    pub(crate) fn from_raw(value: &NetConn_ConnectionProperties) -> Result<Self> {
        let net_addr_len = if value.netAddrListSize <= 0 {
            0
        } else {
            (value.netAddrListSize as usize).min(value.netAddrList.len())
        };
        let dns_len = if value.dnsListSize <= 0 {
            0
        } else {
            (value.dnsListSize as usize).min(value.dnsList.len())
        };
        let route_len = if value.routeListSize <= 0 {
            0
        } else {
            (value.routeListSize as usize).min(value.routeList.len())
        };

        let mut net_addr_list = Vec::with_capacity(net_addr_len);
        for item in value.netAddrList[..net_addr_len].iter() {
            net_addr_list.push(NetAddr::from_raw(item)?);
        }

        let mut dns_list = Vec::with_capacity(dns_len);
        for item in value.dnsList[..dns_len].iter() {
            dns_list.push(NetAddr::from_raw(item)?);
        }

        let mut route_list = Vec::with_capacity(route_len);
        for item in value.routeList[..route_len].iter() {
            route_list.push(Route::from_raw(item)?);
        }

        let iface_name_len = value
            .ifaceName
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.ifaceName.len());
        let iface_name = String::from_utf8(
            value.ifaceName[..iface_name_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;
        let domain_len = value
            .domain
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.domain.len());
        let domain = String::from_utf8(
            value.domain[..domain_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;
        let tcp_buffer_sizes_len = value
            .tcpBufferSizes
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.tcpBufferSizes.len());
        let tcp_buffer_sizes = String::from_utf8(
            value.tcpBufferSizes[..tcp_buffer_sizes_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;

        Ok(Self {
            iface_name,
            domain,
            tcp_buffer_sizes,
            mtu: value.mtu,
            net_addr_list,
            dns_list,
            route_list,
            http_proxy: HttpProxy::from_raw(&value.httpProxy)?,
        })
    }
}

pub fn has_default_net() -> Result<bool> {
    let mut has_default_net = 0;
    let code = unsafe { OH_NetConn_HasDefaultNet(&mut has_default_net) };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    Ok(has_default_net != 0)
}

pub fn get_default_net() -> Result<NetHandle> {
    let mut raw_handle = unsafe { std::mem::zeroed() };
    let code = unsafe { OH_NetConn_GetDefaultNet(&mut raw_handle) };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    Ok(NetHandle::from(raw_handle))
}

pub fn is_default_net_metered() -> Result<bool> {
    let mut is_metered = 0;
    let code = unsafe { OH_NetConn_IsDefaultNetMetered(&mut is_metered) };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    Ok(is_metered != 0)
}

pub fn get_all_nets() -> Result<Vec<NetHandle>> {
    let mut raw_list: NetConn_NetHandleList = unsafe { std::mem::zeroed() };
    let code = unsafe { OH_NetConn_GetAllNets(&mut raw_list) };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }

    let len = if raw_list.netHandleListSize <= 0 {
        0
    } else {
        (raw_list.netHandleListSize as usize).min(raw_list.netHandles.len())
    };
    Ok(raw_list.netHandles[..len]
        .iter()
        .copied()
        .map(NetHandle::from)
        .collect())
}
