use std::{ffi::CString, os::raw::c_char};

use hms_opengtx_sys::*;
use ohos_enum_derive::EnumFrom;

use crate::{OpenGtxError, OpenGtxResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_LTPO_Mode, "OpenGTX_LTPO_Mode_")]
pub enum LtpoMode {
    SceneMode,
    TouchMode,
    AdaptiveMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_EngineType, "OpenGTX_EngineType_")]
pub enum EngineType {
    Unity,
    Unreal,
    Messiah,
    Cocos,
    OthersEngine,
}

impl EngineType {
    pub const OTHERS: Self = Self::OthersEngine;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_PictureQualityMaxLevel, "OpenGTX_PictureQualityMaxLevel_")]
pub enum PictureQualityMaxLevel {
    Sd,
    Hd,
    Fhd,
    Qhd,
    Uhd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_GameType, "OpenGTX_GameType_")]
pub enum GameType {
    Moba,
    Rpg,
    Fps,
    Rac,
    OthersType,
}

impl GameType {
    pub const OTHERS: Self = Self::OthersType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_TempLevel, "OpenGTX_TempLevel_")]
pub enum TempLevel {
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL1")]
    TempLevel1,
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL2")]
    TempLevel2,
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL3")]
    TempLevel3,
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL4")]
    TempLevel4,
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL5")]
    TempLevel5,
    #[alias("OpenGTX_TempLevel_TEMP_LEVEL6")]
    TempLevel6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_SceneID, "OpenGTX_SceneID_")]
pub enum SceneId {
    Login,
    GameInterface,
    Loading,
    Playing,
    Spectator,
    Death,
    HeavyLoad,
    OthersScene,
}

