use std::collections::HashMap;

#[derive(serde::Serialize)]
pub(crate) struct Doc {
    pub(crate) content: Vec<Node>,
    pub(crate) styles: HashMap<String, Vec<(&'static str, String)>>,
}

// #[derive(serde::Serialize, Clone, Default)]
// pub(crate) struct Text {
//     pub(crate) value: String,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub(crate) link: Option<String>,
// }

#[derive(serde::Serialize, Clone, Debug)]
pub(crate) enum Text {
    Text(String),
    TextWithOptions(String, TextOptions),
}

#[derive(serde::Serialize, Clone, Default, Debug)]
pub(crate) struct TextOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) link: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) styles: Styles,
}

pub(crate) type Styles = Vec<(&'static str, String)>;
pub(crate) type HasNewLine = bool;

impl Text {
    pub(crate) fn new(value: String) -> Self {
        Self::Text(value)
    }

    pub(crate) fn new_with_options(value: String, options: TextOptions) -> Self {
        Self::TextWithOptions(value, options)
    }
}

#[derive(serde::Serialize, Clone, Debug)]
pub(crate) enum Node {
    Paragraph(Vec<Node>),
    Code(CodeLang, String),
    Title(Title, Styles),
    Table(Table),
    List(Vec<Node>),
    InlineText(Vec<Text>, HasNewLine),
    Image(String, (String, String), Styles),

    // End
    Clear,
}

type CodeLang = String;

#[derive(serde::Serialize, Clone, Debug)]
pub(crate) struct Title {
    pub(crate) level: u8,
    pub(crate) text: Text,
}

#[derive(serde::Serialize, Clone, Default, Debug)]
pub(crate) struct Table {
    pub(crate) rows: Vec<TableRow>,
}

#[derive(serde::Serialize, Clone, Default, Debug)]
pub(crate) struct TableRow {
    pub(crate) cells: Vec<Node>,
}
