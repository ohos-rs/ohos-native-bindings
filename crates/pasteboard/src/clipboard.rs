//! System clipboard operations confined to the initiating Node-API callback.

use napi_ohos::{module::Module, Env, Result};

/// Stateless access to the API-12 system clipboard through Node-API.
///
/// Call each operation with the current UI callback's environment. Module,
/// clipboard and data handles remain local to that call; only owned text or
/// scalar results leave it. No environment or platform object is retained.
pub struct Clipboard;

impl Clipboard {
    /// Queries whether clipboard data exists without reading its contents.
    pub fn has_data(env: &Env) -> Result<bool> {
        let module = env.load("@ohos.pasteboard")?;
        let system: Module = module.call_without_args("getSystemPasteboard")?;
        system.call_without_args("hasDataSync")
    }

    /// Queries the API-11 text MIME type without reading clipboard contents.
    pub fn has_text(env: &Env) -> Result<bool> {
        let module = env.load("@ohos.pasteboard")?;
        let system: Module = module.call_without_args("getSystemPasteboard")?;
        system.call("hasDataType", "text/plain".to_owned())
    }

    /// Copies the clipboard's primary plain text into an owned Rust string.
    pub fn text(env: &Env) -> Result<String> {
        let module = env.load("@ohos.pasteboard")?;
        let system: Module = module.call_without_args("getSystemPasteboard")?;
        let data: Module = system.call_without_args("getDataSync")?;
        data.call_without_args("getPrimaryText")
    }

    /// Replaces the clipboard with plain text copied during this call.
    pub fn set_text(env: &Env, text: &str) -> Result<()> {
        let module = env.load("@ohos.pasteboard")?;
        let system: Module = module.call_without_args("getSystemPasteboard")?;
        let data: Module = module.call("createPlainTextData", text.to_owned())?;
        system.call("setDataSync", data)
    }
}
