use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_qos_binding::{get_thread_qos, reset_thread_qos, set_thread_qos, QosLevel};

fn to_err(e: ohos_qos_binding::QosError) -> Error {
    Error::from_reason(format!("{e:?}"))
}

fn level_name(level: QosLevel) -> &'static str {
    match level {
        QosLevel::Background => "Background",
        QosLevel::Utility => "Utility",
        QosLevel::Default => "Default",
        QosLevel::UserInitiated => "UserInitiated",
        QosLevel::DeadlineRequest => "DeadlineRequest",
        QosLevel::UserInteractive => "UserInteractive",
    }
}

fn parse_level(name: &str) -> Result<QosLevel> {
    match name {
        "Background" => Ok(QosLevel::Background),
        "Utility" => Ok(QosLevel::Utility),
        "Default" => Ok(QosLevel::Default),
        "UserInitiated" => Ok(QosLevel::UserInitiated),
        "DeadlineRequest" => Ok(QosLevel::DeadlineRequest),
        "UserInteractive" => Ok(QosLevel::UserInteractive),
        other => Err(Error::from_reason(format!("unknown qos level {other}"))),
    }
}

#[napi]
pub fn set_qos(level: String) -> Result<()> {
    set_thread_qos(parse_level(&level)?).map_err(to_err)
}

#[napi]
pub fn current_qos() -> Result<String> {
    get_thread_qos()
        .map(level_name)
        .map(str::to_string)
        .map_err(to_err)
}

#[napi]
pub fn reset_qos() -> Result<()> {
    reset_thread_qos().map_err(to_err)
}

#[napi]
pub fn smoke() -> Result<String> {
    let mut out = String::new();
    for level in [
        QosLevel::Background,
        QosLevel::Utility,
        QosLevel::Default,
        QosLevel::UserInitiated,
        QosLevel::DeadlineRequest,
        QosLevel::UserInteractive,
    ] {
        set_thread_qos(level).map_err(to_err)?;
        let got = get_thread_qos().map_err(to_err)?;
        out.push_str(&format!(
            "set {} -> got {}\n",
            level_name(level),
            level_name(got)
        ));
    }
    reset_thread_qos().map_err(to_err)?;
    match get_thread_qos() {
        Ok(level) => out.push_str(&format!("after reset: {}", level_name(level))),
        Err(e) => out.push_str(&format!("after reset: {e:?} (no qos is ok)")),
    }
    Ok(out)
}
