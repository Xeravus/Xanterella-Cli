pub trait Remote {
    fn ping(&self) -> Result<(), EventsFailed>;
    fn ping_ssh(&self) -> Result<(), EventsFailed>;
    fn get_hardware(&self) -> Result<String, EventsFailed>;
}

impl Remote for Xanterella {
    fn ping(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPing, &self.ip.clone()); 

        let cmd = Command::new("ping")
            .args(["-W", "1"])
            .arg(&self.ip)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::Ping(&self.ip.clone()));
        };
        
        self.log_event(Events::OkPing, &self.ip.clone());
        Ok()
    }

    fn ping_ssh(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunPingSsh, &sekf.ip.clone());

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::PingSsh(&self.ip.clone()));
        };

        self.log_event(Events::OkPingSsh, &self.ip.clone());
        Ok()
    }

    fn get_hardware(&self) -> Result<String, EventsFailed> {
        self.log_event(Events::RunGetHardware(&self.ip.clone()));

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .args(["nixos-generate-config", "--no-filesystem", "--show-hardware-config"])
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err))?;

        if !cmd.status.success() {
            return Err(EventsFailed::GetHardware(&self.ip.clone()));
        }

        self.log_event(Events::OkGetHardware(&self.ip.clone()));
        Ok(String::from_utf8_lossy(&cmd.stdout))
    }
}
