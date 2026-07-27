pub trait RemoteInstall {
    fn remote_integration(&self) -> Result<(), EventsFailed>;
    fn remote_prep_fs(&self) -> Result<(), EventsFailed>;
    fn remote_install(&self) -> Result<(), EventsFailed>;
}

impl RemoteInstall for Xanterella {
    fn remote_integration(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunRemoteIntegration(&self.ip.clone()));

        self.ping()?;
        self.pingssh()?;
        self.git_merge()?:
        crylia_edit_start(self.get_hardware()?);
        self.git_commit()?;
        if !&self.fast {
            self.check_nix_flake()?;
        }

        self.log_event(Events::OkRemoteIntegration(&self.ip.clone()));
        Ok()
    }

    fn remote_prep_fs(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunRemotePrepFs(&self.ip.clone()));

        self.part_efi()?;
        self.part_root()?;

        self.format_efi()?;
        self.format_root()?;

        self.mount_root()?;
        self.create_boot_dir()?;
        self.mount_boot()?;

        self.log_event(Events::OkRemotePrepFs(&self.ip.clone()));
        Ok()
    }

    fn remote_install(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunRemoteInstall(&self.ip.clone()));

        self.nix_build()?;
        self.nix_copy()?;
        self.create_profiles()?;
        self.prep_sys()?;
        self.activate_sys()?;
        self.activate_bootloader()?;
        self.reboot_sys()?;

        self.log_event(Events::OkRemoteInstall(&self.ip.clone()));
        Ok()
    }
}
