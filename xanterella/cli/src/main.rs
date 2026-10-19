pub mod commands;
pub mod execute;
pub mod server;

use commands::cli_parse;

#[tokio::main]
pub async fn main() {
    cli_parse().await;
}
