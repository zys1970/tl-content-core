use crate::{block::ContentBlock, meta::ContentMeta};

#[derive(Debug, Clone)]
pub struct ContentDocument {
    pub id: String,
    pub blocks: Vec<ContentBlock>,
    pub meta: ContentMeta,
}
