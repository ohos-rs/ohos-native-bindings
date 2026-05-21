#![allow(non_upper_case_globals)]

use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

use ohos_net_stack_sys::*;

use ohos_enum_derive::EnumFrom;

use crate::{NetStackError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(NetStack_CertType, "NetStack_CertType_NETSTACK_CERT_TYPE_")]
pub enum CertType {
    Pem,
    Der,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CertBlob<'a> {
    pub cert_type: CertType,
    pub data: &'a [u8],
}

impl CertBlob<'_> {
    pub fn verify(&self, ca_cert: Option<&CertBlob<'_>>) -> Result<()> {
        let raw_cert = self.to_raw()?;
        let raw_ca_cert = ca_cert.map(|cert| cert.to_raw()).transpose()?;
        let code = unsafe {
            OH_NetStack_CertVerification(
                &raw_cert,
                raw_ca_cert.as_ref().map_or(ptr::null(), |value| value),
            )
        };
        if code != 0 {
            return Err(NetStackError::Code(code as i32));
        }
        Ok(())
    }

    fn to_raw(self) -> Result<NetStack_CertBlob> {
        if self.data.len() > u32::MAX as usize {
            return Err(NetStackError::StringTooLong);
        }
        Ok(NetStack_CertBlob {
            type_: self.cert_type.into(),
            size: self.data.len() as u32,
            data: self.data.as_ptr().cast_mut(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(NetStack_CertificatePinningKind, "NetStack_CertificatePinningKind_")]
pub enum CertificatePinningKind {
    PublicKey,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(NetStack_HashAlgorithm, "NetStack_HashAlgorithm_")]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificatePinning {
    pub kind: CertificatePinningKind,
    pub hash_algorithm: HashAlgorithm,
    pub public_key_hash: Option<String>,
}

impl CertificatePinning {
    pub fn for_hostname(hostname: &str) -> Result<Self> {
        let c_hostname = CString::new(hostname).map_err(|_| NetStackError::NullByte)?;
        let mut raw: NetStack_CertificatePinning = unsafe { std::mem::zeroed() };
        let code = unsafe { OH_NetStack_GetPinSetForHostName(c_hostname.as_ptr(), &mut raw) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Self::from_raw(raw)
    }

    fn from_raw(value: NetStack_CertificatePinning) -> Result<Self> {
        Ok(Self {
            kind: CertificatePinningKind::try_from_raw(value.kind)
                .ok_or(NetStackError::Conversion)?,
            hash_algorithm: HashAlgorithm::try_from_raw(value.hashAlgorithm)
                .ok_or(NetStackError::Conversion)?,
            public_key_hash: {
                let value = unsafe { value.__bindgen_anon_1.publicKeyHash };
                if value.is_null() {
                    None
                } else {
                    Some(
                        unsafe { CStr::from_ptr(value) }
                            .to_str()
                            .map_err(|_| NetStackError::Conversion)?
                            .to_owned(),
                    )
                }
            },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Certificates {
    pub content: Vec<String>,
}

impl Certificates {
    pub fn for_hostname(hostname: &str) -> Result<Self> {
        let c_hostname = CString::new(hostname).map_err(|_| NetStackError::NullByte)?;
        let mut raw: NetStack_Certificates = unsafe { std::mem::zeroed() };
        let code = unsafe { OH_NetStack_GetCertificatesForHostName(c_hostname.as_ptr(), &mut raw) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }

        let _guard = CertificatesGuard(&mut raw);
        if raw.content.is_null() || raw.length == 0 {
            return Ok(Self {
                content: Vec::new(),
            });
        }

        let mut content = Vec::with_capacity(raw.length);
        for item in unsafe { slice::from_raw_parts(raw.content, raw.length) } {
            if item.is_null() {
                continue;
            }
            content.push(
                unsafe { CStr::from_ptr(*item) }
                    .to_str()
                    .map_err(|_| NetStackError::Conversion)?
                    .to_owned(),
            );
        }
        Ok(Self { content })
    }
}

struct CertificatesGuard(*mut NetStack_Certificates);

impl Drop for CertificatesGuard {
    fn drop(&mut self) {
        unsafe { OH_Netstack_DestroyCertificatesContent(self.0) };
    }
}

#[cfg(feature = "api-18")]
pub fn is_cleartext_permitted() -> Result<bool> {
    let mut permitted = false;
    let code = unsafe { OH_Netstack_IsCleartextPermitted(&mut permitted) };
    if code != 0 {
        return Err(NetStackError::Code(code));
    }
    Ok(permitted)
}

#[cfg(feature = "api-18")]
pub fn is_cleartext_permitted_by_hostname(hostname: &str) -> Result<bool> {
    let c_hostname = CString::new(hostname).map_err(|_| NetStackError::NullByte)?;
    let mut permitted = false;
    let code =
        unsafe { OH_Netstack_IsCleartextPermittedByHostName(c_hostname.as_ptr(), &mut permitted) };
    if code != 0 {
        return Err(NetStackError::Code(code));
    }
    Ok(permitted)
}

#[cfg(feature = "api-20")]
pub fn is_cleartext_cfg_by_component(component: &str) -> Result<bool> {
    let c_component = CString::new(component).map_err(|_| NetStackError::NullByte)?;
    let mut configured = false;
    let code =
        unsafe { OH_Netstack_IsCleartextCfgByComponent(c_component.as_ptr(), &mut configured) };
    if code != 0 {
        return Err(NetStackError::Code(code));
    }
    Ok(configured)
}
