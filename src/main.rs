use clap::Parser;
use core::panic;
use std::path::PathBuf;

mod dirdiff;

#[derive(Parser)]
#[clap(author, version, about = "A cli tool to compare two directories", long_about)]
struct Args {
    #[clap(value_parser)]
    dir_a: PathBuf,

    #[clap(value_parser)]
    dir_b: PathBuf,

    #[clap(long = "output-dir", value_parser)]
    output_dir: Option<PathBuf>,

    #[clap(long = "ignore", value_parser)]
    ignore_pattern: Option<String>,

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
        if let Some(output_dir) = &self.output_dir {
            if !output_dir.exists() {
                errors.push("argument error: Output dir doesn't exist".to_string());
            }
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

    dirdiff::dirdiff(&args.dir_a, &args.dir_b);
}
