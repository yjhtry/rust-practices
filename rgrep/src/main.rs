mod cli;
mod grep;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    let mut grep = grep::RGrep::new(args.search, args.file_glob);

    grep.print();
}
