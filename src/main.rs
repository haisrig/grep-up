mod file_ops;
mod cli;

fn main() {
    let config= cli::prepare_config();
    file_ops::perform_search(&config);
}
