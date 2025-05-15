#[derive(Debug, PartialEq)]
pub enum XComponentResultCode {
    Success = 0,
    Failed = -1,
    BadParameter = -2,
}

impl From<i32> for XComponentResultCode {
    fn from(value: i32) -> Self {
        match value {
            0 => XComponentResultCode::Success,
            -1 => XComponentResultCode::Failed,
            -2 => XComponentResultCode::BadParameter,
            _ => unimplemented!("Unsupported XComponentResultCode: {}", value),
        }
    }
}
