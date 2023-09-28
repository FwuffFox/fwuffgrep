use crate::config::Config;

use std::error::Error;
use std::fs::{self, File};
use std::io::{stdin, Read, Write};

pub mod config;

#[cfg(test)]
mod tests;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = manage_input(config)?;
    let result = get_result(config, &contents)?;
    manage_output(config, &result)?;
    Ok(())
}

fn manage_input(config: &Config) -> Result<String, Box<dyn Error>> {
    let res = match config.file_path.clone() {
        None => {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf)?;
            buf
        }
        Some(path) => fs::read_to_string(path)?,
    };
    Ok(res)
}

pub fn manage_output(config: &Config, lines: &[&str]) -> Result<(), Box<dyn Error>> {
    match config.output_path.clone() {
        None => {
            for line in lines.iter() {
                println!("{line}");
            }
        }
        Some(path) => {
            let combined_data = lines.join("\n");
            let mut file = File::create(path)?;
            file.write_all(combined_data.as_bytes())?;
        }
    }
    Ok(())
}

pub fn get_result<'a>(config: &Config, text: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let result = if config.case_insensitive {
        search(&config.query, text)
    } else {
        search_insensitive(&config.query, text)
    };

    Ok(result)
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    text.lines()
        .filter(|line| line.to_lowercase().contains(&lowercase_query))
        .collect()
}
