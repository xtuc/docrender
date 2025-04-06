use crate::markup_generator::{
    HtmlGenerator, JSXGenerator, MarkupGenerator, MarkupTag, MarkupTagBuilder,
};
use crate::{hash, ir, syntax_highlighting, BoxError};
use std::cell::RefCell;

#[derive(Default)]
pub struct RenderingOpts {
    pub use_taildwind_css: bool,
    pub use_default_css: bool,
    pub inject_prism_css: bool,
    pub images_baseurl: String,
    pub react: bool,
}

pub struct RenderedImage {
    pub source_url: String,
    pub rendered_url: String,
}

pub struct RenderedDocument {
    pub html: String,
    pub images: Vec<RenderedImage>,
    pub table_of_contents_html: String,
}

struct Renderer<'a> {
    options: &'a RenderingOpts,
    doc: &'a ir::Doc,
    syntax_highlighter: Box<dyn syntax_highlighting::SyntaxHighlighter>,
    images: RefCell<Vec<RenderedImage>>,
}

pub(crate) fn render<'a>(
    doc: &'a ir::Doc,
    options: &'a RenderingOpts,
    syntax_highlighter: Box<dyn syntax_highlighting::SyntaxHighlighter>,
) -> Result<RenderedDocument, BoxError> {
    let renderer = Renderer {
        doc,
        options,
        syntax_highlighter,
        images: RefCell::new(vec![]),
    };
    let html = renderer.render_document()?;
    let table_of_contents_html = renderer.render_table_of_contents()?;

    Ok(RenderedDocument {
        html,
        table_of_contents_html,
        images: renderer.images.into_inner(),
    })
}

impl<'a> Renderer<'a> {
    fn markup_generator(&self) -> Box<dyn MarkupGenerator> {
        if self.options.react {
            Box::new(JSXGenerator::new()) as Box<dyn MarkupGenerator>
        } else {
            Box::new(HtmlGenerator::new()) as Box<dyn MarkupGenerator>
        }
    }

    fn render_text(
        &self,
        node: &ir::Text,
        escape_html: bool,
    ) -> Result<Box<dyn MarkupTagBuilder>, BoxError> {
        let markup_gen = self.markup_generator();

        match node {
            ir::Text::Text(s) => {
                let s = if escape_html {
                    html_escape::encode_text(&s).to_string()
                } else {
                    s.to_owned()
                };

                let mut tag = markup_gen.start_tag(MarkupTag::Text);
                tag.add_content(&s);

                Ok(tag)
            }
            ir::Text::TextWithOptions(s, opts) => {
                let s = if escape_html {
                    html_escape::encode_text(&s).to_string()
                } else {
                    s.to_owned()
                };

                let mut tag = markup_gen.start_tag(MarkupTag::Span);
                tag.add_content(&s);

                if let Some(link) = &opts.link {
                    let mut link_tag = markup_gen.start_tag(MarkupTag::Link);
                    link_tag.add_attr("href", link);
                    link_tag.add_content(&s);

                    tag = link_tag
                }

                if !opts.styles.is_empty() {
                    for (k, v) in &opts.styles {
                        tag.add_style(k, v);
                    }
                }

                Ok(tag)
            }
        }
    }

    fn render_node(&self, node: &ir::Node) -> Result<Option<Box<dyn MarkupTagBuilder>>, BoxError> {
        let markup_gen = self.markup_generator();

        Ok(match node {
            ir::Node::Paragraph(text_nodes) => {
                let mut tag = markup_gen.start_tag(MarkupTag::Paragraph);

                for text_node in text_nodes {
                    if let Some(content) = self.render_node(text_node)? {
                        tag.add_tag(content);
                    }
                }

                Some(tag)
            }
            ir::Node::Code(lang, content) => {
                let mut code_tag = markup_gen.start_tag(MarkupTag::Code);
                code_tag.add_attr("class", &format!("language-{lang}"));

                let lang = "js";
                let content = self.syntax_highlighter.highlight(&content, lang).unwrap();
                code_tag.add_content(&content);

                Some(code_tag)
            }
            ir::Node::List(nodes) => {
                let mut list_tag = markup_gen.start_tag(MarkupTag::List);

                for node in nodes {
                    let mut list_item_tag = markup_gen.start_tag(MarkupTag::ListItem);
                    if let Some(content) = self.render_node(node)? {
                        list_item_tag.add_tag(content);
                    }

                    list_tag.add_tag(list_item_tag);
                }

                Some(list_tag)
            }
            ir::Node::Title(node, styles) => {
                let mut title_tag = markup_gen.start_tag(MarkupTag::Heading(node.level));

                let content = self.render_text(&node.text, true)?;

                for (k, v) in styles {
                    title_tag.add_style(k, v);
                }

                // Support for ToC
                title_tag.add_attr("id", &format!("pb-{}", content.hash()));

                title_tag.add_tag(content);

                Some(title_tag)
            }

            ir::Node::InlineText(text_nodes, has_newline) => {
                let mut text_tag = markup_gen.start_tag(MarkupTag::Text);

                for text_node in text_nodes {
                    let content = self.render_text(text_node, true)?;
                    text_tag.add_tag(content);

                    if *has_newline {
                        text_tag.add_content("\n");
                    }
                }

                Some(text_tag)
            }

            ir::Node::Table(table) => {
                let mut table_tag = markup_gen.start_tag(MarkupTag::Table);

                for row in &table.rows {
                    let mut table_row_tag = markup_gen.start_tag(MarkupTag::TableRow);

                    for cell in &row.cells {
                        let mut table_cell_tag = markup_gen.start_tag(MarkupTag::TableCell);
                        if let Some(content) = self.render_node(cell)? {
                            table_cell_tag.add_tag(content);
                        }

                        table_row_tag.add_tag(table_cell_tag);
                    }

                    table_tag.add_tag(table_row_tag);
                }

                Some(table_tag)
            }

            ir::Node::Image(url, (width, height), styles) => {
                let mut image_tag = markup_gen.start_tag(MarkupTag::Image);

                let rendered_url = format!("{}{}", self.options.images_baseurl, hash(url));

                self.images.borrow_mut().push(RenderedImage {
                    source_url: url.to_owned(),
                    rendered_url: rendered_url.clone(),
                });
                image_tag.add_attr("src", &rendered_url);

                image_tag.add_attr("height", height);
                image_tag.add_attr("width", width);

                for (k, v) in styles {
                    image_tag.add_style(k, v);
                }

                Some(image_tag)
            }

            ir::Node::Clear => None,
        })
    }

