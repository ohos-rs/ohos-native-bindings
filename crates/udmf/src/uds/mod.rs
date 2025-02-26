mod html;
mod plain_text;

pub use html::*;
pub use plain_text::*;

pub enum Uds {
    PlainText(UdsPlainText),
    Html(UdsHtml),
}
