#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

use ohos_net_stack_sys::*;

use ohos_enum_derive::EnumFrom;

use crate::{NetStackError, Result};

pub type ResponseCallback = Http_ResponseCallback;
pub type OnDataReceiveCallback = Http_OnDataReceiveCallback;
pub type OnProgressCallback = Http_OnProgressCallback;
pub type OnHeaderReceiveCallback = Http_OnHeaderReceiveCallback;
pub type OnVoidCallback = Http_OnVoidCallback;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Http_HttpProtocol, "Http_HttpProtocol_OH_")]
pub enum HttpProtocol {
    #[suffix("HTTP_NONE")]
    None,
    #[suffix("HTTP1_1")]
    Http1_1,
    #[suffix("HTTP2")]
    Http2,
    #[suffix("HTTP3")]
    Http3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Http_AddressFamilyType, "Http_AddressFamilyType_HTTP_ADDRESS_FAMILY_")]
pub enum AddressFamilyType {
    Default,
    #[suffix("ONLY_V4")]
    OnlyV4,
    #[suffix("ONLY_V6")]
    OnlyV6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Http_CertType, "Http_CertType_OH_HTTP_")]
pub enum CertType {
    Pem,
    Der,
    #[suffix("P12")]
    P12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Http_ProxyType, "Http_ProxyType_HTTP_PROXY_")]
enum ProxyType {
    NotUse,
    System,
    Custom,
}

#[derive(Debug)]
pub struct Headers {
    raw: *mut Http_Headers,
}

impl Headers {
    pub fn new() -> Result<Self> {
        let raw = unsafe { OH_Http_CreateHeaders() };
        if raw.is_null() {
            return Err(NetStackError::NullPointer);
        }
        Ok(Self { raw })
    }

    pub fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let c_name = CString::new(name).map_err(|_| NetStackError::NullByte)?;
        let c_value = CString::new(value).map_err(|_| NetStackError::NullByte)?;
        let code = unsafe { OH_Http_SetHeaderValue(self.raw, c_name.as_ptr(), c_value.as_ptr()) };
        if code != 0 {
            return Err(NetStackError::Code(code as i32));
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<Vec<String>> {
        let c_name = CString::new(name).map_err(|_| NetStackError::NullByte)?;
        let mut current = unsafe { OH_Http_GetHeaderValue(self.raw, c_name.as_ptr()) };
        let mut values = Vec::new();
        while !current.is_null() {
            let value = unsafe { &*current };
            if value.value.is_null() {
                return Err(NetStackError::NullPointer);
            }
            values.push(
                unsafe { CStr::from_ptr(value.value) }
                    .to_str()
                    .map_err(|_| NetStackError::Conversion)?
                    .to_owned(),
            );
            current = value.next;
        }
        Ok(values)
    }

    pub fn entries(&self) -> Result<HashMap<String, Vec<String>>> {
        let raw_entries = unsafe { OH_Http_GetHeaderEntries(self.raw) };
        if raw_entries.is_null() {
            return Ok(HashMap::new());
        }

        let _guard = HeaderEntriesGuard(raw_entries);
        let mut current = raw_entries;
        let mut entries = HashMap::new();
        while !current.is_null() {
            let entry = unsafe { &*current };
            let mut values = Vec::new();
            let mut current_value = entry.value;
            while !current_value.is_null() {
                let value = unsafe { &*current_value };
                if value.value.is_null() {
                    return Err(NetStackError::NullPointer);
                }
                values.push(
                    unsafe { CStr::from_ptr(value.value) }
                        .to_str()
                        .map_err(|_| NetStackError::Conversion)?
                        .to_owned(),
                );
                current_value = value.next;
            }
            if entry.key.is_null() {
                return Err(NetStackError::NullPointer);
            }
            let key = unsafe { CStr::from_ptr(entry.key) }
                .to_str()
                .map_err(|_| NetStackError::Conversion)?
                .to_owned();
            entries.insert(key, values);
            current = entry.next;
        }
        Ok(entries)
    }

    pub(crate) fn as_mut_ptr(&self) -> *mut Http_Headers {
        self.raw
    }
}

impl Drop for Headers {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { OH_Http_DestroyHeaders(&mut self.raw) };
        }
    }
}

struct HeaderEntriesGuard(*mut Http_HeaderEntry);

