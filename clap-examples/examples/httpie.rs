use std::collections::HashMap;

use anyhow::Result;
use clap::{command, Parser};
use colored::*;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::{Response, StatusCode, Url};
use std::fmt::Write as _;
use std::io::Write as _;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    run: Run,
}

#[derive(Debug, Parser)]
enum Run {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Parser)]
struct Get {
    #[arg(value_parser = parser_url)]
    url: Url,
}

#[derive(Debug, Parser)]
struct Post {
    #[arg(value_parser = parser_url)]
    url: Url,

    /// `key1=value1&key2=value2`
    #[arg(value_parser = parser_body)]
    body: Option<HashMap<String, String>>,
}

fn parser_url(s: &str) -> Result<Url> {
    Url::parse(s).map_err(Into::into)
}

fn parser_body(s: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for pair in s.split('&') {
        let mut iter = pair.split('=');
        let key = iter.next().unwrap();
        let value = iter.next().unwrap();
        map.insert(key.to_string(), value.to_string());
    }

    Ok(map)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.run {
        Run::Get(get) => handle_get(get).await?,
        Run::Post(post) => handle_post(post).await?,
    }

    Ok(())
}

async fn handle_get(get: Get) -> Result<()> {
    let res = reqwest::get(get.url).await.unwrap();

    print_status_line(&res);
    print_headers(&res);

    let headers = res.headers().clone();
    let body = res.text().await?;

    print_body(&headers, body);

    Ok(())
}

async fn handle_post(post: Post) -> Result<()> {
    let client = reqwest::ClientBuilder::new().build()?;
    let res = client.post(post.url).json(&post.body).send().await?;

    print_status_line(&res);
    print_headers(&res);

    let headers = res.headers().clone();
    let body = res.text().await?;

    print_body(&headers, body);

    Ok(())
}

fn print_status_line(res: &Response) {
    match (res.status(), res.version()) {
        (StatusCode::OK, version) => {
            println!("{}", format!("{:?} {}", version, StatusCode::OK).green())
        }
        (StatusCode::NOT_FOUND, version) => {
            println!(
                "{}",
                format!("{:?} {}", version, StatusCode::NOT_FOUND).red()
            )
        }
        (StatusCode::INTERNAL_SERVER_ERROR, version) => {
            println!(
                "{}",
                format!("{:?} {}", version, StatusCode::INTERNAL_SERVER_ERROR).red()
            )
        }
        (code, version) => println!("{}", format!("{:?} {}", version, code).yellow()),
    }

    println!()
}

fn print_headers(res: &Response) {
    let mut output = String::new();

    for (key, value) in res.headers() {
        writeln!(
            &mut output,
            "{}: {}",
            key.to_string().cyan(),
            value.to_str().unwrap().green()
        )
        .unwrap();
    }

    writeln!(std::io::stdout(), "{}", output).unwrap();
}

fn print_body(headers: &HeaderMap, body: String) {
    let content_type = headers
        .get(CONTENT_TYPE)
        .map_or("plain", |h| h.to_str().unwrap_or("plain"));

    let (body, extension) = match content_type {
        "application/json" => (jsonxf::pretty_print(&body).unwrap(), "json"),
        "text/html" => (body, "html"),
        _ => (body, "txt"),
    };

    print_with_theme(&body, extension);
}

fn print_with_theme(s: &str, extension: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps
        .find_syntax_by_extension(extension)
        .unwrap_or(ps.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
}
