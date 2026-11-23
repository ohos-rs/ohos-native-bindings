use std::{
    cell::Cell,
    ffi::c_void,
    marker::PhantomData,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::{self, NonNull},
};

use ohos_audio_base_sys::OH_AudioChannelLayout;
use ohos_audio_sys as sys;

use crate::{
    error::{check_stream, AudioError, Result},
    types::{
        AudioDataCallbackResult, AudioEffectMode, DeviceChangeReason, EncodingType, InterruptMode,
        LatencyMode, PrivacyType, SampleFormat, StreamState, StreamUsage,
    },
};

#[cfg(feature = "api-23")]
use crate::types::LatencyType;
#[cfg(feature = "api-19")]
use crate::types::VolumeMode;
#[cfg(feature = "api-20")]
use crate::types::{InterruptForceType, InterruptHint};

type WriteDataFn = Box<dyn FnMut(&mut [u8]) -> AudioDataCallbackResult + Send>;
type DeviceChangeFn = Box<dyn FnMut(DeviceChangeReason) + Send>;
#[cfg(feature = "api-20")]
type InterruptFn = Box<dyn FnMut(InterruptForceType, InterruptHint) + Send>;
#[cfg(feature = "api-20")]
type ErrorFn = Box<dyn FnMut(AudioError) + Send>;

struct WriteDataBox(WriteDataFn);
struct DeviceChangeBox(DeviceChangeFn);
#[cfg(feature = "api-20")]
struct InterruptBox(InterruptFn);
#[cfg(feature = "api-20")]
struct ErrorBox(ErrorFn);

struct RendererCallbacks {
    write_data: Option<Box<WriteDataBox>>,
    device_change: Option<Box<DeviceChangeBox>>,
    #[cfg(feature = "api-20")]
    interrupt: Option<Box<InterruptBox>>,
    #[cfg(feature = "api-20")]
    error: Option<Box<ErrorBox>>,
}

impl RendererCallbacks {
    fn empty() -> Self {
        Self {
            write_data: None,
            device_change: None,
            #[cfg(feature = "api-20")]
            interrupt: None,
            #[cfg(feature = "api-20")]
            error: None,
        }
    }
}

unsafe extern "C" fn on_write_data_trampoline(
    _renderer: *mut sys::OH_AudioRenderer,
    user_data: *mut c_void,
    audio_data: *mut c_void,
    audio_data_size: i32,
) -> sys::OH_AudioData_Callback_Result {
    if user_data.is_null() || audio_data.is_null() || audio_data_size <= 0 {
        return AudioDataCallbackResult::Invalid.into();
    }
    let state = unsafe { &mut *user_data.cast::<WriteDataBox>() };
    let buffer = unsafe {
        std::slice::from_raw_parts_mut(audio_data.cast::<u8>(), audio_data_size as usize)
    };
    match catch_unwind(AssertUnwindSafe(|| (state.0)(buffer))) {
        Ok(res) => res.into(),
        Err(_) => AudioDataCallbackResult::Invalid.into(),
    }
}

unsafe extern "C" fn on_device_change_trampoline(
    _renderer: *mut sys::OH_AudioRenderer,
    user_data: *mut c_void,
    reason: sys::OH_AudioStream_DeviceChangeReason,
) {
    if user_data.is_null() {
        return;
    }
    let state = unsafe { &mut *user_data.cast::<DeviceChangeBox>() };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        (state.0)(DeviceChangeReason::try_from_raw(reason).unwrap_or(DeviceChangeReason::Unknown));
    }));
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_interrupt_trampoline(
    _renderer: *mut sys::OH_AudioRenderer,
    user_data: *mut c_void,
    force_type: sys::OH_AudioInterrupt_ForceType,
    hint: sys::OH_AudioInterrupt_Hint,
) {
    if user_data.is_null() {
        return;
    }
    let state = unsafe { &mut *user_data.cast::<InterruptBox>() };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if let Some(force) = InterruptForceType::try_from_raw(force_type) {
            (state.0)(
                force,
                InterruptHint::try_from_raw(hint).unwrap_or(InterruptHint::None),
            );
        }
    }));
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_error_trampoline(
    _renderer: *mut sys::OH_AudioRenderer,
    user_data: *mut c_void,
    error: sys::OH_AudioStream_Result,
) {
    if user_data.is_null() {
        return;
    }
    let state = unsafe { &mut *user_data.cast::<ErrorBox>() };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        (state.0)(AudioError::InternalError(error));
    }));
}

