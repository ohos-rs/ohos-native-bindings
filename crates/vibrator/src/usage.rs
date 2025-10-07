use ohos_enum_macro::EnumFrom;
use ohos_vibrator_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(Vibrator_Usage, "Vibrator_Usage_VIBRATOR_USAGE_")]
pub enum VibratorUsage {
    Unknown,
    Alarm,
    Ring,
    Notification,
    Communication,
    Touch,
    Media,
    PhysicalFeedback,
    SimulatedReality,
}