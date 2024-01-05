use clap::Parser;
use core::panic;
use std::path::PathBuf;

use ansi_term::{Style, Colour::*};

mod dirdiff;

#[derive(Parser)]
#[clap(author, version, about = "A cli tool to compare two directories", long_about)]
struct Args {
    #[clap(value_parser)]
    dir_a: PathBuf,

    #[clap(value_parser)]
    dir_b: PathBuf,

    #[clap(long = "ignore", value_parser, use_value_delimiter(true), value_delimiter = ' ', num_args=1..)]
    ignore_pattern: Option<Vec<String>>,

    #[clap(long = "ignore-file", value_parser)]
    ignore_file: Option<PathBuf>,
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

    let result = dirdiff::dirdiff(&args.dir_a, &args.dir_b);

    let bold = Style::new().bold();
    let bold_underline = bold.underline();

    println!();
    if result.only_in_a.is_empty() && result.only_in_b.is_empty() && result.differs.is_empty() {
        let styled_message = bold.
            paint(format!("The directories appear to be the same"));
        println!("{}", styled_message);
        return;
    }

    if !result.only_in_a.is_empty() {
        let styled_message = bold_underline.fg(Yellow)
            .paint(format!("Files that appear only in {}", args.dir_a.to_str().unwrap()));
        println!("{}", styled_message);
        for item in &result.only_in_a {
            println!("{}", item.to_str().unwrap());
        }
        println!();
    }

    if !result.only_in_b.is_empty() {
        let styled_message = bold_underline.fg(Yellow)
            .paint(format!("Files that appear only in {}", args.dir_b.to_str().unwrap()));
        println!("{}", styled_message);
        for item in &result.only_in_b {
            println!("{}", item.to_str().unwrap());
        }
        println!();
    }

    if !result.only_in_b.is_empty() {
        let styled_message = bold_underline.fg(Red)
            .paint(format!("Files that differ"));
        println!("{}", styled_message);
        for item in &result.differs {
            println!("{}", item.to_str().unwrap());
        }
    }
    println!();
}
