use super::{MarkupGenerator, MarkupTag, MarkupTagBuilder};

// Void tags are self-closing
const VOID_TAG: &[MarkupTag] = &[MarkupTag::Image, MarkupTag::NewLine];

const CODE_STYLE: &str = r#"
<style>
script[visible],
style[visible],
pre {
    display: block;
    white-space: pre;
    border: 1px solid #dedede;
    padding: 1em;
    background: #fafafa;
    overflow-x: auto;
    border-left: 0.4em solid cornflowerblue;
    tab-size: 2;
    color: #1a1a1a;
    line-height: 1.6;
}

code:not(pre code), /* output:not(code:has(output) output) */ {
    background: #f7f7f7;
    border: 1px solid rgb(0 0 0 / 0.2);
    padding: 0.1rem 0.3rem;
    margin: 0.1rem 0;
    border-radius: 0.2rem;
    display: inline-block;
}
</style>
"#;

fn tag_name(tag: &MarkupTag) -> String {
    match tag {
        MarkupTag::Heading(l) => format!("h{l}"),
        MarkupTag::Paragraph => "p".to_owned(),
        MarkupTag::Link => "a".to_owned(),
        MarkupTag::Span => "span".to_owned(),
        MarkupTag::Div => "div".to_owned(),
        MarkupTag::Image => "img".to_owned(),
        MarkupTag::List => "ul".to_owned(),
        MarkupTag::ListItem => "li".to_owned(),
        MarkupTag::NewLine => "br".to_owned(),
        MarkupTag::Table => "table".to_owned(),
        MarkupTag::TableRow => "tr".to_owned(),
        MarkupTag::TableCell => "td".to_owned(),
        MarkupTag::Code => "code".to_owned(),

        // No need for a HTML tag to render text.
        MarkupTag::Text => unreachable!(),
    }
}

pub(crate) struct HtmlTag {
    content: String,
    tag_name: MarkupTag,
    attrs: Vec<(&'static str, String)>,
    style: String,
}

impl MarkupTagBuilder for HtmlTag {
    fn add_style(&mut self, n: &'static str, v: &str) {
        self.style += &format!("{n}:{v};")
    }

    fn add_attr(&mut self, n: &'static str, v: &str) {
        assert_ne!(n, "style");
        self.attrs.push((n, v.to_owned()));
    }

    fn add_content(&mut self, v: &str) {
        self.content += v;
    }

    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>) {
        self.content += &tag.finish();
    }

    fn hash(&self) -> String {
        if self.tag_name == MarkupTag::Text {
            return urlencoding::encode(&self.content.trim()).to_string();
        } else {
            unimplemented!("hash a non Text tag")
        }
    }

    fn finish(&self) -> String {
        if self.tag_name == MarkupTag::Text {
            return self.content.clone();
        }

        if matches!(self.tag_name, MarkupTag::Heading(_)) && self.content.trim().is_empty() {
            return "".to_owned();
        }
        if self.tag_name == MarkupTag::Paragraph && self.content.trim().is_empty() {
            return "".to_owned();
        }

        let mut html = "".to_owned();
        let tag_name = tag_name(&self.tag_name);

        if self.tag_name == MarkupTag::Code {
            html += &format!("<pre>");
        }

        html += &format!("<{tag_name}");

        if !self.style.is_empty() {
            html += &format!(" style=\"{}\" ", self.style);
        }

        for (name, value) in &self.attrs {
            if value != "" {
                html += &format!(" {name}=\"{value}\"");
            }
        }

        if VOID_TAG.contains(&self.tag_name) {
            assert!(self.content.is_empty());
            html += &format!("/>");
        } else {
            html += &format!(">");
            html += &self.content;
            html += &format!("</{tag_name}>");

            if self.tag_name == MarkupTag::Code {
                html += &format!("</pre>");
                html += CODE_STYLE;
            }
        }

        html
    }
}

pub(crate) struct HtmlGenerator {
    html: String,
    style: String,
}

impl HtmlGenerator {
    pub(crate) fn new() -> Self {
        HtmlGenerator {
            style: "".to_owned(),
            html: "".to_owned(),
        }
    }
}

impl MarkupGenerator for HtmlGenerator {
    fn start_tag(&self, tag_name: MarkupTag) -> Box<dyn MarkupTagBuilder> {
        Box::new(HtmlTag {
            tag_name,
            content: "".to_owned(),
            style: "".to_owned(),
            attrs: vec![],
        })
    }

    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>) {
        self.html += &tag.finish();
    }

    fn add_content(&mut self, content: &str) {
        self.html += content;
    }

    fn add_style(&mut self, class_name: &str, style: &str) {
        self.style += &format!(".{class_name} {{ {style} }}\n");
    }

    fn finalize(&self) -> String {
        if self.style.is_empty() {
            self.html.clone()
        } else {
            format!("<style>\n{}</style>\n", self.style) + &self.html
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_tag() {
        let mut gen = HtmlGenerator::new();
        let mut tag = gen.start_tag(MarkupTag::Span);

        tag.add_attr("a", "b");
        tag.add_attr("c", "d");
        tag.add_attr("empty", "");
        tag.add_content("content here");

        gen.add_tag(tag);
        gen.add_style("bli", "padding-left: 123px");

        assert_eq!(gen.finalize(), "<style>\n.bli { padding-left: 123px }\n</style>\n<span a=\"b\" c=\"d\">content here</span>");
    }
}
