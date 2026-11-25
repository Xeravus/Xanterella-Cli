#[allow(unused)]
use prolyxena::cli::commands::cli_parse;

fn main() {
    #[cfg(not(test))]
    cli_parse();
}
