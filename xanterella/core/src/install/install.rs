use crate::git::Git;
use crate::install::deploy::Deploy;
use crate::install::drives::Drives;
use crate::install::inject::Inject;
use crate::install::ping::Ping;
use crate::nix::check::Check;
use crate::prelude::*;

pub struct XanterellaInstall<'a> {
    pub xanterella: &'a mut Xanterella,
    pub ip: String,
    pub drive: String,
}

impl<'a> XanterellaInstall<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new(xanterella: &'a mut Xanterella) -> Self {
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
        // crylia_edit_start(self.get_hardware()?);
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

        // crylia_edit_finish();
        self.xanterella.git_commit("Xanterella: Cleanup")?;

        self.xanterella.log_event(Events::OkRemoteInstallCleanup);
        Ok(())
    }
}

#[cfg(test)]
#[path = "install_test.rs"]
mod tests;
