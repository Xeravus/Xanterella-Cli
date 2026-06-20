use clap::{Parser, Subcommand};


use crate::utils::core::*;
use crate::utils::debug::{list_debug, ListDebug};
use crate::usb::core::*;
use crate::installer::core::*;
use crate::daemon::daemon::*;

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Ping {
        ip: String,
    },
    Clean,
    Debug {
        #[arg(value_enum)]
        option: ListDebug,
    },
    RemoteInstall {
        #[arg(long = "automate", short = 'a')]
        automate: bool,
        #[arg(long = "fast", short = 'f')]
        fast: bool,
        #[arg(long = "debug", short = 'd')]
        debug: bool,
    },
    Flash,
    Daemon {
        #[arg(long = "automate", short = 'a')]
        automate: bool,
        #[arg(long = "fast", short = 'f')]
        fast: bool,
        #[arg(long = "init", short = 'i')]
        init: bool,
        #[arg(long = "debug", short = 'd')]
        debug: bool,
    },
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .init();
    match &cli.command {
        Commands::Init => {
            init();
        },
        Commands::Ping { ip } => {
            ping_full(ip);
        },
        Commands::Clean => {
            clean();
        },
        Commands::Debug { option } => {
            list_debug(option);
        },
        Commands::RemoteInstall { automate, fast, debug } => {
            remote_install(automate, fast, debug);
        },
        Commands::Flash => {
            flash_usb(false);
        },
        Commands::Daemon { automate, fast, init, debug } => {
            start_daemon(automate, fast, init, debug).await;
        },
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
