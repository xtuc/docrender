mod html;
mod jsx;

pub(crate) trait MarkupTagBuilder {
    fn add_attr(&mut self, n: &'static str, v: &str);
    fn add_style(&mut self, n: &'static str, v: &str);
    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>);
    fn add_content(&mut self, v: &str);
    fn finish(&self) -> String;
    fn hash(&self) -> String;
}

pub(crate) trait MarkupGenerator {
    fn start_tag(&self, tag: MarkupTag) -> Box<dyn MarkupTagBuilder>;
    fn add_tag(&mut self, tag: Box<dyn MarkupTagBuilder>);
    fn add_content(&mut self, v: &str);
    fn add_style(&mut self, class_name: &str, style: &str);
    fn finalize(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub(crate) enum MarkupTag {
    Text,

    Heading(u8),
    Paragraph,
    Link,
    Span,
    Div,
    Image,
    NewLine,

    List,
    ListItem,

    Code,

    Table,
    TableRow,
    TableCell,
}

pub(crate) use html::HtmlGenerator;
pub(crate) use jsx::JSXGenerator;
