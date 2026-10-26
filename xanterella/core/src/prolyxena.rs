use std::fmt::Write;

use prolyxena::engine::{
    core::NixValue,
    formater::core::Format,
    generator::query::{Query, SearchContent, SearchObjekt},
    lexer::vfs::*,
};
use serde::*;
use serde_json::Value;

pub struct Nixtractor<'a> {
    pub prolyxena: &'a mut FsData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHost {
    pub hostname: String,
    pub ip: String,
    pub system: String,
    pub remote_deploy: bool,
    pub profiles: Vec<Value>,
    pub hardware: Vec<Value>,
    pub options: Vec<Value>,
    pub config: Vec<Value>,
    pub imports: Vec<Value>,
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
    pub config: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplate {
    pub name: String,
    pub path: String,
    pub config: Vec<Value>,
}

impl<'a> Nixtractor<'a> {
    pub async fn new(prolyxena: &'a mut FsData) -> Self {
        Nixtractor {
            prolyxena,
        }
    }

    /// This function is used to extract all information from the Nix Config to generate an output
    /// design for the Database.
    /// This Output includes:
    ///     Name(hostname): The Name of the Client,
    ///     IP:             The Ip of the Client(currently it uses the hostname(DNS)),
    ///     System:         The Type of System of the Client(x86_64-linux for example),
    ///     Remote-Deploy:  If the Client is being remote deployed via Colmena,
    ///     Profiles:       What Profiles are imported,
    ///     Hardware:       A Copy of the hardware-configuration.nix,
    ///     Config:         A Copy of the configuration.nix,
    ///     Imports:        The Modules/Imports in the hosts.nix and colmena-hosts.nix
    pub async fn extract_hosts(&mut self) -> Result<Vec<CreateHost>, String> {
        let mut hosts: Vec<CreateHost> = Vec::new();
        let mut conf_file_path = String::with_capacity(128);

        let dir_host = self.prolyxena.fsnodes.search_dir("hosts")?;
        check_single(&dir_host)?;

        let app_files = self.list_hosts_apps_files()?;
        let files_of_hosts = self.prolyxena.fsnodes.dir_list_files(&dir_host[0])?;
        if files_of_hosts.is_empty() {
            return Err("Query-Fehler: Keine Dateien im Hosts Ordner".to_string());
        }

        for i in files_of_hosts {
            let mut profiles = Vec::new();
            let mut hardware = Vec::new();
            let mut options = Vec::new();
            let mut config = Vec::new();
            let mut imports = Vec::new();
            let mut system = String::new();
            let mut remote_deploy = false;

            let host_name =
                i.split('/').next_back().expect("Extracting Hosts: Konnte das letzte Element nicht extrahieren");
            conf_file_path.clear();
            write!(&mut conf_file_path, "hosts/{}/configuration.nix", host_name).unwrap();
            let config_file = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Warnung: Überspringe Host '{}' - {}", host_name, e);
                    continue;
                }
            };

            config.push(serde_json::json!(config_file.clone()));
            let filtered_content = config_file.query_exact_mut(&["imports"]);
            if let Some(NixValue::List(list)) = filtered_content.first() {
                for item in list.iter() {
                    profiles.push(serde_json::json!(item));
                }
            }

