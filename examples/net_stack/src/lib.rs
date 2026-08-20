use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_net_stack_binding::{
    http::{Headers, Request},
    ssl::{
        is_cleartext_cfg_by_component, is_cleartext_permitted, is_cleartext_permitted_by_hostname,
    },
    websocket::{WebSocketCallbacks, WebSocketClient},
};

fn to_err(e: ohos_net_stack_binding::NetStackError) -> Error {
    Error::from_reason(e.to_string())
}

#[napi]
pub fn headers_roundtrip() -> Result<String> {
    let mut headers = Headers::new().map_err(to_err)?;
    headers.set("Accept", "text/plain").map_err(to_err)?;
    headers.set("X-Demo", "ohos-rs").map_err(to_err)?;
    let accept = headers.get("Accept").map_err(to_err)?;
    let entries = headers.entries().map_err(to_err)?;
    Ok(format!("accept={accept:?} entries={}", entries.len()))
}

#[napi]
pub fn create_request(url: String) -> Result<String> {
    let request = Request::new(&url).map_err(to_err)?;
    Ok(format!("request_id={}", request.request_id()))
}

#[napi]
pub fn cleartext_policy() -> Result<String> {
    let global = is_cleartext_permitted().map_err(to_err)?;
    let host = is_cleartext_permitted_by_hostname("example.com").map_err(to_err)?;
    let component = is_cleartext_cfg_by_component("web").map_err(to_err)?;
    Ok(format!(
        "permitted={global} host_example={host} component_web={component}"
    ))
}

#[napi]
pub fn websocket_construct() -> Result<String> {
    let client = WebSocketClient::new(WebSocketCallbacks::default()).map_err(to_err)?;
    client.destroy().map_err(to_err)?;
    Ok("constructed_and_destroyed".to_string())
}

#[napi]
pub fn smoke() -> Result<String> {
    Ok(format!(
        "{}\n{}\n{}\n{}",
        headers_roundtrip()?,
        create_request("https://www.huawei.com".to_string())?,
        cleartext_policy()?,
        websocket_construct()?
    ))
}
