use crate::prelude::*;

pub trait Ping {
    fn ping(&mut self) -> Result<(), EventsFailed>;
    fn ping_ssh(&mut self) -> Result<(), EventsFailed>;
}

impl Ping for XanterellaInstall {
    fn ping(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPing);

        if !self.xanterella.debug {
            let cmd = Command::new("ping")
                .args(["-c", "3", "-W", "1"])
                .arg(&self.ip)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::Ping(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkPing);
        Ok(())
    }

    fn ping_ssh(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPingSsh);

        if !self.xanterella.debug {
            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

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
    #[test]
    #[ignore]
    fn test_installer_ping_ping() {
        let xanterella1 = Xanterella::new();
        let xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping();
        let result2 = install2.ping();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::Ping(_))));
    }

    #[test]
    fn test_installer_ping_ping_ssh() {
        let xanterella1 = Xanterella::new();
        let xanterella2 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(xanterella1);
        let mut install2 = XanterellaInstall::new(xanterella2);

        install1.xanterella.debug = true;
        install2.ip = "127.127.127.127.127".to_string();

        let result1 = install1.ping_ssh();
        let result2 = install2.ping_ssh();

        assert!(result1.is_ok());
        assert!(result2.is_err());

        assert!(matches!(result2, Err(EventsFailed::PingSsh(_))));
    }
}
