use clap::Parser;
use colored::Colorize;
use fwuffgrep::{config::Config, run};
use std::process;

fn main() {
    let config = Config::parse();
    run(&config).unwrap_or_else(|err| {
        println!("{}: {}", "Finished with error".red().bold(), err);
        process::exit(1);
    })
}
