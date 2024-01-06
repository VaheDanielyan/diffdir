use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use walkdir::WalkDir;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use glob::Pattern;
use ansi_term::{Style, Colour::*};
use md5;

#[derive(Debug, Clone)]
pub enum Hash {
    Valid { hash: String },
    Invalid { error: String },
}

impl Hash {
    pub fn new(path: &Path) -> Hash {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Hash::Invalid { error: e.to_string() }
        };

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Err(e) => return Hash::Invalid { error: e.to_string() },
            _ => {}
        }
        let digest = md5::compute(&buffer);

        Hash::Valid { hash: format!("{:?}", digest)}
    }
}

#[derive(Debug)]
struct FileInfo {
    hash: Hash,
}

impl FileInfo {
    pub fn get_hash(&self) -> Hash {
        self.hash.clone()
    }
}

pub struct DirCmp {
    path_a : PathBuf,
    path_b : PathBuf,
    ignore_patterns : Option<Vec<Pattern>>,
}

impl DirCmp {
    pub fn new(path_a: &PathBuf, path_b: &PathBuf, ignore_patterns : &Option<Vec<Pattern>>) -> DirCmp {
        DirCmp { path_a: path_a.to_owned(), path_b: path_b.to_owned(), ignore_patterns: ignore_patterns.to_owned() }
    }
    pub fn compare_directories(&self) -> CmpResult {
        let path_a_clone = self.path_a.clone(); let path_b_clone = self.path_b.clone();

        let ignore_patterns_1 = self.ignore_patterns.clone();
        let ignore_patterns_2 = self.ignore_patterns.clone();

        let thread_a = std::thread::spawn(move || {
            return DirCmp::process_directory(&path_a_clone, &ignore_patterns_1);
        });

        let thread_b = std::thread::spawn(move || {
            return DirCmp::process_directory(&path_b_clone, &ignore_patterns_2);
        });

        let map1 : HashMap<PathBuf, FileInfo> = thread_a.join().unwrap().ok().unwrap();
        let map2 : HashMap<PathBuf, FileInfo> = thread_b.join().unwrap().ok().unwrap();
        let mut result : CmpResult = CmpResult::new(&self.path_a, &self.path_b);

        for item in &map1 {
            if map2.contains_key(item.0) {
                let item2 = map2.get(item.0).unwrap();
                let hash1 = match item.1.get_hash() {
                    Hash::Valid { hash } => hash,
                    Hash::Invalid { error } => error,
                };
                let hash2 = match item2.get_hash() {
                    Hash::Valid { hash } => hash,
                    Hash::Invalid { error } => error,
                };
                if hash1 != hash2 {
                    result.differs.push(item.0.clone());
                }
            }
            else {
                result.only_in_a.push(item.0.clone());
            }
        }
        for item in &map2 {
            if !map1.contains_key(item.0) {
                result.only_in_b.push(item.0.clone());
            }
        }
        result

    }
    fn process_directory(path: &PathBuf, ignore_patterns : &Option<Vec<Pattern>>) -> Result<HashMap<PathBuf, FileInfo>, String> {
        let files : Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|f| f.ok())
            .filter(|f| {
                let mut ignore = false;
                let file_name = f.file_name().to_str().unwrap();
                if let Some(patters) = ignore_patterns {
                    ignore = patters.iter()
                        .any(|patt| {
                            patt.matches(file_name)
                        });
                }
                f.file_type().is_file() && !ignore
            })
            .map(|f| f.path().to_owned())
            .collect();
        let result_map : HashMap<PathBuf, FileInfo> = files
            .par_iter()
            .map(|f| {
                let file_hash = Hash::new(f);
                (f.strip_prefix(path).unwrap().to_owned(), FileInfo { hash: file_hash })
            })
            .collect();
        Ok(result_map)
    }
}

pub struct CmpResult {
    pub dir_a : PathBuf,
    pub dir_b : PathBuf,
    pub only_in_a : Vec<PathBuf>,
    pub only_in_b: Vec<PathBuf>,
    pub differs : Vec<PathBuf>,
}

impl CmpResult {
    pub fn new(dir_a : &PathBuf, dir_b : &PathBuf) -> CmpResult {
        CmpResult { dir_a: dir_a.to_owned(),
                    dir_b: dir_b.to_owned(),
                    only_in_a: Vec::new(),
                    only_in_b: Vec::new(),
                    differs: Vec::new() }
    }
    pub fn are_different(&self) -> bool {
        if self.only_in_a.is_empty() && self.only_in_b.is_empty() && self.differs.is_empty() {
            return false;
        }
        true
    }
    pub fn format_text(&self, ansi: bool) -> Vec<String> {
        let bold = Style::new().bold();
        let bold_underline = bold.underline();
        let mut result : Vec<String> = Vec::new();
        let mut result_plain: Vec<String> = Vec::new();

        println!();
        if !self.are_different() {
            let message = format!("The directories appear to be the same\n"); 
            let styled_message = bold.
                paint(&message);
            result.push(styled_message.to_string());
            result_plain.push(message);
        }

        if !self.only_in_a.is_empty() {
            let message = format!("Files that appear only in {}\n", self.dir_a.to_str().unwrap());
            let styled_message = bold_underline.fg(Yellow)
                .paint(&message);
            result.push(styled_message.to_string());
            result_plain.push(message);
            for item in &self.only_in_a {
                let file_message = format!("{}\n", item.to_str().unwrap());
                result.push(file_message.clone());
                result_plain.push(file_message);
            }
            result.push("\n".to_string());
            result_plain.push("\n".to_string());
        }

        if !self.only_in_b.is_empty() {
            let message = format!("Files that appear only in {}\n", self.dir_b.to_str().unwrap());
            let styled_message = bold_underline.fg(Yellow)
                .paint(&message);
            result.push(styled_message.to_string());
            result_plain.push(message);
            for item in &self.only_in_b {
                let file_message = format!("{}\n", item.to_str().unwrap());
                result.push(file_message.clone());
                result_plain.push(file_message);
            }
            result.push("\n".to_string());
            result_plain.push("\n".to_string());
        }

        if !self.differs.is_empty() {
            let message = format!("Files that differ\n");
            let styled_message = bold_underline.fg(Red)
                .paint(&message);
            result.push(styled_message.to_string());
            result_plain.push(message);
            for item in &self.differs {
                let file_message = format!("{}\n", item.to_str().unwrap());
                result.push(file_message.clone());
                result_plain.push(file_message);
            }
        }
        if ansi { result } else { result_plain }
    }
}
