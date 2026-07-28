use crate::prelude::*;
use crate::installer::core::XanterellaInstall;

pub trait Helper {
    fn get_sshstring(&mut self, user: User) -> Vec<String>;
    fn get_hardware(&mut self) -> Result<String, EventsFailed>;
    fn sort_drives(&mut self, drives: Drives) -> Drives;
    fn get_drive_size(size: &str) -> u64;
    fn get_drives(&mut self) -> Result<Drives, EventsFailed>;
    fn get_part_name(&mut self, part: i8) -> String;
}

impl<'a> Helper for XanterellaInstall<'a> {
    fn get_sshstring(&mut self, user: User) -> Vec<String> {
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

    fn get_hardware(&mut self) -> Result<String, EventsFailed> {
        self.xanterella.log_event(Events::RunGetHardware);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["nixos-generate-config", "--no-filesystem", "--show-hardware-config"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GetHardware(String::from_utf8_lossy(&cmd.stderr).to_string()));
        }

        self.xanterella.log_event(Events::OkGetHardware);
        Ok(String::from_utf8_lossy(&cmd.stdout).to_string())
    }

    fn sort_drives(&mut self, drives: Drives) -> Drives {
        let mut drives = drives;
        drives.blockdevices.sort_by(|a, b| {
            let size_a = Self::get_drive_size(&a.size);
            let size_b = Self::get_drive_size(&b.size);
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

    fn get_drives(&mut self) -> Result<Drives, EventsFailed> {
        self.xanterella.log_event(Events::RunGetDrives);

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
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !cmd_again.status.success() {
                    return Err(EventsFailed::GetDrives(String::from_utf8_lossy(&cmd.stderr).to_string()));
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
                return Err(EventsFailed::Lsblk(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };

            serde_json::from_slice::<Drives>(&cmd.stdout)
                .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
        };

        self.xanterella.log_event(Events::OkGetDrives);
        Ok(parsed_drives)
    }

    fn get_part_name(&mut self, part: i8) -> String {
        let drive = format!("/dev/{}", &self.drive.clone());
        let p_suffix = if drive.contains("nvme") || drive.contains("mmclblk") { 
            "p"
        } else {
            ""
        };
        format!("{}{}{}", drive, p_suffix, part)
    }
}
