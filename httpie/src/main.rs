mod cli;

use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use cli::Args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.subcmd {
        cli::SubCommand::Get(get) => handle_get(get).await,
        cli::SubCommand::Post(post) => handle_post(post).await,
    }
}

async fn handle_get(args: cli::Get) -> Result<()> {
    let res = reqwest::get(&args.url).await?.text().await?;

    println!("{}", res);

    Ok(())
}

async fn handle_post(args: cli::Post) -> Result<()> {
    let body: HashMap<&str, &str> = args
        .body
        .iter()
        .map(|x| (&x.key[..], &x.value[..]))
        .collect();

    let res = reqwest::Client::new()
        .post(&args.url)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
