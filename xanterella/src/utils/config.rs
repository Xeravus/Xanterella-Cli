use log::{error, info};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::*;
use std::process::{self};

use crate::utils::get::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
}

pub fn config_create_dir() {
    info!("[ RUN ] - Erstelle Config Dir");

    fs::create_dir_all(get_path(Paths::Config)).unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte den Config Ordner nicht erstellen: {}", err);
        process::exit(1);
    });
    info!("[ OK ] - Config Dir erstellt");
}

pub fn config_create_subdir() {
    info!("[ RUN ] - Erstelle Config SubDir");

    fs::create_dir_all(format!("{}/templates", get_path(Paths::Config))).unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte den Config Ordner nicht erstellen: {}", err);
        process::exit(1);
    });
    info!("[ OK ] - Config SubDir erstellt");
}

pub fn config_gen_basic() {
    info!("[ RUN ] - Erstelle config.json");

    let basic = Data {
        tailkey: String::from("tskey-auth-XXXXXXXXXXXXXXXXX-YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"),
        wifi: String::from("Obi Wlan Kenobi"),
    };
    let json_string = serde_json::to_string_pretty(&basic).unwrap();
    let json_path = PathBuf::from(get_path(Paths::Config)).join("config.json").display().to_string();
    fs::write(&json_path, &json_string).expect("Konnte Datei nicht schreiben");
    info!("[ OK ] - config.json erstellt");
}

pub fn config_parse() -> Data {
    let json_path = PathBuf::from(get_path(Paths::Config)).join("config.json").display().to_string();
    let file_content = fs::read_to_string(&json_path).expect("Datei konnte nicht gelesen werden");
    let loaded_config: Data = serde_json::from_str(&file_content).unwrap();
    loaded_config
}

pub fn create_templates_host() {
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
}

pub fn create_templates_modul() {
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
}

pub fn create_templates_default() {
    let path = PathBuf::from(get_path(Paths::Config)).join("templates").join("default.nix");
    let content = 
        "
        {
        imports = [
        ];
        }
        ";

    fs::write(path, content).expect("Konnte Datei nicht schreiben");
}

pub fn create_templates_profile() {
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
}
