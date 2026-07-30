use clap::{Parser, Subcommand};

use xanterella_core::{Xanterella, XanterellaInstall, Config, Ping};

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
    },
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => {
            let mut xanterella = Xanterella::new();
            let _ = xanterella.config_create_dir();
            let _ = xanterella.config_gen_basic();
        }
        Commands::Ping {
            ip,
        } => {
            let mut xanterella = Xanterella::new();
            let mut installer = XanterellaInstall::new(&mut xanterella);
            installer.set_ip(ip);
            let _ = installer.ping();
        }
        Commands::RemoteInstall {
            automate,
            speed,
            debug,
        } => {
            let mut xanterella = Xanterella::new();
            let mut installer = XanterellaInstall::new(&mut xanterella);
            installer.xanterella.set_automate(*automate);
            installer.xanterella.set_fast(*speed);
            installer.xanterella.set_debug(*debug);
            let _ = installer.remote_integration();
            let _ = installer.remote_prep_fs();
            let _ = installer.remote_install();
            let _ = installer.remote_install_cleanup();
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
