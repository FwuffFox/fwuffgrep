pub mod config;
pub mod io;

use config::Config;
use io::{input::{manage_input, InputLines}, output::manage_output};

use regex::Regex;
use std::error::Error;


pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: InputLines = manage_input(config)?;
    let result: Vec<(usize, String)> = get_result(config, contents)?;
    manage_output(config, &result)?;
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

fn matches_query(line: &str, query: &str, ignore_case: bool) -> bool {
    let line_to_compare = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };
    line_to_compare.contains(query)
}

pub fn search(
    query: &str,
    lines: InputLines,
    ignore_case: bool,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let mut results = Vec::new();
    for (line_number, line_result) in lines.enumerate() {
        let line = line_result?;
        if matches_query(&line, query, ignore_case) {
            results.push((line_number + 1, line));
        }
    }
    Ok(results)
}

