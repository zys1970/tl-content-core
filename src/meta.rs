use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ContentMeta {
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub attributes: HashMap<String, String>,
}
