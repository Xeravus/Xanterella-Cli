use clap::{Parser, Subcommand, ValueEnum};

use crate::daemon::daemon::*;
use crate::installer::core::*;
use crate::usb::core::*;
use crate::config::core::*;
use crate::utils::core::*;
use crate::utils::debug::{ListDebug, list_debug};

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short = 'v', long = "verbose", global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    InitTemplates,
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
    Config {
        #[command(subcommand)]
        subsubcommand: Config,
    },
}

#[derive(Subcommand)]
pub enum Config {
    List {
        #[arg(value_enum)]
        to_list: ToList,
    },
    AddHost {
        name: String,
        ip: String,
    },
    AddModul {
        name: String,
        dir: String,
    },
    AddProfil {
        name: String,
        dir: String,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ToList {
    Hosts,
    Modules,
    Profiles,
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    let log_level = if cli.verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info };
    env_logger::builder()
        .filter_level(log_level)
        .format_target(false)
        .format_timestamp(None)
        .format_level(false)
        .init();
    match &cli.command {
        Commands::Init => {
            init();
        }
        Commands::InitTemplates => {
            init_templates();
        }
        Commands::Ping { ip } => {
            ping_full(ip);
        }
        Commands::Clean => {
            clean();
        }
        Commands::Debug { option } => {
            list_debug(option);
        }
        Commands::RemoteInstall { automate, fast, debug } => {
            remote_install(automate, fast, debug);
        }
        Commands::Flash => {
            flash_usb(false);
        }
        Commands::Daemon { automate, fast, init, debug } => {
            start_daemon(*automate, *fast, *init, *debug).await;
        }
        Commands::Config { subsubcommand } => {
            match subsubcommand {
                Config::List { to_list } => {
                    match to_list {
                        ToList::Hosts => list_hosts(),
                        ToList::Modules => list_modules(),
                        ToList::Profiles => list_profiles(),
                    }
                }
                Config::AddHost { name, ip } => {
                }
                Config::AddModul { name, dir } => {
                }
                Config::AddProfil { name, dir } => {
                }
            }
        }
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
