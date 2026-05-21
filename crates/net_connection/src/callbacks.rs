use std::ffi::CString;
use std::ptr;

use ohos_net_connection_sys::*;

use crate::{NetCapabilities, NetConnectionError, Result};

pub type CustomDnsResolverCallback = OH_NetConn_CustomDnsResolver;
pub type AppHttpProxyChange = OH_NetConn_AppHttpProxyChange;
pub type NetworkAvailableCallback = OH_NetConn_NetworkAvailable;
pub type NetCapabilitiesChangeCallback = OH_NetConn_NetCapabilitiesChange;
pub type NetConnectionPropertiesChangeCallback = OH_NetConn_NetConnectionPropertiesChange;
pub type NetLostCallback = OH_NetConn_NetLost;
pub type NetUnavailableCallback = OH_NetConn_NetUnavailable;
pub type NetBlockStatusChangeCallback = OH_NetConn_NetBlockStatusChange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallbackId(pub u32);

impl CallbackId {
    pub fn unregister_app_http_proxy_callback(self) {
        unsafe { OH_NetConn_UnregisterAppHttpProxyCallback(self.0) };
    }

    pub fn unregister_net_conn_callback(self) -> Result<()> {
        let code = unsafe { OH_NetConn_UnregisterNetConnCallback(self.0) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CustomDnsResolver {
    resolver: CustomDnsResolverCallback,
}

impl CustomDnsResolver {
    pub fn new(resolver: CustomDnsResolverCallback) -> Self {
        Self { resolver }
    }

    pub fn callback(&self) -> CustomDnsResolverCallback {
        self.resolver
    }

    #[cfg(feature = "api-13")]
    pub fn register(&self) -> Result<()> {
        let code = unsafe { OH_NetConn_RegisterDnsResolver(self.resolver) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(())
    }

    #[cfg(feature = "api-13")]
    pub fn unregister() -> Result<()> {
        let code = unsafe { OH_NetConn_UnregisterDnsResolver() };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AppHttpProxyChangeCallback {
    callback: AppHttpProxyChange,
}

impl AppHttpProxyChangeCallback {
    pub fn new(callback: AppHttpProxyChange) -> Self {
        Self { callback }
    }

    pub fn register(&self) -> Result<CallbackId> {
        let mut callback_id = 0;
        let code =
            unsafe { OH_NetConn_RegisterAppHttpProxyCallback(self.callback, &mut callback_id) };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(CallbackId(callback_id))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NetSpecifier {
    pub caps: NetCapabilities,
    pub bearer_private_identifier: Option<String>,
}

impl NetSpecifier {
    pub fn register_net_conn_callback(
        &self,
        callback: &NetConnCallback,
        timeout: u32,
    ) -> Result<CallbackId> {
        let mut raw_specifier = self.to_raw_owned()?;
        let mut raw_callback = callback.to_raw();
        let mut callback_id = 0;
        let code = unsafe {
            OH_NetConn_RegisterNetConnCallback(
                raw_specifier.as_mut_ptr(),
                &mut raw_callback,
                timeout,
                &mut callback_id,
            )
        };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(CallbackId(callback_id))
    }

    fn to_raw_owned(&self) -> Result<RawNetSpecifierOwned> {
        let bearer_private_identifier = self
            .bearer_private_identifier
            .as_ref()
            .map(|value| CString::new(value.as_str()))
            .transpose()
            .map_err(|_| NetConnectionError::NullByte)?;
        let mut raw = NetConn_NetSpecifier {
            caps: self.caps.to_raw()?,
            bearerPrivateIdentifier: ptr::null_mut(),
        };
        if let Some(value) = &bearer_private_identifier {
            raw.bearerPrivateIdentifier = value.as_ptr().cast_mut();
        }
        Ok(RawNetSpecifierOwned {
            raw,
            _bearer_private_identifier: bearer_private_identifier,
        })
    }
}

struct RawNetSpecifierOwned {
    raw: NetConn_NetSpecifier,
    _bearer_private_identifier: Option<CString>,
}

impl RawNetSpecifierOwned {
    fn as_mut_ptr(&mut self) -> *mut NetConn_NetSpecifier {
        &mut self.raw
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NetConnCallback {
    pub on_network_available: NetworkAvailableCallback,
    pub on_net_capabilities_change: NetCapabilitiesChangeCallback,
    pub on_connection_properties: NetConnectionPropertiesChangeCallback,
    pub on_net_lost: NetLostCallback,
    pub on_net_unavailable: NetUnavailableCallback,
    pub on_net_block_status_change: NetBlockStatusChangeCallback,
}

impl NetConnCallback {
    pub fn register_default_net_conn_callback(&self) -> Result<CallbackId> {
        let mut raw_callback = self.to_raw();
        let mut callback_id = 0;
        let code = unsafe {
            OH_NetConn_RegisterDefaultNetConnCallback(&mut raw_callback, &mut callback_id)
        };
        if code != 0 {
            return Err(NetConnectionError::Code(code));
        }
        Ok(CallbackId(callback_id))
    }

    fn to_raw(self) -> NetConn_NetConnCallback {
        NetConn_NetConnCallback {
            onNetworkAvailable: self.on_network_available,
            onNetCapabilitiesChange: self.on_net_capabilities_change,
            onConnetionProperties: self.on_connection_properties,
            onNetLost: self.on_net_lost,
            onNetUnavailable: self.on_net_unavailable,
            onNetBlockStatusChange: self.on_net_block_status_change,
        }
    }
}
