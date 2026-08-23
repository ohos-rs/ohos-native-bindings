use crate::error::{BatteryInfoError, Result};
use ohos_battery_info_sys as sys;
use std::fmt;

/// The power source the device is currently plugged into.
///
/// Mirrors the native `BatteryInfo_BatteryPluggedType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluggedType {
    /// The power source is unplugged.
    None,
    /// The power source is an AC charger.
    Ac,
    /// The power source is a USB DC charger.
    Usb,
    /// The power source is a wireless charger.
    Wireless,
    /// The power source type is unknown (native `PLUGGED_TYPE_BUTT`).
    Unknown,
}

impl PluggedType {
    /// Whether the device is drawing power from an external source.
    ///
    /// [`PluggedType::Unknown`] is not treated as charging.
    pub fn is_plugged(&self) -> bool {
        matches!(
            self,
            PluggedType::Ac | PluggedType::Usb | PluggedType::Wireless
        )
    }

    /// A short, stable description of this power source.
    pub fn describe(&self) -> &'static str {
        match self {
            PluggedType::None => "unplugged",
            PluggedType::Ac => "AC charger",
            PluggedType::Usb => "USB DC charger",
            PluggedType::Wireless => "wireless charger",
            PluggedType::Unknown => "unknown power source",
        }
    }

    /// Map a raw `BatteryInfo_BatteryPluggedType` value.
    ///
    /// The constants are matched through qualified `sys::` paths so that an arm
    /// whose constant is absent under the current feature set is a compile
    /// error instead of a catch-all binding pattern.
    pub(crate) fn from_raw(raw: sys::BatteryInfo_BatteryPluggedType) -> Result<Self> {
        match raw {
            sys::BatteryInfo_BatteryPluggedType_PLUGGED_TYPE_NONE => Ok(PluggedType::None),
            sys::BatteryInfo_BatteryPluggedType_PLUGGED_TYPE_AC => Ok(PluggedType::Ac),
            sys::BatteryInfo_BatteryPluggedType_PLUGGED_TYPE_USB => Ok(PluggedType::Usb),
            sys::BatteryInfo_BatteryPluggedType_PLUGGED_TYPE_WIRELESS => Ok(PluggedType::Wireless),
            sys::BatteryInfo_BatteryPluggedType_PLUGGED_TYPE_BUTT => Ok(PluggedType::Unknown),
            other => Err(BatteryInfoError::UnknownPluggedType(other)),
        }
    }
}

impl fmt::Display for PluggedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.describe())
    }
}
