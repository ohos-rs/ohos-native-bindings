use ohos_enum_derive::EnumFrom;
use ohos_hiappevent_sys::*;

/// Event types. Pick the one matching the scenario being logged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(EventType, "EventType_")]
pub enum AppEventType {
    Fault,
    Statistic,
    Security,
    Behavior,
}

impl AppEventType {
    /// The bit this type occupies in the mask taken by
    /// [`Watcher::set_app_event_filter`](crate::Watcher::set_app_event_filter).
    /// It is not the enum value itself.
    pub(crate) fn filter_bit(self) -> u8 {
        1u8 << (EventType::from(self) - 1)
    }
}
