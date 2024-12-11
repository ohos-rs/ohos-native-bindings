use crate::{Action, EventSource, KeyCode};

pub struct KeyEventData {
    pub code: KeyCode,
    pub action: Action,
    pub device_id: i64,
    pub source: EventSource,
    pub timestamp: i64,
}
