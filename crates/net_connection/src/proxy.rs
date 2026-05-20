use std::ffi::CString;

use ohos_net_connection_sys::*;

use crate::{NetConnectionError, Result};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HttpProxy {
    pub host: String,
    pub exclusion_list: Vec<String>,
    pub port: u16,
}

impl HttpProxy {
    pub fn set_app_http_proxy(&self) -> Result<()> {
        let mut raw_proxy = self.to_raw()?;
        let code = unsafe { OH_NetConn_SetAppHttpProxy(&mut raw_proxy) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(())
    }

    pub(crate) fn from_raw(value: &NetConn_HttpProxy) -> Result<Self> {
        let exclusion_len = if value.exclusionListSize <= 0 {
            0
        } else {
            (value.exclusionListSize as usize).min(value.exclusionList.len())
        };
        let mut exclusion_list = Vec::with_capacity(exclusion_len);
        for item in value.exclusionList[..exclusion_len].iter() {
            let len = item
                .iter()
                .position(|value| *value == 0)
                .unwrap_or(item.len());
            exclusion_list.push(
                String::from_utf8(item[..len].iter().map(|value| *value as u8).collect())
                    .map_err(|_| NetConnectionError::Conversion)?,
            );
        }
        let host_len = value
            .host
            .iter()
            .position(|item| *item == 0)
            .unwrap_or(value.host.len());
        let host = String::from_utf8(
            value.host[..host_len]
                .iter()
                .map(|item| *item as u8)
                .collect(),
        )
        .map_err(|_| NetConnectionError::Conversion)?;

        Ok(Self {
            host,
            exclusion_list,
            port: value.port,
        })
    }

    pub(crate) fn to_raw(&self) -> Result<NetConn_HttpProxy> {
        let mut raw: NetConn_HttpProxy = unsafe { std::mem::zeroed() };
        let host = CString::new(self.host.as_str()).map_err(|_| NetConnectionError::NullByte)?;
        let host = host.as_bytes_with_nul();
        if host.len() > raw.host.len() {
            return Err(NetConnectionError::StringTooLong);
        }
        raw.host.fill(0);
        for (index, byte) in host.iter().enumerate() {
            raw.host[index] = *byte as _;
        }

        if self.exclusion_list.len() > raw.exclusionList.len() {
            return Err(NetConnectionError::StringTooLong);
        }
        raw.exclusionListSize = self.exclusion_list.len() as i32;
        for (index, item) in self.exclusion_list.iter().enumerate() {
            let item = CString::new(item.as_str()).map_err(|_| NetConnectionError::NullByte)?;
            let item = item.as_bytes_with_nul();
            if item.len() > raw.exclusionList[index].len() {
                return Err(NetConnectionError::StringTooLong);
            }
            raw.exclusionList[index].fill(0);
            for (byte_index, byte) in item.iter().enumerate() {
                raw.exclusionList[index][byte_index] = *byte as _;
            }
        }

        raw.port = self.port;
        Ok(raw)
    }
}

pub fn get_default_http_proxy() -> Result<HttpProxy> {
    let mut raw_proxy = unsafe { std::mem::zeroed() };
    let code = unsafe { OH_NetConn_GetDefaultHttpProxy(&mut raw_proxy) };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    HttpProxy::from_raw(&raw_proxy)
}
