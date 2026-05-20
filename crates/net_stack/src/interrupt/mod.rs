use crate::http::{EventsHandler, OnVoidCallback};

#[derive(Debug, Clone, Copy, Default)]
pub struct InterruptHandler {
    pub on_canceled: OnVoidCallback,
}

impl InterruptHandler {
    pub fn new(on_canceled: OnVoidCallback) -> Self {
        Self { on_canceled }
    }

    pub fn into_events_handler(self) -> EventsHandler {
        EventsHandler {
            on_canceled: self.on_canceled,
            ..EventsHandler::default()
        }
    }
}
