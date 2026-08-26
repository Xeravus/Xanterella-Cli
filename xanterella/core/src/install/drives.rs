use crate::prelude::*;

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct StorageDrives {
    pub blockdevices: Vec<BlockDevice>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct BlockDevice {
    pub name: String,
    pub size: String,

    #[serde(rename = "type")]
    pub device_type: String,
}

pub trait Drives {
    /// Get / Sort Functions
    fn sort_drives(&mut self, drives: StorageDrives) -> StorageDrives;
    fn get_drive_size(&self, size: &str) -> u64;
    fn get_drives(&mut self) -> Result<StorageDrives, EventsFailed>;
    fn get_part_name(&mut self, part: i8) -> String;

    /// Helper Functions
    fn part_efi(&mut self) -> Result<(), EventsFailed>;
    fn part_root(&mut self) -> Result<(), EventsFailed>;
    fn format_efi(&mut self) -> Result<(), EventsFailed>;
    fn format_root(&mut self) -> Result<(), EventsFailed>;
    fn create_boot_dir(&mut self) -> Result<(), EventsFailed>;
    fn mount_boot(&mut self) -> Result<(), EventsFailed>;
    fn mount_root(&mut self) -> Result<(), EventsFailed>;
}

impl Drives for XanterellaInstall {
    fn sort_drives(&mut self, drives: StorageDrives) -> StorageDrives {
        let mut drives = drives;
        drives.blockdevices.sort_by(|a, b| {
            let size_a = self.get_drive_size(&a.size);
            let size_b = self.get_drive_size(&b.size);
            size_b.cmp(&size_a)
        });
        drives
    }

    fn get_drive_size(&self, size: &str) -> u64 {
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

    fn get_drives(&mut self) -> Result<StorageDrives, EventsFailed> {
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
                    serde_json::from_slice::<StorageDrives>(&cmd_again.stdout)
                        .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
                }
            } else {
                serde_json::from_slice::<StorageDrives>(&cmd.stdout)
                    .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
            }
        } else {
            let cmd =
                Command::new("lsblk").arg("--json").output().map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::Lsblk(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };

            serde_json::from_slice::<StorageDrives>(&cmd.stdout)
                .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
        };

        self.xanterella.log_event(Events::OkGetDrives);
        Ok(parsed_drives)
    }

    fn get_part_name(&mut self, part: i8) -> String {
        let drive = format!("/dev/{}", self.drive.clone());
        let p_suffix = if drive.contains("nvme") || drive.contains("mmclblk") { "p" } else { "" };
        format!("{}{}{}", drive, p_suffix, part)
    }

    fn part_efi(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPartEfi);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["parted", "-s", &self.drive])
                .args(["mklabel", "gpt"])
                .args(["mkpart", "disk-main-boot", "fat32", "1Mib", "512MiB"])
                .args(["set", "1", "esp", "on"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PartEfi(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPartEfi);
        Ok(())
    }

    fn part_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPartRoot);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["parted", "-s", &self.drive])
                .args(["mkpart", "disk-main-root", "ext4", "512MiB", "100%"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PartRoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPartRoot);
        Ok(())
    }

    fn format_efi(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunFormatEfi);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg("mkfs.fat")
                .args([self.get_part_name(1), "-F".to_string(), "32".to_string(), "-n".to_string(), "boot".to_string()])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::FormatEfi(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkFormatEfi);
        Ok(())
    }

    fn format_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunFormatRoot);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg("mkfs.ext4")
                .args([self.get_part_name(2), "-L".to_string(), "nixos".to_string()])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::FormatRoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkFormatRoot);
        Ok(())
    }

    fn create_boot_dir(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunCreateBootDir);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["mkdir", "-p", "/mnt/boot"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::CreateBootDir(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkCreateBootDir);
        Ok(())
    }

    fn mount_boot(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunMountBoot);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["mount", &self.get_part_name(1), "/mnt/boot"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::MountBoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkMountBoot);
        Ok(())
    }

    fn mount_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunMountRoot);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .args(["mount", &self.get_part_name(2), "/mnt"])
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::MountRoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkMountRoot);
        Ok(())
    }
}

#[cfg(test)]
#[path = "drives_test.rs"]
mod tests;
