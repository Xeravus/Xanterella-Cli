use log::{debug, info, error};
use serde::{Deserialize};

use std::collections::HashMap;
use std::collections::HashSet;
use std::process::{self, Command};
use std::env;
use std::path::*;
use std::fs;

use crate::usb::flash::*;

#[derive(Deserialize, Debug)]
pub struct Drives {
    pub blockdevices: Vec<BlockDevice>,
}

#[derive(Deserialize, Debug)]
pub struct BlockDevice {
    pub name: String,
    pub size: String,

    #[serde(rename = "type")]
    pub device_type: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Taildevices {
    #[serde(rename = "Peer")]
    pub devices: HashMap<String, DeviceInfo>,
}

#[derive(serde::Deserialize, Debug)]
pub struct DeviceInfo {
    #[serde(rename = "HostName")]
    pub name: String,
    #[serde(rename = "TailscaleIPs")]
    pub ip: Vec<String>,
    #[serde(rename = "OS")]
    pub os: String,
}

pub enum Paths {
    Home,
    Nixconf,
    Config,
}

pub enum User {
    Root,
    Cato,
}

pub fn get_hardware(ip: &str) -> String {
    info!("[ RUN ] - Generiere Hardware");

    let ssh = Command::new("ssh")
        .args(get_sshstring(ip, User::Root))
        .arg("nixos-generate-config --no-filesystems --show-hardware-config")
        .output()
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte SSH nicht starten: {}", err); 
            process::exit(1); 
        });
    if !ssh.status.success() {
        error!("[ FAILED ] - Fehler beim erstellen der Hardware Config: {}", String::from_utf8_lossy(&ssh.stderr));
        process::exit(1);
    }

    let hardware_config = String::from_utf8_lossy(&ssh.stdout).to_string();
    info!("[ OK ] - Hardware erfolgreich geneiert");
    debug!("{}", hardware_config);
    hardware_config
}

pub fn get_drives(ip: &str) -> Drives {
    info!("[ RUN ] - Parse Drives");

    let parsed_drives;
    if ip != "127.0.0.1" {
        let lsblk = Command::new("ssh")
            .args(get_sshstring(ip, User::Root))
            .arg("lsblk")
            .arg("--json")
            .output()
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte lsblk nicht starten: {}", err); 
                process::exit(1); 
            });
        if !lsblk.status.success() {
            error!("[ FAILED ] - Fehler beim Auslesen der als root Partitionen: {}", String::from_utf8_lossy(&lsblk.stderr));

            let lsblk1 = Command::new("ssh")
                .args(get_sshstring(ip, User::Cato))
                .arg("lsblk")
                .arg("--json")
                .output()
                .unwrap_or_else(|err| { 
                    error!("[ FAILED ] - Konnte lsblk nicht starten: {}", err); 
                    process::exit(1); 
                });
            if !lsblk1.status.success() {
                error!("[ FAILED ] - Fehler beim Auslesen der als cato Partitionen: {}", String::from_utf8_lossy(&lsblk.stderr));
                process::exit(1);

            } else {
                info!("[ OK ] - Drives mit Cato geparsen");
                parsed_drives = serde_json::from_slice::<Drives>(&lsblk1.stdout)
                    .unwrap_or_else(|err| { 
                        error!("[ FAILED ] - Konnte lsblk nicht parsen: {}", err); 
                        process::exit(1); 
                    });
            }
        } else {
            info!("[ OK ] - Drives mit Root geparsen");
            parsed_drives = serde_json::from_slice::<Drives>(&lsblk.stdout)
                .unwrap_or_else(|err| { 
                    error!("[ FAILED ] - Konnte lsblk nicht parsen: {}", err); 
                    process::exit(1); 
                });
        }
    } else {
        let lsblk = Command::new("lsblk")
            .arg("--json")
            .output()
            .unwrap_or_else(|err| {
                error!("[ FAILED ] - Konnte lsblk nicht starten: {}", err);
                process::exit(1);
            });
        if !lsblk.status.success() {
            error!("[ FAILED ] - Fehler beim Auslesen der Partitionen: {}", String::from_utf8_lossy(&lsblk.stderr));
        }
        info!("[ OK ] - Drives lokal geparsen");

        parsed_drives = serde_json::from_slice::<Drives>(&lsblk.stdout)
            .unwrap_or_else(|err| { 
                error!("[ FAILED ] - Konnte lsblk nicht parsen: {}", err); 
                process::exit(1); 
            });
    }
    info!("[ OK ] - Drives erfasst");
    parsed_drives
}

pub fn get_sort_drives(drives: Drives) -> Drives {
    let mut drives = drives;
    drives.blockdevices.sort_by(|a, b| {
        let size_a = get_drives_size(&a.size);
        let size_b = get_drives_size(&b.size);
        size_b.cmp(&size_a)
    });
    drives
}

