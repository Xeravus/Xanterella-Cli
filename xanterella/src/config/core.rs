use log::{info, debug};

use crate::utils::core::*;
use crate::config::query::*;
use crate::config::filepaths::*;
use crate::config::colmena::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn colmena_add(injection_path: &str, name: &str, ip: &str, remotebuilder: bool) {
    info!("[ RUN ] - Füge Host: {} hinzu zu Colmena", name);

    let mut colmena = ColmenaFile::init(injection_path);
    colmena.load();
    colmena.add_host(name, ip, remotebuilder);
    colmena.sort_hosts();
    colmena.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host: {} erfolgreich hinzugefügt zu Colmena", name);
}

pub fn colmena_remove(injection_path: &str, name: Option<&str>, ip: Option<&str>) {
    info!("[ RUN ] - Lösche Host aus Colmena");
    
    let mut colmena = ColmenaFile::init(injection_path);
    colmena.load();
    colmena.remove_host(name, ip);
    colmena.sort_hosts();
    colmena.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host erfolgreich gelöscht aus Colmena");
}

pub fn colmena_rewrite(injection_path: &str) {
    info!("[ RUN ] - Starte Reload der Colmena Hosts");

    let mut colmena = ColmenaFile::init(injection_path);
    colmena.load();
    colmena.sort_hosts();
    colmena.write();
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
