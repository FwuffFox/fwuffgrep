use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Query to search by
    pub query: String,

    /// Path to a file
    pub file_path: PathBuf,

    /// Ignore case
    #[arg(short, long, action)]
    pub case_insensitive: bool,

    /// Path to output file
    #[arg(short, long)]
    pub output_path: Option<PathBuf>,
}
