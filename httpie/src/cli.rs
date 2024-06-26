use std::str::FromStr;

use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version = "1.0.0", author = "junhao.yu <dddd>")]
pub struct Args {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
pub struct Get {
    #[clap(value_parser = parse_url)]
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct Post {
    #[clap(value_parser = parse_url)]
    pub url: String,
    /// parse from key=value
    #[clap(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}

fn parse_url(s: &str) -> Result<String> {
    let url = Url::parse(s)?;

    Ok(url.into())
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split('=');

        let err = || anyhow::anyhow!("Failed to parse {}", s);

        let key = split.next().ok_or_else(err)?;
        let value = split.next().ok_or_else(err)?;

        Ok(KvPair {
            key: key.into(),
            value: value.into(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_success() {
        let url = "https://www.baidu.com".parse::<Url>().unwrap();
        let url_str = url.to_string();
        assert_eq!(url_str, "https://www.baidu.com/");
    }

    #[test]
    fn parse_kv_pair_success() {
        let s = "name=john";
        let kv_pair = s.parse::<KvPair>().unwrap();
        assert_eq!(kv_pair.key, "name");
        assert_eq!(kv_pair.value, "john");
    }
}
