use log::{info, error};

use std::process;

use crate::config::colmena::parse::*;
use crate::utils::core::*;

pub fn colmena_add_host(injection_path: &str, name: &str, ip: &str, remotebuilder: bool) -> ColmenaFile {
    if check_for_host(injection_path, name, ip) {
        error!("[ FAILED ] - Host existiert schon");
        colmena_parse_hosts(injection_path)
    } else {
        let mut input = colmena_parse_hosts(injection_path);
        let imports: Vec<String> = vec![
            format!("./hosts/{}/configuration.nix", name),
            "./profiles/ssh-keys.nix".to_string(),
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

pub fn colmena_remove_host(content: ColmenaFile, name: Option<&str>, ip: Option<&str>) -> ColmenaFile {
    let mut output = content;
    output.hosts.retain(|host| {
        match (name, ip) {
            (Some(search_name), None) => host.name != search_name,
            (None, Some(search_ip)) => host.ip != search_ip,
            (Some(search_name), Some(search_ip)) => host.name != search_name && host.ip != search_ip,
            (None, None) => true,
        }
    });
    output
}

pub fn check_for_host(injection_path: &str, name: &str, ip: &str) -> bool {
    if !check_for_host_name(injection_path, name) && !check_for_host_ip(injection_path, ip) {
        false 
    } else {
        true
    }
}


pub fn check_for_host_name(injection_path: &str, name: &str) -> bool {
    for i in colmena_parse_hosts(injection_path).hosts {
        if i.name == name {
            return true
        }
    }
    false
}

pub fn check_for_host_ip(injection_path: &str, ip: &str) -> bool {
    for i in colmena_parse_hosts(injection_path).hosts {
        if i.ip == ip {
            return true
        }
    }
    false
}
