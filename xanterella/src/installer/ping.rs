use crate::prelude::*;
use crate::installer::core::XanterellaInstall;
use crate::installer::helper::*;

pub trait Ping {
    fn ping(&mut self) -> Result<(), EventsFailed>;
    fn ping_ssh(&mut self) -> Result<(), EventsFailed>;
}

impl<'a> Ping for XanterellaInstall<'a> {
    fn ping(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPing); 

        let cmd = Command::new("ping")
            .args(["-W", "1"])
            .arg(&self.ip)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Ping);
        };
        
        self.xanterella.log_event(Events::OkPing);
        Ok(())
    }

    fn ping_ssh(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunPingSsh);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PingSsh);
        };

        self.xanterella.log_event(Events::OkPingSsh);
        Ok(())
    }
}
