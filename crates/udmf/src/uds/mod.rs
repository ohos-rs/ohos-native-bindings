mod html;
mod plain_text;

pub use html::*;
pub use plain_text::*;

use crate::{UdmfError, UdmfMeta};

pub trait UdsValue {
    /// Get the type of the UdsValue
    fn get_type(&self) -> Result<UdmfMeta, UdmfError>;
}

pub enum Uds {
    PlainText(UdsPlainText),
    Html(UdsHtml),
}
