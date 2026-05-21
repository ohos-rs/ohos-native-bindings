use std::time::Duration;

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_net_connection_binding::get_default_http_proxy;

const GOOGLE_PROBE_URL: &str = "http://www.google.com/robots.txt";
const GOOGLE_EXPECTED_TEXT: &str = "User-agent";
const GOOGLE_PROBE_TIMEOUT: Duration = Duration::from_secs(8);
const BODY_PREVIEW_LIMIT: usize = 1024;

fn to_napi_error(error: ohos_net_connection_binding::NetConnectionError) -> Error {
    Error::from_reason(error.to_string())
}

#[napi]
pub fn current_proxy_info() -> Result<String> {
    let proxy = get_default_http_proxy().map_err(to_napi_error)?;
    Ok(format!("{proxy:#?}"))
}

#[napi]
pub fn google_vpn_probe() -> Result<String> {
    Ok(match request_google() {
        Ok((status, body)) => {
            let uses_system_vpn = status.is_success() && body.contains(GOOGLE_EXPECTED_TEXT);
            format!(
                "uses_system_vpn: {uses_system_vpn}\nstatus: {status}\nbody_preview:\n{}",
                body_preview(&body)
            )
        }
        Err(error) => {
            format!("uses_system_vpn: false\nurl: {GOOGLE_PROBE_URL}\nerror: {error}")
        }
    })
}

fn request_google() -> std::result::Result<(reqwest::StatusCode, String), reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .timeout(GOOGLE_PROBE_TIMEOUT)
        .user_agent("ohos-native-bindings-net-connection-example")
        .build()?;
    let response = client.get(GOOGLE_PROBE_URL).send()?;
    let status = response.status();
    let body = String::from_utf8_lossy(&response.bytes()?).into_owned();
    Ok((status, body))
}

fn body_preview(body: &str) -> &str {
    match body.char_indices().nth(BODY_PREVIEW_LIMIT) {
        Some((index, _)) => &body[..index],
        None => body,
    }
}
