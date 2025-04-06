use crate::{gdoc, ir, list};
use std::cell::RefCell;
use std::collections::HashMap;

const NORMAL_LINE_SPACING: f32 = 1.5;

pub(crate) struct Parser {
    doc: gdoc::Document,
    cursor: usize,

    // states
    in_list: RefCell<Option<String>>,

    nodes: RefCell<Vec<ir::Node>>,
}

fn parse_new_line(v: &str) -> (ir::HasNewLine, String) {
    if let Some(v) = v.strip_suffix("\n") {
        (true, v.to_owned())
    } else {
        (false, v.to_owned())
    }
}

fn parse_document_paragraph_style(data: &gdoc::DocumentParagraphStyle) -> ir::Styles {
    let mut styles = vec![];

    if let Some(alignment) = &data.alignment {
        match alignment.as_str() {
            "CENTER" => {
                styles.push(("text-align", "center".to_owned()));
                styles.push(("margin-left", "auto".to_owned()));
                styles.push(("margin-right", "auto".to_owned()));
            }
            "END" => {
                styles.push(("text-align", "right".to_owned()));
            }
            "START" => { /* aligning left is by default */ }
            v => eprintln!("alignment {v} not supported"),
        }
    }

    styles
}

fn parse_text_style(text_style: &gdoc::DocumentParagraphElementStyle) -> (bool, ir::TextOptions) {
    let mut text_options = ir::TextOptions::default();
    let mut has_text_options = false;

    if let Some(link) = &text_style.link {
        text_options.link = Some(link.url.clone());
        has_text_options = true;
    }

    if text_style.underline {
        text_options
            .styles
            .push(("text-decoration", "underline".to_owned()));
        has_text_options = true;
    }

    if let Some(foreground_color) = &text_style.foreground_color {
        let rgb = format!(
            "rgb({}%, {}%, {}%)",
            foreground_color.color.rgb_color.red.unwrap_or(0.0) * 100.0,
            foreground_color.color.rgb_color.green.unwrap_or(0.0) * 100.0,
            foreground_color.color.rgb_color.blue.unwrap_or(0.0) * 100.0
        );

        text_options.styles.push(("color", rgb));
        has_text_options = true;
    }

    if let Some(background_color) = &text_style.background_color {
        let rgb = format!(
            "rgb({}%, {}%, {}%)",
            background_color.color.rgb_color.red.unwrap_or(0.0) * 100.0,
            background_color.color.rgb_color.green.unwrap_or(0.0) * 100.0,
            background_color.color.rgb_color.blue.unwrap_or(0.0) * 100.0
        );
        text_options.styles.push(("background-color", rgb));
        has_text_options = true;
    }

    if text_style.bold {
        text_options.styles.push(("font-weight", "bold".to_owned()));
        has_text_options = true;
    }

    if text_style.italic {
        text_options
            .styles
            .push(("font-style", "italic".to_owned()));
        has_text_options = true;
    }

    if let Some(font_size) = &text_style.font_size {
        let value = format!("{}{}", font_size.magnitude, font_size.unit);
        text_options.styles.push(("font-size", value));
        has_text_options = true;
    }

    (has_text_options, text_options)
}

impl Parser {
    pub(crate) fn new(doc: gdoc::Document) -> Parser {
        Parser {
            doc,
            cursor: 0,
            in_list: RefCell::new(None),
            nodes: RefCell::new(vec![ir::Node::Clear]),
        }
    }

    pub(crate) fn parse(&mut self) -> ir::Doc {
        let mut styles = HashMap::new();

        for style in &self.doc.named_styles.styles {
            let mut properties = vec![];

            if let Some(line_spacing) = style.paragraph_style.line_spacing {
                // https://developers.google.com/docs/api/reference/rest/v1/documents#ParagraphStyle
                // ```
                // The amount of space between lines, as a percentage of normal,
                // where normal is represented as 100.0. If unset, the value
                // is inherited from the parent.
                // ```
                let line_spacing = line_spacing / 100.0 * NORMAL_LINE_SPACING;
                properties.push(("line-height", line_spacing.to_string()));
            }

            if !properties.is_empty() {
                let class_name = format!("pb{}", &style.named_style_type);
                styles.insert(class_name, properties);
            }
        }

        for _ in 0..self.doc.body.content.len() {
            let content = &self.doc.body.content[self.cursor];

            if let Some(paragraph) = &content.paragraph {
                let named_style_type = &paragraph.paragraph_style.named_style_type;
                self.parse_paragraph(paragraph, named_style_type);
            }

            if let Some(table) = &content.table {
                self.parse_table(table);
            }

            self.cursor += 1;
        }

        ir::Doc {
            content: self.nodes.clone().into_inner(),
            styles,
        }
    }

