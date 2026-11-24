use std::process::Command;

use crate::{Events, EventsFailed, Xanterella, xanterella::CommandRunner};

impl<C: CommandRunner> Xanterella<C> {
    pub fn colmena_apply<T: AsRef<str>>(&mut self, hosts: &[T]) -> Result<(), EventsFailed> {
        self.log_event(Events::RunColmenaApply);

        let dir = &self.load_config().target_directory;

        if !self.debug {
            if !hosts.is_empty() {
                let host_string: Vec<&str> = hosts.iter().map(|s| s.as_ref()).collect();
                let host_string = host_string.join(",");
                let colmena = self
                    .execute("colmena", &["apply", "--on", &host_string], &dir)
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !colmena.status.success() {
                    return Err(EventsFailed::ColmenaApply(String::from_utf8_lossy(&colmena.stderr).to_string()));
                }
            } else {
                let colmena = self
                    .execute("colmena", &["apply"], &dir)
                    .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !colmena.status.success() {
                    return Err(EventsFailed::ColmenaApply(String::from_utf8_lossy(&colmena.stderr).to_string()));
                };
            }
        }

        self.log_event(Events::OkColmenaApply);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        os::unix::process::ExitStatusExt,
        process::{ExitStatus, Output},
    };

    use mockall::{
        automock,
        predicate::{always, eq},
    };

    use super::*;
    use crate::{test_helpers::mock_ok, xanterella::MockCommandRunner};

    #[test]
    fn test_mock_colmena_apply_no_hosts() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| program == "colmena" && args == ["apply"])
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let result = xanterella.colmena_apply::<&str>(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mock_colmena_apply_hosts() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| program == "colmena" && args == ["apply", "--on", "test"])
            .times(1)
            .returning(|_, _, _| mock_ok(""));

        let mut xanterella = Xanterella::new(mock);
        let result = xanterella.colmena_apply::<&str>(&["test"]);
        assert!(result.is_ok());
    }
}
