use crate::cli::{SearchConfig, SearchMode};
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

fn search_in_file<F>(path: &str, pattern: &str, ignore_case: bool, handler: F) -> usize
where F: Fn(&str, usize) {
    search_in_file_with_buf(path, pattern, ignore_case, 0, 0,
                            |line, line_num, _pre_lines, _post_lines| handler(line, line_num))
}

fn search_in_file_with_buf<F>(path: &str, pattern: &str, ignore_case: bool, pre_size: usize, post_size: usize, handler: F) -> usize
    where F: Fn(&str, usize, &[String], &[String]) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => { println!("Can not open file {}: {}", path, e); return 0; }
    };
    let mut count: usize = 0;
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();
    for (i, line) in lines.iter().enumerate() {
        let matches = if ignore_case {
            line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            line.contains(pattern)
        };
        if matches {
            count += 1;
            let pre_start = i.saturating_sub(pre_size);
            let pre_lines = &lines[pre_start..i].to_vec();

            let post_end = (i + 1 + post_size).min(lines.len());
            let post_lines = &lines[i + 1..post_end].to_vec();

            handler(line, i, pre_lines, post_lines);
        }
    }
    count
}

fn find_string(path: &str, pattern: &str, ignore_case: bool, is_recursive: bool, search_mode: &SearchMode) {
    let walk_dir = build_file_walker(path, is_recursive);
    let mut total_count: usize = 0;
    walk_dir
        .map(|e| e.path().display().to_string())
        .for_each(|file| {
            match search_mode {
                SearchMode::Normal => {
                    search_in_file(&file, pattern, ignore_case, |line, line_num| println!("{}:{}:{}", file, line_num, line));
                }
                SearchMode::FileNamesOnly => {
                    search_in_file(&file, pattern, ignore_case, |_line, _line_num| println!("{}", file));
                }
                SearchMode::CountPerFile => {
                    let count = search_in_file(&file, pattern, ignore_case, |_line, _line_num| {});
                    if count > 0 {
                        println!("{}:{}", file, count);
                    }
                }
                SearchMode::TotalCount => {
                    let count = search_in_file(&file, pattern, ignore_case, |_line, _line_num| {});
                    total_count += count;
                }
                SearchMode::PreMatch(size) => {
                    search_in_file_with_buf(&file, pattern, ignore_case, *size, 0, |line, line_num, pre, _post| {
                        for l in pre {
                            println!("{}:{}", file, l);
                        }
                        println!("{}:{}:{}", file, line_num + 1, line);
                    });
                }
                SearchMode::PostMatch(size) => {
                    search_in_file_with_buf(&file, pattern, ignore_case, 0, *size, |line, line_num, _pre, post| {
                        for l in post {
                            println!("{}:{}", file, l);
                        }
                        println!("{}:{}:{}", file, line_num + 1, line);
                    });
                }
            }
        });
    
    if matches!(search_mode, SearchMode::TotalCount) {
        println!("{}", total_count);
    }
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

pub fn perform_search(config: &SearchConfig) {
    let path = &config.path;
    let pattern = &config.pattern;
    let ignore_case = config.ignore_case;
    let find_files = config.find_file;
    let recursive = config.recursive;
    let search_mode = &config.search_mode;

    if find_files {
        find_file_with_names(path, pattern, ignore_case, recursive);
    } else {
        find_string(path, pattern, ignore_case, recursive, search_mode);
    }
}