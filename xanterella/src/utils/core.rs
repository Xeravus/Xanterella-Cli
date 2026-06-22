use log::{info, debug, error};

use std::process::{self, Command};

use crate::utils::check::*;
use crate::utils::get::*;
use crate::utils::config::*;
use crate::config::templates::*;

pub fn ping_full(ip: &str) {
    info!("[ RUN ] - Starte Ping Tests");

    ping(ip);
    ping_ssh(ip);
    info!("[ OK ] - Ping Tests erfolgreich");
}

pub fn init() {
    info!("[ RUN ] - Starte Init Prozess");

    config_create_dir();
    config_create_subdir();
    config_gen_basic();
    create_templates_host();
    create_templates_modul();
    create_templates_index();
    create_templates_profile();
    info!("[ OK ] - Init Prozess erfolgreich");
}

pub fn init_templates() {
    info!("[ RUN ] - Starte Init Prozess");

    config_create_subdir();
    create_templates_host();
    create_templates_modul();
    create_templates_index();
    create_templates_profile();
    info!("[ OK ] - Init Prozess erfolgreich");
}

pub fn files_alejandra() {
    info!("[ RUN ] - Starte Alejandra");

    let alejandra =
        Command::new("alejandra").arg(".").current_dir(get_path(Paths::Nixconf)).output().unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err);
            process::exit(1);
        });
    debug!("Alejandra: \n{}", String::from_utf8_lossy(&alejandra.stdout));
    if !alejandra.status.success() {
        error!(
            "[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}",
            String::from_utf8_lossy(&alejandra.stderr)
        );
        process::exit(1);
    }
    info!("[ OK ] - Alejandra erfolgreich");
}