pub fn get_taildevices() -> Taildevices {
    //info!("[ RUN ] - Parse Tailscale Geräte");
    
    let tail_status = Command::new("tailscale")
        .arg("status")
        .arg("--json")
        .output()
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte 'tailscale status --json' nicht ausführen: {}", err); 
            process::exit(1); 
        });
    if !tail_status.status.success() {
        error!("[ FAILED ] - Tailscale Status ist Fehlgeschlagen, bist du eingelogt, wurde das JSON nicht richtig geparst: {}", String::from_utf8_lossy(&tail_status.stderr));
        process::exit(1);
    }

    //info!("[ OK ] - Parse Tailscale Geräte erfolgreich");
    serde_json::from_slice::<Taildevices>(&tail_status.stdout)
        .unwrap_or_else(|err| { 
            error!("[ FAILED ] - Konnte den Output von Tailscale nicht parsen: {}", err); 
            process::exit(1); 
        })
}

pub fn get_taildevices_specific(devices: Taildevices, name: &str, active_installs: &HashSet<String>) -> Vec<String> {
    let mut ips: Vec<String> = vec![];
    for (_nodekey, device) in devices.devices {
        if device.name == name && device.os == "linux" {
            let ip = device.ip[0].clone();
            if !active_installs.contains(&ip) {
                let _ = &mut ips.push(ip.to_owned());
            }
        }
    };
    ips
}

pub fn get_sshstring(ip: &str, user: User) -> Vec<String> {
    let target = match user {
        User::Root => format!("root@{}", ip),
        User::Cato => format!("cato@{}", ip),
    };
    vec![
        "-o".to_string(),
        "StrictHostKeyChecking=no".to_string(),
        "-o".to_string(),
        "UserKnownHostsFile=/dev/null".to_string(),
        target,
    ]
}

pub fn get_drives_name(primdrive: &str, number: i8) -> String {
    let drive = format!("/dev/{}", primdrive);
    let p_suffix = if primdrive.contains("nvme") || primdrive.contains("mmclblk") {
        "p"
    } else {
        ""
    };
    let partition = format!("{}{}{}", drive, p_suffix, number);
    partition
}

pub fn get_path(option: Paths) -> String {
    let home = env::var("HOME").expect("[ FAILED ] - Konnte die Home Variable nicht extrahieren");
    let nixconfig = PathBuf::from(&home).join("xanterella").join("config");
    let config = PathBuf::from(&home).join(".config").join("xanterella");
    let result: PathBuf = match option {
        Paths::Home => home.into(),
        Paths::Nixconf => nixconfig,
        Paths::Config => config,
    };
    result.to_str().expect("[ FAILED ] - Gen Path ist fehlgeschlagen").to_string()
}

pub fn get_iso(mode: FlashMode, ip: &str) -> String {
    match mode {
        FlashMode::Local => {
            info!("[ RUN ] - Finde ISO lokal");

            let iso_path = std::path::PathBuf::from(get_path(Paths::Nixconf)).join("result").join("iso");
            let entries = fs::read_dir(&iso_path)
                .unwrap_or_else(|err| { 
                    error!("[ FAILED ] - Konnte den Result Ordner nicht auslesen: {}", err); 
                    process::exit(1); 
                });

            for i in entries {
                if let Ok(file) = i {
                    let path: std::path::PathBuf = file.path();
                    if path.is_file() && path.extension().unwrap_or_default() == "iso" {
                        let target_path = path.to_string_lossy().into_owned();
                        info!("[ OK ] - ISO gefunden");
                        return target_path;
                    }
                }
            }
            error!("[ FAILED ] - Keine .iso Datei im result Ordner gefunden");
            process::exit(1);
        },
        FlashMode::Remote => {
            info!("[ RUN ] - Finde ISO remote");

            let iso_path = format!("realpath {}/result/iso/*.iso", get_path(Paths::Nixconf));
            let realpath = Command::new("ssh")
                .args(get_sshstring(ip, User::Root))
                .arg(iso_path)
                .output()
                .unwrap_or_else(|err| { 
                    error!("[ FAILED ] - Konnte den Result Ordner nicht auslesen: {}", err); 
                    process::exit(1); 
                });
            if !realpath.status.success() {
                error!("[ FAILED ] - Kontte den Result Ordner nicht auslesen: {}", String::from_utf8_lossy(&realpath.stderr));
                process::exit(1);
            }
            let output = String::from_utf8_lossy(&realpath.stdout).trim().to_string();
            debug!("ISO Remote Path: {}", output);
            output
        },
    }
}

