use std::collections::HashMap;

use anyhow::Result;
use clap::{command, Parser};
use reqwest::{Response, StatusCode};

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
    url: String,
}

#[derive(Debug, Parser)]
struct Post {
    url: String,

    /// `key1=value1&key2=value2`
    #[arg(value_parser = parser_body)]
    body: HashMap<String, String>,
}

fn parser_body(s: &str) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    println!("s: {:?}", s);
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
    let res = reqwest::get(&get.url).await.unwrap();

    colorful_print(res);

    Ok(())
}

async fn handle_post(post: Post) -> Result<()> {
    let client = reqwest::ClientBuilder::new().build()?;
    let res = client.post(&post.url).json(&post.body).send().await?;

    colorful_print(res);

    Ok(())

    // println!("{:?}", res.text().unwrap());
}

fn colorful_print(res: Response) {
    print_status_line(res);
}

fn print_status_line(res: Response) {
    match (res.status(), res.version()) {
        (StatusCode::OK, version) => println!("{:?} {}", version, StatusCode::OK),
        (StatusCode::NOT_FOUND, version) => println!("{:?} {}", version, StatusCode::NOT_FOUND),
        (code, version) => println!("{:?} {}", version, code),
    }
}
