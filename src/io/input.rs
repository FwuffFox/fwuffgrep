use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Lines, Read, Cursor},
    path::Path,
};

use crate::config::Config;

pub type InputLines = Lines<BufReader<Box<dyn Read>>>;

pub fn input_lines_from_string(input: String) -> InputLines {
    let cursor = Cursor::new(input);
    let reader: Box<dyn Read> = Box::new(cursor);
    BufReader::new(reader).lines()
}

pub fn manage_input(config: &Config) -> std::io::Result<InputLines> {
    let res: InputLines = match &config.path {
        None => {
            let stdin: Box<dyn Read> = Box::new(stdin());
            BufReader::new(stdin).lines()
        },
        Some(ref path) => read_lines(path)?,
    };
    Ok(res)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> std::io::Result<InputLines>
where
    P: AsRef<Path>,
{
    let file: Box<dyn Read> = Box::new(File::open(filename)?);
    Ok(BufReader::new(file).lines())
}
