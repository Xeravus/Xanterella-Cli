use crate::{Events, EventsFailed, User, XanterellaInstall, xanterella::CommandRunner};

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

impl<C: CommandRunner> XanterellaInstall<C> {
    pub fn sort_drives(&mut self, drives: StorageDrives) -> StorageDrives {
        let mut drives = drives;
        drives.blockdevices.sort_by(|a, b| {
            let size_a = self.get_drive_size(&a.size);
            let size_b = self.get_drive_size(&b.size);
            size_b.cmp(&size_a)
        });
        drives
    }

    pub fn get_drive_size(&self, size: &str) -> u64 {
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

    pub fn get_drives(&mut self) -> Result<StorageDrives, EventsFailed> {
        self.xanterella.log_event(Events::RunGetDrives);

        let parsed_drives = if !&self.ip.contains("127.0.0.1") {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push("lsblk --json".to_string());
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                let mut ssh_args = self.get_sshstring(User::Cato);
                ssh_args.push("lsblk --json".to_string());
                let cmd_again = self
                    .xanterella
                    .execute("ssh", &ssh_args, "")
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
            let cmd = self
                .xanterella
                .execute("lsblk", &["--json"], "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::Lsblk(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };

            serde_json::from_slice::<StorageDrives>(&cmd.stdout)
                .map_err(|err| EventsFailed::SerdeJson(err.to_string()))?
        };

        self.xanterella.log_event(Events::OkGetDrives);
        Ok(parsed_drives)
    }

    pub fn get_part_name(&mut self, part: i8) -> String {
        let drive = format!("/dev/{}", self.drive.clone());
        let p_suffix = if drive.contains("nvme") || drive.contains("mmclblk") { "p" } else { "" };
        format!("{}{}{}", drive, p_suffix, part)
    }

    pub fn part_efi(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPartEfi);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!(
                "parted -s /dev/{} mklabel gpt mkpart disk-main-boot fat32 1MiB 512MiB set 1 esp on",
                self.drive
            ));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PartEfi(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPartEfi);
        Ok(())
    }

    pub fn part_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPartRoot);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("parted -s /dev/{} mkpart disk-main-root extr 512MiB 100%", self.drive));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PartRoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPartRoot);
        Ok(())
    }

    pub fn format_efi(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunFormatEfi);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("mkfs.fat {} -F 32 -n boot", self.get_part_name(1)));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::FormatEfi(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkFormatEfi);
        Ok(())
    }

    pub fn format_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunFormatRoot);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("mkfs.ext4 {} -L nixos", self.get_part_name(2)));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::FormatRoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkFormatRoot);
        Ok(())
    }

    pub fn create_boot_dir(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunCreateBootDir);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push("mkdir -p /mnt/boot".to_string());
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::CreateBootDir(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkCreateBootDir);
        Ok(())
    }

    pub fn mount_boot(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunMountBoot);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("mount {} /mnt/boot", self.get_part_name(1)));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::MountBoot(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkMountBoot);
        Ok(())
    }

    pub fn mount_root(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunMountRoot);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("mount {} /mnt", self.get_part_name(2)));
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
mod tests {

    use super::*;
    use crate::{Xanterella, test_helpers::mock_ok, xanterella::MockCommandRunner};

    #[test]
    fn test_get_drive_size() {
        let install = XanterellaInstall::new(Xanterella::new(MockCommandRunner::new()));
        assert_eq!(install.get_drive_size("1T"), 1024 * 1024 * 1024 * 1024);
        assert_eq!(install.get_drive_size("1G"), 1024 * 1024 * 1024);
        assert_eq!(install.get_drive_size("1M"), 1024 * 1024);
        assert_eq!(install.get_drive_size("1K"), 1024);
        assert_eq!(install.get_drive_size(" 1.5g "), (1.5 * 1024.0 * 1024.0 * 1024.0) as u64);
        assert_eq!(install.get_drive_size("512"), 512);
        assert_eq!(install.get_drive_size(""), 0);
        assert_eq!(install.get_drive_size("invalid"), 0);
        assert_eq!(install.get_drive_size("G"), 0);
    }

    #[test]
    fn test_sort_drives() {
        let mut install = XanterellaInstall::new(Xanterella::new(MockCommandRunner::new()));
        let drives = StorageDrives {
            blockdevices: vec![
                BlockDevice {
                    name: "sda".into(),
                    size: "500M".into(),
                    device_type: "disk".into(),
                },
                BlockDevice {
                    name: "nvme0n1".into(),
                    size: "2T".into(),
                    device_type: "disk".into(),
                },
                BlockDevice {
                    name: "sdb".into(),
                    size: "100G".into(),
                    device_type: "disk".into(),
                },
                BlockDevice {
                    name: "sdc".into(),
                    size: "1024K".into(),
                    device_type: "disk".into(),
                },
            ],
        };

        let sorted = install.sort_drives(drives);
        let names: Vec<&str> = sorted.blockdevices.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, vec!["nvme0n1", "sdb", "sda", "sdc"]);
    }

    #[test]
    fn test_mock_part_efi() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "parted -s /dev/sda mklabel gpt mkpart disk-main-boot fat32 1MiB 512MiB set 1 esp on",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.part_efi();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_part_root() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "parted -s /dev/sda mkpart disk-main-root extr 512MiB 100%",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.part_root();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_format_efi() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mkfs.fat /dev/sda1 -F 32 -n boot",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.format_efi();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_format_root() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mkfs.ext4 /dev/sda2 -L nixos",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.format_root();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_create_boot_dir() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mkdir -p /mnt/boot",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.create_boot_dir();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_mount_boot() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mount /dev/sda1 /mnt/boot",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.mount_boot();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_mount_root() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mount /dev/sda2 /mnt",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        xanterellainstall.set_drive("sda");
        let result = xanterellainstall.mount_root();
        assert!(result.is_ok());
    }
}
