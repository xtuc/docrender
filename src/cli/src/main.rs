use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}

type BoxError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let args = Args::parse();

    let mut opts = docrender_engine::RenderingOpts::default();
    opts.react = true;

    let res = reqwest::get(args.url).await?;
    if !res.status().is_success() {
        panic!("Document returned {:?}", res.status());
    }

    let input = res.text().await?;

    let syntax_highlighter =
        Box::new(docrender_engine::syntax_highlighting::TestSyntaxHighlighter {});

    println!("input {:?}", input);
    let res = docrender_engine::render(&input, &opts, syntax_highlighter).unwrap();

    println!("{}", res.html);

    Ok(())
}
