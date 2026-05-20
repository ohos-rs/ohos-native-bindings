use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

use ohos_net_connection_sys::*;

use crate::{NetConnectionError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AddrInfoHints {
    pub flags: i32,
    pub family: i32,
    pub sock_type: i32,
    pub protocol: i32,
}

impl AddrInfoHints {
    pub fn get_addr_info(
        &self,
        host: &str,
        service: Option<&str>,
        net_id: i32,
    ) -> Result<DnsResult> {
        let raw_hint = self.to_raw();
        get_addr_info_with_raw_hint(host, service, Some(raw_hint), net_id)
    }

    fn to_raw(self) -> addrinfo {
        addrinfo {
            ai_flags: self.flags,
            ai_family: self.family,
            ai_socktype: self.sock_type,
            ai_protocol: self.protocol,
            ai_addrlen: 0,
            ai_addr: ptr::null_mut(),
            ai_canonname: ptr::null_mut(),
            ai_next: ptr::null_mut(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddrInfo {
    pub flags: i32,
    pub family: i32,
    pub sock_type: i32,
    pub protocol: i32,
    pub addr_len: usize,
    pub addr: Option<Vec<u8>>,
    pub canon_name: Option<String>,
}

impl AddrInfo {
    fn from_raw(value: &addrinfo) -> Result<Self> {
        let addr_len = value.ai_addrlen as usize;
        let addr = if value.ai_addr.is_null() || addr_len == 0 {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(value.ai_addr.cast::<u8>(), addr_len) }.to_vec())
        };
        let canon_name = if value.ai_canonname.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.ai_canonname) }
                    .to_str()
                    .map_err(|_| NetConnectionError::Conversion)?
                    .to_owned(),
            )
        };

        Ok(Self {
            flags: value.ai_flags,
            family: value.ai_family,
            sock_type: value.ai_socktype,
            protocol: value.ai_protocol,
            addr_len,
            addr,
            canon_name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsResult {
    infos: Vec<AddrInfo>,
}

impl DnsResult {
    pub fn infos(&self) -> &[AddrInfo] {
        &self.infos
    }

    pub fn into_infos(self) -> Vec<AddrInfo> {
        self.infos
    }

    fn from_raw_list(result: *mut addrinfo) -> Result<Self> {
        if result.is_null() {
            return Err(NetConnectionError::Conversion);
        }

        let _guard = DnsResultGuard(result);
        let mut infos = Vec::new();
        let mut current = result;
        while !current.is_null() {
            let item = unsafe { &*current };
            infos.push(AddrInfo::from_raw(item)?);
            current = item.ai_next;
        }

        Ok(Self { infos })
    }
}

struct DnsResultGuard(*mut addrinfo);

impl Drop for DnsResultGuard {
    fn drop(&mut self) {
        unsafe { OH_NetConn_FreeDnsResult(self.0) };
    }
}

pub fn get_addr_info(host: &str, service: Option<&str>, net_id: i32) -> Result<DnsResult> {
    get_addr_info_with_raw_hint(host, service, None, net_id)
}

fn get_addr_info_with_raw_hint(
    host: &str,
    service: Option<&str>,
    hint: Option<addrinfo>,
    net_id: i32,
) -> Result<DnsResult> {
    let c_host = CString::new(host).map_err(|_| NetConnectionError::NullByte)?;
    let c_service = service
        .map(CString::new)
        .transpose()
        .map_err(|_| NetConnectionError::NullByte)?;
    let mut raw_hint = hint;
    let mut result = ptr::null_mut();

    let code = unsafe {
        OH_NetConn_GetAddrInfo(
            c_host.as_ptr().cast_mut(),
            c_service
                .as_ref()
                .map_or(ptr::null_mut(), |value| value.as_ptr().cast_mut()),
            raw_hint
                .as_mut()
                .map_or(ptr::null_mut(), |value| value as *mut _),
            &mut result,
            net_id,
        )
    };
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }

    DnsResult::from_raw_list(result)
}