/// Builder for configuring and generating an [`AudioRenderer`] (`@since 10/12`).
pub struct AudioStreamBuilder {
    raw: NonNull<sys::OH_AudioStreamBuilder>,
    callbacks: RendererCallbacks,
    _not_sync: PhantomData<Cell<()>>,
}

// SAFETY: `OH_AudioStreamBuilder` supports serialized calls across threads.
unsafe impl Send for AudioStreamBuilder {}

impl AudioStreamBuilder {
    /// Creates a new renderer stream builder (`@since 10`).
    pub fn new_renderer() -> Result<Self> {
        let mut raw: *mut sys::OH_AudioStreamBuilder = ptr::null_mut();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_Create(
                &mut raw,
                sys::OH_AudioStream_Type_AUDIOSTREAM_TYPE_RENDERER,
            )
        })?;
        let raw = NonNull::new(raw).ok_or_else(|| {
            AudioError::CommonError("OH_AudioStreamBuilder_Create returned null".into())
        })?;
        Ok(Self {
            raw,
            callbacks: RendererCallbacks::empty(),
            _not_sync: PhantomData,
        })
    }

    /// Sets sampling rate in Hz (for example `48000`) (`@since 10`).
    pub fn sampling_rate(&mut self, rate: i32) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetSamplingRate(self.raw.as_ptr(), rate)
        })?;
        Ok(self)
    }

    /// Sets channel count (for example `1` for mono or `2` for stereo) (`@since 10`).
    pub fn channel_count(&mut self, count: i32) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetChannelCount(self.raw.as_ptr(), count)
        })?;
        Ok(self)
    }

    /// Sets PCM sample format (`@since 10`).
    pub fn sample_format(&mut self, format: SampleFormat) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetSampleFormat(self.raw.as_ptr(), format.into())
        })?;
        Ok(self)
    }

    /// Sets stream encoding type (`@since 10`).
    pub fn encoding_type(&mut self, encoding: EncodingType) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetEncodingType(self.raw.as_ptr(), encoding.into())
        })?;
        Ok(self)
    }

    /// Sets stream latency mode (`@since 10`).
    pub fn latency_mode(&mut self, mode: LatencyMode) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetLatencyMode(self.raw.as_ptr(), mode.into())
        })?;
        Ok(self)
    }

    /// Sets explicit channel layout (`@since 12`).
    pub fn channel_layout(&mut self, layout: OH_AudioChannelLayout) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetChannelLayout(self.raw.as_ptr(), layout)
        })?;
        Ok(self)
    }

    /// Sets renderer usage scenario (`@since 10`).
    pub fn usage(&mut self, usage: StreamUsage) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererInfo(self.raw.as_ptr(), usage.into())
        })?;
        Ok(self)
    }

    /// Sets audio focus interruption mode (`@since 12`).
    pub fn interrupt_mode(&mut self, mode: InterruptMode) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererInterruptMode(self.raw.as_ptr(), mode.into())
        })?;
        Ok(self)
    }

    /// Sets playback capture privacy policy (`@since 12`).
    pub fn privacy(&mut self, privacy: PrivacyType) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererPrivacy(self.raw.as_ptr(), privacy.into())
        })?;
        Ok(self)
    }

    /// Sets preferred frame count per write callback (`@since 11`).
    pub fn frame_size_in_callback(&mut self, frames: i32) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetFrameSizeInCallback(self.raw.as_ptr(), frames)
        })?;
        Ok(self)
    }

    /// Sets renderer volume mode (`@since 19`).
    #[cfg(feature = "api-19")]
    pub fn volume_mode(&mut self, mode: VolumeMode) -> Result<&mut Self> {
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetVolumeMode(self.raw.as_ptr(), mode.into())
        })?;
        Ok(self)
    }

    /// Registers the PCM write-data callback (`@since 12`).
    ///
    /// Ownership of `callback` transfers into the built [`AudioRenderer`] and
    /// stays alive until native renderer release completes.
    pub fn write_data_callback<F>(&mut self, callback: F) -> Result<&mut Self>
    where
        F: FnMut(&mut [u8]) -> AudioDataCallbackResult + Send + 'static,
    {
        let mut boxed = Box::new(WriteDataBox(Box::new(callback)));
        let ptr = (&mut *boxed as *mut WriteDataBox).cast::<c_void>();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererWriteDataCallback(
                self.raw.as_ptr(),
                Some(on_write_data_trampoline),
                ptr,
            )
        })?;
        self.callbacks.write_data = Some(boxed);
        Ok(self)
    }

    /// Registers the output device change callback (`@since 11`).
    pub fn output_device_change_callback<F>(&mut self, callback: F) -> Result<&mut Self>
    where
        F: FnMut(DeviceChangeReason) + Send + 'static,
    {
        let mut boxed = Box::new(DeviceChangeBox(Box::new(callback)));
        let ptr = (&mut *boxed as *mut DeviceChangeBox).cast::<c_void>();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererOutputDeviceChangeCallback(
                self.raw.as_ptr(),
                Some(on_device_change_trampoline),
                ptr,
            )
        })?;
        self.callbacks.device_change = Some(boxed);
        Ok(self)
    }

    /// Registers the audio interrupt callback (`@since 20`).
    #[cfg(feature = "api-20")]
    pub fn interrupt_callback<F>(&mut self, callback: F) -> Result<&mut Self>
    where
        F: FnMut(InterruptForceType, InterruptHint) + Send + 'static,
    {
        let mut boxed = Box::new(InterruptBox(Box::new(callback)));
        let ptr = (&mut *boxed as *mut InterruptBox).cast::<c_void>();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererInterruptCallback(
                self.raw.as_ptr(),
                Some(on_interrupt_trampoline),
                ptr,
            )
        })?;
        self.callbacks.interrupt = Some(boxed);
        Ok(self)
    }

    /// Registers the renderer error callback (`@since 20`).
    #[cfg(feature = "api-20")]
    pub fn error_callback<F>(&mut self, callback: F) -> Result<&mut Self>
    where
        F: FnMut(AudioError) + Send + 'static,
    {
        let mut boxed = Box::new(ErrorBox(Box::new(callback)));
        let ptr = (&mut *boxed as *mut ErrorBox).cast::<c_void>();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_SetRendererErrorCallback(
                self.raw.as_ptr(),
                Some(on_error_trampoline),
                ptr,
            )
        })?;
        self.callbacks.error = Some(boxed);
        Ok(self)
    }

    /// Consumes the builder and generates an owned [`AudioRenderer`] (`@since 10`).
    pub fn build(mut self) -> Result<AudioRenderer> {
        let mut raw: *mut sys::OH_AudioRenderer = ptr::null_mut();
        check_stream(unsafe {
            sys::OH_AudioStreamBuilder_GenerateRenderer(self.raw.as_ptr(), &mut raw)
        })?;
        let raw = NonNull::new(raw).ok_or_else(|| {
            AudioError::CommonError("OH_AudioStreamBuilder_GenerateRenderer returned null".into())
        })?;
        let callbacks = std::mem::replace(&mut self.callbacks, RendererCallbacks::empty());
        Ok(AudioRenderer {
            raw: Some(raw),
            callbacks: Some(callbacks),
            _not_sync: PhantomData,
        })
    }
}

