use crate::inline::InlineContent;

#[derive(Debug, Clone)]
pub enum ContentBlock {
    Heading {
        level: u8,
        content: InlineContent,
    },
    Paragraph {
        content: InlineContent,
    },
    Code {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<InlineContent>,
    },
    Quote {
        content: Vec<InlineContent>,
    },
    Divider,
}
