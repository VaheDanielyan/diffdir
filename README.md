![build status](https://github.com/VaheDanielyan/dirdiff.rs/actions/workflows/rust.yml/badge.svg) ![https://google.com](https://img.shields.io/crates/v/diffdir?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fdiffdir) [![stability-beta](https://img.shields.io/badge/stability-beta-33bbff.svg)](https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#beta)


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
AUTHORS
       Vahe Danielyan <danielyan.vahe@gmail.com>

```

