use std::sync::atomic::{AtomicU32, Ordering};

use hms_opengtx_binding::{
    ConfigDescription, EngineType, FrameRenderInfo, GameSceneInfo, GameType, LtpoMode, NetworkInfo,
    NetworkLatency, OpenGtxContext, PictureQualityMaxLevel, ResolutionValue, SceneId, Vector3,
};
use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};

fn to_err(e: hms_opengtx_binding::OpenGtxError) -> Error {
    Error::from_reason(e.to_string())
}

static TEMP_EVENTS: AtomicU32 = AtomicU32::new(0);

fn demo_config() -> ConfigDescription {
    ConfigDescription {
        mode: LtpoMode::AdaptiveMode,
        target_fps: 60,
        package_name: "com.richerfu.ohos_example".to_string(),
        app_version: "1.0.0".to_string(),
        engine_type: EngineType::OTHERS,
        engine_version: "0.0.0".to_string(),
        game_type: GameType::OTHERS,
        picture_quality_max_level: PictureQualityMaxLevel::Hd,
        resolution_max_value: ResolutionValue {
            width: 1080,
            height: 1920,
        },
        game_main_thread_id: 0,
        game_render_thread_id: 0,
        game_key_thread_ids: [0; 5],
        vulkan_support: true,
    }
}

#[napi]
pub fn smoke() -> Result<String> {
    TEMP_EVENTS.store(0, Ordering::SeqCst);
    OpenGtxContext::set_temp_callback(|_| {
        TEMP_EVENTS.fetch_add(1, Ordering::SeqCst);
    });
    let mut ctx = OpenGtxContext::with_temp_callback(|_| {
        TEMP_EVENTS.fetch_add(1, Ordering::SeqCst);
    })
    .map_err(to_err)?;
    ctx.set_configuration(&demo_config()).map_err(to_err)?;
    ctx.activate().map_err(to_err)?;
    ctx.dispatch_frame_render_info(FrameRenderInfo {
        main_camera_position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        main_camera_rotate: Vector3::default(),
    })
    .map_err(to_err)?;
    ctx.dispatch_game_scene_info(&GameSceneInfo {
        scene_id: SceneId::Playing,
        description: "demo".to_string(),
        recommend_fps: 60,
        min_fps: 30,
        max_fps: 120,
        resolution_cur_value: ResolutionValue {
            width: 1080,
            height: 1920,
        },
    })
    .map_err(to_err)?;
    ctx.dispatch_network_info(&NetworkInfo {
        network_latency: NetworkLatency {
            total: 20,
            up: 10,
            down: 10,
        },
        network_server_ip: "127.0.0.1".to_string(),
    })
    .map_err(to_err)?;
    ctx.deactivate().map_err(to_err)?;
    OpenGtxContext::clear_temp_callback();
    Ok(format!(
        "context ok temp_events={}",
        TEMP_EVENTS.load(Ordering::SeqCst)
    ))
}

#[napi]
pub fn create_only() -> Result<String> {
    let ctx = OpenGtxContext::new().map_err(to_err)?;
    Ok(format!("raw={:?}", ctx.raw()))
}
