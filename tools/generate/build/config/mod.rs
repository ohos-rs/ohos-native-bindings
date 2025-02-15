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
mod input;
mod input_method;
mod native_buffer;
mod native_display_soloist;
mod native_window;
mod raw;
mod vsync;

pub use ability::*;
pub use arkui::*;
pub use asset::*;
pub use bundle::*;
pub use hilog::*;
pub use init::*;
pub use input::*;
pub use input_method::*;
pub use native_buffer::*;
pub use native_display_soloist::*;
pub use native_window::*;
pub use raw::*;
pub use vsync::*;
