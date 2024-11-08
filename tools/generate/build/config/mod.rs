pub(crate) struct SysConfig {
    /// crate name
    pub name: &'static str,
    /// include headers
    pub headers: Vec<&'static str>,
    pub white_list: Vec<&'static str>,
    pub block_list: Vec<&'static str>,

    pub extra: &'static str,
}

mod ability;
mod arkui;
mod asset;
mod bundle;
mod hilog;
mod init;
mod raw;

pub use ability::*;
pub use arkui::*;
pub use asset::*;
pub use bundle::*;
pub use hilog::*;
pub use init::*;
pub use raw::*;
