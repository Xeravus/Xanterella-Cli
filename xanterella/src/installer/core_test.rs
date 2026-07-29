use crate::installer::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_core_new() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        assert!(matches!(
            install,
            XanterellaInstall {
                xanterella: _,
                ip: _,
                drive: _,
            }
        ));
        assert!(matches!(
            install.xanterella,
            Xanterella {
                path: _,
                home: _,
                fast: _,
                debug: _,
                automate: _,
            }
        ));
        assert!(matches!(&install.ip, String));
        assert!(matches!(&install.drive, String));

        assert!(install.ip.is_empty());
        assert!(install.drive.is_empty());
    }

    #[test]
    fn test_installer_core_set_ip() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.set_ip("127.0.0.1");
        assert!(matches!(&install.ip, String));
        assert!(!install.ip.is_empty());
        assert_eq!(install.ip, String::from("127.0.0.1"));
    }

    #[test]
    fn test_installer_core_set_drive() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.set_drive("nvme");
        assert!(matches!(&install.drive, String));
        assert!(!install.drive.is_empty());
        assert_eq!(install.drive, String::from("nvme"));
    }

    #[test]
    fn test_installer_core_remote_integration() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.xanterella.set_debug(true);

        let result = install.remote_integration();

        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_prep_fs() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.xanterella.set_debug(true);

        let result = install.remote_prep_fs();

        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_install() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.xanterella.set_debug(true);

        let result = install.remote_install();

        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_install_cleanup() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        install.xanterella.set_debug(true);

        let result = install.remote_install_cleanup();

        assert!(result.is_ok());
    }
}
