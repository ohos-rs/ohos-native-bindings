#[derive(Debug, Clone)]
pub struct Web {
    web_tag: String,
}

impl Web {
    pub fn new(web_tag: String) -> Self {
        Self { web_tag }
    }
}
