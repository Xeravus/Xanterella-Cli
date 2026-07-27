use xanterella::cli::commands::cli_parse;

#[tokio::main]
pub async fn main() {
    cli_parse().await;
}
