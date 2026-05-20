#![allow(non_upper_case_globals)]

use std::ffi::CString;

use ohos_net_connection_sys::*;

use crate::{NetConnectionError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacketsType {
    #[default]
    Icmp,
    Udp,
    Unknown(u32),
}

impl From<NetConn_PacketsType> for PacketsType {
    fn from(value: NetConn_PacketsType) -> Self {
        match value {
            NetConn_PacketsType_NETCONN_PACKETS_ICMP => Self::Icmp,
            NetConn_PacketsType_NETCONN_PACKETS_UDP => Self::Udp,
            value => Self::Unknown(value),
        }
    }
}

impl From<PacketsType> for NetConn_PacketsType {
    fn from(value: PacketsType) -> Self {
        match value {
            PacketsType::Icmp => NetConn_PacketsType_NETCONN_PACKETS_ICMP,
            PacketsType::Udp => NetConn_PacketsType_NETCONN_PACKETS_UDP,
            PacketsType::Unknown(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeResultInfo {
    pub loss_rate: u8,
    pub rtt: [u32; 4],
}

impl From<NetConn_ProbeResultInfo> for ProbeResultInfo {
    fn from(value: NetConn_ProbeResultInfo) -> Self {
        Self {
            loss_rate: value.lossRate,
            rtt: value.rtt,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceRouteOption {
    pub max_jump_number: u8,
    pub packets_type: PacketsType,
}

impl Default for TraceRouteOption {
    fn default() -> Self {
        Self {
            max_jump_number: 30,
            packets_type: PacketsType::default(),
        }
    }
}

impl TraceRouteOption {
    pub fn query_trace_route(&self, destination: &str) -> Result<TraceRouteInfo> {
        let c_destination = CString::new(destination).map_err(|_| NetConnectionError::NullByte)?;
        let mut raw_option = self.to_raw();
        let mut raw_info = unsafe { std::mem::zeroed() };
        let code = unsafe {
            OH_NetConn_QueryTraceRoute(
                c_destination.as_ptr().cast_mut(),
                &mut raw_option,
                &mut raw_info,
            )
        };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        TraceRouteInfo::from_raw(&raw_info)
    }

    fn to_raw(self) -> NetConn_TraceRouteOption {
        NetConn_TraceRouteOption {
            maxJumpNumber: self.max_jump_number,
            packetsType: self.packets_type.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceRouteInfo {
    pub jump_no: u8,
    pub address: String,
    pub rtt: [u32; 4],
}

impl TraceRouteInfo {
    fn from_raw(value: &NetConn_TraceRouteInfo) -> Result<Self> {
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
            jump_no: value.jumpNo,
            address,
            rtt: value.rtt,
        })
    }
}

pub fn query_probe_result(destination: &str, duration: i32) -> Result<ProbeResultInfo> {
    let c_destination = CString::new(destination).map_err(|_| NetConnectionError::NullByte)?;
    let mut raw_info = unsafe { std::mem::zeroed() };
    let code = unsafe {
        OH_NetConn_QueryProbeResult(c_destination.as_ptr().cast_mut(), duration, &mut raw_info)
    };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    Ok(ProbeResultInfo::from(raw_info))
}
