use std::{fs::File, io::{Write, stdout}};

use crate::config::Config;

pub fn manage_output(config: &Config, lines: &[(usize, String)]) -> std::io::Result<()> {
    let mut output: Box<dyn Write> = if let Some(path) = &config.output_path {
        Box::new(File::create(path)?)
    } else {
        Box::new(stdout())
    };

    for (line_number, line) in lines.iter() {
        write_output_line(&mut output, *line_number, line, config.numerate)?;
    }

    Ok(())
}

fn write_output_line<W: Write>(
    output: &mut W,
    line_number: usize,
    line: &str,
    numerate: bool,
) -> std::io::Result<()> {
    if numerate {
        writeln!(output, "{}: {}", line_number, line)?;
    } else {
        writeln!(output, "{}", line)?;
    }
    Ok(())
}