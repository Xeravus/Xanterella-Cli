use xanterella_cli::commands::cli_parse;

#[tokio::main]
pub async fn main() {
    cli_parse().await;
}
