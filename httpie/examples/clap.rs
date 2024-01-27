use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    port: u16, // Args

    #[arg(short, long)] // Options
    enable: bool,

    #[clap(subcommand)] // subcommands
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Show(Show),
}

#[derive(Parser, Debug)]
struct Show {
    #[arg(short, long)]
    name: String,
    habits: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}
