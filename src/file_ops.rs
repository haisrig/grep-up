use crate::OptionConfig;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;


fn build_file_walker(path: &str, is_recursive: bool) -> impl Iterator<Item = walkdir::DirEntry> {
    let mut walk_dir = WalkDir::new(path);
    if !is_recursive {
        walk_dir = walk_dir.max_depth(1);
    }
     walk_dir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
}
fn search_in_file(path: &str, pattern: &str, ignore_case: bool) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => { println!("Can not open file {}: {}", path, e); return; }
    };
    let reader = BufReader::new(file);
    reader.lines().enumerate().for_each(|(i, line_result)| {
        let line = match line_result { Ok(l) => l, Err(_) => return };
        let matches = if ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };
        if matches { println!("{}:{}:{}", path, i + 1, line); }
    });
}

fn find_string(path: &str, pattern: &str, ignore_case: bool, is_recursive: bool) {
    let walk_dir = build_file_walker(path, is_recursive);
    walk_dir
        .map(|e| e.path().display().to_string())
        .for_each(|file| {
            search_in_file(&file, pattern, ignore_case);
        });
}

fn find_file_with_names(path: &str, pattern: &str, ignore_case: bool, is_recursive: bool) {
    let walk_dir = build_file_walker(path, is_recursive);
        walk_dir.filter(|e| {
            let file_name = e.file_name().to_string_lossy();
            if ignore_case {
                file_name.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                file_name.contains(pattern)
            }
        })
        .for_each(|file| {
            println!("{}", file.path().display());
        });
}

pub fn perform_search(option_config: &OptionConfig) {
    let path = &option_config.path;
    let pattern = &option_config.pattern;
    let ignore_case = option_config.ignore_case;
    let find_files = option_config.find_file;
    let recursive = option_config.recursive;

    if find_files {
        find_file_with_names(path, pattern, ignore_case, recursive);
    } else {
        find_string(path, pattern, ignore_case, recursive);
    }
}