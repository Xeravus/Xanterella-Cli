use crate::xanterella::CommandRunner;
use crate::Events;
use crate::EventsFailed;
use std::process::Command;
use crate::Xanterella;

impl<C: CommandRunner> Xanterella<C> {
    pub fn colmena_apply<T: AsRef<str>>(&mut self, hosts: &[T]) -> Result<(), EventsFailed> {
        self.log_event(Events::RunColmenaApply);

        if !self.debug {
            let mut colmena = Command::new("colmena");
            colmena.arg("apply");

            if !hosts.is_empty() {
                let host_string: Vec<&str> = hosts.iter().map(|s| s.as_ref()).collect();
                let host_string = host_string.join(",");
                colmena.args(["--on", &host_string]);
            }

            let output = colmena
                .current_dir(self.load_config().target_directory)
                .output()
                .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;

            if !output.status.success() {
                return Err(EventsFailed::ColmenaApply(String::from_utf8_lossy(&output.stderr).to_string()));
            };
        }

        self.log_event(Events::OkColmenaApply);
        Ok(())
    }
}
