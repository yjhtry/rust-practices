use std::env;

use anyhow::Result;
use kv::{start_server_with_config, ServerConfig};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let config = match env::var("KV_SERVER_CONFIG") {
        Ok(path) => fs::read_to_string(&path).await?,
        Err(_) => include_str!("../fixtures/server.conf").to_string(),
    };
    let config: ServerConfig = toml::from_str(&config)?;

    start_server_with_config(&config).await?;

    Ok(())
}
