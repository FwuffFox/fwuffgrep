use regex::Regex;

use crate::config::Config;

use std::error::Error;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{self, stdin, BufRead, BufReader, Lines, Read, Write};
use std::iter::Enumerate;
use std::path::Path;

pub mod config;

#[cfg(test)]
mod tests;

type InputLines = Lines<BufReader<Box<dyn Read>>>;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: InputLines = manage_input(config)?;
    let result = get_result(config, contents)?;
    manage_output(config, &result)?;
    Ok(())
}

fn manage_input(config: &Config) -> Result<InputLines, Box<dyn Error>> {
    let res: Lines<BufReader<Box<dyn Read>>> = match config.file_path.clone() {
        None => {
            let reader: Box<dyn Read> = Box::new(stdin());
            BufReader::new(reader).lines()
        }
        Some(path) => read_lines(path)?,
    };
    Ok(res)
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
                file.write(format!("{}: {}", line.0, line.1).as_bytes());
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
        search(&config.query, lines, config.case_insensitive)
    }
}

pub fn search_regex<'a>(
    pattern: &str,
    lines: InputLines,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let regex = Regex::new(pattern)?;
    let lines = lines.map(|x| x.unwrap()).enumerate();
    Ok(lines.filter(|line| regex.is_match(&line.1)).collect())
}

pub fn search(
    query: &str,
    lines: InputLines,
    ignore_case: bool,
) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let mut query: String = query.to_owned();
    let lines = lines.map(|x| x.unwrap()).enumerate();
    if ignore_case {
        todo!();
    }
    Ok(lines.filter(|x| x.1.contains(&query)).collect())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<Box<dyn Read>>>>
where
    P: AsRef<Path>,
{
    let file: Box<dyn Read> = Box::new(File::open(filename)?);
    Ok(io::BufReader::new(file).lines())
}