    fn render_style(
        &self,
        class_name: &str,
        properties: &[(&'static str, String)],
    ) -> Result<String, BoxError> {
        let mut markup_gen = self.markup_generator();

        let style = properties
            .iter()
            .map(|(k, v)| format!("{k}:{v}"))
            .collect::<Vec<String>>()
            .join(";");
        markup_gen.add_style(&class_name, &style);

        Ok(markup_gen.finalize())
    }

    fn render_document(&self) -> Result<String, BoxError> {
        let mut markup_gen = self.markup_generator();
        let mut out = "".to_owned();

        for (class_name, properties) in &self.doc.styles {
            out += &self.render_style(&class_name, properties)?;
        }

        for node in &self.doc.content {
            if let Some(tag) = self.render_node(&node)? {
                markup_gen.add_tag(tag);
            }
        }

        Ok(out + &markup_gen.finalize())
    }

    fn render_table_of_contents(&self) -> Result<String, BoxError> {
        let mut markup_gen = self.markup_generator();

        markup_gen.add_style("pb-h1", "padding-left: 0px");
        markup_gen.add_style("pb-h2", "padding-left: 40px");
        markup_gen.add_style("pb-h3", "padding-left: 80px");
        markup_gen.add_style("pb-h4", "padding-left: 180px");

        for node in &self.doc.content {
            match node {
                ir::Node::Title(node, _styles) => {
                    if node.level == 1 {
                        // Don't include the doc's title in the TOC
                        continue;
                    }

                    let level = node.level - 1;

                    let mut a_tag = markup_gen.start_tag(MarkupTag::Link);

                    let content = self.render_text(&node.text, true)?;

                    a_tag.add_attr("href", &format!("#pb-{}", content.hash()));

                    a_tag.add_tag(content);

                    let mut div_tag = markup_gen.start_tag(MarkupTag::Div);
                    div_tag.add_attr("class", &format!("pb-h{}", level));
                    div_tag.add_tag(a_tag);

                    markup_gen.add_tag(div_tag);
                }
                _ => {}
            }
        }

        Ok(markup_gen.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gdoc, parser};
    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::process::Command;
    use std::process::Stdio;

    fn assert_expected(actual: &str, filename: &str) {
        let parser = if filename.ends_with(".jsx") {
            "babel"
        } else {
            "html"
        };
        let filename = Path::new(filename);
        let actual = prettier(actual, parser);

        if filename.is_file() {
            let contents = fs::read_to_string(filename).expect("missing file");
            pretty_assertions::assert_eq!(actual, contents);
        } else {
            let gen_expected = env::var("GEN_EXPECTED").is_ok();

            if gen_expected {
                eprintln!("creating {filename:?}");

                let mut file = File::create(filename).unwrap();
                file.write_all(actual.as_bytes()).unwrap();
            } else {
                panic!("expected file {filename:?} missing");
            }
        }
    }

    fn prettier(content: &str, parser: &str) -> String {
        let mut prettier = Command::new("prettier")
            .arg("--parser")
            .arg(parser)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start prettier");

        // Get the stdin of the subprocess and write the input data
        {
            let stdin = prettier.stdin.as_mut().expect("Failed to open stdin");
            stdin
                .write_all(content.as_bytes())
                .expect("Failed to write to stdin");
        }

        // Read the output from the subprocess
        let output = prettier.wait_with_output().expect("Failed to read stdout");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    fn doc(json: &str) -> ir::Doc {
        let json = serde_json::from_str::<gdoc::Document>(json).unwrap();
        let mut parser = parser::Parser::new(json);
        parser.parse()
    }

    macro_rules! test_expected {
        ( $name:ident, $file:expr ) => {
            paste::paste! {
                #[test]
                fn [< $name _html >]() {
                    let opts = RenderingOpts::default();
                    let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

                    let input_filename = format!("./test-data/actual/{}.json", $file);
                    let expected_filename = format!("./test-data/expected/{}.html", $file);

                    let ir = doc(&fs::read_to_string(&input_filename)
                        .expect(&format!("missing file {input_filename}")));

                    let res = render(&ir, &opts, syntax_highlighter).unwrap();
                    assert_expected(&res.html, &expected_filename);
                }

                #[test]
                fn [< $name _react >]() {
                    let mut opts = RenderingOpts::default();
                    opts.react = true;

                    let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

                    let input_filename = format!("./test-data/actual/{}.json", $file);
                    let expected_filename = format!("./test-data/expected/{}.jsx", $file);

                    let ir = doc(&fs::read_to_string(&input_filename)
                        .expect(&format!("missing file {input_filename}")));

                    let res = render(&ir, &opts, syntax_highlighter).unwrap();

                    assert_expected(&res.html, &expected_filename);
                }
            }
        };
    }

    #[test]
    fn test_render_links() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/links.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();
        assert_expected(&res.html, "./test-data/expected/links.html");
    }

    #[test]
    fn test_render_text_style() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/text-style.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/text-style.html");
    }

    #[test]
    fn test_render_headings() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/headings.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/headings.html");
    }

    #[test]
    fn test_render_headings_table_of_contents() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/headings.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();

        assert_expected(
            &res.table_of_contents_html,
            "./test-data/expected/headings-table-of-contents.html",
        );
    }

