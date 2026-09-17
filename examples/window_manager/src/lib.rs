use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result, Status};
use ohos_image_native_binding::{
    PixelFormat, PixelMap, PixelMapAlphaType, PixelMapInitializationOptions,
};
use ohos_window_manager_binding::{
    AvoidAreaType, DensityInfoRef, FrameMetrics, Input_KeyEvent, Input_MouseEvent,
    Input_TouchEvent, RawDensityInfo, RawFrameMetrics, RawPixelMap, Window, WindowManager,
    WindowSnapshotConfig,
};

#[link(name = "ohinput")]
unsafe extern "C" {
    fn OH_Input_CreateTouchEvent() -> *mut Input_TouchEvent;
    fn OH_Input_DestroyTouchEvent(event: *mut *mut Input_TouchEvent);
    fn OH_Input_SetTouchEventAction(event: *mut Input_TouchEvent, action: i32);
    fn OH_Input_SetTouchEventFingerId(event: *mut Input_TouchEvent, id: i32);
    fn OH_Input_SetTouchEventDisplayX(event: *mut Input_TouchEvent, display_x: i32);
    fn OH_Input_SetTouchEventDisplayY(event: *mut Input_TouchEvent, display_y: i32);
    fn OH_Input_SetTouchEventDisplayId(event: *mut Input_TouchEvent, display_id: i32);
    fn OH_Input_SetTouchEventActionTime(event: *mut Input_TouchEvent, action_time: i64);
    fn OH_Input_SetTouchEventWindowId(event: *mut Input_TouchEvent, window_id: i32);
    fn OH_Input_SetTouchEventGlobalX(event: *mut Input_TouchEvent, global_x: i32);
    fn OH_Input_SetTouchEventGlobalY(event: *mut Input_TouchEvent, global_y: i32);
    fn OH_Input_SetTouchEventWindowX(event: *mut Input_TouchEvent, window_x: i32);
    fn OH_Input_SetTouchEventWindowY(event: *mut Input_TouchEvent, window_y: i32);
    fn OH_Input_SetTouchEventDownTime(event: *mut Input_TouchEvent, down_time: i64);
}

static SNAPSHOT_CALLBACKS: AtomicUsize = AtomicUsize::new(0);
static SNAPSHOT_RELEASES: AtomicUsize = AtomicUsize::new(0);
static FRAME_CALLBACKS: AtomicUsize = AtomicUsize::new(0);
static FRAME_METRICS_OK_MASK: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn key_event_filter(_event: *mut Input_KeyEvent) -> bool {
    false
}

unsafe extern "C" fn mouse_event_filter(_event: *mut Input_MouseEvent) -> bool {
    false
}

unsafe extern "C" fn touch_event_filter(_event: *mut Input_TouchEvent) -> bool {
    false
}

unsafe extern "C" fn density_info_callback(_window_id: i32, info: *const RawDensityInfo) {
    if let Some(info) = unsafe { DensityInfoRef::from_raw(info) } {
        let _ = info.default_density();
        let _ = info.system_density();
        let _ = info.custom_density();
    }
}

unsafe extern "C" fn frame_metrics_callback(_window_id: i32, metrics: *const RawFrameMetrics) {
    FRAME_CALLBACKS.fetch_add(1, Ordering::Relaxed);
    let Some(metrics) = (unsafe { FrameMetrics::from_raw(metrics) }) else {
        return;
    };
    let mut mask = 0;
    if metrics.is_first_draw_frame().is_ok() {
        mask |= 1;
    }
    if metrics.input_handling_duration().is_ok() {
        mask |= 1 << 1;
    }
    if metrics.layout_measure_duration().is_ok() {
        mask |= 1 << 2;
    }
    if metrics.vsync_timestamp().is_ok() {
        mask |= 1 << 3;
    }
    FRAME_METRICS_OK_MASK.fetch_or(mask, Ordering::Relaxed);
}

