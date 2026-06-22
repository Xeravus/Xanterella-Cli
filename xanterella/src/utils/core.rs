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

pub fn files_alejandra(injection_path: &str) {
    info!("[ RUN ] - Starte Alejandra");

    let path = if injection_path == get_path(Paths::Colmena) {
        get_path(Paths::Nixconf)
    } else {
        injection_path.to_string()
    };
    println!("Path: {}", path);
    let alejandra = Command::new("alejandra")
        .arg(path)
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err);
            //process::exit(1);
            panic!("Alejandra 1");
        });
    debug!("Alejandra: \n{}", String::from_utf8_lossy(&alejandra.stdout));
    if !alejandra.status.success() {
        error!(
            "[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}",
            String::from_utf8_lossy(&alejandra.stderr)
        );
        //process::exit(1);
        panic!("Alejandra 2");
    }
    info!("[ OK ] - Alejandra erfolgreich");
}