impl Drop for HeaderEntriesGuard {
    fn drop(&mut self) {
        let mut raw = self.0;
        unsafe { OH_Http_DestroyHeaderEntries(&mut raw) };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientCert {
    pub cert_path: String,
    pub cert_type: CertType,
    pub key_path: String,
    pub key_password: Option<String>,
}

impl ClientCert {
    fn to_raw_owned(&self) -> Result<RawClientCertOwned> {
        let cert_path =
            CString::new(self.cert_path.as_str()).map_err(|_| NetStackError::NullByte)?;
        let key_path = CString::new(self.key_path.as_str()).map_err(|_| NetStackError::NullByte)?;
        let key_password = self
            .key_password
            .as_deref()
            .map(CString::new)
            .transpose()
            .map_err(|_| NetStackError::NullByte)?;
        let raw = Http_ClientCert {
            certPath: cert_path.as_ptr().cast_mut(),
            type_: self.cert_type.into(),
            keyPath: key_path.as_ptr().cast_mut(),
            keyPassword: key_password
                .as_ref()
                .map_or(ptr::null_mut(), |value| value.as_ptr().cast_mut()),
        };
        Ok(RawClientCertOwned {
            raw,
            _cert_path: cert_path,
            _key_path: key_path,
            _key_password: key_password,
        })
    }
}

#[derive(Debug)]
struct RawClientCertOwned {
    raw: Http_ClientCert,
    _cert_path: CString,
    _key_path: CString,
    _key_password: Option<CString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomProxy {
    pub host: String,
    pub port: i32,
    pub exclusion_lists: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proxy {
    NotUse,
    System,
    Custom(CustomProxy),
}

impl Proxy {
    fn to_raw_owned(&self) -> Result<RawProxyOwned> {
        let host = match self {
            Self::Custom(proxy) => {
                Some(CString::new(proxy.host.as_str()).map_err(|_| NetStackError::NullByte)?)
            }
            _ => None,
        };
        let exclusion_lists = match self {
            Self::Custom(proxy) => proxy
                .exclusion_lists
                .as_deref()
                .map(CString::new)
                .transpose()
                .map_err(|_| NetStackError::NullByte)?,
            _ => None,
        };
        let proxy_type = match self {
            Self::NotUse => ProxyType::NotUse.into(),
            Self::System => ProxyType::System.into(),
            Self::Custom(_) => ProxyType::Custom.into(),
        };
        let custom = match self {
            Self::Custom(proxy) => Http_CustomProxy {
                host: host.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
                port: proxy.port,
                exclusionLists: exclusion_lists
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
            },
            _ => Http_CustomProxy {
                host: ptr::null(),
                port: 0,
                exclusionLists: ptr::null(),
            },
        };
        Ok(RawProxyOwned {
            raw: Http_Proxy {
                proxyType: proxy_type,
                customProxy: custom,
            },
            _host: host,
            _exclusion_lists: exclusion_lists,
        })
    }
}

#[derive(Debug)]
struct RawProxyOwned {
    raw: Http_Proxy,
    _host: Option<CString>,
    _exclusion_lists: Option<CString>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestOptions {
    pub method: Option<String>,
    pub priority: u32,
    pub headers: HashMap<String, String>,
    pub read_timeout: u32,
    pub connect_timeout: u32,
    pub http_protocol: HttpProtocol,
    pub proxy: Option<Proxy>,
    pub ca_path: Option<String>,
    pub resume_from: i64,
    pub resume_to: i64,
    pub client_cert: Option<ClientCert>,
    pub dns_over_https: Option<String>,
    pub address_family: AddressFamilyType,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            method: None,
            priority: 0,
            headers: HashMap::new(),
            read_timeout: 0,
            connect_timeout: 0,
            http_protocol: HttpProtocol::None,
            proxy: None,
            ca_path: None,
            resume_from: 0,
            resume_to: 0,
            client_cert: None,
            dns_over_https: None,
            address_family: AddressFamilyType::Default,
        }
    }
}

impl RequestOptions {
    fn to_raw_owned(&self) -> Result<RawRequestOptionsOwned> {
        let method = self
            .method
            .as_deref()
            .map(CString::new)
            .transpose()
            .map_err(|_| NetStackError::NullByte)?;
        let mut headers = if self.headers.is_empty() {
            None
        } else {
            Some(Headers::new()?)
        };
        if let Some(headers) = headers.as_mut() {
            for (name, value) in &self.headers {
                headers.set(name, value)?;
            }
        }
        let proxy = self.proxy.as_ref().map(Proxy::to_raw_owned).transpose()?;
        let ca_path = self
            .ca_path
            .as_deref()
            .map(CString::new)
            .transpose()
            .map_err(|_| NetStackError::NullByte)?;
        let mut client_cert = self
            .client_cert
            .as_ref()
            .map(ClientCert::to_raw_owned)
            .transpose()?;
        let dns_over_https = self
            .dns_over_https
            .as_deref()
            .map(CString::new)
            .transpose()
            .map_err(|_| NetStackError::NullByte)?;
        let mut proxy = proxy;
        let raw = Http_RequestOptions {
            method: method.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            priority: self.priority,
            headers: headers
                .as_ref()
                .map_or(ptr::null_mut(), Headers::as_mut_ptr),
            readTimeout: self.read_timeout,
            connectTimeout: self.connect_timeout,
            httpProtocol: self.http_protocol.into(),
            httpProxy: proxy
                .as_mut()
                .map_or(ptr::null_mut(), |value| &mut value.raw),
            caPath: ca_path.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            resumeFrom: self.resume_from,
            resumeTo: self.resume_to,
            clientCert: client_cert
                .as_mut()
                .map_or(ptr::null_mut(), |value| &mut value.raw),
            dnsOverHttps: dns_over_https
                .as_ref()
                .map_or(ptr::null(), |value| value.as_ptr()),
            addressFamily: self.address_family.into(),
        };
        Ok(RawRequestOptionsOwned {
            raw,
            _method: method,
            _headers: headers,
            _proxy: proxy,
            _ca_path: ca_path,
            _client_cert: client_cert,
            _dns_over_https: dns_over_https,
        })
    }
}

#[derive(Debug)]
struct RawRequestOptionsOwned {
    raw: Http_RequestOptions,
    _method: Option<CString>,
    _headers: Option<Headers>,
    _proxy: Option<RawProxyOwned>,
    _ca_path: Option<CString>,
    _client_cert: Option<RawClientCertOwned>,
    _dns_over_https: Option<CString>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EventsHandler {
    pub on_data_receive: OnDataReceiveCallback,
    pub on_upload_progress: OnProgressCallback,
    pub on_download_progress: OnProgressCallback,
    pub on_headers_receive: OnHeaderReceiveCallback,
    pub on_data_end: OnVoidCallback,
    pub on_canceled: OnVoidCallback,
}

impl EventsHandler {
    pub(crate) fn to_raw(self) -> Http_EventsHandler {
        Http_EventsHandler {
            onDataReceive: self.on_data_receive,
            onUploadProgress: self.on_upload_progress,
            onDownloadProgress: self.on_download_progress,
            onHeadersReceive: self.on_headers_receive,
            onDataEnd: self.on_data_end,
            onCanceled: self.on_canceled,
        }
    }
}

#[derive(Debug)]
pub struct Request {
    raw: *mut Http_Request,
    options: Option<RawRequestOptionsOwned>,
}

impl Request {
    pub fn new(url: &str) -> Result<Self> {
        let c_url = CString::new(url).map_err(|_| NetStackError::NullByte)?;
        let raw = unsafe { OH_Http_CreateRequest(c_url.as_ptr()) };
        if raw.is_null() {
            return Err(NetStackError::NullPointer);
        }
        Ok(Self { raw, options: None })
    }

    pub fn set_options(&mut self, options: &RequestOptions) -> Result<()> {
        let mut raw_options = options.to_raw_owned()?;
        unsafe {
            (*self.raw).options = &mut raw_options.raw;
        }
        self.options = Some(raw_options);
        Ok(())
    }

    pub fn request(&mut self, callback: ResponseCallback, handler: EventsHandler) -> Result<()> {
        let code = unsafe { OH_Http_Request(self.raw, callback, handler.to_raw()) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }

    pub fn request_id(&self) -> u32 {
        unsafe { (*self.raw).requestId }
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { OH_Http_Destroy(&mut self.raw) };
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceTiming {
    pub dns_timing: f64,
    pub tcp_timing: f64,
    pub tls_timing: f64,
    pub first_send_timing: f64,
    pub first_receive_timing: f64,
    pub total_finish_timing: f64,
    pub redirect_timing: f64,
}

impl From<Http_PerformanceTiming> for PerformanceTiming {
    fn from(value: Http_PerformanceTiming) -> Self {
        Self {
            dns_timing: value.dnsTiming,
            tcp_timing: value.tcpTiming,
            tls_timing: value.tlsTiming,
            first_send_timing: value.firstSendTiming,
            first_receive_timing: value.firstReceiveTiming,
            total_finish_timing: value.totalFinishTiming,
            redirect_timing: value.redirectTiming,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub body: Vec<u8>,
    pub response_code: u32,
    pub cookies: Option<String>,
    pub performance_timing: Option<PerformanceTiming>,
}

impl Response {
    /// Copies a C response pointer into an owned Rust value.
    ///
    /// # Safety
    ///
    /// The pointer must be provided by the netstack HTTP callback and remain valid for this call.
    pub unsafe fn from_raw(value: *mut Http_Response) -> Result<Self> {
        unsafe {
            if value.is_null() {
                return Err(NetStackError::NullPointer);
            }
            let value = &*value;
            let body = if value.body.buffer.is_null() || value.body.length == 0 {
                Vec::new()
            } else {
                slice::from_raw_parts(value.body.buffer.cast::<u8>(), value.body.length as usize)
                    .to_vec()
            };
            let performance_timing = if value.performanceTiming.is_null() {
                None
            } else {
                Some(PerformanceTiming::from(*value.performanceTiming))
            };
            Ok(Self {
                body,
                response_code: value.responseCode,
                cookies: if value.cookies.is_null() {
                    None
                } else {
                    Some(
                        CStr::from_ptr(value.cookies)
                            .to_str()
                            .map_err(|_| NetStackError::Conversion)?
                            .to_owned(),
                    )
                },
                performance_timing,
            })
        }
    }
}
