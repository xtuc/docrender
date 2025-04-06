use super::{MarkupGenerator, MarkupTag, MarkupTagBuilder};
use regex::Regex;

const WRAPPER_START: &str = r#"
import { createElement, Fragment } from "https://unpkg.com/es-react";

function Heading({style, id, level, children}) {
  const tag = "h" + level;
  return createElement(tag, { style, id }, children);
}

function Paragraph({children}) {
  return createElement("p", {}, children);
}

function Code({children}) {
  return createElement("code", {}, children);
}

function Image({src}) {
  return createElement("img", {src});
}

function Link({href}) {
  return createElement("a", {href});
}

function List({children}) {
  return createElement("ul", {}, children);
}

function ListItem({children}) {
  return createElement("li", {}, children);
}

export default function Doc({
  heading = Heading,
  paragraph = Paragraph,
  code = Code,
  image = Image,
  link = Link,
  list = List,
  listItem = ListItem,
} = {}) {
"#;

const WRAPPER_END: &str = r#"
}
"#;

/// Convert the markup tag into a React component or regular HTML tags, when in
/// in quotes.
fn component_name(tag: &MarkupTag) -> &'static str {
    match tag {
        MarkupTag::Heading(_) => "heading",
        MarkupTag::Paragraph => "paragraph",
        MarkupTag::Link => "link",
        MarkupTag::Span => "\"span\"",
        MarkupTag::Div => "\"div\"",
        MarkupTag::Image => "image",
        MarkupTag::List => "list",
        MarkupTag::ListItem => "listItem",
        MarkupTag::NewLine => "br",
        MarkupTag::Table => "table",
        MarkupTag::TableRow => "tableRow",
        MarkupTag::TableCell => "tableCell",
        MarkupTag::Code => "code",

        // No need for a HTML tag or Component to render text.
        MarkupTag::Text => unreachable!(),
    }
}

pub(crate) struct JSXTag {
    tag_name: MarkupTag,
    children: Vec<String>,
    attrs: Vec<(&'static str, String)>,
    style: String,
}

fn css_prop_to_camelcase(prop: &str) -> String {
    let re = Regex::new(r"-([a-z])").unwrap();
    re.replace_all(prop, |caps: &regex::Captures| caps[1].to_uppercase())
        .to_string()
}

impl MarkupTagBuilder for JSXTag {
    fn add_style(&mut self, n: &'static str, v: &str) {
        let n = css_prop_to_camelcase(n);
        self.style += &format!("{n}: \"{v}\",");
    }

    fn add_attr(&mut self, n: &'static str, v: &str) {
        assert_ne!(n, "style");

        self.attrs.push((n, v.to_owned()));
    }

    fn add_content(&mut self, v: &str) {
        let v = v.replace("`", "\\`");
        self.children.push(format!("`{v}`"));
    }

    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>) {
        self.children.push(tag.finish());
    }

    fn hash(&self) -> String {
        if self.tag_name == MarkupTag::Text {
            "todo".to_owned()
        } else {
            unimplemented!("hash a non Text tag")
        }
    }

    fn finish(&self) -> String {
        if self.tag_name == MarkupTag::Text {
            return self.children.join(",");
        }

        let mut js = "".to_owned();

        js += &format!("createElement({}, {{ ", component_name(&self.tag_name));

        if let MarkupTag::Heading(level) = &self.tag_name {
            js += &format!("level: {level}, ");
        }

        if !self.style.is_empty() {
            js += &format!("style: {{ {} }},", self.style);
        }

        for (name, value) in &self.attrs {
            if value != "" {
                js += &format!("{name}: \"{value}\",");
            }
        }

        js += " }, ";

        for child in &self.children {
            js += child;
            js += ",";
        }

        js += ")";
        js
    }
}

pub(crate) struct JSXGenerator {
    children: Vec<String>,
}

impl JSXGenerator {
    pub(crate) fn new() -> Self {
        JSXGenerator { children: vec![] }
    }
}

impl MarkupGenerator for JSXGenerator {
    fn start_tag(&self, tag_name: MarkupTag) -> Box<dyn MarkupTagBuilder> {
        Box::new(JSXTag {
            tag_name,
            children: vec![],
            attrs: vec![],
            style: "".to_owned(),
        })
    }

    fn add_content(&mut self, content: &str) {
        self.children.push(content.to_owned());
    }

    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>) {
        self.children.push(tag.finish());
    }

    fn add_style(&mut self, class_name: &str, style: &str) {}

    fn finalize(&self) -> String {
        let mut js = "".to_owned();
        if self.children.is_empty() {
            return js;
        }

        js += WRAPPER_START;

        js += "return createElement(Fragment, {}, ";

        for child in &self.children {
            js += child;
            js += ",";
        }

        js += ");";

        js += WRAPPER_END;
        js
    }
}