impl SceneId {
    pub const OTHERS: Self = Self::OthersScene;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResolutionValue {
    pub height: i32,
    pub width: i32,
}

impl From<ResolutionValue> for OpenGTX_ResolutionValue {
    fn from(value: ResolutionValue) -> Self {
        Self {
            height: value.height,
            width: value.width,
        }
    }
}

impl From<OpenGTX_ResolutionValue> for ResolutionValue {
    fn from(value: OpenGTX_ResolutionValue) -> Self {
        Self {
            height: value.height,
            width: value.width,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector3> for OpenGTX_Vector3 {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<OpenGTX_Vector3> for Vector3 {
    fn from(value: OpenGTX_Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct FrameRenderInfo {
    pub main_camera_position: Vector3,
    pub main_camera_rotate: Vector3,
}

impl From<FrameRenderInfo> for OpenGTX_FrameRenderInfo {
    fn from(value: FrameRenderInfo) -> Self {
        Self {
            mainCameraPosition: value.main_camera_position.into(),
            mainCameraRotate: value.main_camera_rotate.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigDescription {
    pub mode: LtpoMode,
    pub target_fps: i32,
    pub package_name: String,
    pub app_version: String,
    pub engine_type: EngineType,
    pub engine_version: String,
    pub game_type: GameType,
    pub picture_quality_max_level: PictureQualityMaxLevel,
    pub resolution_max_value: ResolutionValue,
    pub game_main_thread_id: i32,
    pub game_render_thread_id: i32,
    pub game_key_thread_ids: [i32; 5],
    pub vulkan_support: bool,
}

impl Default for ConfigDescription {
    fn default() -> Self {
        Self {
            mode: LtpoMode::AdaptiveMode,
            target_fps: 60,
            package_name: String::new(),
            app_version: String::new(),
            engine_type: EngineType::OTHERS,
            engine_version: String::new(),
            game_type: GameType::OTHERS,
            picture_quality_max_level: PictureQualityMaxLevel::Fhd,
            resolution_max_value: ResolutionValue::default(),
            game_main_thread_id: 0,
            game_render_thread_id: 0,
            game_key_thread_ids: [0; 5],
            vulkan_support: false,
        }
    }
}

impl ConfigDescription {
    pub(crate) fn to_raw(&self) -> OpenGtxResult<RawConfigDescription> {
        let package_name =
            CString::new(self.package_name.as_str()).map_err(|_| OpenGtxError::InvalidString)?;
        let app_version =
            CString::new(self.app_version.as_str()).map_err(|_| OpenGtxError::InvalidString)?;
        let engine_version =
            CString::new(self.engine_version.as_str()).map_err(|_| OpenGtxError::InvalidString)?;

        Ok(RawConfigDescription {
            raw: OpenGTX_ConfigDescription {
                mode: self.mode.into(),
                targetFPS: self.target_fps,
                packageName: package_name.as_ptr() as *mut c_char,
                appVersion: app_version.as_ptr() as *mut c_char,
                engineType: self.engine_type.into(),
                engineVersion: engine_version.as_ptr() as *mut c_char,
                gameType: self.game_type.into(),
                pictureQualityMaxLevel: self.picture_quality_max_level.into(),
                resolutionMaxValue: self.resolution_max_value.into(),
                gameMainThreadId: self.game_main_thread_id,
                gameRenderThreadId: self.game_render_thread_id,
                gameKeyThreadIds: self.game_key_thread_ids,
                vulkanSupport: self.vulkan_support,
            },
            _package_name: package_name,
            _app_version: app_version,
            _engine_version: engine_version,
        })
    }
}

pub(crate) struct RawConfigDescription {
    raw: OpenGTX_ConfigDescription,
    _package_name: CString,
    _app_version: CString,
    _engine_version: CString,
}

impl RawConfigDescription {
    pub(crate) fn as_ptr(&self) -> *const OpenGTX_ConfigDescription {
        &self.raw
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSceneInfo {
    pub scene_id: SceneId,
    pub description: String,
    pub recommend_fps: i32,
    pub min_fps: i32,
    pub max_fps: i32,
    pub resolution_cur_value: ResolutionValue,
}

impl GameSceneInfo {
    pub(crate) fn to_raw(&self) -> OpenGtxResult<RawGameSceneInfo> {
        let description =
            CString::new(self.description.as_str()).map_err(|_| OpenGtxError::InvalidString)?;

        Ok(RawGameSceneInfo {
            raw: OpenGTX_GameSceneInfo {
                sceneID: self.scene_id.into(),
                description: description.as_ptr() as *mut c_char,
                recommendFPS: self.recommend_fps,
                minFPS: self.min_fps,
                maxFPS: self.max_fps,
                resolutionCurValue: self.resolution_cur_value.into(),
            },
            _description: description,
        })
    }
}

pub(crate) struct RawGameSceneInfo {
    raw: OpenGTX_GameSceneInfo,
    _description: CString,
}

impl RawGameSceneInfo {
    pub(crate) fn as_ptr(&self) -> *const OpenGTX_GameSceneInfo {
        &self.raw
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NetworkLatency {
    pub total: i32,
    pub up: i32,
    pub down: i32,
}

impl From<NetworkLatency> for OpenGTX_NetworkLatency {
    fn from(value: NetworkLatency) -> Self {
        Self {
            total: value.total,
            up: value.up,
            down: value.down,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInfo {
    pub network_latency: NetworkLatency,
    pub network_server_ip: String,
}

impl NetworkInfo {
    pub(crate) fn to_raw(&self) -> OpenGtxResult<RawNetworkInfo> {
        let network_server_ip = CString::new(self.network_server_ip.as_str())
            .map_err(|_| OpenGtxError::InvalidString)?;

        Ok(RawNetworkInfo {
            raw: OpenGTX_NetworkInfo {
                networkLatency: self.network_latency.into(),
                networkServerIP: network_server_ip.as_ptr() as *mut c_char,
            },
            _network_server_ip: network_server_ip,
        })
    }
}

pub(crate) struct RawNetworkInfo {
    raw: OpenGTX_NetworkInfo,
    _network_server_ip: CString,
}

impl RawNetworkInfo {
    pub(crate) fn as_ptr(&self) -> *const OpenGTX_NetworkInfo {
        &self.raw
    }
}
