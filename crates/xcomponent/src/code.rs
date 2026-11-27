#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XComponentResultCode {
    Success,
    Failed,
    BadParameter,
    Other(i32),
}

impl From<i32> for XComponentResultCode {
    fn from(value: i32) -> Self {
        match value {
            0 => XComponentResultCode::Success,
            -1 => XComponentResultCode::Failed,
            -2 => XComponentResultCode::BadParameter,
            other => XComponentResultCode::Other(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, EventSource, KeyCode, MouseAction, MouseButton, TouchEvent};

    #[test]
    fn preserves_unknown_result_codes_and_input_discriminants() {
        assert_eq!(XComponentResultCode::from(0), XComponentResultCode::Success);
        assert_eq!(
            XComponentResultCode::from(-99),
            XComponentResultCode::Other(-99)
        );
        assert_eq!(KeyCode::from(999_999u32), KeyCode::Unknown);
        assert_eq!(Action::from(99), Action::Unknown);
        assert_eq!(EventSource::from(99), EventSource::Unknown);
        assert_eq!(TouchEvent::from(99), TouchEvent::Unknown);
        assert_eq!(MouseAction::from(99), MouseAction::None);
        assert_eq!(MouseButton::from(99), MouseButton::NoneButton);
    }
}
