use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(author = "Fwuff", version = "1.0.0", about = "Basic implementation of a grep command written in rust", long_about = None)]
pub struct Config {
    /// Query to search by
    pub query: String,

    /// Path to a file to read from. If none, input is read from stdin
    pub file_path: Option<PathBuf>,

    /// Ignore case. Doesn't work if using Regex search
    #[arg(short, long, action)]
    pub ignore_case: bool,

    /// Use regex search. Overrides an ignore case flag
    #[arg(short, long, action)]
    pub regex: bool,

    /// Path to output file. If none, text is written to a stdout
    #[arg(short , long)]
    pub output_path: Option<PathBuf>,

    /// If passed, output lines will be numerated by their position in a file.
    #[arg(short, long, action)]
    pub numerate: bool,
}
