![build status](https://github.com/VaheDanielyan/dirdiff.rs/actions/workflows/rust.yml/badge.svg) ![crates io](https://img.shields.io/crates/v/diffdir)

# Diffdir

A command line tool to compare two directories. 

Uses hashes to compares files with the same name. Also lists the unique files for both directories.

## Installation

If you don't have rust installed, go ahead and [Install Rust](https://www.rust-lang.org/tools/install)

Clone this repository

```
git clone git@github.com:VaheDanielyan/diffdir.git
```

Build and install

```
cargo install --path .
```

## Usage

```
NAME
       diffdir - A cli tool to compare two directories
SYNOPSIS
       diffdir [--ignore] [--ignore-file] [--no-colors] [-h|--help] [-V|--version] <DIR_A> <DIR_B>
DESCRIPTION
       A cli tool to compare two directories
OPTIONS
       --ignore=IGNORE_PATTERNS
       --ignore-file=IGNORE_FILE
       --no-colors
       -h, --help
              Print help
       -V, --version
              Print version
       <DIR_A>
       <DIR_B>
VERSION
       v0.4.1
AUTHORS
       Vahe Danielyan <danielyan.vahe@gmail.com>

```

