use log::{info, debug};

use crate::utils::core::*;
use crate::config::query::*;
use crate::config::filepaths::*;
use crate::config::colmena::change::*;
use crate::config::colmena::parse::*;
use crate::config::colmena::hosts::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn write_add_host(injection_path: &str, name: &str, ip: &str, remotebuilder: bool) {
    info!("[ RUN ] - Füge Host: {} hinzu zu Colmena", name);

    write_hosts(injection_path, &colmena_get_content(&injection_path), sort_hosts(colmena_add_host(injection_path, name, ip, remotebuilder)));
    files_alejandra(injection_path);
    info!("[ OK ] - Host: {} erfolgreich hinzugefügt zu Colmena", name);
}

pub fn write_remove_host(injection_path: &str, name: Option<&str>, ip: Option<&str>) {
    info!("[ RUN ] - Lösche Host aus Colmena");
    
    write_hosts(injection_path, &colmena_get_content(injection_path), sort_hosts(colmena_remove_host(colmena_parse_hosts(injection_path), name, ip)));
    files_alejandra(injection_path);
    info!("[ OK ] - Host erfolgreich gelöscht aus Colmena");
}

pub fn rewrite_hosts(injection_path: &str) {
    info!("[ RUN ] - Starte Reload der Colmena Hosts");

    debug!("rewrite_hosts(content): \n{:#?}\n - - - - - - - - - - - - - ", colmena_get_content(injection_path));
    debug!("rewrite_hosts(parsed hosts): \n{:#?}\n - - - - - - - - - - - - - - - - - ", colmena_parse_hosts(injection_path));
    write_hosts(injection_path, &colmena_get_content(injection_path), sort_hosts(colmena_parse_hosts(injection_path)));
    files_alejandra(injection_path);
    info!("[ OK ] - Reload der Colmena Hosts erfolgreich");
}

pub fn list_modules() {
    info!("Modules: ");
    for i in query_modules_all() {
        info!("{}", convert_filepath(&i, OutPath::Shortend, false));
    }
}

pub fn list_profiles() {
    info!("Profiles: ");
    for i in query_profiles() {
        info!("{}", convert_filepath(&i, OutPath::Shortend, false));
    }
}
