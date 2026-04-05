mod file_ops;

use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct GrepUpCommand {
    #[arg[short, long, help="ignore case"]]
    i: bool,

    #[arg[short, long, help="find files"]]
    f: bool,
    
    #[arg[short, long, help="recursively search for"]]
    r: bool,
    
    query_str: String,

    #[clap(default_value = "*", help="Folder or file to search.")]
    file_path: String
}

struct OptionConfig {
    pattern: String,
    ignore_case: bool,
    find_file: bool,
    recursive: bool,
    path: String
}

fn parse_arguments(command: &GrepUpCommand) -> OptionConfig {
    OptionConfig {
        pattern: command.query_str.clone(),
        path: command.file_path.clone(),
        ignore_case: command.i,
        find_file: command.f,
        recursive: command.r
    }
}

fn main() {
    let args = GrepUpCommand::parse();
    let option_config= parse_arguments(&args);
    file_ops::perform_search(&option_config);
}
