type BoxError = Box<dyn std::error::Error + Sync + Send>;

pub trait SyntaxHighlighter {
    fn highlight(&self, text: &str, language: &str) -> Result<String, BoxError>;
}

// TODO: create bindings to prismjs library instead of using JS glue

mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(raw_module = "../../../docrender/src/prism.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub fn highlight(text: &str, language: &str) -> Result<String, JsValue>;
    }
}

pub struct JsSyntaxHighlighter {}
impl SyntaxHighlighter for JsSyntaxHighlighter {
    fn highlight(&self, text: &str, language: &str) -> Result<String, BoxError> {
        js::highlight(text, language)
            .map_err(|err| format!("failed to run syntax highlighting: {err:?}").into())
    }
}

pub struct TestSyntaxHighlighter {}
impl SyntaxHighlighter for TestSyntaxHighlighter {
    fn highlight(&self, text: &str, language: &str) -> Result<String, BoxError> {
        let tag = format!("codeblock-{language}");
        Ok(format!("<{tag}>{text}</{tag}>"))
    }
}
