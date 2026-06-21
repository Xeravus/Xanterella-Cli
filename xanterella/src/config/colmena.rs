use crate::utils::get::*;

pub struct ColmenaFile {
    hosts: Vec<ColmenaHost>,
}

pub struct ColmenaHost {
    name: String,
    ip: String,
    remotebuilder: bool,
    imports: Vec<String>,
}

/*
pub fn parse_colmena_hosts() -> ColmenaFile {
    let content = fs::read_to_string(get_path(Paths::Colmena)).expect("[ FAILED ] - Konnte die Colmena Host Datei nicht auslesen");
}
*/

pub fn colmena_split_marker(content: String) -> String {
    let (_start, hosts_start) = content
        .split_once("# --- Xanterella Hosts Start ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts Start ---)");
    let (host_final, _end) = hosts_start
        .split_once("# --- Xanterella Hosts End ---")
        .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts End ---)");
    host_final.to_string()
}

