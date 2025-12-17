#[derive(Debug, Clone)]
pub struct InlineContent {
    pub spans: Vec<InlineSpan>,
}

#[derive(Debug, Clone)]
pub enum InlineSpan {
    Text(String),
    Emphasis(String),
    Strong(String),
    Code(String),
    Link {
        text: String,
        url: String,
    },
}
