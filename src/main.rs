use fwuffgrep::{config::Config, run};

use clap::Parser;
use colored::Colorize;
use std::process;

fn main() {
    let config: Config = Config::parse();
    run(&config).unwrap_or_else(|err| {
        println!("{}: {}", "Finished with error".red().bold(), err);
        process::exit(1);
    })
}
