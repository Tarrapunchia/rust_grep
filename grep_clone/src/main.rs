use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use anyhow::{Context, Result};
use colored::*;

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

/// Finds the patterns in the file, reading line by line
fn find_pattern_in_line(reader: &mut BufReader<File>,
                        pattern: &str,
                        mut writer: impl std::io::Write) 
                        -> Result<()>
{
    let patterns = check_for_mult_patters(pattern);
    let mut buffer = String::new();
    let mut counter = 1;
    let mut flag = false;

    match patterns {
        Multi::One(pattern) => {
            while reader.read_line(&mut buffer)? != 0 {
                if buffer.contains(pattern) {
                    buffer = buffer
                        .trim_end_matches('\n')
                        .to_string()
                        .replace(pattern, &format!("{}", pattern.red()));
                    writeln!(writer, "# \"{pattern}\" found at line [{counter}]:\n\t\"{buffer}\"")?;
                }
                counter += 1;
                buffer = String::from("");
            }
        }
        Multi::Two(patterns) => {
            while reader.read_line(&mut buffer)? != 0 {
                for pattern in &patterns {
                    if buffer.contains(pattern) {
                        flag = true;
                        buffer = buffer
                            .trim_end_matches('\n')
                            .to_string()
                            .replace(pattern, &format!("{}", pattern.red()));
                    }
                    if flag == true {
                        writeln!(writer, "#\t{buffer}")?;
                        flag = false;
                    }
                }
                buffer = String::from("");
            }
        }
    }
    Ok(())
}

/// Enum for conditional returning of the function below
enum Multi<'a> {
    One(&'a str),
    Two(Vec<&'a str>),
}

/// Checks if the pattern in input is multiple or not and returns an enum
/// conditionally: &str or Vec<&str>
fn check_for_mult_patters(pattern: &str) -> Multi {
    // let mut mult<str> = Vec::new();
    if pattern.contains("\\|") {
        let mult: Vec<&str> = pattern.split("\\|").collect();
        Multi::Two(mult)
    } else {
        Multi::One(pattern)
    }
}