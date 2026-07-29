use crate::installer::helper::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_helper_get_sshstring() {
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

        install1.ip = "127.0.0.1".to_string();
        install2.ip = "127.0.0.1".to_string();

        install3.ip = "test".to_string();
        install4.ip = "test".to_string();

        let result1 = install1.get_sshstring(User::Root);
        let result2 = install2.get_sshstring(User::Cato);
        let result3 = install3.get_sshstring(User::Root);
        let result4 = install4.get_sshstring(User::Cato);
        let result5 = install5.get_sshstring(User::Root);

        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
        assert!(!result3.is_empty());
        assert!(!result4.is_empty());
        assert!(!result5.is_empty());

        assert_eq!(result1[0], "-o".to_string());
        assert_eq!(result2[0], "-o".to_string());
        assert_eq!(result3[0], "-o".to_string());
        assert_eq!(result4[0], "-o".to_string());
        assert_eq!(result5[0], "-o".to_string());

        assert_eq!(result1[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result2[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result3[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result4[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result5[1], "StrictHostKeyChecking=no".to_string());

        assert_eq!(result1[2], "-o".to_string());
        assert_eq!(result2[2], "-o".to_string());
        assert_eq!(result3[2], "-o".to_string());
        assert_eq!(result4[2], "-o".to_string());
        assert_eq!(result5[2], "-o".to_string());

        assert_eq!(result1[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result2[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result3[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result4[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result5[3], "UserKnownHostsFile=/dev/null".to_string());

        assert_eq!(result1[4], "root@127.0.0.1".to_string());
        assert_eq!(result2[4], "cato@127.0.0.1".to_string());
        assert_eq!(result3[4], "root@test".to_string());
        assert_eq!(result4[4], "cato@test".to_string());
        assert_eq!(result5[4], "root@".to_string());
    }

    #[test]
    fn test_installer_helper_get_hardware() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        let result = install.get_hardware();

        assert!(result.is_err());
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
