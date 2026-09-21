use crate::prelude::*;
use std::fmt::Write;

use prolyxena::*;
use prolyxena::engine::lexer::vfs::*;
use prolyxena::engine::generator::query::{SearchObjekt, SearchContent};
use prolyxena::engine::generator::query::Query;
use prolyxena::engine::core::NixValue;
use serde::*;
use serde_json::Value;

pub struct Nixtractor<'a> {
    pub prolyxena: &'a mut FsData,
}

#[derive(Debug, Deserialize)]
pub struct CreateHost {
    pub hostname: String,
    pub ip: String, 
    pub profiles: Vec<Value>,
    pub options: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProfile {
    pub name: String,
    pub dir: String,
    pub options: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateModul {
    pub name: String,
    pub desc: String,
    pub category: String,
    pub options: Vec<Value>,
}

impl<'a> Nixtractor<'a> {
    pub async fn new(prolyxena: &'a mut FsData) -> Self {
        Nixtractor {
            prolyxena,
        }
    }

    pub async fn extract_hosts(&mut self) -> Result<Vec<CreateHost>, String> {
        let mut hosts: Vec<CreateHost> = Vec::new();
        let mut conf_file_path = String::with_capacity(128);

        let hosts_dir = self.prolyxena.fsnodes.search_dir("hosts")?;
        if hosts_dir.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if hosts_dir.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };

        let app_files = self.list_hosts_apps_files()?;
        let hosts_files = self.prolyxena.fsnodes.dir_list_files(&hosts_dir[0])?;
        if hosts_files.is_empty() {
            return Err("Query-Fehler: Keine Dateien im Hosts Ordner".to_string());
        }

        println!("Hosts: {:#?}", hosts_files);
        for i in hosts_files {
            let name = i.split('/').last().expect("Extracting Hosts: Konnte das letzte Element nicht extrahieren");
            conf_file_path.clear();
            write!(&mut conf_file_path, "hosts/{}/configuration.nix", name).unwrap();
            println!("Path: {}\nName: {}", conf_file_path, name);
            let config_file = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Warnung: Überspringe Host '{}' - {}", name, e);
                    continue;
                }
            };
            println!("Found: Path: {} Name: {}", conf_file_path, name);
            let imports_node = config_file.query_exact_mut(&["imports"]);
            let mut profiles = Vec::new();
            let mut options = Vec::new();
            if let Some(NixValue::List(l)) = imports_node.first() {
                for j in l {
                    if let NixValue::Path(p) = j {
                        profiles.push(serde_json::json!(p));
                    }
                }
            }

            let app = app_files.iter().find(|f| f.contains(&i));
            if let Some(o) = app {
                let app_path = format!("profiles/apps/{}", o);
                if let Ok(file) = self.prolyxena.search_tree(&app_path) {
                    let config_node = file.query_exact_mut(&["xanterella"]);
                    if let Some(NixValue::AttrSet(map)) = config_node.first() {
                        options.push(serde_json::json!(map));
                    }
                } else {
                    eprintln!("Extracting Hosts: App-Datei '{}' konnte im VFS unter '{}' nicht geladen werden.", o, app_path);
                }
            }

            hosts.push(CreateHost {
                hostname: name.to_string(),
                ip: name.to_string(),
                profiles,
                options,
            });
        }
        Ok(hosts)
    }

    fn list_hosts_apps_files<'b>(&'b mut self) -> Result<Vec<String>, String> {
        let mut dir = self.prolyxena.fsnodes.search_dir("apps")?;
        dir.retain(|f| f.contains("profiles"));

        let mut path = "";
        if let Some(first) = dir.first() {
            if first.is_empty() {
                return Err("Extracting Profiles: Keine Dateien in profiles/apps/ gefunden".to_string());
            }
            path = first;
        } else {
            return Err("Extracting Profiles: Konnte das erste Element nicht extrahieren".to_string());
        }
        self.prolyxena.fsnodes.dir_list_files(path)
    }

    pub async fn extract_profiles(&mut self) -> Result<Vec<CreateProfile>, String> {
        let mut profiles: Vec<CreateProfile> = Vec::new();

        let profiles_dir = self.prolyxena.fsnodes.search_dir("profiles")?;
        if profiles_dir.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if profiles_dir.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };

        let files = self.prolyxena.fsnodes.dir_list_files(&profiles_dir[0])?;
        if files.is_empty() {
            return Err("Query-Fehler: Keine Dateien im Profile Ordner".to_string());
        }

        for i in files {
            if i.ends_with(".nix") {
                let mut options = Vec::new();
                let name = match i.split('/').last() {
                    Some(p) => p.trim_end_matches(".nix"),
                    None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                };
                let config_file = self.prolyxena.search_tree(&i)?;
                let profile_node = config_file.query_exact_mut(&["xanterella"]);
                if let Some(NixValue::AttrSet(map)) = profile_node.first() {
                        options.push(serde_json::json!(map));
                }
                profiles.push(CreateProfile {
                    name: name.to_string(),
                    dir: String::from("base"),
                    options,
                });
            } else {
                let files_depth = self.prolyxena.fsnodes.dir_list_files(&i)?;
                for j in files_depth {
                    let mut options = Vec::new();
                    let dir = i.split('/').last().expect("Query-Fehler: Konnte die Kategory des Modules nicht extrahieren");

                    let name = match j.split('/').last() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let config_file = self.prolyxena.search_tree(&j)?;
                    let profile_node = config_file.query_exact_mut(&["xanterella"]);
                    if let Some(NixValue::AttrSet(map)) = profile_node.first() {
                            options.push(serde_json::json!(map));
                    }
                    profiles.push(CreateProfile {
                        name: name.to_string(),
                        dir: dir.to_string(),
                        options,
                    });
                }
            }
        }
        Ok(profiles)
    }

    pub async fn extract_modules(&mut self) -> Result<Vec<CreateModul>, String> {
        let mut modules: Vec<CreateModul> = Vec::new();
        let modul_dir = self.prolyxena.fsnodes.search_dir("modules")?;
        if modul_dir.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if modul_dir.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };
        
        let dirs = self.prolyxena.fsnodes.dir_list_files(&modul_dir[0])?;
        if dirs.is_empty() {
            return Err("Query-Fehler: Keine Dateien/Ordner im Modul Ordner".to_string());
        }

        for i in dirs {
            let files = self.prolyxena.fsnodes.dir_list_files(&i)?;
            for j in files {
                if j.ends_with(".nix") {
                    let mut options = Vec::new();
                    let category = i.split('/').last().expect("Query-Fehler: Konnte die Kategory des Modules nicht extrahieren");
                    let name = match j.split('/').last() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let config_file = self.prolyxena.search_tree(&j)?;
                    let modul_node = config_file.query_exact_mut(&["config"]);
                    if let Some(NixValue::AttrSet(map)) = modul_node.first() {
                            options.push(serde_json::json!(map));
                    }
                    modules.push(CreateModul {
                        name: name.to_string(),
                        desc: String::new(),
                        category: category.to_string(),
                        options,
                    });
                } else {
                    let files_depth = self.prolyxena.fsnodes.dir_list_files(&j)?;
                    for k in files_depth {
                        let mut options = Vec::new();
                        let category = j.split('/').last().expect("Query-Fehler: Konnte die Kategory des Modules nicht extrahieren");
                        let name = match k.split('/').last() {
                            Some(p) => p.trim_end_matches(".nix"),
                            None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                        };
                        let config_file = self.prolyxena.search_tree(&k)?;
                        let modul_node = config_file.query_exact_mut(&["config"]);
                        if let Some(NixValue::AttrSet(map)) = modul_node.first() {
                                options.push(serde_json::json!(map));
                        }
                        modules.push(CreateModul {
                            name: name.to_string(),
                            desc: String::new(),
                            category: category.to_string(),
                            options,
                        });
                    }
                }
            }
        }
        Ok(modules)
    }
}
