use crate::{Events, EventsFailed, User, XanterellaInstall, xanterella::CommandRunner};

impl<C: CommandRunner> XanterellaInstall<C> {
    pub fn get_hardware(&mut self) -> Result<String, EventsFailed> {
        self.xanterella.log_event(Events::RunGetHardware);

        if !self.xanterella.debug {
            let mut ssh_args = self.get_sshstring(User::Root);
            ssh_args.push("nixos-generate-config --no-filesystem --show-hardware-config".to_string());
            let cmd = self
                .xanterella
                .execute("ssh", &ssh_args, "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::GetHardware(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };

            self.xanterella.log_event(Events::OkGetHardware);
            Ok(String::from_utf8_lossy(&cmd.stdout).to_string())
        } else {
            Ok(String::new())
        }
    }

    pub fn get_sshstring(&mut self, user: User) -> Vec<String> {
        let target = match user {
            User::Root => format!("root@{}", self.ip),
            User::Cato => format!("cato@{}", self.ip),
        };
        vec![
            "-o".to_string(),
            "StrictHostKeyChecking=no".to_string(),
            "-o".to_string(),
            "UserKnownHostsFile=/dev/null".to_string(),
            target,
        ]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Xanterella, test_helpers::mock_ok, xanterella::MockCommandRunner};

    #[test]
    fn test_mock_get_hardware() {
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
                            "nixos-generate-config --no-filesystem --show-hardware-config",
                        ]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.get_hardware();
        assert!(result.is_ok());
    }
}