impl Drop for AudioStreamBuilder {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_AudioStreamBuilder_Destroy(self.raw.as_ptr()) };
    }
}

/// Owned native OHAudio PCM renderer stream (`@since 10/12`).
///
/// Control methods require `&mut self` to serialize native operations. Dropping
/// the renderer releases the native stream before reclaiming callback closures.
pub struct AudioRenderer {
    raw: Option<NonNull<sys::OH_AudioRenderer>>,
    callbacks: Option<RendererCallbacks>,
    _not_sync: PhantomData<Cell<()>>,
}

// SAFETY: `OH_AudioRenderer` supports serialized calls across threads; `&mut self`
// and `!Sync` prevent concurrent native operations.
unsafe impl Send for AudioRenderer {}

impl AudioRenderer {
    fn raw(&self) -> Result<*mut sys::OH_AudioRenderer> {
        self.raw
            .map(NonNull::as_ptr)
            .ok_or_else(|| AudioError::CommonError("audio renderer is closed".into()))
    }

    /// Starts playback (`@since 10`).
    pub fn start(&mut self) -> Result<()> {
        check_stream(unsafe { sys::OH_AudioRenderer_Start(self.raw()?) })
    }

    /// Pauses playback (`@since 10`).
    pub fn pause(&mut self) -> Result<()> {
        check_stream(unsafe { sys::OH_AudioRenderer_Pause(self.raw()?) })
    }

