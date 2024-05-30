use std::collections::HashMap;

use anyhow::Result;
use clap::{command, Parser};

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

fn main() {
    let cli = Cli::parse();

    match cli.run {
        Run::Get(get) => handle_get(get),
        Run::Post(post) => handle_post(post),
    }
}

fn handle_get(get: Get) {
    let req = reqwest::blocking::get(&get.url).unwrap();

    println!("{:?}", req.text().unwrap());
}

fn handle_post(post: Post) {
    let client = reqwest::blocking::ClientBuilder::new().build().unwrap();
    let res = client.post(&post.url).json(&post.body).send().unwrap();

    println!("post body: {:?}", post.body);

    // println!("{:?}", res.text().unwrap());
}
