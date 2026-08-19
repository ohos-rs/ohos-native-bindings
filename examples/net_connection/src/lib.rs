use std::time::Duration;

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_net_connection_binding::{
    get_addr_info, get_all_nets, get_default_http_proxy, get_default_net,
    is_default_net_metered,
};

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

/// Whether a default network exists.
#[napi]
pub fn has_default_net() -> Result<bool> {
    ohos_net_connection_binding::has_default_net().map_err(to_napi_error)
}

/// Whether the default network is metered.
#[napi]
pub fn default_net_metered() -> Result<bool> {
    is_default_net_metered().map_err(to_napi_error)
}

/// Net ids of every connected network.
#[napi]
pub fn all_net_ids() -> Result<String> {
    let nets = get_all_nets().map_err(to_napi_error)?;
    let ids: Vec<String> = nets.iter().map(|n| format!("{}", n.net_id)).collect();
    Ok(ids.join(", "))
}

/// Capabilities and connection properties of the default network.
#[napi]
pub fn default_net_info() -> Result<String> {
    let net = get_default_net().map_err(to_napi_error)?;
    let caps = net.net_capabilities().map_err(to_napi_error)?;
    let props = net.connection_properties().map_err(to_napi_error)?;
    Ok(format!(
        "capabilities:\n{caps:#?}\n\nconnection properties:\niface={} mtu={} addresses={} dns={}\nhttp_proxy={:?}",
        props.iface_name,
        props.mtu,
        props
            .net_addr_list
            .iter()
            .map(|a| a.address.clone())
            .collect::<Vec<_>>()
            .join(", "),
        props
            .dns_list
            .iter()
            .map(|a| a.address.clone())
            .collect::<Vec<_>>()
            .join(", "),
        props.http_proxy,
    ))
}

/// Resolve a host name through the default network's DNS.
#[napi]
pub fn resolve_host(host: String) -> Result<String> {
    let net = get_default_net().map_err(to_napi_error)?;
    let result = get_addr_info(&host, None, net.net_id).map_err(to_napi_error)?;
    let mut out = String::new();
    for info in result.infos() {
        out.push_str(&format!(
            "family={} sock_type={} protocol={} canon={:?}\n",
            info.family, info.sock_type, info.protocol, info.canon_name
        ));
    }
    Ok(out)
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
