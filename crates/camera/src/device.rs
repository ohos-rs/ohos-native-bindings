use std::ffi::CString;
use std::sync::mpsc::Sender;
#[cfg(feature = "api-20")]
use std::time::{Duration, Instant};

use ohos_camera_sys as sys;
use ohos_image_native_binding::ImageReceiver;
#[cfg(feature = "api-20")]
use ohos_image_native_binding::ImageReceiverOptions;

use crate::callbacks::{CameraCallbackRouter, CameraInstanceLease};
#[cfg(feature = "api-20")]
use crate::image::CameraImageReader;
use crate::native::{CameraInput, CameraManager, CaptureSession, PhotoOutput, PreviewOutput};
use crate::profiles::CameraProfileCatalog;
use crate::{
    CameraCapabilities, CameraConfiguration, CameraError, CameraEvent, CameraPosition,
    CameraResult, CameraSessionInfo, CameraSize,
};

/// Fully configured owner of one CameraKit device and all of its resources.
pub(crate) struct CameraDevice {
    // Drop order is intentional: stop/unregister the session and outputs before
    // releasing the receiver, input, and manager they depend on.
    session: CaptureSession,
    photo_output: Option<PhotoOutput>,
    preview_output: PreviewOutput,
    _frame_output: Option<PreviewOutput>,
    _frame_receiver: Option<ImageReceiver>,
    _input: CameraInput,
    manager: CameraManager,
}

