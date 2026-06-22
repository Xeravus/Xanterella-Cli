use log::{info, error};

use std::process;

use crate::config::colmena::parse::*;

pub fn colmena_add_host(name: &str, ip: &str, remotebuilder: bool) -> ColmenaFile {
    if check_for_host(name, ip) {
        error!("[ FAILED ] - Host existiert schon");
        colmena_parse_hosts()
    } else {
        let mut input = colmena_parse_hosts();
        let imports: Vec<String> = vec![
            format!("./hosts/{}/configuration.nix", name),
            "./profile/ssh-keys.nix".to_string(),
        ];
        let add_host = ColmenaHost {
            name: name.to_string(),
            ip: ip.to_string(),
            remotebuilder,
            imports,
        };
        info!("[ RUN ] - Füge Host Hinzu");
        input.hosts.push(add_host);
        input
    }
}

pub fn colmena_remove_host(name: Option<&str>, ip: Option<&str>) -> ColmenaFile {
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
