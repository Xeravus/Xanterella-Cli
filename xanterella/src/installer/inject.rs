use crate::prelude::*;

pub trait Inject {
    fn inject_tailscale(&self) -> Result<(), EventsFailed>;
    fn inject_wifi(&self) -> Result<(), EventsFailed>;
}

impl Inject for Xanterella {
    fn inject_tailscale(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunInjectTailscale);

        let inject_cmd = format!("touch /mnt/etc/tailscale_key && echo '{}' > /mnt/etc/tailscale_key && chmod 600 /mnt/etc/tailscale_key", self.config_parse()?.tailkey);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(inject_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::InjectTailscale);
        };

        self.log_event(Events::OkInjectTailscale);
        Ok(())
    }

    fn inject_wifi(&self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunInjectWifi);

        let inject_cmd = format!("touch /mnt/etc/wifi_secrets && echo '{}' > /mnt/etc/wifi_secrets && chmod 600 /mnt/etc/wifi_secrets", self.config_parse()?.wifi);

        let cmd = Command::new("ssh")
            .args(self.get_sshstring(User::Root))
            .arg(inject_cmd)
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::InjectWifi);
        };

        self.log_event(Events::OkInjectWifi);
        Ok(())
    }
}
