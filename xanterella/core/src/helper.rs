use crate::prelude::*;

pub trait Helper {
    fn get_hardware(&mut self) -> Result<String, EventsFailed>;
}

impl<'a> Helper for XanterellaInstall<'a> {
    fn get_hardware(&mut self) -> Result<String, EventsFailed> {
        self.xanterella.log_event(Events::RunGetHardware);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["nixos-generate-config", "--no-filesystem", "--show-hardware-config"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GetHardware(String::from_utf8_lossy(&cmd.stderr).to_string()));
        };

        self.xanterella.log_event(Events::OkGetHardware);
        Ok(String::from_utf8_lossy(&cmd.stdout).to_string())
    }
}

#[cfg(test)]
#[path = "helper_test.rs"]
mod tests;