            conf_file_path.clear();
            write!(&mut conf_file_path, "hosts/{}/hardware-configuration.nix", host_name).unwrap();
            let hardware_file = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!(
                        "Extraction-Fehler: Keine Hardware-Configuration File found for Host: '{}'. {}",
                        host_name, e
                    );
                    continue;
                }
            };
            hardware.push(serde_json::json!(hardware_file));

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

            let flake_dir = self.prolyxena.fsnodes.search_dir("flake-modules")?;
            check_single(&flake_dir)?;

            conf_file_path.clear();
            write!(&mut conf_file_path, "{}/hosts.nix", flake_dir[0]).unwrap();

            let flake_hosts = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(h) => h,
                Err(err) => {
                    eprintln!("Extraction-Fehler: Hosts.nix konnte nicht gefunden werden: {}", err);
                    continue;
                }
            };

            if check_single(&flake_hosts.query_exact_mut(&["flake", "nixosConfigurations", &host_name])).is_ok() {
                let with_system = flake_hosts.query_exact_mut(&["flake", "nixosConfigurations", &host_name, "system"]);
                match check_single(&with_system) {
                    Ok(_) => {
                        let sys_string = with_system[0].format_nix(0);
                        let mut sys_chars = sys_string.trim().chars();
                        sys_chars.next();
                        sys_chars.next_back();
                        system.push_str(sys_chars.as_str());
                    }
                    Err(err) => {
                        eprintln!("Extraction-Fehler: Host {} hat kein angegebenes System: {}", host_name, err);
                        continue;
                    }
                };

                let hosts_imports =
                    flake_hosts.query_exact_mut(&["flake", "nixosConfigurations", &host_name, "modules"]);
                match check_single(&hosts_imports) {
                    Ok(_) => {
                        if let Some(NixValue::List(list)) = hosts_imports.first() {
                            for item in list.iter() {
                                imports.push(serde_json::json!(item));
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Extraction-Fehler: Host {} hat keine Imports/Module: {}", host_name, err);
                        continue;
                    }
                }
            } else {
                eprintln!(
                    "Extraction-Fehler: Host {} hat keinen Eintrag in der Hosts.nix. Colmena-Fallback",
                    host_name
                );

                conf_file_path.clear();
                write!(&mut conf_file_path, "{}/colmena-hosts.nix", flake_dir[0]).unwrap();
                let colmena_hosts = match self.prolyxena.search_tree(&conf_file_path) {
                    Ok(c) => c,
                    Err(err) => {
                        eprintln!("Extraction-Fehler: Colmena-hosts.nix konnte nicht gefunden werden: {}", err);
                        continue;
                    }
                };

                system.push_str("x86_64-linux");

                let hosts_imports = colmena_hosts.query_exact_mut(&[&host_name, "imports"]);
                match check_single(&hosts_imports) {
                    Ok(_) => {
                        if let Some(NixValue::List(list)) = hosts_imports.first() {
                            for item in list.iter() {
                                imports.push(serde_json::json!(item));
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "Extraction-Fehler: Host {} hat auch keinen Eintrag in der Colmena-hosts.nix: {}",
                            host_name, err
                        );
                        continue;
                    }
                }
            }

            conf_file_path.clear();
            write!(&mut conf_file_path, "{}/colmena-hosts.nix", flake_dir[0]).unwrap();

            let colmena_hosts = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(h) => h,
                Err(err) => {
                    eprintln!("Extraction-Fehler: Colmena-hosts.nix konnte nicht gefunden werden: {}", err);
                    continue;
                }
            };

            let build_on_target = colmena_hosts.query_exact_mut(&[&host_name, "deployment", "buildOnTarget"]);
            match check_single(&build_on_target) {
                Ok(_) => {
                    let outcome = build_on_target[0].format_nix(0);
                    if outcome.contains("true") {
                        remote_deploy = true;
                    } else if outcome.contains("false") {
                    } else {
                        eprintln!(
                            "Extraction-Fehler: Host {} hat keinen validen Eintrag zu Remote-Deployment. Wird nicht als Builder eingetragen",
                            host_name
                        );
                    }
                }
                Err(err) => {
                    eprintln!(
                        "Extraction-Fehler: Host {} hat keinen Eintrag zu Remote-Deployment. Wird nicht als Builder eingetragen: {}",
                        host_name, err
                    );
                }
            }

            hosts.push(CreateHost {
                hostname: host_name.to_string(),
                ip: host_name.to_string(),
                system: system.to_string(),
                remote_deploy,
                profiles,
                hardware,
                options,
                config,
                imports,
            });
        }
        Ok(hosts)
    }

    fn list_hosts_apps_files(&mut self) -> Result<Vec<String>, String> {
        let mut dir = self.prolyxena.fsnodes.search_dir("apps")?;
        dir.retain(|f| f.contains("profiles"));

        let path;
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
        check_single(&dir_profiles)?;

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
                let name = match i.split('/').next_back() {
                    Some(p) => p.trim_end_matches(".nix"),
                    None => return Err(
                        "Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)"
                            .to_string(),
                    ),
                };
                let profile_file = self.prolyxena.search_tree(&i)?;
                options.push(serde_json::json!(profile_file));
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
                    let dir = i
                        .split('/')
                        .next_back()
                        .expect("Query-Fehler: Konnte die Kategory des Modules nicht extrahieren");

                    let name = match j.split('/').next_back() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let profile_file = self.prolyxena.search_tree(&j)?;
                    options.push(serde_json::json!(profile_file));
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
        check_single(&main_dir)?;

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
                if j.contains("default.nix") {
                    continue;
                }
                let file_name = format!("{}/{}", full_dir_name, j);
                if file_name.ends_with(".nix") {
                    let mut options = Vec::new();
                    let mut config = Vec::new();
                    let name = match file_name.split('/').next_back() {
                        Some(p) => p.trim_end_matches(".nix"),
                        None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                    };
                    let modul_file = self.prolyxena.search_tree(&file_name)?;
                    let filtered_content = modul_file.query_exact_mut(&["options", "xanterella"]);
                    if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
                        options.push(serde_json::json!(map));
                    }
                    config.push(serde_json::json!(modul_file));
                    modules.push(CreateModul {
                        name: name.to_string(),
                        desc: String::new(),
                        category: i.to_string(),
                        options,
                        config,
                    });
                } else {
                    let files_depth = self.prolyxena.fsnodes.dir_list_files(&file_name)?;
                    for k in files_depth {
                        if k.contains("default.nix") {
                            continue;
                        }
                        let category = format!("{}/{}", i, j);
                        let mut options = Vec::new();
                        let mut config = Vec::new();
                        let name = match k.split('/').next_back() {
                            Some(p) => p.trim_end_matches(".nix"),
                            None => return Err("Query-Fehler: Datei hat keine gültige Dateiendung(konnte nicht angemessen entfernt werden)".to_string()),
                        };
                        let file_name_inner = format!("{}/{}", file_name, k);
                        let modul_file = self.prolyxena.search_tree(&file_name_inner)?;
                        let filtered_content = modul_file.query_exact_mut(&["options", "xanterella"]);
                        if let Some(NixValue::AttrSet(map)) = filtered_content.first() {
                            options.push(serde_json::json!(map));
                        }
                        config.push(serde_json::json!(modul_file));
                        modules.push(CreateModul {
                            name: name.to_string(),
                            desc: String::new(),
                            category,
                            options,
                            config,
                        });
                    }
                }
            }
        }
        Ok(modules)
    }

    pub async fn extract_templates(&mut self) -> Result<Vec<CreateTemplate>, String> {
        let mut templates: Vec<CreateTemplate> = Vec::new();

        let flake_path = self.prolyxena.fsnodes.search_file("flake.nix")?;
        check_single(&flake_path)?;
        let flake = self.prolyxena.search_tree(&flake_path[0])?;
        templates.push(CreateTemplate {
            name: "flake.nix".to_string(),
            path: flake_path[0].to_string(),
            config: vec![serde_json::json!(flake)],
        });

        let flake_dir = self.prolyxena.fsnodes.search_dir("flake-modules")?;
        check_single(&flake_dir)?;
        let mut flake_parts = self.prolyxena.fsnodes.dir_list_files(&flake_dir[0])?;
        if flake_parts.is_empty() {
            return Err("Query-Fehler: Keine flake-parts(dateien im \"flake-modules\" Ordner".to_string());
        }
        flake_parts.retain(|f| !f.contains("host"));
        for i in flake_parts {
            dbg!(&i);
            let path = format!("flake-modules/{}", i);
            let template_file = self.prolyxena.search_tree(&path)?;
            templates.push(CreateTemplate {
                name: i.to_string(),
                path,
                config: vec![serde_json::json!(template_file)],
            });
        }

        Ok(templates)
    }
}

pub(crate) fn check_single<T>(list: &[T]) -> Result<(), String> {
    match list.len() {
        0 => Err("Query-Fehler: Kein Pfad/Option gefunden".to_string()),
        1 => Ok(()),
        _ => Err("Query-Fehler: Zu viele Pfade/Optionen gefunden".to_string()),
    }
}
