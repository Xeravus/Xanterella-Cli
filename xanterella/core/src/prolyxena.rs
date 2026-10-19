use std::fmt::Write;

use prolyxena::engine::{
    core::NixValue,
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

            let flake_hosts     = match self.prolyxena.search_tree(&conf_file_path) { 
                Ok(h) => h,
                Err(err) => {
                    eprintln!("Extraction-Fehler: Hosts.nix konnte nicht gefunden werden: {}", err);
                    continue;
                },
            };

            let with_system = flake_hosts.query_exact_mut(&["flake", "nixosConfigurations", host_name]); 
            //check_single(&with_system)?;

            dbg!(with_system);

            conf_file_path.clear();
            write!(&mut conf_file_path, "{}/hosts.nix", flake_dir[0]).unwrap(); 

            let colmena_hosts   = match self.prolyxena.search_tree(&conf_file_path) {
                Ok(h) => h,
                Err(err) => {
                    eprintln!("Extraction-Fehler: Colmena-hosts.nix konnte nicht gefunden werden: {}", err);
                    continue;
                },
            };

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
}

pub(crate) fn check_single<T>(list: &[T]) -> Result<(), String> {
    match list.len() {
        0 => Err("Query-Fehler: Kein Pfad/Option gefunden".to_string()),
        1 => Ok(()), 
        _ => Err("Query-Fehler: Zu viele Pfade/Optionen gefunden".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use prolyxena::engine::{core::NixValue, lexer::vfs::FsNodes};

    use super::*;

    fn dummy_file(name: &str, ast: NixValue) -> FsNodes {
        FsNodes::File {
            name: name.to_string(),
            ast,
        }
    }

    fn dummy_dir(items: Vec<(&str, FsNodes)>) -> FsNodes {
        let mut map = IndexMap::new();
        for (k, v) in items {
            map.insert(k.to_string(), v);
        }
        FsNodes::Dir(map)
    }

    #[tokio::test]
    async fn test_nixtractor_extract_hosts_success() {
        let mut fs_data = FsData::new("/fake");

        let config_ast = NixValue::AttrSet(IndexMap::from([(
            "imports".to_string(),
            NixValue::List(vec![NixValue::Path("./hardware.nix".to_string())]),
        )]));

        let app_ast = NixValue::AttrSet(IndexMap::from([(
            "config".to_string(),
            NixValue::AttrSet(IndexMap::from([(
                "xanterella".to_string(),
                NixValue::AttrSet(IndexMap::from([("enable".to_string(), NixValue::Bool(true))])),
            )])),
        )]));

        fs_data.fsnodes = dummy_dir(vec![
            (
                "hosts",
                dummy_dir(vec![(
                    "test-node",
                    dummy_dir(vec![("configuration.nix", dummy_file("configuration.nix", config_ast))]),
                )]),
            ),
            (
                "profiles",
                dummy_dir(vec![(
                    "apps",
                    dummy_dir(vec![("test-node-apps.nix", dummy_file("test-node-apps.nix", app_ast))]),
                )]),
            ),
        ]);

        let mut extractor = Nixtractor::new(&mut fs_data).await;
        let hosts = extractor.extract_hosts().await.unwrap();

        assert_eq!(hosts.len(), 0);
    }

    #[tokio::test]
    async fn test_nixtractor_extract_profiles_success() {
        let mut fs_data = FsData::new("/fake");

        let profile_ast = NixValue::AttrSet(IndexMap::from([(
            "xanterella".to_string(),
            NixValue::AttrSet(IndexMap::from([("is_server".to_string(), NixValue::Bool(true))])),
        )]));

        fs_data.fsnodes = dummy_dir(vec![(
            "profiles",
            dummy_dir(vec![
                ("base.nix", dummy_file("base.nix", profile_ast.clone())),
                ("server", dummy_dir(vec![("web.nix", dummy_file("web.nix", profile_ast))])),
            ]),
        )]);

        let mut extractor = Nixtractor::new(&mut fs_data).await;
        let profiles = extractor.extract_profiles().await.unwrap();

        assert_eq!(profiles.len(), 2);

        let base = profiles.iter().find(|p| p.name == "base").unwrap();
        assert_eq!(base.dir, "base");
        assert_eq!(base.options.len(), 1);

        let web = profiles.iter().find(|p| p.name == "web").unwrap();
        assert_eq!(web.dir, "server");
        assert_eq!(web.options.len(), 1);
    }

    #[tokio::test]
    async fn test_nixtractor_extract_modules_success() {
        let mut fs_data = FsData::new("/fake");

        let mod_ast = NixValue::AttrSet(IndexMap::from([(
            "config".to_string(),
            NixValue::AttrSet(IndexMap::from([("sys".to_string(), NixValue::Bool(true))])),
        )]));

        fs_data.fsnodes = dummy_dir(vec![(
            "modules",
            dummy_dir(vec![(
                "system",
                dummy_dir(vec![
                    ("network.nix", dummy_file("network.nix", mod_ast.clone())),
                    ("deep", dummy_dir(vec![("storage.nix", dummy_file("storage.nix", mod_ast))])),
                ]),
            )]),
        )]);

        let mut extractor = Nixtractor::new(&mut fs_data).await;
        let modules = extractor.extract_modules().await.unwrap();

        assert_eq!(modules.len(), 2);

        let network = modules.iter().find(|m| m.name == "network").unwrap();
        assert_eq!(network.category, "system");
        assert_eq!(network.options.len(), 0);

        let storage = modules.iter().find(|m| m.name == "storage").unwrap();
        assert_eq!(storage.options.len(), 0);
    }

    #[tokio::test]
    async fn test_nixtractor_empty_vfs_errors() {
        let mut fs_data = FsData::new("/fake");
        fs_data.fsnodes = dummy_dir(vec![]);

        let mut extractor = Nixtractor::new(&mut fs_data).await;

        let hosts_err = extractor.extract_hosts().await;
        assert!(hosts_err.is_err());
        assert!(hosts_err.unwrap_err().contains("Kein Pfad gefunden"));

        let profiles_err = extractor.extract_profiles().await;
        assert!(profiles_err.is_err());

        let modules_err = extractor.extract_modules().await;
        assert!(modules_err.is_err());
    }
}
