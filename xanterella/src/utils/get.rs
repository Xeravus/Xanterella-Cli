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
    fn get_sshstring(&self, user: User) -> Vec<String>;
    fn get_path(&self, path: Paths) -> String;
    fn get_part_name(&self, part: i8) -> String;
    fn get_drives(&self) -> Result<Drives, EventsFailed>;
    fn sort_drives(drives: Drives) -> Drives;
    fn get_drive_size(size: &str) -> u64;
    fn get_taildevices() -> Result<Taildevices, EventsFailed>;
    fn get_taildevices_specific(devices: Taildevices, name: &str, active_installs: &HashSet<String>) -> Vec<String>;
    fn get_hardware(&self) -> Result<String, EventsFailed>;
}

impl Get for Xanterella {
    fn get_sshstring(&self, user: User) -> Vec<String> {
        let target = match user {
            User::Root => format!("root@{}", &self.ip),
            User::Cato => format!("cato@{}", &self.ip),
        };
        vec![
            "-o".to_string(),
            "StrictHostKeyChecking=no".to_string(),
            "-o".to_string(),
            "UserKnownHostsFile=/dev/null".to_string(),
            target,
        ]
    }

    fn get_path(&self, path: Paths) -> String {
        let config = PathBuf::from(&self.home).join(".config").join("xanterella");
        let result: PathBuf = match path {
            Paths::Nixconf => &self.path.into(),
            Paths::Config => config,
        };
        result.to_atr().expect("[ FAILED ] - Get Path is fehlgeschlagen").to_string()
    }

    fn get_part_name(&self, part: i8) -> String {
        let drive = format!("/dev/{}", &self.drive);
        let p_suffix = if &self.drive.contains("nvme") || &self.drive.contains("mmclblk") { 
            "p"
        } else {
            ""
        };
        format!("{}{}{}", &self.drive, p_suffix, part)
    }

    fn get_drives(&self) -> Result<Drives, EventsFailed> {
        self.log_event(Events::RunGetDrives(&self.ip.clone()));

        let parsed_drives = if !&self.ip.contains("127.0.0.1") {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["lslbk", "--json"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                let cmd_again = Command::new("ssh")
                    .args(self.get_sshstring(User::Cato))
                    .args(["lslbk", "--json"])
                    .output()
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string))?;

                if !cmd_again.status.success() {
                    return Err(EventsFailed::GetDrives);
                } else {
                    serde_json::from_slice::<Drives>(&cmd_again.stdout)
                        .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
                }
            } else {
                serde_json::from_slice::<Drives>(&cmd.stdout)
                    .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
            }
        } else {
            let cmd = Command::new("lsblk")
                .arg("--json")
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::Lsblk);
            };

            serde_json::from_slice::<Drives>(&cmd.stdout)
                .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
        };

        self.log_event(Events::OkGetDrives(&self.ip.clone()));
        Ok(parsed_drives)
    }

    fn sort_drives(&self, drives: Drives) -> Drives {
        let mut drives = drives;
        drives.blockdevices.sort_by(|a, b| {
            let size_a = self.get_drive_size(&a.size);
            let size_b = self.get_drive_size(&b.size);
            size_b.cmp(&size_a)
        });
        drives
    }

    fn get_drive_size(size: &str) -> u64 {
        let size = size.trim().to_uppercase();
        let mut multiplier: f64 = 1.0;
        let mut num_str = size.as_str();

        if size.ends_with('T') {
            multiplier = 1024.0 * 1024.0 * 1024.0 * 1024.0;
            num_str = &size[..size.len() - 1];
        } else if size.ends_with('G') {
            multiplier = 1024.0 * 1024.0 * 1024.0;
            num_str = &size[..size.len() - 1];
        } else if size.ends_with('M') {
            multiplier = 1024.0 * 1024.0;
            num_str = &size[..size.len() - 1];
        } else if size.ends_with('K') {
            multiplier = 1024.0;
            num_str = &size[..size.len() - 1];
        };

        let val: f64 = num_str.parse().unwrap_or(0.0);

        (val * multiplier) as u64
    }

    fn get_taildevices() -> Result<Taildevices, EventsFailed> {
        let cmd = Command::new("tailscale")
            .args(["status", "--json"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Tailscale);
        }
        serde_json::from_slice::<Taildevices>(&cmd.stdout)
            .map_err(|err| EventsFailed::Tailscale(err.to_string()))
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

    fn get_hardware(&self) -> Result<String, EventsFailed> {
        self.log_event(Events::RunGetHardware(&self.ip.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["nixos-generate-config", "--no-filesystem", "--show-hardware-config"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GetHardware);
        }

        self.log_event(Events::OkGetHardware(self.ip.clone()));
        Ok(String::from_utf8_lossy(&cmd.stdout).to_string())
    }
}
