use ohos_location_sys as sys;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::NonNull;

/// Technology the location fix was produced with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationSourceType {
    /// Global navigation satellite system.
    Gnss,
    /// Network positioning.
    Network,
    /// High-precision indoor positioning.
    Indoor,
    /// Real-time kinematic high-precision positioning.
    Rtk,
}

impl LocationSourceType {
    /// The raw `Location_SourceType` value.
    pub fn to_raw(self) -> sys::Location_SourceType {
        match self {
            LocationSourceType::Gnss => sys::Location_SourceType_LOCATION_SOURCE_TYPE_GNSS,
            LocationSourceType::Network => sys::Location_SourceType_LOCATION_SOURCE_TYPE_NETWORK,
            LocationSourceType::Indoor => sys::Location_SourceType_LOCATION_SOURCE_TYPE_INDOOR,
            LocationSourceType::Rtk => sys::Location_SourceType_LOCATION_SOURCE_TYPE_RTK,
        }
    }

    /// Build from a raw `Location_SourceType` value, or `None` for a value this
    /// binding does not know.
    ///
    /// The arms are qualified `sys::` paths, so a constant missing under the
    /// current feature set is a compile error rather than a catch-all pattern.
    pub fn from_raw(raw: sys::Location_SourceType) -> Option<Self> {
        match raw {
            sys::Location_SourceType_LOCATION_SOURCE_TYPE_GNSS => Some(LocationSourceType::Gnss),
            sys::Location_SourceType_LOCATION_SOURCE_TYPE_NETWORK => {
                Some(LocationSourceType::Network)
            }
            sys::Location_SourceType_LOCATION_SOURCE_TYPE_INDOOR => {
                Some(LocationSourceType::Indoor)
            }
            sys::Location_SourceType_LOCATION_SOURCE_TYPE_RTK => Some(LocationSourceType::Rtk),
            _ => None,
        }
    }
}

/// Size of the buffer handed to `OH_LocationInfo_GetAdditionalInfo`.
///
/// The native documentation asks for at least 256 bytes; one extra byte is
/// reserved beyond the length reported to the service so that the buffer always
/// ends in a terminator even if the service filled it completely.
const ADDITIONAL_INFO_CAPACITY: usize = 512;

/// One location fix, owned by the caller.
///
/// This is a snapshot of the native `Location_BasicInfo` plus the additional
/// information string: the native `Location_Info` instance only lives for the
/// duration of the callback, this type outlives it.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationInfo {
    /// Latitude in degrees, positive north, in the range -90..=90 (WGS84).
    pub latitude: f64,
    /// Longitude in degrees, positive east, in the range -180..=180 (WGS84).
    pub longitude: f64,
    /// Altitude in meters.
    pub altitude: f64,
    /// Horizontal accuracy in meters.
    pub accuracy: f64,
    /// Speed in meters per second.
    pub speed: f64,
    /// Heading in degrees, in the range 0..=360.
    pub direction: f64,
    /// Time of the fix in milliseconds since the Unix epoch.
    pub time_for_fix: i64,
    /// Time since boot in nanoseconds, including deep sleep.
    pub time_since_boot: i64,
    /// Vertical accuracy in meters.
    pub altitude_accuracy: f64,
    /// Speed accuracy in meters per second.
    pub speed_accuracy: f64,
    /// Heading accuracy in degrees, in the range 0..=360.
    pub direction_accuracy: f64,
    /// Uncertainty of [`LocationInfo::time_since_boot`] in nanoseconds.
    pub uncertainty_of_time_since_boot: i64,
    /// Technology the fix came from, or `None` for a source this binding does
    /// not know.
    pub source_type: Option<LocationSourceType>,
    /// Additional information reported alongside the fix, a JSON document.
    /// `None` when the service reported none.
    pub additional_info: Option<String>,
}

impl LocationInfo {
    /// Copy a native location instance into an owned [`LocationInfo`].
    ///
    /// # Safety
    ///
    /// `location` must point at a live `Location_Info` instance. Such an
    /// instance is only handed out by the native reporting callback and is
    /// recycled as soon as that callback returns, so this may only be called
    /// from inside the callback with the pointer the callback received.
    pub(crate) unsafe fn from_raw(location: NonNull<sys::Location_Info>) -> Self {
        // SAFETY: the caller guarantees `location` is live; the native call
        // returns the basic information struct by value.
        let basic = unsafe { sys::OH_LocationInfo_GetBasicInfo(location.as_ptr()) };

        let mut buffer = [0 as c_char; ADDITIONAL_INFO_CAPACITY];
        // The reported length is one byte short of the buffer so that the last
        // byte stays zero and the content is terminated in any case.
        let usable = (ADDITIONAL_INFO_CAPACITY - 1) as u32;
        // SAFETY: the caller guarantees `location` is live; `buffer` is an
        // owned array of `ADDITIONAL_INFO_CAPACITY` elements and the length
        // passed to the service stays below that.
        let code = unsafe {
            sys::OH_LocationInfo_GetAdditionalInfo(location.as_ptr(), buffer.as_mut_ptr(), usable)
        };
        let additional_info = if code == sys::Location_ResultCode_LOCATION_SUCCESS {
            // SAFETY: `buffer` is zero initialized and the service was told it
            // is one byte shorter than it is, so a terminator is always within
            // bounds.
            let text = unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy();
            if text.is_empty() {
                None
            } else {
                Some(text.into_owned())
            }
        } else {
            None
        };

        LocationInfo {
            latitude: basic.latitude,
            longitude: basic.longitude,
            altitude: basic.altitude,
            accuracy: basic.accuracy,
            speed: basic.speed,
            direction: basic.direction,
            time_for_fix: basic.timeForFix,
            time_since_boot: basic.timeSinceBoot,
            altitude_accuracy: basic.altitudeAccuracy,
            speed_accuracy: basic.speedAccuracy,
            direction_accuracy: basic.directionAccuracy,
            uncertainty_of_time_since_boot: basic.uncertaintyOfTimeSinceBoot,
            source_type: LocationSourceType::from_raw(basic.locationSourceType),
            additional_info,
        }
    }
}
