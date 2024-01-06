extern crate diffdir;
use diffdir::diffcmp::{DirCmp, CmpResult};
use diffdir::args::Args;

use core::panic;
use atty::Stream;
use clap::Parser;

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

    let dir_comparator = DirCmp::new(&args.dir_a, &args.dir_b, &merged_patterns);
    let result: CmpResult  = dir_comparator.compare_directories();
    if !args.quiet {
        let text = if atty::is(Stream::Stdout) {
            result.format_text(!args.no_colors)
        } else {
            result.format_text(false)
        };

        for item in text {
            print!("{}", item);
        }
    }
    if result.are_different() { std::process::exit(42) }
}
