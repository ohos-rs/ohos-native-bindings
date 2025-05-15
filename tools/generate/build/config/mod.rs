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
mod ark_data;
mod arkui;
mod asset;
mod basic;
mod bundle;
mod hilog;
mod image;
mod init;
mod input;
mod input_method;
mod native_buffer;
mod native_display_soloist;
mod native_window;
mod raw;
mod vsync;
mod ark_web;

pub use ability::*;
pub use ark_data::*;
pub use arkui::*;
pub use asset::*;
pub use basic::*;
pub use bundle::*;
pub use hilog::*;
pub use image::*;
pub use init::*;
pub use input::*;
pub use input_method::*;
pub use native_buffer::*;
pub use native_display_soloist::*;
pub use native_window::*;
pub use raw::*;
pub use vsync::*;
pub use ark_web::*;
