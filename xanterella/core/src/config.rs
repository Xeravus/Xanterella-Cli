use std::{
    env,
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{EventsFailed, Xanterella, xanterella::CommandRunner};

#[derive(PartialEq, Debug, Clone)]
pub struct AppPaths {
    pub config_file: PathBuf,
    pub database_url: String,
}

impl AppPaths {
    pub fn init() -> Self {
        let config_file = env::var("XANTERELLA_CONFIG").map(PathBuf::from).unwrap_or_else(|_| {
            let home = env::var("HOME").expect("Init-Fheler: HOME-Env-Variable nicht gesetzt. HOME Var wird genutzt, da XANTERELLA_CONFIG nicht gesetzt wurde");
            let path = PathBuf::from(home).join(".config/xanterella/config.toml");
            if let Some(parent) = path.parent() {
                create_dir_all(parent).ok();
            }
            path
        });

        let db_path = env::var("XANTERELLA_DB").map(PathBuf::from).unwrap_or_else(|_| {
            let home = env::var("HOME").expect("Init-Fheler: HOME-Env-Variable nicht gesetzt. HOME Var wird genutzt, da XANTERELLA_DB nicht gesetzt wurde");
            let path = PathBuf::from(home).join(".local/share/xanterella/xanterella.db");
            if let Some(parent) = path.parent() {
                create_dir_all(parent).ok();
            }
            path
        });

        let database_url = format!("sqlite://{}?mode=rwc", db_path.display());

        Self {
            config_file,
            database_url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub target_directory: String,
    pub ip: String,
    pub port: u64,
    pub repo_url: String,
    pub repo_main_branch: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            target_directory: "/etc/nixos".to_string(),
            ip: "0.0.0.0".to_string(),
            port: 3000,
            repo_url: "".to_string(),
            repo_main_branch: "main".to_string(),
        }
    }
}

impl AppConfig {
    pub fn load(path: &AppPaths) -> Self {
        if let Ok(content) = read_to_string(path.config_file.clone()) {
            match toml::from_str(&content) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!(
                        "Config-Fehler: Config.toml enthält nicht korrektes toml. \n{} \nFallback auf Standartwerte",
                        e
                    );
                    let old_config = format!("{}.old", path.config_file.display());
                    let _ = write(old_config, content);
                    let _ = Self::default().save(&AppPaths::init());
                    eprintln!(
                        "Config-Fehler: Ursprüngliche Config.toml wird zu 'config.toml.old' und neue Config.toml mit Standartwerte wird erstellt."
                    );
                    Self::default()
                }
            }
        } else {
            eprintln!("Config-Fehler: Config.toml existiert nicht. Fallback auf Standartwerte");
            let default_config = Self::default();
            let _ = default_config.save(path);
            default_config
        }
    }

    pub fn save(&self, path: &AppPaths) -> Result<(), std::io::Error> {
        let content = toml::to_string(self).expect("Config-Fehler: Content konnte nicht in toml umgewandelt werden");
        write(path.config_file.clone(), content)
    }
}

impl<C: CommandRunner> Xanterella<C> {
    pub fn load_config(&mut self) -> AppConfig {
        AppConfig::load(&self.config)
    }

    pub fn save_config(&mut self, content: AppConfig) -> Result<(), EventsFailed> {
        let config = content;
        config.save(&self.config).map_err(|err| EventsFailed::ConfigSave(err.to_string()))?;
        Ok(())
    }
}
