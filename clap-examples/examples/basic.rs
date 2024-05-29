use std::path::PathBuf;

use clap::{command, Parser};

/// A basic example
/// struct attribute is the command arguments
/// Option type represents the type of the option
/// #[arg] attribute is used to specify the name of the option
/// #[command(subcommand)] attribute is used to specify the subcommand
/// #[command(flatten)] attribute is used to flatten the subcommand to the parent command

#[derive(Debug, Parser)]
#[command(author, about, version, next_line_help = true)]
struct Cli {
    /// Option name to operate on
    name: Option<String>,

    /// Optional configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Activate debug mode
    #[arg(short, long, action =clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser)]
enum Command {
    /// Get the value of an option
    Get {
        /// Option value to get
        value: String,
    },

    /// Set the value of an option
    Set {
        /// Option value to set
        value: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.config.is_some() {
        println!("Using config file: {:?}", cli.config.unwrap());
    }

    if cli.debug > 0 {
        println!("Debug mode is active: {}", cli.debug);
    }

    match cli.command {
        Some(Command::Get { value }) => {
            println!("Getting value: {}", value);
        }
        Some(Command::Set { value }) => {
            println!("Setting value: {}", value);
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
