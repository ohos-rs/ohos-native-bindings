pub(crate) struct SysConfig {
    /// crate name
    pub name: &'static str,
    /// include headers
    pub headers: Vec<&'static str>,
    pub white_list: Vec<&'static str>,
    pub block_list: Vec<&'static str>,

    // dynamic library name
    pub dynamic_library: Vec<&'static str>,

    pub extra: &'static str,
}

mod ability;
mod ability_base;
mod ark_data;
mod ark_web;
mod arkui;
mod asset;
mod background_process_manager;
mod basic;
mod basic_services;
mod bundle;
mod camera;
mod color_space;
mod connectivity;
mod crypto;
mod data_protection;
mod device_certificate;
mod file;
mod hicollie;
mod hilog;
mod hiviewdfx;
mod hms;
mod huks;
mod image;
mod init;
mod input_method;
mod ipc;
mod jsvm;
mod location;
mod native_buffer;
mod native_display_soloist;
mod native_effect;
mod native_fence;
mod native_window;
mod net;
mod notification;
mod ohaudio;
mod purgeable_memory;
mod qos;
mod raw;
mod sensors;
mod teekit;
mod transient_task;
mod usb;
mod vsync;

pub use ability::*;
pub use ability_base::*;
pub use ark_data::*;
pub use ark_web::*;
pub use arkui::*;
pub use asset::*;
pub use background_process_manager::*;
pub use basic::*;
pub use basic_services::*;
pub use bundle::*;
pub use camera::*;
pub use color_space::*;
pub use connectivity::*;
pub use crypto::*;
pub use data_protection::*;
pub use device_certificate::*;
pub use file::*;
pub use hicollie::*;
pub use hilog::*;
pub use hiviewdfx::*;
pub use hms::*;
pub use huks::*;
pub use image::*;
pub use init::*;
pub use input_method::*;
pub use ipc::*;
pub use jsvm::*;
pub use location::*;
pub use native_buffer::*;
pub use native_display_soloist::*;
pub use native_effect::*;
pub use native_fence::*;
pub use native_window::*;
pub use net::*;
pub use notification::*;
pub use ohaudio::*;
pub use purgeable_memory::*;
pub use qos::*;
pub use raw::*;
pub use sensors::*;
pub use teekit::*;
pub use transient_task::*;
pub use usb::*;
pub use vsync::*;
