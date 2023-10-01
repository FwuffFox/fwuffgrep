pub mod config;
pub mod input;

use config::Config;
use input::{manage_input, InputLines};

use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::Write,
};


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: InputLines = manage_input(config)?;
    let result: Vec<(usize, String)> = get_result(config, contents)?;
    manage_output(config, &result)?;
    Ok(())
}

pub fn manage_output(config: &Config, lines: &[(usize, String)]) -> Result<(), Box<dyn Error>> {
    match config.output_path.clone() {
        None => {
            for line in lines.iter() {
                println!("{}: {}", line.0, line.1);
            }
        }
        Some(path) => {
            let mut file = File::create(path)?;
            for line in lines.iter() {
                file.write(format!("{}: {}\n", line.0, line.1).as_bytes())?;
            }
        }
    }
    Ok(())
}

pub fn get_result<'a>(
    config: &Config,
    lines: InputLines,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    if config.regex {
        search_regex(&config.query, lines)
    } else {
        search(&config.query, lines, config.ignore_case)
    }
}

pub fn search_regex(
    pattern: &str,
    lines: InputLines,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let regex: Regex = Regex::new(pattern)?;
    let lines = lines.map(|x| x.unwrap()).enumerate();
    Ok(lines
        .filter(|line| regex.is_match(&line.1))
        .map(|(line_number, line)| (line_number + 1, line))
        .collect())
}

pub fn search(
    query: &str,
    lines: InputLines,
    ignore_case: bool,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let mut results: Vec<(usize, String)> = Vec::new();
    let lowercase_query: String = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    for (line_number, line_result) in lines.enumerate() {
        let line: String = line_result?;

        let line_to_compare: String = if ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if line_to_compare.contains(&lowercase_query) {
            results.push((line_number + 1, line)); // Adding 1 to line number to make it 1-indexed
        }
    }

    Ok(results)
}
