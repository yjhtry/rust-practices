use clap::{command, Parser};

/// group can be used to group options together
/// #[arg(requires = group_name)] attribute is used to specify that the option requires a group

#[derive(Debug, Parser)]
#[command()]
struct Cli {
    #[arg(short, long, group = "input")]
    name: Option<String>,

    #[arg(long, group = "input")]
    habit: Option<String>,

    #[arg(short, long, requires = "input")]
    age: u8,
}

fn main() {
    let _cli = Cli::parse();
}
