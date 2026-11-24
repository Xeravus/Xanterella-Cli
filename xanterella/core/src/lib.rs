pub mod config;
pub mod database;
pub mod get;
pub mod git;
pub mod install;
pub mod nix;
pub mod prolyxena;
pub mod result;
pub mod xanterella;

pub use get::{Paths, User};
pub use install::install::XanterellaInstall;
pub use result::{Events, EventsFailed};
pub use xanterella::Xanterella;

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::io::Result as IoResult;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    pub fn mock_output(code: i32, stdout: &str, stderr: &str) -> IoResult<Output> {
        Ok(Output {
            status: ExitStatus::from_raw(code << 8),
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        })
    }

    /// Kurzform für den häufigsten Fall (Erfolg)
    pub fn mock_ok(stdout: &str) -> IoResult<Output> {
        mock_output(0, stdout, "")
    }

    /// Kurzform für Fehlerfälle
    pub fn mock_err(stderr: &str) -> IoResult<Output> {
        mock_output(1, "", stderr)
    }
}
