use crate::xanterella::CommandRunner;
use crate::Events;
use crate::EventsFailed;
use std::process::Command;
use crate::Xanterella;

impl<C: CommandRunner> Xanterella<C> {
    pub fn check_nix_flake(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunCheckNix);

        eprintln!("Starte Check");
        if !self.debug {
            let dir = &self.load_config().target_directory;
            let cmd = self.runner.execute("nix", &["flake", "check", "--impure"], dir)
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !cmd.status.success() {
                return Err(EventsFailed::CheckNix(String::from_utf8_lossy(&cmd.stderr).to_string()));
            };
        };

        eprintln!("Beende Check");
        self.log_event(Events::OkCheckNix);
        Ok(())
    }
}
