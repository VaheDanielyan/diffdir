use clap::Parser;
use core::panic;
use std::path::PathBuf;

use atty::Stream;
use glob::Pattern;
use std::fs::{File};
use std::io::{BufReader, Read};
mod dirdiff;

#[derive(Parser)]
#[clap(author, version, about = "A cli tool to compare two directories", long_about)]
struct Args {
    #[clap(value_parser)]
    dir_a: PathBuf,

    #[clap(value_parser)]
    dir_b: PathBuf,

    #[clap(long = "ignore", value_parser, use_value_delimiter(true), value_delimiter = ' ', num_args=1..)]
    ignore_patterns: Option<Vec<Pattern>>,

    #[clap(long = "ignore-file", value_parser)]
    ignore_file: Option<PathBuf>,

    #[clap(long = "no-colors", value_parser)]
    no_colors: bool,
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

fn main() {
    let args = Args::parse(); 

    match args.verify() {
        Err(message) => panic!("{}", message),
        Ok(()) => {}
    };

    let ignore_file_patterns = match args.ignore_file {
        Some(file) => Some(Args::parse_ignore_file(file)),
        _ => None
    };

    let merged_patterns = match (ignore_file_patterns, args.ignore_patterns) {
        (Some(mut vec1), Some(vec2)) => {
            vec1.extend(vec2);
            Some(vec1)
        },
        (Some(vec), None) | (None, Some(vec)) => Some(vec),
        (None, None) => None,
    };

    let dir_comparator = dirdiff::DirCmp::new(&args.dir_a, &args.dir_b, &merged_patterns);
    let result = dir_comparator.compare_directories();
    let text = if atty::is(Stream::Stdout) {
        result.format_text(!args.no_colors)
    } else {
        result.format_text(false)
    };
    for item in text {
        print!("{}", item);
    }
}