    #[test]
    fn test_render_lists() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/lists.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/lists.html");
    }

    #[test]
    fn test_render_images() {
        let opts = RenderingOpts {
            use_taildwind_css: false,
            inject_prism_css: false,
            use_default_css: false,
            react: false,
            images_baseurl: "https://test.com/i/".to_owned(),
        };
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let doc = doc(include_str!("../test-data/actual/images.json"));
        let res = render(&doc, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/images.html");

        assert_eq!(res.images.len(), 5);

        assert_eq!(res.images[0].source_url, "https://lh7-rt.googleusercontent.com/docsz/AD_4nXdQWBfwce4u73xbSlFK1Cxno-5O8_QMCUXuyGdOBHUV6d33o45oCBW-5sUaq0T6gZpFF98l4v4snJGLM95_RnXmIqxr4i8qh5bQZI52pWc2-hoNy-ZYmvCGxLDQ7I0tFnFv-uRPZ4l0AK95dune4cakn_-M?key=L81KPS631quh92g5K29tLA");
        assert_eq!(
            res.images[0].rendered_url,
            "https://test.com/i/c0b01de66a16cbbf4e1d43883e4f66fd26bd1015b6bebf37364742f7a9de7440"
        );
    }

    #[test]
    fn test_render_paragraph_html() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let ir = doc(include_str!("../test-data/actual/paragraph.json"));
        let res = render(&ir, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/paragraph.html");
    }

    #[test]
    fn test_render_paragraph_react() {
        let mut opts = RenderingOpts::default();
        opts.react = true;
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let ir = doc(include_str!("../test-data/actual/paragraph.json"));
        let res = render(&ir, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/paragraph.jsx");
    }

    #[test]
    fn test_render_demo_html() {
        let opts = RenderingOpts::default();
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let ir = doc(include_str!("../test-data/actual/demo.json"));
        let res = render(&ir, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/demo.html");
    }

    #[test]
    fn test_render_demo_react() {
        let mut opts = RenderingOpts::default();
        opts.react = true;
        let syntax_highlighter = Box::new(syntax_highlighting::TestSyntaxHighlighter {});

        let ir = doc(include_str!("../test-data/actual/demo.json"));
        let res = render(&ir, &opts, syntax_highlighter).unwrap();

        assert_expected(&res.html, "./test-data/expected/demo.jsx");
    }

    test_expected!(
        test_render_sauleau_disassemble_a_go_binary,
        "sauleau.com/disassemble-a-go-binary"
    );
    test_expected!(
        test_render_sauleau_lambda_calculus_functional_condition,
        "sauleau.com/Lambda-Calculus-Functional-condition"
    );
    test_expected!(
        test_render_sauleau_the_pipeline_operator_is_already_available_in_JavaScript,
        "sauleau.com/the-pipeline-operator-is-already-available-in-JavaScript"
    );
    test_expected!(
        test_render_sauleau_SNI_support_in_SMTP,
        "sauleau.com/SNI-support-in-SMTP"
    );
    test_expected!(
        test_render_politique_de_gestion_des_donnees_personnelles,
        "politique-de-gestion-des-donnees-personnelles"
    );
}
