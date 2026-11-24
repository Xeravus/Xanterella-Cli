use crate::xanterella::CommandRunner;
use crate::Events;
use crate::EventsFailed;
use crate::User;
use std::process::Command;
use std::fs;
use crate::XanterellaInstall;

impl<C: CommandRunner> XanterellaInstall<C> {
    pub fn nix_build(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunNixBuild);

        if !self.xanterella.debug {
            let dir = &self.xanterella.load_config().target_directory;
            let cmd = self.xanterella.runner.execute("nix", &["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"], &dir) 
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::NixBuild(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkNixBuild);
        Ok(())
    }

    fn nix_copy(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunNixCopy);

        let fast_cmd = format!(
            "nix-store --export $(nix.store -qR ./result) | zstd -T0 -3 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C root@{} 'zstdcat | nix-store --store /mnt --import'",
            self.ip
        );

        if !self.xanterella.debug {
            let dir = &self.xanterella.load_config().target_directory;
            let cmd = self.xanterella.runner.execute("sh", &["-c", &fast_cmd], &dir)
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::NixCopy(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkNixCopy);
        Ok(())
    }

    pub fn create_profile(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunCreateProfile);

        if !self.xanterella.debug {
            let dir = &self.xanterella.load_config().target_directory;
            let sys_path = fs::read_link(format!("{}/result", self.xanterella.load_config().target_directory))
                .map_err(|err| EventsFailed::ReadSymLink(err.to_string()))?
                .to_string_lossy()
                .into_owned();
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push(format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", sys_path));
            let cmd = self.xanterella.runner.execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::CreateProfile(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkCreateProfile);
        Ok(())
    }

    pub fn prep_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPrepSys);

        let mut ssh_args = self.get_sshstring(User::Root);
        ssh_args.push("mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS".to_string());

        if !self.xanterella.debug {
            let cmd = self.xanterella.runner.execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PrepSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPrepSys);
        Ok(())
    }

    pub fn activate_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunActivateSys);

        let mut ssh_args = self.get_sshstring(User::Root);
        ssh_args.push("NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'".to_string());

        if !self.xanterella.debug {
            let cmd = self.xanterella.runner.execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::ActivateSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkActivateSys);
        Ok(())
    }

    pub fn activate_bootloader(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunActivateBootloader);

        let mut ssh_args = self.get_sshstring(User::Root);
        ssh_args.push("nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'".to_string());

        if !self.xanterella.debug {
            let cmd = self.xanterella.runner.execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::ActivateBootloader(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkActivateBootloader);
        Ok(())
    }

    pub fn reboot_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRebootSys);

        let mut ssh_args = self.get_sshstring(User::Root);
        ssh_args.push("nohup sh -c 'sleep 3 && tailscale logout && reboot' > /dev/null 2>&1 &".to_string());

        if !self.xanterella.debug {
            let cmd = self.xanterella.runner.execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::RebootSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkRebootSys);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_deploy_debug() -> XanterellaInstall {
        let xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(xanterella);
        install.xanterella.debug = true;
        install
    }

    #[test]
    fn test_installer_deploy_nix_build() {
        let result1 = test_deploy_debug().nix_build();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_copy() {
        let result1 = test_deploy_debug().nix_copy();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_create_profile() {
        let result1 = test_deploy_debug().create_profile();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_prep_sys() {
        let result1 = test_deploy_debug().prep_sys();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_activate_sys() {
        let result1 = test_deploy_debug().activate_sys();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_activate_bootloader() {
        let result1 = test_deploy_debug().activate_bootloader();
        assert!(result1.is_ok());
    }

    #[test]
    fn test_installer_deploy_nix_reboot_sys() {
        let result1 = test_deploy_debug().reboot_sys();
        assert!(result1.is_ok());
    }
}
