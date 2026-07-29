use crate::installer::drives::*;

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
}
