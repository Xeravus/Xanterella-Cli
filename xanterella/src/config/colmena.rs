use log::{info, error, debug};

use std::process::{self, Command};
use std::fs;

use crate::utils::core::*;

#[derive(Debug)]
pub struct ColmenaFile {
    pub hosts: Vec<ColmenaHost>,
    pub injection_path: String,
    pub content: String,
}

#[derive(Debug)]
pub struct ColmenaHost {
    pub name: String,
    pub ip: String,
    pub builder: bool,
    pub imports: Vec<String>,
    pub deploy_block: String,
    pub finished_string: String,
}

impl ColmenaFile {
    pub fn init(injection_path: &str) -> Self {
        Self {
            hosts: vec![],
            injection_path: injection_path.to_string(),
            content: String::new(),
        }
    }

    pub fn load(&mut self) {
        self.get_content();
        self.parse_hosts();
    }

    pub fn get_content(&mut self) {
        self.alejandra();
        self.content = fs::read_to_string(&self.injection_path).expect("[ FAILED ] - Konnte die Colmena Host Datei nicht auslesen");
    }

    pub fn gen_finished_string(&self) -> String {
        let mut output = vec![];
        for i in &self.hosts {
            output.push(i.finished_string.clone())
        }
        output.join("\n")
    }

    pub fn parse_hosts(&mut self) {
        for i in Self::content_split_hosts(&self.content_split_marker()) {
            self.hosts.push(ColmenaHost::new_from_content(&i))
        };
    }

    pub fn write(&self) {
        let _ = fs::write(&self.injection_path, &self.content_replace());
        self.alejandra();
    }

    pub fn content_replace(&self) -> String {
        let output = self.content.replace(&self.content_split_marker(), &self.gen_finished_string());
        output
    }

    fn content_split_marker(&self) -> String {
        let (_start, hosts_start) = self.content
            .split_once("# --- Xanterella Hosts Start ---")
            .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts Start ---)");
        let (host_final, _end) = hosts_start
            .split_once("# --- Xanterella Hosts End ---")
            .expect("[ FAILED ] - Konnte die Colmena Hosts nicht zerschneiden(# --- Xanterella Hosts End ---)");
        host_final.to_string()
    }

    pub fn content_split_hosts(content: &str) -> Vec<String> {
        let teile: Vec<&str> = content.trim().split("];\n  };\n").collect();
        let mut output: Vec<String> = vec![];
        debug!("colmena_split_hosts(input): \n{:#?}\n - - - - - - - - - - - - - - - - - - - - - - - - ", output);
        for i in teile {
            if !i.trim().is_empty() {
                output.push(format!("{} ]; }}; ", i.trim()))
            }
        }
        debug!("colmena_split_hosts(output): \n{:#?}\n - - - - - - - - - - - - - - - - - - - - - - - - ", output);
        output
    }

    pub fn check_for_host(&self, name: Option<&str>, ip: Option<&str>) -> bool {
        self.hosts.iter().any(|host| {
            return match (name, ip) {
                (Some(search_name), None) => host.name == search_name,
                (None, Some(search_ip)) => host.ip == search_ip,
                (Some(search_name), Some(search_ip)) => host.name == search_name && host.ip == search_ip,
                (None, None) => false,
            };
        })
    }

    pub fn sort_hosts(&mut self) {
        self.hosts.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn add_host(&mut self, name: &str, ip: &str, builder: bool) {
        if !self.check_for_host(Some(name), Some(ip)) {
            self.hosts.push(ColmenaHost::new(name, ip, builder, vec![]))
        } else {
            error!("[ FAILED ] - Host existiert schon");
        }
    }

    pub fn remove_host(&mut self, name: Option<&str>, ip: Option<&str>) {
        self.hosts.retain(|host| {
            let matches = match (name, ip) {
                (Some(search_name), None) => host.name == search_name,
                (None, Some(search_ip)) => host.ip == search_ip,
                (Some(search_name), Some(search_ip)) => host.name == search_name && host.ip == search_ip,
                (None, None) => false,
            };
            !matches
        })
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
}

impl ColmenaHost {
    pub fn new(name: &str, ip: &str, builder: bool, input_imports: Vec<String>) -> Self {
        let imports = Self::gen_imports(name, input_imports);
        let deploy_block = Self::gen_deploy_block(builder, ip);
        let finished_string = Self::gen_finished_string(name, ip, &deploy_block, &imports);
        Self {
            name: name.to_string(),
            ip: ip.to_string(),
            builder,
            imports,
            deploy_block,
            finished_string,
        }
    }

    pub fn new_from_content(content: &str) -> Self {
        let name = Self::extract_name(content);
        let ip = Self::extract_ip(content);
        let builder = Self::extract_builder(content);
        let imports = Self::extract_imports(content);
        let deploy_block = Self::gen_deploy_block(builder, &ip);
        let finished_string = Self::gen_finished_string(&name, &ip, &deploy_block, &imports);
        Self {
            name,
            ip,
            builder,
            imports,
            deploy_block,
            finished_string,
        }
    }

    fn extract_name(content: &str) -> String {
        let (name, _rest) = content.trim().split_once(" = {").expect(&format!("[ FAILED ] - Fehler beim extrahieren des Names: {}", content));
        name.trim().to_string()
    }

    fn extract_ip(content: &str) -> String {
        let (_rest, ip_teil) = content.split_once("targetHost = ").unwrap();
        let (ip, _rest) = ip_teil.split_once(";").unwrap();
        ip.replace('"', "")
    }

    fn extract_builder(content: &str) -> bool {
        let (_rest, remote_teil) = content.split_once("buildOnTarget = ").unwrap();
        let (remote, _rest) = remote_teil.split_once(";").unwrap();
        let remote_builder = remote.trim().parse::<bool>().unwrap();
        remote_builder
    }

    fn extract_imports(content: &str) -> Vec<String> {
        let (_rest, remote_teil) = content.split_once("imports = [").unwrap();
        let (remote, _rest) = remote_teil.split_once("];").unwrap();
        let mut output: Vec<String> = vec![];
        for i in remote.trim().lines() {
            output.push(i.trim().to_string())
        }
        output
    }

    fn gen_imports(name: &str, input_imports: Vec<String>) -> Vec<String> {
        let mut output_vec: Vec<String> = vec![
            format!("./hosts/{}/configuration.nix", name),
            "./profiles/ssh-keys.nix".to_string(),
        ];
        output_vec.extend(input_imports);
        output_vec.sort();
        output_vec.dedup();
        output_vec
    }

    fn gen_deploy_block(builder: bool, ip: &str) -> String {
        if builder {
            String::from("targetHost = null;\nallowLocalDeployment = true;\nbuildOnTarget = true;")
        } else {
            format!("targetHost = \"{}\";\nkeys = commonSSHKeys;\nbuildOnTarget = false;", ip)
        }
    }

    fn gen_finished_string(name: &str, ip: &str, deploy_block: &str, imports: &Vec<String>) -> String {
        format!("
        {} = {{
        deployment = {{
        {}
        }};
        imports = [
        {}
        ];
        }};\n",
        name, deploy_block, imports.join("\n"))
    }
}

#[cfg(test)]
#[path = "colmena_test.rs"]
mod test;
