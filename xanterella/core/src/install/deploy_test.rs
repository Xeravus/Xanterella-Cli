use crate::install::deploy::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_deploy_nix_build() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.xanterella.home = "/test".to_string();

        let result1 = install1.nix_build();
        let result2 = install2.nix_build();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::FailedCmd(_))));
    }

    #[test]
    fn test_installer_deploy_nix_copy() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.xanterella.home = "/test".to_string();

        let result1 = install1.nix_copy();
        let result2 = install2.nix_copy();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::FailedCmd(_))));
    }

    #[test]
    fn test_installer_deploy_nix_create_profile() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.xanterella.home = "/test".to_string();

        let result1 = install1.create_profile();
        let result2 = install2.create_profile();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::ReadSymLink(_))));
    }

    #[test]
    fn test_installer_deploy_nix_prep_sys() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.ip = "127.127.127.127.127".to_string();

        let result1 = install1.prep_sys();
        let result2 = install2.prep_sys();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PrepSys(_))));
    }

    #[test]
    fn test_installer_deploy_nix_activate_sys() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.ip = "127.127.127.127.127".to_string();

        let result1 = install1.activate_sys();
        let result2 = install2.activate_sys();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::ActivateSys(_))));
    }

    #[test]
    fn test_installer_deploy_nix_activate_bootloader() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.activate_bootloader();
        let result2 = install2.activate_bootloader();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::ActivateBootloader(_))));
    }

    #[test]
    fn test_installer_deploy_nix_reboot_sys() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install1.ip = "127.127.127.127.127".to_string();

        let result1 = install1.reboot_sys();
        let result2 = install2.reboot_sys();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::RebootSys(_))));
    }
}
