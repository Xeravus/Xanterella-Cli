use clap::{Parser, Subcommand};
use xanterella_core::{Ping, Xanterella, XanterellaInstall};

use crate::{
    execute::*,
    server::{
        import::{import_all, import_hosts, import_modules, import_profiles, import_raw, import_templates},
        requests::{request_check, request_clear, request_generate, request_wipe},
    },
};

#[derive(Parser)]
#[command(name = "Xanterella")]
#[command(about = "Verwaltung der Nix & Nixos Configuration von Xanterella für einen und mehrere Hosts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Ping {
        ip: String,
    },
    Extract {
        flake: String,
    },
    Import {
        #[arg(short = 'H', long)]
        hosts: bool,
        #[arg(short, long)]
        profiles: bool,
        #[arg(short, long)]
        modules: bool,
        #[arg(short, long)]
        templates: bool,
        #[arg(short, long)]
        raw: bool,
        #[arg(short, long)]
        all: bool,

        flake: String,
        url: String,
    },
    Clear {
        url: String,
    },
    Generate {
        url: String,
    },
    Wipe {
        url: String,
    },
    Check {
        url: String,
    },
}

pub async fn cli_parse() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Ping {
            ip,
        } => {
            let xanterella = Xanterella::new();
            let mut installer = XanterellaInstall::new(xanterella);
            installer.set_ip(ip);
            let _ = installer.ping();
        }
        Commands::Extract {
            flake,
        } => {
            execute_extract(flake).await;
        }
        Commands::Import {
            flake,
            url,
            hosts,
            profiles,
            modules,
            templates,
            raw,
            all: _,
        } => {
            if *hosts {
                import_hosts(flake, url).await;
            } else if *profiles {
                import_profiles(flake, url).await;
            } else if *modules {
                import_modules(flake, url).await;
            } else if *templates {
                import_templates(flake, url).await;
            } else if *raw {
                import_raw(flake, url).await;
            } else {
                import_all(flake, url).await;
            }
        }
        Commands::Clear {
            url,
        } => {
            request_clear(url).await;
        }
        Commands::Generate {
            url,
        } => {
            request_generate(url).await;
        }
        Commands::Wipe {
            url,
        } => {
            request_wipe(url).await;
        }
        Commands::Check {
            url,
        } => {
            request_check(url).await;
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