    fn is_last_node_codeblock(&self) -> bool {
        let nodes = self.nodes.borrow();
        nodes
            .last()
            .map(|n| matches!(n, ir::Node::Code(_, _)))
            .unwrap_or(false)
    }

    fn is_last_clear(&self) -> bool {
        let nodes = self.nodes.borrow();
        nodes
            .last()
            .map(|n| matches!(n, ir::Node::Clear))
            .unwrap_or(false)
    }

    fn parse_table(&self, data: &gdoc::DocumentTable) {
        let mut table_node = ir::Table::default();

        for row in &data.table_rows {
            let mut row_node = ir::TableRow::default();

            for cell in &row.table_cells {
                let mut text_nodes = vec![];

                for content in &cell.content {
                    if let Some(paragraph) = &content.paragraph {
                        let paragraph_styles =
                            parse_document_paragraph_style(&paragraph.paragraph_style);

                        // TODO: share logic with parse_paragraph
                        // self.parse_paragraph(paragraph, style);

                        for element in &paragraph.elements {
                            if let Some(text_run) = &element.text_run {
                                let content = text_run.content.clone();
                                let (_has_newline, content) = parse_new_line(&content);

                                if content.is_empty() {
                                    // The table cell has ended.
                                    // Push the text nodes we have so far and start new.
                                    row_node
                                        .cells
                                        .push(ir::Node::InlineText(text_nodes.clone(), false));
                                    text_nodes.clear();
                                    continue;
                                }

                                let (mut has_text_options, mut text_options) =
                                    if let Some(text_style) = &text_run.text_style {
                                        parse_text_style(text_style)
                                    } else {
                                        (false, ir::TextOptions::default())
                                    };

                                if !paragraph_styles.is_empty() {
                                    has_text_options = true;

                                    // extend text node style with parent
                                    // paragraph styles
                                    text_options.styles.extend(paragraph_styles.clone());
                                }

                                let text_node = if has_text_options {
                                    ir::Text::new_with_options(content.to_owned(), text_options)
                                } else {
                                    ir::Text::new(content.to_owned())
                                };

                                text_nodes.push(text_node);
                            }
                        }
                    }
                }

                if !text_nodes.is_empty() {
                    // We reached the end of the elements and we still have
                    // buffered text_nodes, flush them.
                    row_node
                        .cells
                        .push(ir::Node::InlineText(text_nodes.clone(), false));
                }
            }

            table_node.rows.push(row_node);
        }

        self.nodes.borrow_mut().push(ir::Node::Table(table_node));
        self.nodes.borrow_mut().push(ir::Node::Clear);
    }