pub fn get_drives_size(size_str: &str) -> u64 {
    let size_str = size_str.trim().to_uppercase();
    let mut multiplier: f64 = 1.0;
    let mut num_str = size_str.as_str();

    if size_str.ends_with('T') {
        multiplier = 1024.0 * 1024.0 * 1024.0 * 1024.0;
        num_str = &size_str[..size_str.len() - 1];
    } else if size_str.ends_with('G') {
        multiplier = 1024.0 * 1024.0 * 1024.0;
        num_str = &size_str[..size_str.len() - 1];
    } else if size_str.ends_with('M') {
        multiplier = 1024.0 * 1024.0;
        num_str = &size_str[..size_str.len() - 1];
    }

    let val: f64 = num_str.parse().unwrap_or(0.0);
    
    (val * multiplier) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sshstring() {
        assert_eq!(get_sshstring("192.125.142.2", User::Root), ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null", "root@192.125.142.2"])
    }

    #[test]
    fn test_drivenames() {
        assert_eq!(get_drives_name("nvme0n1", 1), "/dev/nvme0n1p1");
        assert_eq!(get_drives_name("sda", 2), "/dev/sda2");
    }

    #[test]
    fn test_parse_lsblk_json() {
        let mock_json = r#"{
            "blockdevices": [
                {"name": "nvme0n1", "size": "1T", "type": "disk"},
                {"name": "sda", "size": "500G", "type": "disk"}
            ]
        }"#;
        let parsed: Drives = serde_json::from_str(mock_json).expect("JSON Parser ist fehlgeschlagen");

        assert_eq!(parsed.blockdevices.len(), 2);
        assert_eq!(parsed.blockdevices[0].name, "nvme0n1");
        assert_eq!(parsed.blockdevices[0].size, "1T");
        assert_eq!(parsed.blockdevices[1].device_type, "disk");
    }

    #[test]
    fn test_parse_tailscale() {
        let mock_json = r#"{
            "Peer": {
                "nodekey:242c6813b0d0bd1e74d701aa31a405843156713124d72d1738ab058c193cd525": {
                    "HostName": "Redmi Note 13 Pro",
                    "TailscaleIPs": ["100.124.213.38", "fd7a:115c:a1e0::1901:d5a1"],
                    "OS": "android"
                },
                "nodekey:121e0b120d30e153fe6a4094cea25820625b7383b4a14f08a8f0f7e967a8c822": {
                    "HostName": "SmartHome",
                    "TailscaleIPs": ["100.104.200.35", "fd7a:115c:a1e0::6739:c823"],
                    "OS": "linux"
                }
            }
        }"#;
        let parsed: Taildevices = serde_json::from_str(mock_json).expect("JSON Parser ist fehlgeschlagen");
        let redmi_key = "nodekey:242c6813b0d0bd1e74d701aa31a405843156713124d72d1738ab058c193cd525";
        let redmi_device = parsed.devices.get(redmi_key).expect("Redmi Nodekey nicht in HashMap gefunden!");
        assert_eq!(parsed.devices.len(), 2);
        assert_eq!(redmi_device.name, "Redmi Note 13 Pro");
        assert_eq!(redmi_device.os, "android");
        assert_eq!(redmi_device.ip[0], "100.124.213.38"); 
    }
    #[test]
    fn test_get_drives_size() {
        let size = "1M";
        assert_eq!(get_drives_size(&size), 1048576);
    }
    #[test]
    fn test_parse_drives_json() {
        let mock_json = r#"{
            "blockdevices": [
                {
                    "name": "nvme0n1",
                    "size": "1T",
                    "type": "disk"
                },
                {
                    "name": "sda",
                    "size": "500G",
                    "type": "disk"
                },
                {
                    "name": "sda1",
                    "size": "500G",
                    "type": "part"
                }
            ]
        }"#;
        let parsed: Drives = serde_json::from_str(mock_json).expect("Konnte das JSON nicht formatieren");
        assert_eq!(parsed.blockdevices.len(), 3);
        assert_eq!(parsed.blockdevices[0].name, "nvme0n1");
        assert_eq!(parsed.blockdevices[0].size, "1T");
        assert_eq!(parsed.blockdevices[0].device_type, "disk");
        assert_eq!(parsed.blockdevices[2].name, "sda1");
        assert_eq!(parsed.blockdevices[2].device_type, "part");
    }
    #[test]
    fn test_sort_drives() {
        let mock_json = r#"{
            "blockdevices": [
                {
                    "name": "nvme0n1",
                    "size": "1T",
                    "type": "disk"
                },
                {
                    "name": "sda",
                    "size": "500G",
                    "type": "disk"
                },
                {
                    "name": "sdb1",
                    "size": "800G",
                    "type": "part"
                },
                {
                    "name": "sdc1",
                    "size": "600G",
                    "type": "part"
                },
                {
                    "name": "sdd1",
                    "size": "400G",
                    "type": "part"
                }
            ]
        }"#;
        let parsed: Drives = serde_json::from_str(mock_json).expect("Konnte das JSON nicht formatieren");
        let sorted: Drives = get_sort_drives(parsed);
        assert_eq!(sorted.blockdevices.len(), 5);
        assert_eq!(sorted.blockdevices[0].name, "nvme0n1");
        assert_eq!(sorted.blockdevices[0].size, "1T");
        assert_eq!(sorted.blockdevices[0].device_type, "disk");
        assert_eq!(sorted.blockdevices[1].name, "sdb1");
        assert_eq!(sorted.blockdevices[1].device_type, "part");
        assert_eq!(sorted.blockdevices[4].name, "sdd1");
    }
}
