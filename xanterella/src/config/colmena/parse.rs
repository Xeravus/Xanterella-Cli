use log::debug;

use std::fs;

use crate::utils::get::*;
use crate::utils::core::*;

#[derive(Debug)]
pub struct ColmenaFile {
    pub hosts: Vec<ColmenaHost>,
}

#[derive(Debug)]
pub struct ColmenaHost {
    pub name: String,
    pub ip: String,
    pub remotebuilder: bool,
    pub imports: Vec<String>,
}

pub fn colmena_get_content(injection_path: &str) -> String {
    files_alejandra();
    fs::read_to_string(injection_path).expect("[ FAILED ] - Konnte die Colmena Host Datei nicht auslesen")
}

pub fn colmena_parse_hosts(injection_path: &str) -> ColmenaFile {
    let mut output = ColmenaFile {
        hosts: vec![],
    };
    for i in colmena_split_hosts(&colmena_split_marker(&colmena_get_content(injection_path))) {
        let host = ColmenaHost {
            name: colmena_extract_name(&i),
            ip: colmena_extract_ip(&i),
            remotebuilder: colmena_extract_remotebuilder(&i),
            imports: colmena_extract_imports(&i),
        };
        output.hosts.push(host)
    }
    output
}

pub fn colmena_split_marker(content: &str) -> String {
    let (_start, hosts_start) = content
        .split_once("# --- Xanterella Hosts Start ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts Start ---)");
    let (host_final, _end) = hosts_start
        .split_once("# --- Xanterella Hosts End ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts End ---)");
    host_final.to_string()
}

pub fn colmena_split_hosts(content: &str) -> Vec<String> {
    let teile: Vec<&str> = content.trim().split("
    ];\n  };\n
  ").collect();
    let mut output: Vec<String> = vec![];
    debug!("colmena_split_hosts(input): \n{:#?}\n - - - - - - - - - - - - - - - - - - - - - - - - ", output);
    for i in teile {
        output.push(format!("{} ]; }}; ", i))
    }
    debug!("colmena_split_hosts(output): \n{:#?}\n - - - - - - - - - - - - - - - - - - - - - - - - ", output);
    output
}

pub fn colmena_extract_name(content: &str) -> String {
    let (name, _rest) = content.trim().split_once(" = {").expect(&format!("[ FAILED ] - Fehler beim extrahieren des Names: {}", content));
    name.trim().to_string()
}

pub fn colmena_extract_ip(content: &str) -> String {
    let (_rest, ip_teil) = content.split_once("targetHost = ").unwrap();
    let (ip, _rest) = ip_teil.split_once(";").unwrap();
    ip.replace('"', "")
}

pub fn colmena_extract_remotebuilder(content: &str) -> bool {
    let (_rest, remote_teil) = content.split_once("buildOnTarget = ").unwrap();
    let (remote, _rest) = remote_teil.split_once(";").unwrap();
    let remote_builder = remote.trim().parse::<bool>().unwrap();
    remote_builder
}

pub fn colmena_extract_imports(content: &str) -> Vec<String> {
    let (_rest, remote_teil) = content.split_once("imports = [").unwrap();
    let (remote, _rest) = remote_teil.split_once("];").unwrap();
    let mut output: Vec<String> = vec![];
    for i in remote.trim().lines() {
        output.push(i.trim().to_string())
    }
    output
}

#[cfg(test)]
#[path = "parse_test.rs"]
mod tests;
