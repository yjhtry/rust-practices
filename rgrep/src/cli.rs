use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // match the pattern in the file
    pub search: String,
    // glob pattern for the file
    pub file_glob: String,
}
