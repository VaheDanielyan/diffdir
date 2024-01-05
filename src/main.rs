use clap::Parser;
use core::panic;
use std::path::PathBuf;

use atty::Stream;
use glob::Pattern;
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
}

fn main() {
    let args = Args::parse(); 

    match args.verify() {
        Err(message) => panic!("{}", message),
        Ok(()) => {}
    };

    let dir_comparator = dirdiff::DirCmp::new(&args.dir_a, &args.dir_b, &args.ignore_patterns);
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
