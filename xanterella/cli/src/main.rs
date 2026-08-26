mod commands;
mod execute;
mod ui_elements;

use commands::cli_parse;

#[tokio::main]
pub async fn main() {
    cli_parse().await;
}
