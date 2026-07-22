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
mod content_embed;
mod crypto;
mod data_protection;
mod device_certificate;
mod distributed_hardware;
mod ffrt;
mod file;
mod game_controller;
mod hicollie;
mod hid;
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
mod mindspore;
mod multimodalinput;
mod native_buffer;
mod native_display_soloist;
mod native_effect;
mod native_fence;
mod native_image;
mod native_window;
mod net;
mod nnrt;
mod notification;
mod ohaudio;
mod ohaudiosuite;
mod ohmidi;
mod purgeable_memory;
mod qos;
mod raw;
mod sensors;
mod teekit;
mod telephony;
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
pub use content_embed::*;
pub use crypto::*;
pub use data_protection::*;
pub use device_certificate::*;
pub use distributed_hardware::*;
pub use ffrt::*;
pub use file::*;
pub use game_controller::*;
pub use hicollie::*;
pub use hid::*;
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
pub use mindspore::*;
pub use multimodalinput::*;
pub use native_buffer::*;
pub use native_display_soloist::*;
pub use native_effect::*;
pub use native_fence::*;
pub use native_image::*;
pub use native_window::*;
pub use net::*;
pub use nnrt::*;
pub use notification::*;
pub use ohaudio::*;
pub use ohaudiosuite::*;
pub use ohmidi::*;
pub use purgeable_memory::*;
pub use qos::*;
pub use raw::*;
pub use sensors::*;
pub use teekit::*;
pub use telephony::*;
pub use transient_task::*;
pub use usb::*;
pub use vsync::*;
