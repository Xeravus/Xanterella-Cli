pub trait Deploy {
    fn nix_build(&self) -> Result<(), EventsFailed>;
    fn nix_copy(&self) -> Result<(), EventsFailed>;
    fn create_profile(&self) -> Result<(), EventsFailed>;
    fn prep_sys(&self) -> Result<(), EventsFailed>;
    fn activate_sys(&self) -> Result<(), EventsFailed>;
    fn activate_bootloader(&self) -> Result<(), EventsFailed>;
    fn reboot_sys(&self) -> Result<(), EventsFailed>;
}

impl Deploy for Xanterella {
    fn nix_build(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunNixBuild);

        let cmd = Command::new("nix")
            .args(["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::NixBuild);
        };

        self.log_event(Events::OkNixBuild);
        Ok()
    }

    fn nix_copy(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunNixCopy);

        let fast_cmd = format!("nix-store --export $(nix.store -qR ./result) | zstd -T0 -3 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C root@{} 'zstdcat | nix-store --store /mnt --import'", &self.ip);        

        let cmd = Command::new("sh")
            .arg("-c")
            .arg(fast_cmd)
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::NixCopy);
        };

        self.log_event(Events::OkNixCopy);
        Ok()
    }

    fn create_profile(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunCreateProfile);

        let sys_path = fs::read_link(format!("{}/result", self.get_path(Paths::Nixconf)))
            .map_err(|err| EventsFailed::ReadSymLink)
            .to_string_lossy()
            .into_owned();

        let profile_cmd = format!("nix-env --store /mnt -p /mnt/nix/var/nix/profiles/system --set {}", sys_path);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(profile_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::CreateProfile);
        };

        self.log_event(Events::OkCreateProfile);
        Ok()
    }

    fn prep_sys(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPrepSys);

        let prep_cmd = "mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS";

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(prep_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PrepSys);
        };

        self.log_event(Events::OkPrepSys);
        Ok()
    }

    fn activate_sys(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunActivateSys);

        let activate_cmd = "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'";

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(activate_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::ActivateSys);
        };

        self.log_event(Events::OkActivateSys);
        Ok()
    }

    fn activate_bootloader(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunActivateBootloader);

        let bootloader_cmd = "nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'";

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(bootloader_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::ActivateBootloader);
        };

        self.log_event(Events::OkActivateBootloader);
        Ok()
    }

    fn reboot(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunReboot);

        let reboot_cmd = "nohup sh -c 'sleep 3 && tailscale logout && reboot' > /dev/null 2>&1 &";

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg()
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Reboot);
        };

        self.log_event(Events::OkReboot);
        Ok()
    }
}
