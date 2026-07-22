use crate::error::{LocationError, Result};
use ohos_location_sys as sys;
use std::os::raw::c_int;
use std::ptr::NonNull;

/// Scenario the application requests locations for.
///
/// The use scenario takes precedence over
/// [`LocationPowerConsumptionScene`]: once it is set, the power consumption
/// scenario is ignored. When neither is set the service behaves as if
/// [`LocationUseScene::DailyLifeService`] had been requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationUseScene {
    /// Outdoor navigation, GNSS based, high power consumption.
    Navigation,
    /// Sport tracking, GNSS based, high power consumption.
    Sport,
    /// Transport such as taxi or public transport, GNSS based, high power
    /// consumption.
    Transport,
    /// Everyday services that do not need a precise location, network based,
    /// low power consumption.
    DailyLifeService,
}

impl LocationUseScene {
    /// The raw `Location_UseScene` value.
    pub fn to_raw(self) -> sys::Location_UseScene {
        match self {
            LocationUseScene::Navigation => sys::Location_UseScene_LOCATION_USE_SCENE_NAVIGATION,
            LocationUseScene::Sport => sys::Location_UseScene_LOCATION_USE_SCENE_SPORT,
            LocationUseScene::Transport => sys::Location_UseScene_LOCATION_USE_SCENE_TRANSPORT,
            LocationUseScene::DailyLifeService => {
                sys::Location_UseScene_LOCATION_USE_SCENE_DAILY_LIFE_SERVICE
            }
        }
    }

    /// Build from a raw `Location_UseScene` value, or `None` for a value this
    /// binding does not know.
    pub fn from_raw(raw: sys::Location_UseScene) -> Option<Self> {
        match raw {
            sys::Location_UseScene_LOCATION_USE_SCENE_NAVIGATION => {
                Some(LocationUseScene::Navigation)
            }
            sys::Location_UseScene_LOCATION_USE_SCENE_SPORT => Some(LocationUseScene::Sport),
            sys::Location_UseScene_LOCATION_USE_SCENE_TRANSPORT => {
                Some(LocationUseScene::Transport)
            }
            sys::Location_UseScene_LOCATION_USE_SCENE_DAILY_LIFE_SERVICE => {
                Some(LocationUseScene::DailyLifeService)
            }
            _ => None,
        }
    }
}

/// Power budget the application grants the location service.
///
/// Only taken into account when no [`LocationUseScene`] is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationPowerConsumptionScene {
    /// Mainly GNSS based, falling back to network positioning until GNSS is
    /// stable; consumes a lot of hardware resources and power.
    HighPowerConsumption,
    /// Network based, for scenarios that do not need a precise location.
    LowPowerConsumption,
    /// Never triggers positioning on its own; a location is only reported when
    /// another application is being located.
    NoPowerConsumption,
}

impl LocationPowerConsumptionScene {
    /// The raw `Location_PowerConsumptionScene` value.
    pub fn to_raw(self) -> sys::Location_PowerConsumptionScene {
        match self {
            LocationPowerConsumptionScene::HighPowerConsumption => {
                sys::Location_PowerConsumptionScene_LOCATION_HIGH_POWER_CONSUMPTION
            }
            LocationPowerConsumptionScene::LowPowerConsumption => {
                sys::Location_PowerConsumptionScene_LOCATION_LOW_POWER_CONSUMPTION
            }
            LocationPowerConsumptionScene::NoPowerConsumption => {
                sys::Location_PowerConsumptionScene_LOCATION_NO_POWER_CONSUMPTION
            }
        }
    }

    /// Build from a raw `Location_PowerConsumptionScene` value, or `None` for a
    /// value this binding does not know.
    pub fn from_raw(raw: sys::Location_PowerConsumptionScene) -> Option<Self> {
        match raw {
            sys::Location_PowerConsumptionScene_LOCATION_HIGH_POWER_CONSUMPTION => {
                Some(LocationPowerConsumptionScene::HighPowerConsumption)
            }
            sys::Location_PowerConsumptionScene_LOCATION_LOW_POWER_CONSUMPTION => {
                Some(LocationPowerConsumptionScene::LowPowerConsumption)
            }
            sys::Location_PowerConsumptionScene_LOCATION_NO_POWER_CONSUMPTION => {
                Some(LocationPowerConsumptionScene::NoPowerConsumption)
            }
            _ => None,
        }
    }
}

/// An owned native location request parameter instance.
///
/// Created through [`RequestConfig::new`] or [`RequestConfig::builder`] and
/// destroyed on drop, so `OH_Location_CreateRequestConfig` and
/// `OH_Location_DestroyRequestConfig` are always paired.
///
/// The instance is not shared with the location service until it is handed to
/// [`crate::LocationSession::start`], which takes ownership of it for as long
/// as the session runs.
pub struct RequestConfig {
    handle: NonNull<sys::Location_RequestConfig>,
}

