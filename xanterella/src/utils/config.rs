use serde::{Deserialize, Serialize};
use log::{info, error};

use std::path::*; 
use std::fs;
use std::process::{self};

use crate::utils::get::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub tailkey: String,
    pub wifi: String,
}

pub fn config_create_dir() {
    info!("[ RUN ] - Erstelle Config Dir");

    fs::create_dir_all(get_path(Paths::Config))
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte den Config Ordner nicht erstellen: {}", err); 
            process::exit(1); 
        });
    info!("[ OK ] - Config Dir erstellt");
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
