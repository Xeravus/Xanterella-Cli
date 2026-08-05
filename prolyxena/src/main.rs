use prolyxena::cli::commands::*;

fn main() {
    #[cfg(not(test))]
    cli_parse();
}
