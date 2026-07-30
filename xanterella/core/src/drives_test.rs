use crate::drives::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_drives_part_efi() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.part_efi();
        let result2 = install2.part_efi();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PartEfi(_))));
    }

    #[test]
    fn test_installer_drives_part_root() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.part_root();
        let result2 = install2.part_root();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PartRoot(_))));
    }

    #[test]
    fn test_installer_drives_format_efi() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.format_efi();
        let result2 = install2.format_efi();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::FormatEfi(_))));
    }

    #[test]
    fn test_installer_drives_format_root() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.format_root();
        let result2 = install2.format_root();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::FormatRoot(_))));
    }

    #[test]
    fn test_installer_drives_create_boot_dir() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.create_boot_dir();
        let result2 = install2.create_boot_dir();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::CreateBootDir(_))));
    }

    #[test]
    fn test_installer_drives_mount_boot() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.mount_boot();
        let result2 = install2.mount_boot();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::MountBoot(_))));
    }

    #[test]
    fn test_installer_drives_mount_root() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.mount_root();
        let result2 = install2.mount_root();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::MountRoot(_))));
    }

    #[test]
    fn test_installer_helper_sort_drives() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

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
        let parsed: Drives = serde_json::from_str(mock_json).unwrap();
        let sorted: Drives = install.sort_drives(parsed);

        assert_eq!(sorted.blockdevices.len(), 5);
        assert_eq!(sorted.blockdevices[0].name, "nvme0n1");
        assert_eq!(sorted.blockdevices[0].size, "1T");
        assert_eq!(sorted.blockdevices[0].device_type, "disk");

        assert_eq!(sorted.blockdevices[1].name, "sdb1");
        assert_eq!(sorted.blockdevices[1].size, "800G");
        assert_eq!(sorted.blockdevices[1].device_type, "part");

        assert_eq!(sorted.blockdevices[2].name, "sdc1");
        assert_eq!(sorted.blockdevices[2].size, "600G");
        assert_eq!(sorted.blockdevices[2].device_type, "part");

        assert_eq!(sorted.blockdevices[3].name, "sda");
        assert_eq!(sorted.blockdevices[3].size, "500G");
        assert_eq!(sorted.blockdevices[3].device_type, "disk");

        assert_eq!(sorted.blockdevices[4].name, "sdd1");
        assert_eq!(sorted.blockdevices[4].size, "400G");
        assert_eq!(sorted.blockdevices[4].device_type, "part");
    }

    #[test]
    fn test_installer_helper_get_drive_size() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        let size1 = "1K";
        let size2 = "1M";
        let size3 = "1G";
        let size4 = "1T";

        assert_eq!(install.get_drive_size(&size1), 1024);
        assert_eq!(install.get_drive_size(&size2), 1048576);
        assert_eq!(install.get_drive_size(&size3), 1073741824);
        assert_eq!(install.get_drive_size(&size4), 1099511627776);
    }

    #[test]
    fn test_installer_helper_get_drives() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);

        install1.ip = "127.0.0.1".to_string();
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.get_drives();
        let result2 = install2.get_drives();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::GetDrives(_))));
    }

    #[test]
    fn test_installer_helper_get_part_name() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();
        let mut xanterella3 = Xanterella::new();
        let mut xanterella4 = Xanterella::new();
        let mut xanterella5 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);
        let mut install3 = XanterellaInstall::new(&mut xanterella3);
        let mut install4 = XanterellaInstall::new(&mut xanterella4);
        let mut install5 = XanterellaInstall::new(&mut xanterella5);

        install1.drive = "nvme".to_string();
        install2.drive = "mmclblk".to_string();
        install3.drive = "sda".to_string();
        install4.drive = "sdc".to_string();
        install5.drive = "sdd".to_string();

        assert_eq!(install1.get_part_name(1), "/dev/nvmep1".to_string());
        assert_eq!(install2.get_part_name(2), "/dev/mmclblkp2".to_string());
        assert_eq!(install3.get_part_name(3), "/dev/sda3".to_string());
        assert_eq!(install4.get_part_name(4), "/dev/sdc4".to_string());
        assert_eq!(install5.get_part_name(5), "/dev/sdd5".to_string());
    }
}
