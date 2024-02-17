use anyhow::Result;
use clap::Parser;
use rrgrep::*;

fn main() -> Result<()> {
    let config: GrepConfig = GrepConfig::parse();
    config.match_with_default_strategy()?;

    Ok(())
}
