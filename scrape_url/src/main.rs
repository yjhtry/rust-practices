use std::{env, fs};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to fetch
    #[arg(short, long)]
    url: String,

    /// Output file
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", env::args());

    let args = Args::parse();

    let url: &str = args.url.as_str();
    let output: &str = args.output.as_str();

    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");

    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;

    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}
