use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let f = std::fs::File::open(&args.path)
        .with_context(|| {format!("Error opening \'{}\'", args.path.display())})?;
    let mut reader = std::io::BufReader::new(f);

    println!(
    "Pattern:\t{:?}\nPath:\t\t{:?}",
        args.pattern,
        args.path
    );
    find_pattern_in_line(&mut reader, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}

fn find_pattern_in_line(reader: &mut BufReader<File>, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    let mut buffer = String::new();
    let mut  counter = 21;
    while reader.read_line(&mut buffer)? != 0 {
        if buffer.contains(pattern) {
            buffer = buffer.trim_end_matches('\n').to_string();
            writeln!(writer, "# \"{pattern}\" found at line [{counter}]:\n\t\"{buffer}\"")?;
            return Ok(());
        }
        buffer = String::from("");
        counter += 1;
    }
    writeln!(writer, "# \"{pattern}\" not found in this document.")?;
    Ok(())
}
