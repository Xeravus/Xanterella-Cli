use crate::installer::core::XanterellaInstall;
use crate::installer::helper::*;
use crate::prelude::*;

pub trait Ping {
    fn ping(&mut self) -> Result<(), EventsFailed>;
    fn ping_ssh(&mut self) -> Result<(), EventsFailed>;
}

impl<'a> Ping for XanterellaInstall<'a> {
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
#[path = "ping_test.rs"]
mod tests;
