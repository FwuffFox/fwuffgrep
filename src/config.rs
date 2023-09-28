use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Query to search by
    pub query: String,

    /// Path to a file. If none, read from stdin
    pub file_path: Option<PathBuf>,

    /// Ignore case
    #[arg(short, long, action)]
    pub case_insensitive: bool,

    /// Use regex search
    #[arg(short, long, action)]
    pub regex: bool,

    /// Path to output file
    #[arg(short, long)]
    pub output_path: Option<PathBuf>,
}
