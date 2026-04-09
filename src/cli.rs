use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct GrepUpCommand {
    #[arg[short='i', long, help="Ignore case"]]
    ignore_case: bool,

    #[arg[short='f', long, help="Find files by name"]]
    find_files: bool,

    #[arg[short='r', long, help="Recursively search"]]
    recursive: bool,

    #[arg[short='l', long, help="Show file names only"]]
    list_files: bool,

    #[arg[short='c', long, help="Show match count only"]]
    count: bool,

    #[arg[default_value="0", short='B', long, help="show the lines before match"]]
    pre_match: usize,

    #[arg[default_value="0", short='A', long, help="show the lines after match"]]
    post_match: usize,

    query_str: String,

    #[arg(default_value = ".", help="Folder or file to search.")]
    file_path: String
}

pub enum SearchMode {
    Normal,
    FileNamesOnly,
    CountPerFile,
    TotalCount,
    PreMatch(usize),
    PostMatch(usize),
}

pub struct SearchConfig {
    pub pattern: String,
    pub path: String,
    pub ignore_case: bool,
    pub find_file: bool,
    pub recursive: bool,
    pub search_mode: SearchMode,
}

fn derive_mode(cmd: &GrepUpCommand) -> SearchMode {
    if cmd.list_files && cmd.count {
        SearchMode::CountPerFile
    } else if cmd.list_files {
        SearchMode::FileNamesOnly
    } else if cmd.count {
        SearchMode::TotalCount
    } else if cmd.pre_match > 0 {
        SearchMode::PreMatch(cmd.pre_match)
    } else if cmd.post_match > 0 {
        SearchMode::PostMatch(cmd.post_match)
    }
    else {
        SearchMode::Normal
    }
}

pub fn prepare_config() -> SearchConfig {
    let cmd = GrepUpCommand::parse();
    let mode = derive_mode(&cmd);
    SearchConfig {
        pattern: cmd.query_str,
        path: cmd.file_path,
        ignore_case: cmd.ignore_case,
        find_file: cmd.find_files,
        recursive: cmd.recursive,
        search_mode: mode,
    }
}