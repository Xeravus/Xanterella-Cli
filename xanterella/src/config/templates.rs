use log::{error, info};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::*;
use std::process::{self};

use crate::utils::get::*;

pub enum Template {
    Host,
    Profile,
    Modul,
    Index,
}

pub fn parse_template(template: Template) -> String {
    match template {
        Template::Host => {
            fs::read_to_string(PathBuf::from(get_path(Paths::Config)).join("templates").join("host.nix")).expect("Fehler")
        }
        Template::Profile => {
            fs::read_to_string(PathBuf::from(get_path(Paths::Config)).join("templates").join("profile.nix")).expect("Fehler")
        }
        Template::Modul => {
            fs::read_to_string(PathBuf::from(get_path(Paths::Config)).join("templates").join("modul.nix")).expect("Fehler")
        }
        Template::Index => {
            fs::read_to_string(PathBuf::from(get_path(Paths::Config)).join("templates").join("index.nix")).expect("Fehler")
        }
    }
}

pub fn create_templates_host() {
    info!("[ RUN ] - Erstelle Host Template");
    let path = PathBuf::from(get_path(Paths::Config)).join("templates").join("host.nix");
    let content = 
        "
        {
        config,
        lib,
        pkgs,
        ...
        }: {
        imports = [
        ];
        networking = {
        hostName = ;
        };
        system = {
        stateVersion = ;
        };
        }
        ";

    fs::write(path, content).expect("Konnte Datei nicht schreiben");
    info!("[ OK ] - Host Template erfolgreich erstellt");
}

pub fn create_templates_modul() {
    info!("[ RUN ] - Erstelle Modul Template");
    let path = PathBuf::from(get_path(Paths::Config)).join("templates").join("modul.nix");
    let content = 
        "
        {
        config,
        lib,
        pkgs,
        ...
        }: {
        options = {
        xanterella = {
        };
        };
        }
        ";

    fs::write(path, content).expect("Konnte Datei nicht schreiben");
    info!("[ OK ] - Modul Template erfolgreich erstellt");
}

pub fn create_templates_index() {
    info!("[ RUN ] - Erstelle Index Template");
    let path = PathBuf::from(get_path(Paths::Config)).join("templates").join("index.nix");
    let content = 
        "
        {
        imports = [
        ];
        }
        ";

    fs::write(path, content).expect("Konnte Datei nicht schreiben");
    info!("[ OK ] - Index Template erfolgreich erstellt");
}

pub fn create_templates_profile() {
    info!("[ RUN ] - Erstelle Profil Template");
    let path = PathBuf::from(get_path(Paths::Config)).join("templates").join("profile.nix");
    let content = 
        "
        {
        config,
        lib,
        ...
        }: {
        config = {
        xanterella = {
        };
        };
        }
        ";

    fs::write(path, content).expect("Konnte Datei nicht schreiben");
    info!("[ OK ] - Profil Template erfolgreich erstellt");
}
