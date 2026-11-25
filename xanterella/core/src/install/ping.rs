use crate::{Events, EventsFailed, User, XanterellaInstall, xanterella::CommandRunner};

impl<C: CommandRunner> XanterellaInstall<C> {
    pub fn ping(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPing);

        if !self.xanterella.debug {
            let cmd = self
                .xanterella
                .execute("ping", &["-c", "3", "-W", "1", &self.ip], "")
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::Ping(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPing);
        Ok(())
    }

    pub fn ping_ssh(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPingSsh);

        if !self.xanterella.debug {
            let ssh_args = &self.get_sshstring(User::Root);
            let cmd =
                self.xanterella.execute("ssh", ssh_args, "").map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::PingSsh(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPingSsh);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Xanterella, test_helpers::mock_ok, xanterella::MockCommandRunner};

    #[test]
    fn test_mock_ping() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| program == "ping" && args == ["-c", "3", "-W", "1", "127.0.0.1"])
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.ping();
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_ping_ssh() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| {
                program == "ssh"
                    && args
                        == ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null", "root@127.0.0.1"]
            })
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let xanterella = Xanterella::new(mock);
        let mut xanterellainstall = XanterellaInstall::new(xanterella);
        xanterellainstall.set_ip("127.0.0.1");
        let result = xanterellainstall.ping_ssh();
        assert!(result.is_ok());
    }
}
