pub trait Drives {
    fn part_efi(&self) -> Result<(), EventsFailed>;
    fn part_root(&self) -> Result<(), EventsFailed>;
    fn format_efi(&self) -> Result<(), EventsFailed>;
    fn format_root(&self) -> Result<(), EventsFailed>;
    fn create_boot_dir(&self) -> Result<(), EventsFailed>;
    fn mount_boot(&self) -> Result<(), EventsFailed>;
    fn mount_root(&self) -> Result<(), EventsFailed>;
}

impl Drives for Xanterella {
    fn part_efi(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPartEfi(&self.drive.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["parted", "-s", &self.drive])
            .args(["mklabel", "gpt"])
            .args(["mkpart", "disk-main-boot", "fat32", "1Mib", "512MiB"])
            .args(["set", "1", "esp", "on"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PartEfi);
        };

        self.log_event(Events::OkPartEfi(&self.drive.clone()));
        Ok()
    }

    fn part_root(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPartRoot(&self.drive.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["parted", "-s", &self.drive])
            .args(["mkpart", "disk-main-root", "ext4", "512MiB", "100%"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PartRoot);
        };

        self.log_event(Events::OkPartRoot(&self.drive.clone()));
        Ok()
    }

    fn format_efi(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunFormatEfi(&self.drive.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg("mkfs.fat")
            .args([self.get_part_name(1), "-F", "32", "-n", "boot"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::FormatEfi);
        };

        self.log_event(Events::OkFormatEfi(&self.drive.clone()));
        Ok()
    }

    fn format_root(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunFormatRoot(&self.drive.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg("mkfs.ext4")
            .args([self.get_part_name(2), "-L", "nixos"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::FormatRoot);
        };

        self.log_event(Events::OkFormatRoot(&self.drive.clone()));
        Ok()
    }

    fn create_boot_dir(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunCreateBootDir);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["mkdir", "-p", "/mnt/boot"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::CreateBootDir);
        };

        self.log_event(Events::OkCreateBootDir);
        Ok()
    }

    fn mount_boot(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunMountBoot);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["mount", self.get_part_name(1), "/mnt/boot"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::MountBoot);
        };

        self.log_event(Events::OkMountBoot);
        Ok()
    }

    fn mount_root(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunMountRoot);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["mount", self.get_part_name(2), "/mnt"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::MountRoot);
        };

        self.log_event(Events::OkMountRoot);
        Ok()
    }
}