impl RequestConfig {
    /// Create a request parameter instance with the native defaults.
    pub fn new() -> Result<Self> {
        // SAFETY: the native call takes no argument and returns either a fresh
        // instance or null.
        let handle = unsafe { sys::OH_Location_CreateRequestConfig() };
        NonNull::new(handle)
            .map(|handle| RequestConfig { handle })
            .ok_or(LocationError::AllocationFailed)
    }

    /// Start describing a request through [`RequestConfigBuilder`].
    pub fn builder() -> RequestConfigBuilder {
        RequestConfigBuilder::default()
    }

    /// Set the use scenario, which takes precedence over any power consumption
    /// scenario.
    pub fn set_use_scene(&mut self, use_scene: LocationUseScene) {
        // SAFETY: `self.handle` is a live instance owned by `self`.
        unsafe {
            sys::OH_LocationRequestConfig_SetUseScene(self.handle.as_ptr(), use_scene.to_raw())
        }
    }

    /// Set the power consumption scenario, which only applies when no use
    /// scenario is set.
    pub fn set_power_consumption_scene(&mut self, scene: LocationPowerConsumptionScene) {
        // SAFETY: `self.handle` is a live instance owned by `self`.
        unsafe {
            sys::OH_LocationRequestConfig_SetPowerConsumptionScene(
                self.handle.as_ptr(),
                scene.to_raw(),
            )
        }
    }

    /// Set the reporting interval in seconds; the native default is one second.
    ///
    /// Returns [`LocationError::InvalidInterval`] for zero and for values that
    /// do not fit into a C `int`.
    pub fn set_interval(&mut self, seconds: u32) -> Result<()> {
        let interval = validate_interval(seconds)?;
        // SAFETY: `self.handle` is a live instance owned by `self` and the
        // interval was checked to be within the documented range.
        unsafe { sys::OH_LocationRequestConfig_SetInterval(self.handle.as_ptr(), interval) }
        Ok(())
    }

    /// The underlying instance, for handing it to the native locating calls.
    pub(crate) fn as_ptr(&self) -> *mut sys::Location_RequestConfig {
        self.handle.as_ptr()
    }
}

impl Drop for RequestConfig {
    fn drop(&mut self) {
        // SAFETY: `self.handle` was created by
        // `OH_Location_CreateRequestConfig`, is owned by `self` and is
        // destroyed exactly once here. A session that used this instance has
        // already been stopped, because it owns the instance and stops itself
        // before dropping it.
        unsafe { sys::OH_Location_DestroyRequestConfig(self.handle.as_ptr()) }
    }
}

// SAFETY: the handle is an opaque owned allocation; every native call taking it
// receives it as an argument rather than through thread local state, and
// `RequestConfig` hands out no interior mutability, so moving one between
// threads is sound.
unsafe impl Send for RequestConfig {}

fn validate_interval(seconds: u32) -> Result<c_int> {
    if seconds == 0 || seconds > i32::MAX as u32 {
        return Err(LocationError::InvalidInterval(seconds));
    }
    Ok(seconds as c_int)
}

/// Builder for a [`RequestConfig`].
///
/// Nothing is allocated until [`RequestConfigBuilder::build`] is called, so a
/// builder can be assembled and validated without touching the location
/// service.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RequestConfigBuilder {
    use_scene: Option<LocationUseScene>,
    power_consumption_scene: Option<LocationPowerConsumptionScene>,
    interval: Option<u32>,
}

impl RequestConfigBuilder {
    /// Request locations for a use scenario, which takes precedence over any
    /// power consumption scenario.
    pub fn use_scene(mut self, use_scene: LocationUseScene) -> Self {
        self.use_scene = Some(use_scene);
        self
    }

    /// Request locations under a power budget; only applied when no use
    /// scenario is set on the same builder.
    pub fn power_consumption_scene(mut self, scene: LocationPowerConsumptionScene) -> Self {
        self.power_consumption_scene = Some(scene);
        self
    }

    /// Report a location every `seconds` seconds; the native default is one
    /// second.
    pub fn interval(mut self, seconds: u32) -> Self {
        self.interval = Some(seconds);
        self
    }

    /// Allocate the native instance and apply the collected parameters.
    pub fn build(self) -> Result<RequestConfig> {
        if let Some(seconds) = self.interval {
            validate_interval(seconds)?;
        }
        let mut config = RequestConfig::new()?;
        if let Some(use_scene) = self.use_scene {
            config.set_use_scene(use_scene);
        }
        if let Some(scene) = self.power_consumption_scene {
            config.set_power_consumption_scene(scene);
        }
        if let Some(seconds) = self.interval {
            config.set_interval(seconds)?;
        }
        Ok(config)
    }
}
