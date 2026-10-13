use crate::git::Git;
use crate::install::deploy::Deploy;
use crate::install::drives::Drives;
use crate::install::inject::Inject;
use crate::install::ping::Ping;
use crate::nix::check::Check;
use crate::nix::edit::Edit;
use crate::prelude::*;

pub struct XanterellaInstall {
    pub xanterella: Xanterella,
    pub ip: String,
    pub drive: String,
}

impl XanterellaInstall {
    #[allow(clippy::new_without_default)]
    pub fn new(xanterella: Xanterella) -> Self {
        XanterellaInstall {
            xanterella,
            ip: String::new(),
            drive: String::new(),
        }
    }

    pub fn set_ip(&mut self, value: &str) {
        self.ip = value.to_string();
    }

    pub fn set_drive(&mut self, value: &str) {
        self.drive = value.to_string();
    }

    pub fn remote_integration(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRemoteIntegration);

        self.ping()?;
        self.ping_ssh()?;
        self.xanterella.git_merge()?;
        self.crylia_edit_start()?;
        self.xanterella.git_commit("Xanterella: Remote-Install")?;
        if !&self.xanterella.fast {
            self.xanterella.check_nix_flake()?;
        }

        self.xanterella.log_event(Events::OkRemoteIntegration);
        Ok(())
    }

    pub fn remote_prep_fs(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRemotePrepFs);

        self.part_efi()?;
        self.part_root()?;

        self.format_efi()?;
        self.format_root()?;

        self.mount_root()?;
        self.create_boot_dir()?;
        self.mount_boot()?;

        self.xanterella.log_event(Events::OkRemotePrepFs);
        Ok(())
    }

    pub fn remote_install(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRemoteInstall);

        self.nix_build()?;
        self.nix_copy()?;
        self.create_profile()?;
        self.prep_sys()?;
        self.activate_sys()?;
        self.activate_bootloader()?;
        self.inject_tailscale()?;
        self.inject_wifi()?;
        self.reboot_sys()?;

        self.xanterella.log_event(Events::OkRemoteInstall);
        Ok(())
    }

    pub fn remote_install_cleanup(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRemoteInstallCleanup);

        self.crylia_edit_finish()?;
        self.xanterella.git_commit("Xanterella: Cleanup")?;

        self.xanterella.log_event(Events::OkRemoteInstallCleanup);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_install_debug() -> XanterellaInstall {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);
        install.xanterella.debug = true;
        install
    }

    #[test]
    fn test_installer_core_new() {
        let xanterella = Xanterella::new();
        let install = XanterellaInstall::new(xanterella);

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
                sender: _,
            }
        ));
        assert!(install.ip.is_empty());
        assert!(install.drive.is_empty());
    }

    #[test]
    fn test_installer_core_set_ip() {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);

        install.set_ip("127.0.0.1");
        assert!(!install.ip.is_empty());
        assert_eq!(install.ip, String::from("127.0.0.1"));
    }

    #[test]
    fn test_installer_core_set_drive() {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);

        install.set_drive("nvme");
        assert!(!install.drive.is_empty());
        assert_eq!(install.drive, String::from("nvme"));
    }

    #[test]
    fn test_installer_core_remote_integration() {
        let result = test_install_debug().remote_integration();
        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_prep_fs() {
        let result = test_install_debug().remote_prep_fs();
        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_install() {
        let result = test_install_debug().remote_install();
        assert!(result.is_ok());
    }

    #[test]
    fn test_installer_core_remote_install_cleanup() {
        let result = test_install_debug().remote_install_cleanup();
        assert!(result.is_ok());
    }
}
