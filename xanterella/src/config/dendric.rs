use log::{info, error};

use std::fs;
use std::path::PathBuf;
use std::process;

pub fn create_host(injection_path: &str, name: &str) {
    info!("[ RUN ] - Erstelle Ordner für den Host: {}", name);

    let path = PathBuf::from(injection_path).join(name);
    fs::create_dir_all(path)
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte Ordner nicht erstellen: {}", err);
            process::exit(1);
        });
    info!("[ OK ] - Ordner für den Host: {} erfolgreich erstellt", name);
}

/*
pub fn fill_folder(injection_path: &str, name: &str) {
    info!("[ RUN ] - Befülle den Host Ordner");

    let copy_path = get_path(Paths::Templates).join("host.nix");
    let dest_path = PathBuf::from(injection_path).join(name);
    fs::copy(copy_path, dest_path);
}
*/