impl CameraDevice {
    pub(crate) fn open(
        configuration: CameraConfiguration,
        events: Sender<CameraEvent>,
        instance: &mut CameraInstanceLease,
    ) -> CameraResult<(Self, CameraSessionInfo, CameraCapabilities)> {
        #[cfg(not(feature = "api-20"))]
        if configuration.frame_output.is_some() {
            return Err(CameraError::unsupported(
                "CameraSession::open",
                "analysis frame output requires the api-20 feature",
            ));
        }

        let manager = CameraManager::new()?;
        let cameras = manager.supported_cameras()?;
        let camera = cameras
            .devices()
            .iter()
            .find(|camera| {
                camera.cameraPosition == sys::Camera_Position::from(configuration.position)
            })
            .ok_or_else(|| CameraError::no_camera(configuration.position))?;
        let actual_position =
            CameraPosition::try_from_raw(camera.cameraPosition).unwrap_or(configuration.position);
        let capability = manager.output_capability(camera)?;
        let profiles = CameraProfileCatalog::from_capability(&capability);
        let preview_profile =
            profiles.select_preview(configuration.surface.size, configuration.preview_size)?;
        let photo_profile = configuration
            .enable_photo_output
            .then(|| profiles.select_photo(configuration.photo_size))
            .transpose()?
            .flatten();
        #[cfg(feature = "api-20")]
        let frame_profile = configuration
            .frame_output
            .map(|output| profiles.select_frame(output.size))
            .transpose()?;
        #[cfg(not(feature = "api-20"))]
        let frame_profile: Option<sys::Camera_Profile> = None;

        let info = CameraSessionInfo {
            position: actual_position,
            preview_size: CameraSize::new(preview_profile.size.width, preview_profile.size.height),
            photo_size: photo_profile
                .map(|profile| CameraSize::new(profile.size.width, profile.size.height)),
            frame_size: frame_profile
                .map(|profile| CameraSize::new(profile.size.width, profile.size.height)),
        };
        let capabilities = profiles.capabilities(actual_position);

        let mut input = manager.create_input(camera)?;
        let mut session = manager.create_capture_session()?;
        let preview_surface = Self::surface_id(configuration.surface.id, "surface id")?;
        let preview_output = manager.create_preview_output(
            &preview_profile,
            &preview_surface,
            "OH_CameraManager_CreatePreviewOutput",
        )?;
        let mut photo_output = photo_profile
            .as_ref()
            .map(|profile| manager.create_photo_output(profile))
            .transpose()?;

        #[cfg(feature = "api-20")]
        let mut frame_receiver = None;
        #[cfg(not(feature = "api-20"))]
        let frame_receiver = None;
        #[cfg(feature = "api-20")]
        let mut frame_output = None;
        #[cfg(not(feature = "api-20"))]
        let frame_output = None;
        #[cfg(feature = "api-20")]
        if let (Some(profile), Some(frame_configuration)) =
            (frame_profile, configuration.frame_output)
        {
            let mut options = ImageReceiverOptions::new().map_err(|error| {
                CameraError::image("OH_ImageReceiverOptions_Create", error.code)
            })?;
            options
                .set_size(ohos_image_native_binding::types::ImageSize {
                    width: profile.size.width,
                    height: profile.size.height,
                })
                .map_err(|error| {
                    CameraError::image("OH_ImageReceiverOptions_SetSize", error.code)
                })?;
            options
                .set_capacity(frame_configuration.capacity.clamp(1, 8) as i32)
                .map_err(|error| {
                    CameraError::image("OH_ImageReceiverOptions_SetCapacity", error.code)
                })?;
            let mut receiver = ImageReceiver::new(&mut options)
                .map_err(|error| CameraError::image("OH_ImageReceiverNative_Create", error.code))?;
            let frame_size = info.frame_size.ok_or_else(|| {
                CameraError::invalid_state(
                    "CameraSession::open",
                    "selected analysis profile has no frame size",
                )
            })?;
            let frame_interval = Duration::from_secs_f64(
                1.0 / f64::from(frame_configuration.max_frames_per_second.clamp(1, 30)),
            );
            let frame_events = events.clone();
            let mut last_frame_at = None;
            receiver
                .on_image_arrive(move |receiver| {
                    let now = Instant::now();
                    let image = match receiver.read_latest_image() {
                        Ok(image) => image,
                        Err(error) => {
                            let _ = frame_events.send(CameraEvent::Error(CameraError::image(
                                "OH_ImageReceiverNative_ReadLatestImage",
                                error.code,
                            )));
                            return;
                        }
                    };
                    // Always drain the receiver. Only the expensive copy and
                    // decoder handoff are throttled.
                    if last_frame_at
                        .is_some_and(|last: Instant| now.duration_since(last) < frame_interval)
                    {
                        return;
                    }
                    last_frame_at = Some(now);
                    let event = match CameraImageReader::copy_frame(image, frame_size) {
                        Ok(frame) => CameraEvent::Frame(frame),
                        Err(error) => CameraEvent::Error(error),
                    };
                    let _ = frame_events.send(event);
                })
                .map_err(|error| {
                    CameraError::image("OH_ImageReceiverNative_OnImageArrive", error.code)
                })?;
            let frame_surface_id = receiver.receiving_surface_id().map_err(|error| {
                CameraError::image("OH_ImageReceiverNative_GetReceivingSurfaceId", error.code)
            })?;
            let surface = Self::surface_id(frame_surface_id, "analysis surface id")?;
            frame_output = Some(manager.create_preview_output(
                &profile,
                &surface,
                "OH_CameraManager_CreatePreviewOutput(analysis)",
            )?);
            frame_receiver = Some(receiver);
        }

        // Discovery pointers are no longer needed once all profiles and the
        // input have been copied/created. Release them before the live session.
        drop(capability);
        drop(cameras);

        instance.activate(
            events,
            input.raw_key(),
            session.raw_key(),
            photo_output.as_ref().map(PhotoOutput::raw_key),
            info.photo_size,
        )?;
        input.register_error_callback(CameraCallbackRouter::input_error_callback())?;
        session.register_callbacks(
            CameraCallbackRouter::focus_callback(),
            CameraCallbackRouter::session_error_callback(),
        )?;
        if let Some(photo) = photo_output.as_mut() {
            photo.register_callbacks(
                CameraCallbackRouter::photo_error_callback(),
                CameraCallbackRouter::photo_available_callback(),
            )?;
        }
        session.configure(
            &input,
            &preview_output,
            photo_output.as_ref(),
            frame_output.as_ref(),
        )?;
        session.start()?;

        Ok((
            Self {
                session,
                photo_output,
                preview_output,
                _frame_output: frame_output,
                _frame_receiver: frame_receiver,
                _input: input,
                manager,
            },
            info,
            capabilities,
        ))
    }

    pub(crate) fn manager(&self) -> &CameraManager {
        &self.manager
    }

    pub(crate) fn session(&self) -> &CaptureSession {
        &self.session
    }

    pub(crate) fn preview_output(&self) -> &PreviewOutput {
        &self.preview_output
    }

    pub(crate) fn photo_output(&self) -> Option<&PhotoOutput> {
        self.photo_output.as_ref()
    }

    fn surface_id(id: u64, label: &'static str) -> CameraResult<CString> {
        CString::new(id.to_string())
            .map_err(|_| CameraError::surface(label, "surface id contained an interior NUL"))
    }
}
