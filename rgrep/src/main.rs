mod cli;
mod grep;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
