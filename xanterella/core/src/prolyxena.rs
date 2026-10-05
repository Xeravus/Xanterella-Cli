use std::fmt::Write;

use prolyxena::engine::core::LambdaTypes;
use prolyxena::engine::core::NixValue;
use prolyxena::engine::formater::core::Format;
use prolyxena::engine::generator::query::Query;
use prolyxena::engine::generator::query::{SearchContent, SearchObjekt};
use prolyxena::engine::lexer::vfs::*;
use prolyxena::*;
use serde::*;
use serde_json::Value;

use crate::prelude::*;

pub struct Nixtractor<'a> {
    pub prolyxena: &'a mut FsData,
}

#[derive(Debug, Serialize, Deserialize)]
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

        let dir_host = self.prolyxena.fsnodes.search_dir("hosts")?;
        if dir_host.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if dir_host.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };

        let app_files = self.list_hosts_apps_files()?;
        let files_of_hosts = self.prolyxena.fsnodes.dir_list_files(&dir_host[0])?;
        if files_of_hosts.is_empty() {
            return Err("Query-Fehler: Keine Dateien im Hosts Ordner".to_string());
        }

        for i in files_of_hosts {
            let mut profiles = Vec::new();
            let mut options = Vec::new();

            let host_name = i.split('/').last().expect("Extracting Hosts: Konnte das letzte Element nicht extrahieren");
            conf_file_path.clear();
            write!(&mut conf_file_path, "hosts/{}/configuration.nix", host_name).unwrap();
            let config_file = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Warnung: Überspringe Host '{}' - {}", host_name, e);
                    continue;
                }
            };

            let filtered_content = config_file.query_exact_mut(&["imports"]);
            if let Some(NixValue::List(list)) = filtered_content.first() {
                for item in list.iter() {
                    profiles.push(serde_json::json!(item));
                }
            }

            let host_app_file = app_files.iter().find(|f| f.contains(&i));

            if let Some(o) = host_app_file {
                let file_path = format!("profiles/apps/{}", o);
                if let Ok(file) = self.prolyxena.search_tree(&file_path) {
                    let filtered_content = file.query_exact_mut(&["config", "xanterella"]);
                    if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
                        options.push(serde_json::json!(map));
                    }
                } else {
                    eprintln!(
                        "Extracting Hosts: App-Datei '{}' konnte im VFS unter '{:#?}' nicht geladen werden.",
                        o, host_app_file
                    );
                }
            }

            hosts.push(CreateHost {
                hostname: host_name.to_string(),
                ip: host_name.to_string(),
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

        let dir_profiles = self.prolyxena.fsnodes.search_dir("profiles")?;
        if dir_profiles.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if dir_profiles.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };

        let profiles_files = self.prolyxena.fsnodes.dir_list_files(&dir_profiles[0])?;
        if profiles_files.is_empty() {
            return Err("Query-Fehler: Keine Dateien im Profile Ordner".to_string());
        }

        let mut files_with_full_name = Vec::new();
        for i in profiles_files {
            files_with_full_name.push(format!("profiles/{}", i));
        }

        for i in files_with_full_name {
            if i.ends_with(".nix") {
                let mut options = Vec::new();
                let name = match i.split('/').last() {
                    Some(p) => p.trim_end_matches(".nix"),
                    None => return Err(
                        "Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)"
                            .to_string(),
                    ),
                };
                let profile_file = self.prolyxena.search_tree(&i)?;
                let filtered_content = profile_file.query_exact_mut(&["xanterella"]);
                if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
                    options.push(serde_json::json!(map));
                }
                profiles.push(CreateProfile {
                    name: name.to_string(),
                    dir: String::from("base"),
                    options,
                });
            } else {
                let mut files_with_full_name_inner = Vec::new();
                let files_depth = self.prolyxena.fsnodes.dir_list_files(&i)?;

                for j in files_depth {
                    files_with_full_name_inner.push(format!("{}/{}", i, j));
                }

                for j in files_with_full_name_inner {
                    let mut options = Vec::new();
                    let dir =
                        i.split('/').last().expect("Query-Fehler: Konnte die Kategory des Modules nicht extrahieren");

                    let name = match j.split('/').last() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let profile_file = self.prolyxena.search_tree(&j)?;
                    let filtered_content = profile_file.query_exact_mut(&["xanterella"]);
                    if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
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

        let main_dir = self.prolyxena.fsnodes.search_dir("modules")?;
        if main_dir.is_empty() {
            return Err("Query-Fehler: Kein Pfad gefunden".to_string());
        } else if main_dir.len() > 1 {
            return Err("Query-Fehler: Zu viele Pfade gefunden".to_string());
        };

        let dirs = self.prolyxena.fsnodes.dir_list_files(&main_dir[0])?;
        if dirs.is_empty() {
            return Err("Query-Fehler: Keine Dateien/Ordner im Modul Ordner".to_string());
        }

        for i in dirs {
            let full_dir_name = format!("modules/{}", i);

            let files = if !i.ends_with(".nix") {
                self.prolyxena.fsnodes.dir_list_files(&full_dir_name)?
            } else {
                continue;
            };

            for j in files {
                let file_name = format!("{}/{}", full_dir_name, j);
                if file_name.ends_with(".nix") {
                    let mut options = Vec::new();
                    let name = match file_name.split('/').last() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let modul_file = self.prolyxena.search_tree(&file_name)?;
                    let filtered_content = modul_file.query_exact_mut(&["config"]);
                    if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
                        options.push(serde_json::json!(map));
                    }
                    modules.push(CreateModul {
                        name: name.to_string(),
                        desc: String::new(),
                        category: i.to_string(),
                        options,
                    });
                } else {
                    let files_depth = self.prolyxena.fsnodes.dir_list_files(&file_name)?;
                    for k in files_depth {
                        let mut options = Vec::new();
                        let name = match k.split('/').last() {
                            Some(p) => p.trim_end_matches(".nix"),
                            None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                        };
                        let file_name_inner = format!("{}/{}", file_name, k);
                        let config_file = self.prolyxena.search_tree(&file_name_inner)?;
                        let modul_node = config_file.query_exact_mut(&["config"]);
                        if let Some(NixValue::AttrSet(map)) = modul_node.first() {
                            options.push(serde_json::json!(map));
                        }
                        modules.push(CreateModul {
                            name: name.to_string(),
                            desc: String::new(),
                            category: j.to_string(),
                            options,
                        });
                    }
                }
            }
        }
        Ok(modules)
    }
}