    /// Stops playback (`@since 10`).
    pub fn stop(&mut self) -> Result<()> {
        check_stream(unsafe { sys::OH_AudioRenderer_Stop(self.raw()?) })
    }

    /// Flushes queued audio buffers (`@since 10`).
    pub fn flush(&mut self) -> Result<()> {
        check_stream(unsafe { sys::OH_AudioRenderer_Flush(self.raw()?) })
    }

    /// Returns the current lifecycle state (`@since 10`).
    pub fn state(&mut self) -> Result<StreamState> {
        let mut state = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetCurrentState(self.raw()?, &mut state) })?;
        Ok(StreamState::try_from_raw(state).unwrap_or(StreamState::Invalid))
    }

    /// Returns the sampling rate in Hz (`@since 10`).
    pub fn sampling_rate(&mut self) -> Result<i32> {
        let mut rate = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetSamplingRate(self.raw()?, &mut rate) })?;
        Ok(rate)
    }

    /// Returns the channel count (`@since 10`).
    pub fn channel_count(&mut self) -> Result<i32> {
        let mut count = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetChannelCount(self.raw()?, &mut count) })?;
        Ok(count)
    }

    /// Returns the sample format (`@since 10`).
    pub fn sample_format(&mut self) -> Result<SampleFormat> {
        let mut format = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetSampleFormat(self.raw()?, &mut format) })?;
        SampleFormat::try_from_raw(format)
            .ok_or_else(|| AudioError::CommonError(format!("unknown sample format {format}")))
    }

    /// Returns the latency mode (`@since 10`).
    pub fn latency_mode(&mut self) -> Result<LatencyMode> {
        let mut mode = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetLatencyMode(self.raw()?, &mut mode) })?;
        LatencyMode::try_from_raw(mode)
            .ok_or_else(|| AudioError::CommonError(format!("unknown latency mode {mode}")))
    }

    /// Returns the system stream ID (`@since 10`).
    pub fn stream_id(&mut self) -> Result<u32> {
        let mut id = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetStreamId(self.raw()?, &mut id) })?;
        Ok(id)
    }

    /// Returns the callback buffer size in frames (`@since 10`).
    pub fn frame_size_in_callback(&mut self) -> Result<i32> {
        let mut size = 0;
        check_stream(unsafe {
            sys::OH_AudioRenderer_GetFrameSizeInCallback(self.raw()?, &mut size)
        })?;
        Ok(size)
    }

    /// Returns the total frames written since stream creation (`@since 10`).
    pub fn frames_written(&mut self) -> Result<i64> {
        let mut frames = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetFramesWritten(self.raw()?, &mut frames) })?;
        Ok(frames)
    }

    /// Sets playback volume in `0.0..=1.0` (`@since 12`).
    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        if !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
            return Err(AudioError::CommonError(
                "volume must be finite and within 0.0..=1.0".into(),
            ));
        }
        check_stream(unsafe { sys::OH_AudioRenderer_SetVolume(self.raw()?, volume) })
    }

    /// Ramps playback volume to `volume` (`0.0..=1.0`) over `duration_ms` (`@since 12`).
    pub fn set_volume_with_ramp(&mut self, volume: f32, duration_ms: i32) -> Result<()> {
        if !volume.is_finite() || !(0.0..=1.0).contains(&volume) || duration_ms < 0 {
            return Err(AudioError::CommonError(
                "volume must be within 0.0..=1.0 and duration_ms >= 0".into(),
            ));
        }
        check_stream(unsafe {
            sys::OH_AudioRenderer_SetVolumeWithRamp(self.raw()?, volume, duration_ms)
        })
    }

    /// Returns the current playback volume (`@since 12`).
    pub fn volume(&mut self) -> Result<f32> {
        let mut volume = 0.0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetVolume(self.raw()?, &mut volume) })?;
        Ok(volume)
    }

    /// Sets playback speed in `0.25..=4.0` (`@since 11`).
    pub fn set_speed(&mut self, speed: f32) -> Result<()> {
        if !speed.is_finite() || !(0.25..=4.0).contains(&speed) {
            return Err(AudioError::CommonError(
                "speed must be finite and within 0.25..=4.0".into(),
            ));
        }
        check_stream(unsafe { sys::OH_AudioRenderer_SetSpeed(self.raw()?, speed) })
    }

    /// Returns the current playback speed (`@since 11`).
    pub fn speed(&mut self) -> Result<f32> {
        let mut speed = 0.0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetSpeed(self.raw()?, &mut speed) })?;
        Ok(speed)
    }

    /// Enables or disables silent mixing with other streams (`@since 12`).
    pub fn set_silent_mode_and_mix_with_others(&mut self, enabled: bool) -> Result<()> {
        check_stream(unsafe {
            sys::OH_AudioRenderer_SetSilentModeAndMixWithOthers(self.raw()?, enabled)
        })
    }

    /// Returns whether silent-mix mode is active (`@since 12`).
    pub fn silent_mode_and_mix_with_others(&mut self) -> Result<bool> {
        let mut enabled = false;
        check_stream(unsafe {
            sys::OH_AudioRenderer_GetSilentModeAndMixWithOthers(self.raw()?, &mut enabled)
        })?;
        Ok(enabled)
    }

    /// Sets the audio effect mode (`@since 12`).
    pub fn set_effect_mode(&mut self, mode: AudioEffectMode) -> Result<()> {
        check_stream(unsafe { sys::OH_AudioRenderer_SetEffectMode(self.raw()?, mode.into()) })
    }

    /// Returns the stream underflow count (`@since 12`).
    pub fn underflow_count(&mut self) -> Result<u32> {
        let mut count = 0;
        check_stream(unsafe { sys::OH_AudioRenderer_GetUnderflowCount(self.raw()?, &mut count) })?;
        Ok(count)
    }

    /// Queries `(frame_position, timestamp_ns)` in monotonic clock timebase (`@since 15`).
    #[cfg(feature = "api-15")]
    pub fn audio_timestamp_info(&mut self) -> Result<(i64, i64)> {
        let mut pos = 0;
        let mut ts = 0;
        check_stream(unsafe {
            sys::OH_AudioRenderer_GetAudioTimestampInfo(self.raw()?, &mut pos, &mut ts)
        })?;
        Ok((pos, ts))
    }

    /// Sets loudness gain in dB (`-90.0..=24.0`) (`@since 20`).
    #[cfg(feature = "api-20")]
    pub fn set_loudness_gain(&mut self, gain_db: f32) -> Result<()> {
        if !gain_db.is_finite() || !(-90.0..=24.0).contains(&gain_db) {
            return Err(AudioError::CommonError(
                "gain_db must be finite and within -90.0..=24.0".into(),
            ));
        }
        check_stream(unsafe { sys::OH_AudioRenderer_SetLoudnessGain(self.raw()?, gain_db) })
    }

    /// Queries the estimated output latency in milliseconds (`@since 23`).
    #[cfg(feature = "api-23")]
    pub fn latency(&mut self, latency_type: LatencyType) -> Result<i32> {
        let mut latency_ms = 0;
        check_stream(unsafe {
            sys::OH_AudioRenderer_GetLatency(self.raw()?, latency_type.into(), &mut latency_ms)
        })?;
        Ok(latency_ms)
    }

    /// Releases the native renderer and reclaims its callbacks (`@since 10`).
    pub fn close(mut self) -> Result<()> {
        self.release_inner()
    }

    fn release_inner(&mut self) -> Result<()> {
        let Some(raw) = self.raw.take() else {
            return Ok(());
        };
        let result = check_stream(unsafe { sys::OH_AudioRenderer_Release(raw.as_ptr()) });
        if result.is_ok() {
            self.callbacks = None;
        } else if let Some(callbacks) = self.callbacks.take() {
            // Native release failed; retain callback closures rather than risking UAF.
            std::mem::forget(callbacks);
        }
        result
    }
}

impl Drop for AudioRenderer {
    fn drop(&mut self) {
        let _ = self.release_inner();
    }
}
