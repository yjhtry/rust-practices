use clap::{command, Parser};

/// group can be used to group options together
/// #[group(id = "input")] attribute is used to specify the group

#[derive(Debug, Parser)]
#[command()]
struct Cli {
    #[command(flatten)]
    other: Other,

    #[arg(short, long, requires = "input")]
    age: u8,
}

#[derive(Debug, Parser)]
#[group(id = "input")]
struct Other {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(long)]
    habit: Option<String>,
}

fn main() {
    let _cli = Cli::parse();
}
