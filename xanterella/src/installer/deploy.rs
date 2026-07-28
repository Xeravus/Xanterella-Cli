use crate::prelude::*;
use crate::installer::helper::*;
use crate::installer::core::XanterellaInstall;

pub trait Deploy {
    fn nix_build(&mut self) -> Result<(), EventsFailed>;
    fn nix_copy(&mut self) -> Result<(), EventsFailed>;
    fn create_profile(&mut self) -> Result<(), EventsFailed>;
    fn prep_sys(&mut self) -> Result<(), EventsFailed>;
    fn activate_sys(&mut self) -> Result<(), EventsFailed>;
    fn activate_bootloader(&mut self) -> Result<(), EventsFailed>;
    fn reboot_sys(&mut self) -> Result<(), EventsFailed>;
}

impl<'a> Deploy for XanterellaInstall<'a> {
    fn nix_build(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunNixBuild);
        
        if !self.xanterella.debug {
            let cmd = Command::new("nix")
                .args(["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"])
                .current_dir(self.xanterella.get_path(Paths::Nixconf))
                .output()
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

        let fast_cmd = format!("nix-store --export $(nix.store -qR ./result) | zstd -T0 -3 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C root@{} 'zstdcat | nix-store --store /mnt --import'", &self.ip);        

        if !self.xanterella.debug {
            let cmd = Command::new("sh")
                .arg("-c")
                .arg(fast_cmd)
                .current_dir(self.xanterella.get_path(Paths::Nixconf))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::NixCopy(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkNixCopy);
        Ok(())
    }

    fn create_profile(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunCreateProfile);

        let sys_path = fs::read_link(format!("{}/result", self.xanterella.get_path(Paths::Nixconf)))
            .map_err(|err| EventsFailed::ReadSymLink(err.to_string()))?
            .to_string_lossy()
            .into_owned();

        let profile_cmd = format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", sys_path);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(profile_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::CreateProfile(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkCreateProfile);
        Ok(())
    }

    fn prep_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPrepSys);

        let prep_cmd = "mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS";

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(prep_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PrepSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
            };

            self.xanterella.log_event(Events::OkPrepSys);
        Ok(())
    }

    fn activate_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunActivateSys);

        let activate_cmd = "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'";

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(activate_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::ActivateSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkActivateSys);
        Ok(())
    }

    fn activate_bootloader(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunActivateBootloader);

        let bootloader_cmd = "nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'";

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(bootloader_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::ActivateBootloader(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkActivateBootloader);
        Ok(())
    }

    fn reboot_sys(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunRebootSys);

        let reboot_cmd = "nohup sh -c 'sleep 3 && tailscale logout && reboot' > /dev/null 2>&1 &";

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(reboot_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::RebootSys(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkRebootSys);
        Ok(())
    }
}
