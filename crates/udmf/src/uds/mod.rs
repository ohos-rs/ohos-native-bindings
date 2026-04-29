mod html;
#[cfg(feature = "api-13")]
mod pixel_map;
mod plain_text;

pub use html::*;
#[cfg(feature = "api-13")]
pub use pixel_map::*;
pub use plain_text::*;

use crate::{UdmfError, UdmfMeta};

pub trait UdsValue {
    /// Get the type of the UdsValue
    fn get_type(&self) -> Result<UdmfMeta, UdmfError>;
}

pub enum Uds {
    PlainText(UdsPlainText),
    Html(UdsHtml),
    #[cfg(feature = "api-13")]
    PixelMap(UdsPixelMap),
}
