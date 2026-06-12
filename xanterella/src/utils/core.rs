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
    config_gen_basic();
    info!("[ OK ] - Init Prozess erfolgreich");
}
