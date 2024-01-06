use std::path::PathBuf;

use glob::Pattern;
use std::fs::File;
use std::io::{BufReader, Read};
use clap::*;

#[derive(Parser)]
#[clap(author, version, about = "A cli tool to compare two directories", long_about)]
pub struct Args {
    #[clap(value_parser)]
    pub dir_a: PathBuf,

    #[clap(value_parser)]
    pub dir_b: PathBuf,

    #[clap(long = "ignore", value_parser, use_value_delimiter(true), value_delimiter = ' ', num_args=1..)]
    pub ignore_patterns: Option<Vec<Pattern>>,

    #[clap(long = "ignore-file", value_parser)]
    pub ignore_file: Option<PathBuf>,

    #[clap(long = "no-colors", value_parser)]
    pub no_colors: bool,
}

impl Args {
    pub fn verify(&self) -> Result<(), String> {
        let mut errors : Vec<String> = Vec::new();
        if !self.dir_a.exists() {
            errors.push("argument error: Dir A doesn't exist".to_string());
        }
        if !self.dir_a.is_dir() {
            errors.push("argument error: A is not a directory".to_string());
        }
        if !self.dir_b.exists() {
            errors.push("argument error: Dir B doesn't exist".to_string());
        }
        if !self.dir_b.is_dir() {
            errors.push("argument error: B is not a directory".to_string());
        }
        if let Some(ignore_file) = &self.ignore_file {
            if !ignore_file.exists() {
                errors.push("argument error: Ignore file doesn't exist".to_string());
            }
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors.join("\n"))
    }
    pub fn parse_ignore_file(path: PathBuf) -> Vec<Pattern> {
        let file = File::open(path).expect("Error opening ignore file");
        let mut reader = BufReader::new(file);

        let mut patterns = Vec::new();
        let mut file_contents: String = String::new();
        _ = reader.read_to_string(&mut file_contents);
        for line in file_contents.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            match Pattern::new(&line) {
                Ok(pattern) => patterns.push(pattern),
                Err(e) => eprintln!("Invalid pattern in ignore file, line - '{}': {}", line, e),
            }
        }
        patterns
    }
}

