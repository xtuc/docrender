use sha2::{Digest, Sha256};

pub mod gdoc;
mod ir;
mod list;
mod markup_generator;
mod parser;
mod render;
mod style;
pub mod syntax_highlighting;

pub use render::RenderingOpts;

pub(crate) type BoxError = Box<dyn std::error::Error + Sync + Send>;

pub(crate) fn hash(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());

    hex::encode(hasher.finalize())
}

pub fn render(
    s: &str,
    options: &RenderingOpts,
    syntax_highlighter: Box<dyn syntax_highlighting::SyntaxHighlighter>,
) -> Result<render::RenderedDocument, BoxError> {
    let gdoc = gdoc::parse(s).map_err(|err| format!("failed to parse document: {err}"))?;

    let mut parser = parser::Parser::new(gdoc);
    let ir = parser.parse();

    render::render(&ir, options, syntax_highlighter)
}
