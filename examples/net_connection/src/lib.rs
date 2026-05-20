use std::fmt::Debug;

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_net_connection_binding::{
    get_default_http_proxy, get_default_net, has_default_net, is_default_net_metered,
};

fn to_napi_error(error: ohos_net_connection_binding::NetConnectionError) -> Error {
    Error::from_reason(error.to_string())
}

fn push_result<T: Debug>(
    lines: &mut Vec<String>,
    label: &str,
    result: ohos_net_connection_binding::Result<T>,
) {
    match result {
        Ok(value) => lines.push(format!("{label}: {value:#?}")),
        Err(error) => lines.push(format!("{label}: error: {error}")),
    }
}

#[napi]
pub fn default_network_info() -> Result<String> {
    let has_default_net = has_default_net().map_err(to_napi_error)?;
    let mut lines = vec![format!("has_default_net: {has_default_net}")];

    if !has_default_net {
        return Ok(lines.join("\n"));
    }

    let net_handle = get_default_net().map_err(to_napi_error)?;
    lines.push(format!("default_net: {net_handle:#?}"));

    push_result(
        &mut lines,
        "is_default_net_metered",
        is_default_net_metered(),
    );
    push_result(
        &mut lines,
        "default_net_capabilities",
        net_handle.net_capabilities(),
    );
    push_result(
        &mut lines,
        "default_connection_properties",
        net_handle.connection_properties(),
    );
    push_result(&mut lines, "default_http_proxy", get_default_http_proxy());

    Ok(lines.join("\n"))
}
