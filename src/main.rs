use clap::Parser;
use minigrep::{config::Config, run};
use std::process;

fn main() {
    let config = Config::parse();
    if let Err(err) = run(&config) {
        println!("Finished with error: {}", err);
        process::exit(1);
    }
}
