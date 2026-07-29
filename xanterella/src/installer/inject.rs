use crate::installer::core::XanterellaInstall;
use crate::installer::helper::*;
use crate::prelude::*;
use crate::utils::config::*;

pub trait Inject {
    fn inject_tailscale(&mut self) -> Result<(), EventsFailed>;
    fn inject_wifi(&mut self) -> Result<(), EventsFailed>;
}

impl<'a> Inject for XanterellaInstall<'a> {
    fn inject_tailscale(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunInjectTailscale);

        if !self.xanterella.debug {
            let inject_cmd = format!(
                "touch /mnt/etc/tailscale_key && echo '{}' > /mnt/etc/tailscale_key && chmod 600 /mnt/etc/tailscale_key",
                self.xanterella.config_parse()?.tailkey
            );

            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(inject_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::InjectTailscale(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkInjectTailscale);
        Ok(())
    }

    fn inject_wifi(&mut self) -> Result<(), EventsFailed> {
        self.xanterella.log_event(Events::RunInjectWifi);

        if !self.xanterella.debug {
            let inject_cmd = format!(
                "touch /mnt/etc/wifi_secrets && echo '{}' > /mnt/etc/wifi_secrets && chmod 600 /mnt/etc/wifi_secrets",
                self.xanterella.config_parse()?.wifi
            );

            let cmd = Command::new("ssh")
                .args(self.get_sshstring(User::Root))
                .arg(inject_cmd)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::InjectWifi(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        self.xanterella.log_event(Events::OkInjectWifi);
        Ok(())
    }
}
