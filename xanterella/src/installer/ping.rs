use crate::prelude::*;

pub trait Ping {
    fn ping(&self) -> Result<(), EventsFailed>;
    fn ping_ssh(&self) -> Result<(), EventsFailed>;
}

impl Ping for Xanterella {
    fn ping(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPing, &self.ip.clone()); 

        let cmd = Command::new("ping")
            .args(["-W", "1"])
            .arg(&self.ip)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Ping(&self.ip.clone()));
        };
        
        self.log_event(Events::OkPing, &self.ip.clone());
        Ok(())
    }

    fn ping_ssh(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPingSsh, &self.ip.clone());

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PingSsh(&self.ip.clone()));
        };

        self.log_event(Events::OkPingSsh, &self.ip.clone());
        Ok(())
    }
}
