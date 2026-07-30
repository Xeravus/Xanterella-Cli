use crate::installer::core::*;
use crate::installer::helper::*;
use crate::prelude::*;

pub trait Drives {
    /// Get / Sort Functions
    fn sort_drives(&mut self, drives: Drives) -> Drives;
    fn get_drive_size(&self, size: &str) -> u64;
    fn get_drives(&mut self) -> Result<Drives, EventsFailed>;
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

impl<'a> Drives for Xanterella<'a> {
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
