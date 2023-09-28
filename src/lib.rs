use crate::config::Config;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub mod config;

#[cfg(test)]
mod tests;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.file_path);
    let contents = fs::read_to_string(path)?;
    let result = get_result(config, &contents)?;
    manage_output(config, &result)?;
    Ok(())
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
