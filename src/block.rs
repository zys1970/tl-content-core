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
        items: Vec<ContentBlock>,
    },
    Quote {
        content: Vec<ContentBlock>,
    },
    Table {
        alignment: Vec<Alignment>,
        header: Vec<InlineContent>,
        rows: Vec<Vec<InlineContent>>,
    },
    Divider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    None,  // 或 Default
}
