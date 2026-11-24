use std::{fs, process::Command};

use crate::{Events, EventsFailed, User, XanterellaInstall, xanterella::CommandRunner};

impl<C: CommandRunner> XanterellaInstall<C> {
    pub fn nix_build(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunNixBuild);

        if !self.xanterella.debug {
            let dir = &self.xanterella.load_config().target_directory;
            let cmd = self
                .xanterella
                .execute("nix", &["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"], &dir)
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
            let cmd = self
                .xanterella
                .execute("sh", &["-c", &fast_cmd], &dir)
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
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
        ssh_args.push(
            "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'"
                .to_string(),
        );

        if !self.xanterella.debug {
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
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
    use std::{
        os::unix::process::ExitStatusExt,
        process::{ExitStatus, Output},
    };

    use mockall::{
        automock,
        predicate::{always, eq},
    };

    use super::*;
    use crate::{Xanterella, test_helpers::mock_ok, xanterella::MockCommandRunner};

    #[test]
    fn test_mock_nix_build() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "nix" && args == ["build", ".#nixosConfigurations.crylia.config.system.build.toplevel"]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        let result = xanterellainstall.nix_build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_nix_copy() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
        .withf(|program, args, _dir| {
            program == "sh" && args == ["-c", "nix-store --export $(nix.store -qR ./result) | zstd -T0 -3 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -C root@127.0.0.1 'zstdcat | nix-store --store /mnt --import'"] 
        }).times(1).returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.nix_copy();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_prep_sys() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "mkdir -m 0755 -p /mnt/etc && touch /mnt/etc/NIXOS",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.prep_sys();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_activate_sys() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
        .withf(|program, args, _dir| {
            program == "ssh" && args == ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null", "root@127.0.0.1", "NIXOS_INSTALL_BOOTLOADER=1 nixos-enter --root /mnt --command '/nix/var/nix/profiles/system/activate'"] 
        }).times(1).returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.activate_sys();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_activate_bootloader() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
        .withf(|program, args, _dir| {
            program == "ssh" && args == ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null", "root@127.0.0.1", "nixos-enter --root /mnt --command 'NIXOS_INSTALL_BOOTLOADER=1 /nix/var/nix/profiles/system/bin/switch-to-configuration boot'"] 
        }).times(1).returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.activate_bootloader();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_reboot_sys() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == [
                            "-o",
                            "StrictHostKeyChecking=no",
                            "-o",
                            "UserKnownHostsFile=/dev/null",
                            "root@127.0.0.1",
                            "nohup sh -c 'sleep 3 && tailscale logout && reboot' > /dev/null 2>&1 &",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.reboot_sys();
        assert!(result.is_ok());
    }
}