unsafe extern "C" fn main_window_snapshot_callback(
    snapshots: *mut *const RawPixelMap,
    _snapshot_count: usize,
) {
    SNAPSHOT_CALLBACKS.fetch_add(1, Ordering::Relaxed);
    if !snapshots.is_null() {
        unsafe { WindowManager::release_main_window_snapshots(snapshots.cast()) };
        SNAPSHOT_RELEASES.fetch_add(1, Ordering::Relaxed);
    }
}

struct TouchEvent {
    raw: NonNull<Input_TouchEvent>,
}

impl TouchEvent {
    fn new() -> Option<Self> {
        NonNull::new(unsafe { OH_Input_CreateTouchEvent() }).map(|raw| Self { raw })
    }
}

impl Drop for TouchEvent {
    fn drop(&mut self) {
        let mut raw = self.raw.as_ptr();
        unsafe { OH_Input_DestroyTouchEvent(&mut raw) };
    }
}

fn time_part_to_i64<T: Into<i64>>(value: T) -> i64 {
    value.into()
}

fn push_status<T>(
    report: &mut Vec<String>,
    name: &str,
    result: ohos_window_manager_binding::Result<T>,
) {
    match result {
        Ok(_) => report.push(format!("{name}=ok")),
        Err(error) => report.push(format!(
            "{name}=err:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string())
        )),
    }
}

fn make_pixel_map() -> std::result::Result<PixelMap, String> {
    let mut options = PixelMapInitializationOptions::new().map_err(|error| error.to_string())?;
    options.set_width(8).map_err(|error| error.to_string())?;
    options.set_height(8).map_err(|error| error.to_string())?;
    options
        .set_pixel_format(PixelFormat::Rgba8888)
        .map_err(|error| error.to_string())?;
    options
        .set_alpha_type(PixelMapAlphaType::Opaque)
        .map_err(|error| error.to_string())?;
    let mut data = vec![0_u8; 8 * 8 * 4];
    PixelMap::create(&mut data, &mut options).map_err(|error| error.to_string())
}

#[napi]
pub fn layout_smoke(display_id: i64) -> String {
    match WindowManager::visible_window_layouts(display_id) {
        Ok(layouts) => format!("layout_count={}", layouts.len()),
        Err(error) => format!(
            "layout_error={}:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string()),
            error
        ),
    }
}

#[napi]
pub fn smoke(window_id: i32) -> Result<String> {
    let window = Window::from_id(window_id);
    let properties = window
        .properties()
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
    let shown = window
        .is_shown()
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;
    let avoid_area = window
        .avoid_area(AvoidAreaType::System)
        .map_err(|error| Error::new(Status::GenericFailure, error.to_string()))?;

    Ok(format!(
        "id={};display_id={};size={}x{};shown={shown};avoid_top={}x{}",
        properties.id,
        properties.display_id,
        properties.window_rect.width,
        properties.window_rect.height,
        avoid_area.top.width,
        avoid_area.top.height,
    ))
}

/// Executes every WindowManager API available on the API-26 QEMU image.
///
/// Each operation is recorded independently so one platform failure does not
/// prevent the remaining native entry points from being exercised.
#[napi]
pub fn api_report(window_id: i32) -> String {
    build_api_report(window_id, false)
}

/// Executes the API matrix for a window whose ArkUI content is loaded.
#[napi]
pub fn ui_content_report(window_id: i32) -> String {
    build_api_report(window_id, true)
}

