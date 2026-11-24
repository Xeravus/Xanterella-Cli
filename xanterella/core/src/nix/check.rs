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
            let cmd = self.execute("nix", &["flake", "check", "--impure"], dir)
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

#[cfg(test)]
mod tests {
    use super::*;
use std::process::ExitStatus;
use std::process::Output;
use mockall::predicate::always;
use mockall::predicate::eq;
use std::os::unix::process::ExitStatusExt;
use crate::xanterella::MockCommandRunner;
use mockall::automock;
use crate::test_helpers::mock_ok;
use crate::Xanterella;

#[test]
fn test_mock_check_nix_flake() {
    let mut mock = MockCommandRunner::new();

    mock.expect_run_cmd()
        .withf(|program, args, _dir| {
            program == "nix" && args == ["flake", "check", "--impure"] 
        }).times(1).returning(|_, _, _| mock_ok(""));

    let mut xanterella = Xanterella::new(mock);
    let result = xanterella.check_nix_flake();
    assert!(result.is_ok());
}
}