    fn parse_paragraph(&self, data: &gdoc::DocumentParagraph, named_style_type: &str) {
        if let Some(_heading_id) = &data.paragraph_style.heading_id {
            assert_eq!(data.elements.len(), 1);

            let text = data.elements[0].text_run.as_ref().unwrap().content.clone();
            let (has_newline, text) = parse_new_line(&text);

            let level = match named_style_type {
                "TITLE" => 1,
                "HEADING_1" => 2,
                "HEADING_2" => 3,
                "HEADING_3" => 4,
                "HEADING_4" => 5,
                "HEADING_5" => 6,
                _ => 0,
            };

            let paragraph_styles = parse_document_paragraph_style(&data.paragraph_style);
            let text_node = ir::Text::new(text);

            if level > 0 {
                self.nodes.borrow_mut().push(ir::Node::Title(
                    ir::Title {
                        text: text_node,
                        level,
                    },
                    paragraph_styles,
                ));
                self.nodes.borrow_mut().push(ir::Node::Clear);
            } else {
                self.nodes
                    .borrow_mut()
                    .push(ir::Node::Paragraph(vec![ir::Node::InlineText(
                        vec![text_node],
                        has_newline,
                    )]));
                self.nodes.borrow_mut().push(ir::Node::Clear);
            }
        } else if let Some(bullet) = &data.bullet {
            let list = self
                .doc
                .lists
                .get(&bullet.list_id)
                .ok_or("missing list")
                .unwrap(); // FIXME: remove unwrap
            let properties = &list.list_properties.nesting_levels[bullet.nesting_level];

            let mut in_list = self.in_list.borrow_mut();
            if in_list.is_none() {
                // We don't have a list yet, start one.
                *in_list = Some(bullet.list_id.clone());
                self.nodes.borrow_mut().push(ir::Node::Clear);
            }

            if let Some(in_list) = in_list.as_mut() {
                if *in_list != bullet.list_id {
                    // It's a list but a different one
                    *in_list = bullet.list_id.clone();
                    self.nodes.borrow_mut().push(ir::Node::Clear);
                }
            }

            let mut text_nodes = vec![];

            for element in &data.elements {
                if let Some(text_run) = &element.text_run {
                    if text_run.content.trim().is_empty() {
                        continue;
                    }

                    let (has_text_options, text_options) =
                        if let Some(text_style) = &text_run.text_style {
                            parse_text_style(text_style)
                        } else {
                            (false, ir::TextOptions::default())
                        };

                    let (has_newline, text) = parse_new_line(&text_run.content);

                    let text = if has_text_options {
                        ir::Text::new_with_options(text, text_options)
                    } else {
                        ir::Text::new(text)
                    };

                    text_nodes.push(text);

                    if has_newline {
                        let mut nodes = self.nodes.borrow_mut();
                        let last_node = nodes.last_mut();

                        if let Some(ir::Node::List(list_text_nodes)) = last_node {
                            list_text_nodes
                                .push(ir::Node::InlineText(text_nodes.clone(), has_newline));
                        } else {
                            nodes.push(ir::Node::List(vec![ir::Node::InlineText(
                                text_nodes.clone(),
                                has_newline,
                            )]));
                        }

                        text_nodes.clear();
                    }
                }
            }

            if !text_nodes.is_empty() {
                let mut nodes = self.nodes.borrow_mut();
                let last_node = nodes.last_mut();

                let has_newline = false;

                if let Some(ir::Node::List(list_text_nodes)) = last_node {
                    list_text_nodes.push(ir::Node::InlineText(text_nodes.clone(), has_newline));
                } else {
                    nodes.push(ir::Node::List(vec![ir::Node::InlineText(
                        text_nodes.clone(),
                        has_newline,
                    )]));
                }

                text_nodes.clear();
            }
        } else {
            let paragraph_styles = parse_document_paragraph_style(&data.paragraph_style);

            {
                // If we were generating a list but there's no bullet
                // configuration anymore it means we should stop.
                let mut in_list = self.in_list.borrow_mut();
                if in_list.is_some() {
                    *in_list = None;
                    self.nodes.borrow_mut().push(ir::Node::Clear);
                }
            }

            for element in &data.elements {
                // Start a block of text
                if let Some(text_run) = &element.text_run {
                    if !self.is_last_node_codeblock() && text_run.content.trim().is_empty() {
                        self.nodes.borrow_mut().push(ir::Node::Paragraph(vec![]));
                        continue;
                    }

                    // Codeblocks
                    {
                        if !self.is_last_node_codeblock() {
                            if let Some(lang) = text_run.content.trim().strip_prefix("```") {
                                let lang = match &*lang.to_lowercase() {
                                    "javascript" => "js".to_owned(),
                                    l => l.to_owned(),
                                };

                                // Start a Code node
                                self.nodes
                                    .borrow_mut()
                                    .push(ir::Node::Code(lang, "".to_owned()));
                                continue;
                            }
                        }

                        if self.is_last_node_codeblock() && text_run.content.trim() == "```" {
                            // terminate the current code block
                            self.nodes.borrow_mut().push(ir::Node::Clear);
                            continue;
                        }
                    }

                    // Start a paragraph
                    if self.is_last_clear() {
                        self.nodes.borrow_mut().push(ir::Node::Paragraph(vec![]));
                    }

                    let (has_newline, text) = parse_new_line(&text_run.content);

                    // Add content to the current node
                    {
                        let mut nodes = self.nodes.borrow_mut();
                        let last_node = nodes.last_mut();

                        if let Some(ir::Node::Paragraph(text_nodes)) = last_node {
                            let (has_text_options, text_options) =
                                if let Some(text_style) = &text_run.text_style {
                                    parse_text_style(text_style)
                                } else {
                                    (false, ir::TextOptions::default())
                                };

                            let text = if has_text_options {
                                ir::Text::new_with_options(text, text_options)
                            } else {
                                ir::Text::new(text)
                            };

                            text_nodes.push(ir::Node::InlineText(vec![text], has_newline));
                        }

                        if let Some(ir::Node::Code(_lang, s)) = last_node {
                            *s = s.clone() + &text_run.content.clone();
                        }
                    }

                    if !self.is_last_node_codeblock() && has_newline {
                        // Close the current paragraph to start a new one
                        self.nodes.borrow_mut().push(ir::Node::Clear);
                    }
                }

                if let Some(person) = &element.person {
                    let mailto = format!("mailto:{}", person.person_properties.email);

                    let mut text_options = ir::TextOptions::default();
                    text_options.link = Some(mailto);

                    self.nodes
                        .borrow_mut()
                        .push(ir::Node::Paragraph(vec![ir::Node::InlineText(
                            vec![ir::Text::new_with_options(
                                person.person_properties.name.clone(),
                                text_options,
                            )],
                            false,
                        )]));
                    self.nodes.borrow_mut().push(ir::Node::Clear);
                }

                if let Some(inline_object_element) = &element.inline_object_element {
                    if let Some(inline_object) = self
                        .doc
                        .inline_objects
                        .get(&inline_object_element.inline_object_id)
                    {
                        let object = &inline_object.inline_object_properties.embedded_object;
                        let url = &object.image_properties.content_uri;

                        // FIXME: In theory, we should convert pt to px. gdoc gives us
                        // pt.
                        let height = format!("{}", object.size.height.magnitude.floor() as i32);
                        let width = format!("{}", object.size.width.magnitude.floor() as i32);

                        let image_node = ir::Node::Image(
                            url.to_owned(),
                            (width, height),
                            paragraph_styles.clone(),
                        );

                        let mut nodes = self.nodes.borrow_mut();
                        let last_node = nodes.last_mut();

                        if let Some(ir::Node::Paragraph(text_nodes)) = last_node {
                            // Previous node is a paragraph, just append the image
                            // to it.
                            text_nodes.push(image_node);
                        } else if let Some(ir::Node::Clear) = last_node {
                            // Nothing was before the image, inject as a paragraph.
                            nodes.push(ir::Node::Paragraph(vec![image_node]));
                            nodes.push(ir::Node::Clear);
                        } else {
                            unimplemented!("image preceded by unsupported node: {last_node:?}");
                        }
                    } else {
                        eprintln!(
                            "inline object {} doesn't exists",
                            inline_object_element.inline_object_id
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    macro_rules! t {
        ($name:ident, $file:expr) => {
            #[test]
            fn $name() {
                let actual = include_str!(concat!("../test-data/actual/", $file, ".json"));
                let actual = serde_json::from_str::<gdoc::Document>(actual).unwrap();

                let mut p = Parser::new(actual);
                let out = p.parse();

                let ron =
                    ron::ser::to_string_pretty(&out, ron::ser::PrettyConfig::default()).unwrap();

                if env::var("OVERWRITE").is_ok() {
                    let mut file =
                        File::create(format!("./test-data/expected/{}.ir", $file)).unwrap();
                    file.write_all(&ron.as_bytes()).unwrap();
                } else {
                    let expected = include_str!(concat!("../test-data/expected/", $file, ".ir"));
                    pretty_assertions::assert_eq!(ron, expected.trim());
                }
            }
        };
    }

    t!(
        test_parse_sauleau_com_disassemble_a_go_binary,
        "sauleau.com/disassemble-a-go-binary"
    );
    t!(
        test_parse_lambda_calculus_functional_condition,
        "sauleau.com/Lambda-Calculus-Functional-condition"
    );
    t!(test_parse_links, "links");
    t!(test_parse_text_style, "text-style");
    t!(
        test_parse_sauleau_SNI_support_in_SMTP,
        "sauleau.com/SNI-support-in-SMTP"
    );
    t!(test_parse_images, "images");
    t!(
        test_parse_sauleau_the_pipeline_operator_is_already_available_in_JavaScript,
        "sauleau.com/the-pipeline-operator-is-already-available-in-JavaScript"
    );
    t!(test_parse_demo, "demo");
    t!(test_parse_lists, "lists");
    t!(test_parse_headings, "headings");
    t!(
        test_parse_politique_de_gestion_des_donnees_personnelles,
        "politique-de-gestion-des-donnees-personnelles"
    );
}
