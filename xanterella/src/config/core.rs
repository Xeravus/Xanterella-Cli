use log::{info, debug};

use crate::utils::core::*;
use crate::config::query::*;
use crate::config::filepaths::*;
use crate::config::colmena::*;

use crate::config::xanterella::host::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", convert_filepath(&i, OutPath::Last, false));
    }
}

pub fn host_add(injection_path: &str, name: &str, ip: &str, remotebuilder: bool, sysversion: Option<f32>, hardware_string: Option<String>) {
    colmena_add(&convert_path(injection_path, Target::Colmena), name, ip, remotebuilder);
    xanterella_add_host(&convert_path(injection_path, Target::Hosts), name, sysversion, hardware_string);
}

pub fn host_remove(injection_path: &str, name: &str, ip: Option<&str>) {
    colmena_remove(&convert_path(injection_path, Target::Colmena), Some(name), ip);
    xanterella_remove_host(&convert_path(injection_path, Target::Hosts), name);
}

pub fn sort_host(injection_path: &str) {
    colmena_rewrite(&convert_path(injection_path, Target::Colmena));
    xanterella_sort_hosts(&convert_path(injection_path, Target::Hosts));
}

pub fn colmena_add(injection_path: &str, name: &str, ip: &str, remotebuilder: bool) {
    info!("[ RUN ] - Füge Host: {} hinzu zu Colmena", name);

    let mut colmena = ColmenaManager::init(injection_path);
    colmena.load();
    colmena.add_host(name, ip, remotebuilder);
    colmena.sort_hosts();
    colmena.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host: {} erfolgreich zu Colmena hinzugefügt", name);
}

pub fn colmena_remove(injection_path: &str, name: Option<&str>, ip: Option<&str>) {
    info!("[ RUN ] - Lösche Host aus Colmena");
    
    let mut colmena = ColmenaManager::init(injection_path);
    colmena.load();
    colmena.remove_host(name, ip);
    colmena.sort_hosts();
    colmena.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host erfolgreich gelöscht aus Colmena");
}

pub fn colmena_rewrite(injection_path: &str) {
    info!("[ RUN ] - Starte Reload der Colmena Hosts");

    let mut colmena = ColmenaManager::init(injection_path);
    colmena.load();
    colmena.sort_hosts();
    colmena.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Reload der Colmena Hosts erfolgreich");
}

pub fn xanterella_add_host(injection_path: &str, name: &str, sysversion: Option<f32>, hardware_string: Option<String>) {
    info!("[ RUN ] - Füge Host: {} zu Xanterella hinzu", name);

    let mut xanterella = XanterellaHostManager::init(injection_path);
    xanterella.load();
    xanterella.add_host(name, sysversion, hardware_string);
    xanterella.clean();
    xanterella.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host: {} erfolgrecih zu Xanterella hinzugefügt", name);
}

pub fn xanterella_remove_host(injection_path: &str, name: &str) {
    info!("[ RUN ] - Lösche Host aus Xanterella");
    
    let mut xanterella = XanterellaHostManager::init(injection_path);
    xanterella.load();
    xanterella.remove_host(name);
    xanterella.clean();
    xanterella.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Host erfolgreich gelöscht aus Colmena");
}

pub fn xanterella_sort_hosts(injection_path: &str) {
    info!("[ RUN ] - Starte Reload der Xanterella Hosts");

    let mut xanterella = XanterellaHostManager::init(injection_path);
    xanterella.load();
    xanterella.clean();
    xanterella.write();
    files_alejandra(injection_path);
    info!("[ OK ] - Reload der Xanterella Hosts erfolgreich");
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
