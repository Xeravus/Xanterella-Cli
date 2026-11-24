use crate::xanterella::CommandRunner;
use crate::Events;
use crate::EventsFailed;
use std::process::Command;
use crate::Xanterella;

impl<C: CommandRunner> Xanterella<C> {
    pub fn colmena_apply<T: AsRef<str>>(&mut self, hosts: &[T]) -> Result<(), EventsFailed> {
        self.log_event(Events::RunColmenaApply);

        let dir = &self.load_config().target_directory;

        if !self.debug {
            if !hosts.is_empty() {
                let host_string: Vec<&str> = hosts.iter().map(|s| s.as_ref()).collect();
                let host_string = host_string.join(",");
                let colmena = self.runner.execute("colmena", &["apply", "--on", &host_string], &dir)
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !colmena.status.success() {
                    return Err(EventsFailed::ColmenaApply(String::from_utf8_lossy(&colmena.stderr).to_string()));
                }
            } else {
                let colmena = self.runner.execute("colmena", &["apply"], &dir).map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

                if !colmena.status.success() {
                    return Err(EventsFailed::ColmenaApply(String::from_utf8_lossy(&colmena.stderr).to_string()));
                };
            }
        }

        self.log_event(Events::OkColmenaApply);
        Ok(())
    }
}
