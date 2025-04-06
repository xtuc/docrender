use crate::BoxError;
use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Document {
    pub(crate) title: String,
    pub(crate) body: DocumentBody,

    #[serde(rename = "namedStyles")]
    pub(crate) named_styles: DocumentNamedStyles,

    #[serde(default, rename = "inlineObjects")]
    pub(crate) inline_objects: HashMap<String, DocumentInlineObject>,

    #[serde(default)]
    pub(crate) lists: HashMap<String, DocumentListObject>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentListObject {
    #[serde(rename = "listProperties")]
    pub(crate) list_properties: DocumentListProperties,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentListProperties {
    #[serde(rename = "nestingLevels")]
    pub(crate) nesting_levels: Vec<DocumentListNestingLevel>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentListNestingLevel {
    #[serde(rename = "glyphType")]
    pub(crate) glyph_type: Option<String>,
    #[serde(rename = "glyphSymbol")]
    pub(crate) glyph_symbol: Option<String>,
    #[serde(default, rename = "glyphFormat")]
    pub(crate) glyph_format: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentNamedStyles {
    pub(crate) styles: Vec<DocumentNamedStyle>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentNamedStyle {
    #[serde(rename = "namedStyleType")]
    pub(crate) named_style_type: String,
    #[serde(rename = "paragraphStyle")]
    pub(crate) paragraph_style: DocumentParagraphStyle,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Bullet {
    #[serde(rename = "listId")]
    pub(crate) list_id: String,
    #[serde(rename = "nestingLevel", default)]
    pub(crate) nesting_level: usize,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentInlineObject {
    #[serde(rename = "objectId")]
    pub(crate) object_id: String,
    #[serde(rename = "inlineObjectProperties")]
    pub(crate) inline_object_properties: DocumentInlineObjectProperties,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentInlineObjectProperties {
    #[serde(rename = "embeddedObject")]
    pub(crate) embedded_object: DocumentEmbeddedObject,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentEmbeddedObject {
    #[serde(rename = "imageProperties")]
    pub(crate) image_properties: DocumentImageProperties,
    pub(crate) size: DocumentEmbeddedObjectSize,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentEmbeddedObjectSize {
    pub(crate) height: Size,
    pub(crate) width: Size,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentImageProperties {
    #[serde(rename = "contentUri")]
    pub(crate) content_uri: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentBody {
    pub(crate) content: Vec<DocumentContent>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentContent {
    #[serde(default)]
    pub(crate) paragraph: Option<DocumentParagraph>,
    #[serde(default)]
    pub(crate) table: Option<DocumentTable>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentPerson {
    #[serde(rename = "personProperties")]
    pub(crate) person_properties: DocumentPersonProperties,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentPersonProperties {
    pub(crate) name: String,
    pub(crate) email: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentTable {
    #[serde(rename = "tableRows")]
    pub(crate) table_rows: Vec<DocumentTableRow>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentTableRow {
    #[serde(rename = "tableCells")]
    pub(crate) table_cells: Vec<DocumentBody>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentTableCell {
    pub(crate) content: Vec<DocumentTableCell>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraph {
    pub(crate) elements: Vec<DocumentParagraphElement>,
    #[serde(rename = "paragraphStyle")]
    pub(crate) paragraph_style: DocumentParagraphStyle,
    #[serde(default)]
    pub(crate) bullet: Option<Bullet>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraphStyle {
    #[serde(default, rename = "headingId")]
    pub(crate) heading_id: Option<String>,
    #[serde(rename = "namedStyleType")]
    pub(crate) named_style_type: String,
    direction: String,
    #[serde(default, rename = "lineSpacing")]
    pub(crate) line_spacing: Option<f32>,
    #[serde(default)]
    pub(crate) alignment: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraphElement {
    #[serde(default, rename = "textRun")]
    pub(crate) text_run: Option<DocumentParagraphElementText>,
    #[serde(default, rename = "inlineObjectElement")]
    pub(crate) inline_object_element: Option<DocumentParagraphElementInlineObject>,
    #[serde(default)]
    pub(crate) person: Option<DocumentPerson>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraphElementInlineObject {
    #[serde(rename = "inlineObjectId")]
    pub(crate) inline_object_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraphElementStyle {
    #[serde(default, rename = "paragraphStyle")]
    pub(crate) paragraph_style: Option<DocumentParagraphStyle>,

    #[serde(default, rename = "foregroundColor")]
    pub(crate) foreground_color: Option<ForegroundColor>,

    #[serde(default, rename = "backgroundColor")]
    pub(crate) background_color: Option<ForegroundColor>,

    #[serde(default, rename = "fontSize")]
    pub(crate) font_size: Option<Size>,

    #[serde(default)]
    pub(crate) bold: bool,
    #[serde(default)]
    pub(crate) underline: bool,
    #[serde(default)]
    pub(crate) italic: bool,

    #[serde(default)]
    pub(crate) link: Option<Link>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Link {
    pub(crate) url: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Size {
    pub(crate) magnitude: f32,
    pub(crate) unit: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct ForegroundColor {
    pub(crate) color: Color,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct BackgroundColor {
    pub(crate) color: Color,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct Color {
    #[serde(rename = "rgbColor")]
    pub(crate) rgb_color: RGBColor,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct RGBColor {
    #[serde(default)]
    pub(crate) red: Option<f32>,
    #[serde(default)]
    pub(crate) green: Option<f32>,
    #[serde(default)]
    pub(crate) blue: Option<f32>,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct DocumentParagraphElementText {
    pub(crate) content: String,
    #[serde(default, rename = "textStyle")]
    pub(crate) text_style: Option<DocumentParagraphElementStyle>,
}

pub fn parse(v: &str) -> Result<Document, BoxError> {
    let json = serde_json::from_str::<Document>(v)
        .map_err(|err| format!("Failed to parse document: {err}"))?;
    Ok(json)
}
