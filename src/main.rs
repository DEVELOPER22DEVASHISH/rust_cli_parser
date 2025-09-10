use clap::Parser;
use maud::{html, Markup, DOCTYPE};
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    /// input markdown file path
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// output HTML file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    let page = html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Markdown to HTML" }
                style { r#"
                    body {
                        font-family: Arial, sans-serif;
                        margin: 2em;
                        line-height: 1.6;
                    }
                    h1, h2, h3, h4, h5, h6 {
                        color: #333;
                    }
                    pre {
                        background-color: #f4f4f4;
                        padding: 1em;
                        overflow-x: auto;
                    }
                    code {
                        background-color: #f4f4f4;
                        padding: 0.2em 0.4em;
                        border-radius: 3px;
                    }
                    a {
                        color: #0366d6;
                        text-decoration: none;
                    }
                    a:hover {
                        text-decoration: underline;
                    }
                "# }
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    };
    page
}

fn main() {
    let args = Args::parse();
    // let markdown_input = fs::read_to_string(&args.input).expect("Failed to read input file");
    let markdown_input = match args.input {
        Some(path) => fs::read_to_string(path).expect("Failed to read input file"),
        None => panic!("Input file path is required"),
    };
//     let markdown_input = match args.input {
//     Some(path) => match fs::read_to_string(path) {
//         Ok(content) => content,
//         Err(err) => panic!("Failed to read input file: {}", err),
//     },
//     None => {
//         println!("Error: Input file path is required");
//         std::process::exit(1);
//     }
// };
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let  full_html_output = render_html_page(&html_output).into_string();

    match &args.output {
        Some(path) => {
            fs::write(path, full_html_output).expect("Failed to write output file");
        }
        None => {
            println!("Path not ");
        }
    }
}
