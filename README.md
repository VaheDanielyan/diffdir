![build status](https://github.com/VaheDanielyan/dirdiff.rs/actions/workflows/rust.yml/badge.svg) [![crates-io](https://img.shields.io/crates/v/diffdir?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fdiffdir)](https://crates.io/crates/diffdir) ![stability-beta](https://img.shields.io/badge/stability-beta-33bbff.svg)
# Diffdir

A command line tool to compare two directories. 

Uses hashes to compares files with the same name. Also lists the unique files for both directories.

## Installation

If you don't have rust installed, go ahead and [Install Rust](https://www.rust-lang.org/tools/install)

Clone this repository

```sh
git clone git@github.com:VaheDanielyan/diffdir.git
```

Build and install

```sh
cargo install --path .
```

## Usage

```sh
Usage: diffdir [OPTIONS] <dir a> <dir b>

Arguments:
  <dir a>
  <dir b>

Options:
      --ignore <IGNORE_PATTERNS>...
      --ignore-file <IGNORE_FILE>
      --quiet                        Surpress output
      --no-colors                    will not format into ansi string and / or include colors
  -h, --help                         Print help
  -V, --version                      Print version
```

## Output

In Addition to the standard text output the program will return **42** if there are any differences between the directories and **0** in case of them being identical. This can be handy when calling this tool from other programs.
