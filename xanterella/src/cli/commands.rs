use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        _ => todo!(),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
