use log::{info, error};
use walkdir::WalkDir;

use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};

#[derive(Debug)]
pub struct XanterellaHostManager {
    pub hosts: Vec<XanterellaHost>,
    pub injection_path: String,
}

#[derive(Debug)]
pub struct XanterellaHost {
    pub name: String,
    pub sysversion: f32,
    pub imports: Vec<String>,
    pub finished_string: String,
    pub hardware_string: String,
    pub extra_attributes: Vec<String>,
    pub injection_path: String,
}

impl XanterellaHostManager {
    pub fn init(injection_path: &str) -> Self {
        Self {
            hosts: vec![],
            injection_path: injection_path.to_string(),
        }
    }

    pub fn load(&mut self) {
        let hosts = self.list_hosts();
        self.alejandra();
        for i in hosts {
            let path = PathBuf::from(&self.injection_path).join(i).join("configuration");
            let content = fs::read_to_string(path).unwrap();
            self.hosts.push(XanterellaHost::new_from_content(&self.injection_path, &content))
        }
    }

    pub fn alejandra(&self) {
        let alejandra = Command::new("alejandra")
            .arg(&self.injection_path)
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte Alejandra nicht starten: {}", err);
                //process::exit(1);
                panic!("Alejandra 1");
            });
        if !alejandra.status.success() {
            error!(
                "[ FAILED ] - Konnte die Dateien mit Alejandra nicht formatieren: {}",
                String::from_utf8_lossy(&alejandra.stderr)
            );
            //process::exit(1);
            panic!("Alejandra 2");
        }
        info!("[ OK ] - Alejandra erfolgreich");
    }

    pub fn list_hosts(&self) -> Vec<String> {
        WalkDir::new(&self.injection_path)
            .min_depth(1)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_dir())
            .filter_map(|entry| {
                entry.path().to_str().map(|s| s.to_string())
            })
            .collect()
    }

    pub fn check_for_host(&self, name: &str) -> bool {
        self.hosts.iter().any(|h| h.name == name)
    }

    pub fn add_host(&mut self, name: &str, sysversion: Option<f32>, hardware_string: Option<String>,) {
        let sys_version = match sysversion {
            Some(version) => version,
            None => 25.05,
        };
        if !self.check_for_host(name) {
            self.hosts.push(XanterellaHost::new(&self.injection_path, name, sys_version, vec![], hardware_string, None))
        } else {
            error!("[ FAILED ] - Host existiert schon");
        }
    }

    pub fn remove_host(&mut self, name: &str) {
        self.hosts.retain(|host| {
            host.name != name
        });
    }

    pub fn write(&self) {
        for i in &self.hosts {
            i.create_dir();
            i.create_file();
        }
    }

    pub fn clean(&self) {
        for i in &self.hosts {
            i.remove_dir()
        }
    }
}

impl XanterellaHost {
    pub fn new(injection_path: &str, name: &str, sysversion: f32, input_imports: Vec<String>, hardware_string: Option<String>, extra_attributes: Option<Vec<String>>) -> Self {
        let imports = Self::gen_imports(input_imports);
        let hardware = match hardware_string {
            Some(string) => string,
            None => String::new(),
        };
        let extra: Vec<String> = match extra_attributes {
            Some(string) => string,
            None => vec![],
        };
        let finished_string = Self::gen_finished_string(name, sysversion, &imports, &extra);
        Self {
            name: name.to_string(),
            sysversion,
            imports,
            finished_string,
            hardware_string: hardware,
            extra_attributes: extra,
            injection_path: injection_path.to_string(),
        }
    }

    pub fn new_from_content(injection_path: &str, content: &str) -> Self {
        let name = Self::extract_name(content);
        let sysversion = Self::extract_sysversion(content);
        let imports = Self::extract_imports(content);
        let hardware = Self::extract_hardware(injection_path, &name);
        let extra_attributes = Self::extract_extra_attributes(content, &imports);
        let finished_string = Self::gen_finished_string(&name, sysversion, &imports, &extra_attributes);
        Self {
            name,
            sysversion,
            imports,
            finished_string,
            hardware_string: hardware,
            extra_attributes,
            injection_path: injection_path.to_string(),
        }
    }

    pub fn create_dir(&self) {
        let path = PathBuf::from(&self.injection_path).join(&self.name);
        fs::create_dir_all(path)
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte Ordner nicht erstellen: {}", err);
                process::exit(1);
            });
    }

    pub fn create_file(&self) {
        let config = PathBuf::from(&self.injection_path).join(&self.name).join("configuration.nix");
        let hardware = PathBuf::from(&self.injection_path).join(&self.name).join("hardware-configuration.nix");
        let _ = fs::write(config, &self.finished_string);
        let _ = fs::write(hardware, &self.hardware_string);
    }

    pub fn remove_dir(&self) {
        let path = PathBuf::from(&self.injection_path).join(&self.name);
        fs::remove_dir_all(path)
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte Ordner nicht löschen: {}", err);
                process::exit(1);
            });
    }

    pub fn extract_name(content: &str) -> String {
        let (_rest, network) = content.split_once("hostName = ").unwrap();
        let (name, _rest) = network.split_once(";").unwrap();
        name.replace('"', "")
    }

    pub fn extract_sysversion(content: &str) -> f32 {
        let (_rest, network) = content.split_once("stateVersion = ").unwrap();
        let (version, _rest) = network.split_once(";").unwrap();
        let number = version.replace('"', "").trim().parse::<f32>().unwrap();
        number
    }

    pub fn extract_imports(content: &str) -> Vec<String> {
        let (_rest, conf) = content.split_once("imports = [").unwrap();
        let (imports, _rest) = conf.split_once("];").unwrap();
        let mut output: Vec<String> = vec![];
        for i in imports.trim().lines() {
            output.push(i.trim().to_string())
        }
        output
    }

    pub fn extract_hardware(injection_path: &str, name: &str) -> String {
        let path = PathBuf::from(injection_path).join(name).join("hardware-configuration.nix");
        match fs::read_to_string(path) {
            Ok(output) => output,
            Err(_) => String::new(),
        }
    }

    pub fn extract_extra_attributes(content: &str, imports: &Vec<String>) -> Vec<String> {
        let (start, _rest) = content.split_once("networking = {").unwrap();
        let (_rest, extra) = start.split_once(&imports.join("\n")).unwrap();
        let mut output: Vec<String> = vec![];
        for i in extra.trim().lines() {
            output.push(i.trim().to_string())
        }
        output
    }

    pub fn gen_imports(input_imports: Vec<String>) -> Vec<String> {
        let mut output_vec: Vec<String> = vec![
            "./hardware-configuration.nix".to_string(),
            "./../../modules".to_string(),
            "./../../profiles/boot/boot.nix".to_string(),
        ];
        output_vec.extend(input_imports);
        output_vec.sort();
        output_vec.dedup();
        output_vec
    }

    pub fn gen_finished_string(name: &str, sysversion: f32, imports: &Vec<String>, extra_attributes: &Vec<String>) -> String {
        format!("
        {{
        config,
        pkgs,
        lib,
        ...
        }}: {{
        imports = [
        {}
        ];
        {}
        networking = {{
        hostName = \"{}\";
        }};
        system = {{
        stateVersion = \"{}\";
        }};
        }}",
        imports.join("\n"), extra_attributes.join("\n"), name, sysversion)
    }
}

/*
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
*/