fn build_api_report(window_id: i32, has_ui_content: bool) -> String {
    let window = Window::from_id(window_id);
    let mut report = Vec::new();
    let properties = window.properties();
    push_status(&mut report, "properties", properties.clone());

    push_status(
        &mut report,
        "avoid_area",
        window.avoid_area(AvoidAreaType::System),
    );
    push_status(&mut report, "is_shown", window.is_shown());
    push_status(&mut report, "show", window.show());
    push_status(
        &mut report,
        "status_bar_enabled",
        window.set_status_bar_enabled(true, false),
    );
    push_status(
        &mut report,
        "status_bar_color",
        window.set_status_bar_color(0xff00_0000),
    );
    push_status(
        &mut report,
        "navigation_bar_enabled",
        window.set_navigation_bar_enabled(true, false),
    );

    let (
        touchable,
        focusable,
        brightness,
        keep_screen_on,
        privacy_mode,
        display_id,
        global_x,
        global_y,
    ) = properties
        .map(|value| {
            (
                value.touchable,
                value.focusable,
                value.brightness,
                value.keep_screen_on,
                value.privacy_mode,
                i64::from(value.display_id),
                value.window_rect.x.saturating_add(1),
                value.window_rect.y.saturating_add(1),
            )
        })
        .unwrap_or((true, true, -1.0, false, false, 0, 1, 1));
    push_status(&mut report, "touchable", window.set_touchable(touchable));
    push_status(&mut report, "focusable", window.set_focusable(focusable));
    if has_ui_content {
        push_status(
            &mut report,
            "background_color",
            window.set_background_color("#00000000"),
        );
    } else {
        // On the API-26 QEMU image this native function aborts inside
        // libnative_window_manager.so when the test window has no UIContent.
        // Keep it out of the aggregate probe so the remaining APIs still run.
        report.push("background_color=known_qemu_abort:no_uicontent".to_owned());
    }
    push_status(&mut report, "brightness", window.set_brightness(brightness));
    push_status(
        &mut report,
        "keep_screen_on",
        window.set_keep_screen_on(keep_screen_on),
    );
    push_status(
        &mut report,
        "privacy_mode",
        window.set_privacy_mode(privacy_mode),
    );

    match make_pixel_map() {
        Ok(pixel_map) => push_status(&mut report, "snapshot", window.snapshot(pixel_map.handle())),
        Err(error) => report.push(format!("snapshot=setup_err:{error}")),
    }

    push_status(
        &mut report,
        "register_key_filter",
        window.register_key_event_filter(key_event_filter),
    );
    match window.key_event_filter() {
        Ok(Some(_)) => report.push("get_key_filter=some".to_owned()),
        Ok(None) => report.push("get_key_filter=none".to_owned()),
        Err(error) => report.push(format!(
            "get_key_filter=err:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string())
        )),
    }
    push_status(
        &mut report,
        "unregister_key_filter",
        window.unregister_key_event_filter(),
    );
    push_status(
        &mut report,
        "register_mouse_filter",
        window.register_mouse_event_filter(mouse_event_filter),
    );
    match window.mouse_event_filter() {
        Ok(Some(_)) => report.push("get_mouse_filter=some".to_owned()),
        Ok(None) => report.push("get_mouse_filter=none".to_owned()),
        Err(error) => report.push(format!(
            "get_mouse_filter=err:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string())
        )),
    }
    push_status(
        &mut report,
        "unregister_mouse_filter",
        window.unregister_mouse_event_filter(),
    );
    push_status(
        &mut report,
        "register_touch_filter",
        window.register_touch_event_filter(touch_event_filter),
    );
    match window.touch_event_filter() {
        Ok(Some(_)) => report.push("get_touch_filter=some".to_owned()),
        Ok(None) => report.push("get_touch_filter=none".to_owned()),
        Err(error) => report.push(format!(
            "get_touch_filter=err:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string())
        )),
    }
    push_status(
        &mut report,
        "unregister_touch_filter",
        window.unregister_touch_event_filter(),
    );
    push_status(
        &mut report,
        "visible_layouts",
        WindowManager::visible_window_layouts(display_id),
    );

    match TouchEvent::new() {
        Some(event) => {
            let mut now = libc::timespec::default();
            let now_micros = unsafe {
                libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now);
                let now_micros = time_part_to_i64(now.tv_sec)
                    .saturating_mul(1_000_000)
                    .saturating_add(time_part_to_i64(now.tv_nsec) / 1_000);
                OH_Input_SetTouchEventAction(event.raw.as_ptr(), 1);
                OH_Input_SetTouchEventFingerId(event.raw.as_ptr(), 0);
                OH_Input_SetTouchEventDisplayX(event.raw.as_ptr(), 1);
                OH_Input_SetTouchEventDisplayY(event.raw.as_ptr(), 1);
                OH_Input_SetTouchEventDisplayId(
                    event.raw.as_ptr(),
                    i32::try_from(display_id).unwrap_or(0),
                );
                OH_Input_SetTouchEventWindowId(event.raw.as_ptr(), window_id);
                OH_Input_SetTouchEventGlobalX(event.raw.as_ptr(), global_x);
                OH_Input_SetTouchEventGlobalY(event.raw.as_ptr(), global_y);
                OH_Input_SetTouchEventWindowX(event.raw.as_ptr(), 1);
                OH_Input_SetTouchEventWindowY(event.raw.as_ptr(), 1);
                OH_Input_SetTouchEventActionTime(event.raw.as_ptr(), now_micros);
                OH_Input_SetTouchEventDownTime(event.raw.as_ptr(), now_micros);
                now_micros
            };
            push_status(&mut report, "inject_touch_event", unsafe {
                window.inject_touch_event(event.raw, 1, 1)
            });
            unsafe {
                OH_Input_SetTouchEventAction(event.raw.as_ptr(), 3);
                OH_Input_SetTouchEventActionTime(event.raw.as_ptr(), now_micros.saturating_add(1));
            }
            push_status(&mut report, "inject_touch_event_up", unsafe {
                window.inject_touch_event(event.raw, 1, 1)
            });
        }
        None => report.push("inject_touch_event=setup_err:null".to_owned()),
    }

    push_status(&mut report, "main_windows", WindowManager::main_windows());
    let mut window_ids = [window_id];
    push_status(
        &mut report,
        "main_window_snapshots",
        WindowManager::request_main_window_snapshots(
            &mut window_ids,
            WindowSnapshotConfig { use_cache: false },
            main_window_snapshot_callback,
        ),
    );
    push_status(&mut report, "lock_cursor", window.lock_cursor(false));
    push_status(&mut report, "unlock_cursor", window.unlock_cursor());

    match window.density_info() {
        Ok(Some(info)) => {
            report.push("density_info=some".to_owned());
            push_status(&mut report, "default_density", info.default_density());
            push_status(&mut report, "system_density", info.system_density());
            push_status(&mut report, "custom_density", info.custom_density());
        }
        Ok(None) => report.push("density_info=none".to_owned()),
        Err(error) => report.push(format!(
            "density_info=err:{}",
            error
                .code()
                .map_or_else(|| "none".to_owned(), |code| code.to_string())
        )),
    }
    push_status(
        &mut report,
        "register_density_callback",
        window.register_density_info_change_callback(density_info_callback),
    );
    push_status(
        &mut report,
        "unregister_density_callback",
        window.unregister_density_info_change_callback(density_info_callback),
    );

    FRAME_CALLBACKS.store(0, Ordering::Relaxed);
    FRAME_METRICS_OK_MASK.store(0, Ordering::Relaxed);
    push_status(
        &mut report,
        "register_frame_callback",
        window.register_frame_metrics_measured_callback(frame_metrics_callback),
    );
    if has_ui_content {
        push_status(
            &mut report,
            "frame_redraw",
            window.set_background_color("#ff000000"),
        );
    }

    report.join(";")
}

/// Reports asynchronous callbacks and unregisters the frame callback.
#[napi]
pub fn callback_report(window_id: i32) -> String {
    let mut report = vec![
        format!(
            "snapshot_callbacks={}",
            SNAPSHOT_CALLBACKS.load(Ordering::Relaxed)
        ),
        format!(
            "snapshot_releases={}",
            SNAPSHOT_RELEASES.load(Ordering::Relaxed)
        ),
        format!(
            "frame_callbacks={}",
            FRAME_CALLBACKS.load(Ordering::Relaxed)
        ),
        format!(
            "frame_metrics_ok_mask={}",
            FRAME_METRICS_OK_MASK.load(Ordering::Relaxed)
        ),
    ];
    push_status(
        &mut report,
        "unregister_frame_callback",
        Window::from_id(window_id)
            .unregister_frame_metrics_measured_callback(frame_metrics_callback),
    );
    report.join(";")
}
