use crate::prelude::*;

pub trait Helper {
    fn get_hardware(&mut self) -> Result<String, EventsFailed>;
    fn get_sshstring(&mut self, user: User) -> Vec<String>;
}

impl<'a> Helper for XanterellaInstall<'a> {
    fn get_hardware(&mut self) -> Result<String, EventsFailed> {
        self.xanterella.log_event(Events::RunGetHardware);

        if !self.xanterella.debug {
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
        } else {
            return Ok(String::new());
        }
    }

    fn get_sshstring(&mut self, user: User) -> Vec<String> {
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
#[path = "helper_test.rs"]
mod tests;
