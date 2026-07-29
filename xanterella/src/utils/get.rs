use std::env;

use crate::prelude::*;

pub enum User {
    Root,
    Cato,
}

pub enum Paths {
    Nixconf,
    Config,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Drives {
    pub blockdevices: Vec<BlockDevice>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct BlockDevice {
    pub name: String,
    pub size: String,

    #[serde(rename = "type")]
    pub device_type: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Taildevices {
    #[serde(rename = "Peer")]
    pub devices: HashMap<String, DeviceInfo>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    #[serde(rename = "HostName")]
    pub name: String,
    #[serde(rename = "TailscaleIPs")]
    pub ip: Vec<String>,
    #[serde(rename = "OS")]
    pub os: String,
}

pub trait Get {
    fn get_path(&mut self, path: Paths) -> String;
    fn get_taildevices() -> Result<Taildevices, EventsFailed>;
    fn get_taildevices_specific(devices: Taildevices, name: &str, active_installs: &HashSet<String>) -> Vec<String>;
}

impl Get for Xanterella {
    fn get_path(&mut self, path: Paths) -> String {
        let home = if self.home.is_empty() {
            env::var("HOME").expect("Konnte die Home Varialbe nichht extrahieren")
        } else {
            self.home.clone()
        };
        let config = PathBuf::from(&home).join(".config").join("xanterella");
        let result: PathBuf = match path {
            Paths::Nixconf => self.path.as_str().into(),
            Paths::Config => config,
        };
        result.to_str().expect("[ FAILED ] - Get Path is fehlgeschlagen").to_string()
    }

    fn get_taildevices() -> Result<Taildevices, EventsFailed> {
        let cmd = Command::new("tailscale")
            .args(["status", "--json"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Tailscale(String::from_utf8_lossy(&cmd.stderr).to_string()));
        }
        serde_json::from_slice::<Taildevices>(&cmd.stdout).map_err(|err| EventsFailed::SerdeJson(err.to_string()))
    }

    fn get_taildevices_specific(devices: Taildevices, name: &str, active_installs: &HashSet<String>) -> Vec<String> {
        let mut ips: Vec<String> = vec![];
        for (_nodekey, device) in devices.devices {
            if device.name == name && device.os == "linux" {
                let ip = device.ip[0].clone();
                if !active_installs.contains(&ip) {
                    let _ = &mut ips.push(ip.to_owned());
                }
            }
        }
        ips
    }
}
