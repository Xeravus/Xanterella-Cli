use log::info;

use crate::utils::check::*;
use crate::utils::config::*;

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
    create_templates_default();
    create_templates_profile();
    info!("[ OK ] - Init Prozess erfolgreich");
}

pub fn init_templates() {
    info!("[ RUN ] - Starte Init Prozess");

    config_create_subdir();
    create_templates_host();
    create_templates_modul();
    create_templates_default();
    create_templates_profile();
    info!("[ OK ] - Init Prozess erfolgreich");
}
