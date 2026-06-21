use log::{info, error};

use std::process;

use crate::config::colmena::parse::*;

pub fn add_host(name: &str, ip: &str) {
    if check_for_host(name, ip) {
        error!("[ FAILED ] - Host existiert schon");
        process::exit(1);
    }
    info!("[ RUN ] - Füge Host Hinzu");
}

pub fn check_for_host(name: &str, ip: &str) -> bool {
    if !check_for_host_name(name) && !check_for_host_ip(ip) {
        false 
    } else {
        true
    }
}


pub fn check_for_host_name(name: &str) -> bool {
    for i in colmena_parse_hosts().hosts {
        if i.name == name {
            return true
        }
    }
    false
}

pub fn check_for_host_ip(ip: &str) -> bool {
    for i in colmena_parse_hosts().hosts {
        if i.ip == ip {
            return true
        }
    }
    false
}
