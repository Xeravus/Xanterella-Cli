use crate::utils::get::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sshstring() {
        assert_eq!(
            get_sshstring("192.125.142.2", User::Root),
            ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null", "root@192.125.142.2"]
        )
    }

    #[test]
    fn test_get_drivenames() {
        assert_eq!(get_drives_name("nvme0n1", 1), "/dev/nvme0n1p1");
        assert_eq!(get_drives_name("sda", 2), "/dev/sda2");
    }

    #[test]
    fn test_serde_json_convert_tailscale() {
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
    fn test_serde_json_convert_lsblk() {
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
    fn test_get_sort_drives() {
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
