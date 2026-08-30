use clap::{Parser, Subcommand};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

use crate::execute::*;

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Ping {
        ip: String,
    },
    RemoteInstall {
        #[arg(short = 'a', long = "automate")]
        automate: bool,
        #[arg(short = 's', long = "speed")]
        speed: bool,
        #[arg(short = 'd', long = "debug")]
        debug: bool,
        #[arg(short = 'f', long = "flake-dir")]
        flake: String,
    },
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            execute_init_config().await;
        }
        Commands::Ping {
            ip,
        } => {
            let xanterella = Xanterella::new();
            let mut installer = XanterellaInstall::new(xanterella);
            installer.set_ip(ip);
            let _ = installer.ping();
        }
        Commands::RemoteInstall {
            automate,
            speed,
            debug,
            flake,
        } => {
            execute_remote_install(*automate, *speed, *debug, flake).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
