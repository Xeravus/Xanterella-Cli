use crate::prelude::*;

pub trait Check {
    fn check_nix_flake(&mut self) -> Result<(), EventsFailed>;
}

impl Check for Xanterella {
    fn check_nix_flake(&mut self) -> Result<(), EventsFailed> {
        self.log_event(Events::RunCheckNix);

        let cmd = Command::new("nixos-rebuild")
            .args(["dry-build", "--flake", ".#crylia"])
            .current_dir(self.get_path(Paths::Nixconf))
            .output()
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

        if !cmd.status.success() {
            return Err(EventsFailed::CheckNix);
        };

        self.log_event(Events::OkCheckNix);
        Ok(())
    }
}
