use std::collections::{HashMap, HashSet};

use crate::{Events, EventsFailed, Xanterella, xanterella::CommandRunner};

pub enum User {
    Root,
    Cato,
}

pub enum Paths {
    Nixconf,
    Config,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Taildevices {
    #[serde(rename = "Peer")]
    pub devices: HashMap<String, DeviceInfo>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    #[serde(rename = "HostName")]
    pub name: String,
    #[serde(rename = "TailscaleIPs")]
    pub ip: Vec<String>,
    #[serde(rename = "OS")]
    pub os: String,
}

impl<C: CommandRunner> Xanterella<C> {
    pub fn get_taildevices(&self) -> Result<Taildevices, EventsFailed> {
        let cmd = self
            .execute("tailscale", &["status", "--json"], "")
            .map_err(|err| EventsFailed::FailedCmd(err.to_string()))?;
        if !cmd.status.success() {
            return Err(EventsFailed::Tailscale(String::from_utf8_lossy(&cmd.stderr).to_string()));
        }
        serde_json::from_slice::<Taildevices>(&cmd.stdout).map_err(|err| EventsFailed::SerdeJson(err.to_string()))
    }

    pub fn get_taildevices_specific(
        devices: Taildevices, name: &str, active_installs: &HashSet<String>,
    ) -> Vec<String> {
        let mut ips: Vec<String> = vec![];
        for (_nodekey, device) in devices.devices {
            if device.name == name && device.os == "linux" {
                let ip = device.ip[0].clone();
                if !active_installs.contains(&ip) {
                    let _ = &mut ips.push(ip.to_owned());
                }
            }
        }
        ips
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
    use crate::xanterella::MockCommandRunner;

    #[test]
    fn test_mock_get_taildevices() {
        let mut mock = MockCommandRunner::new();

        mock.expect_run_cmd()
            .withf(|program, args, _dir| program == "tailscale" && args == ["status", "--json"])
            .times(1)
            .returning(|_, _, _| {
                Ok(Output {
                    status: ExitStatus::from_raw(0),
                    stdout: br#"{"Peer": {}}"#.to_vec(),
                    stderr: Vec::new(),
                })
            });

        let mut xanterella = Xanterella::new(mock);
        let result = xanterella.get_taildevices();
        assert!(result.is_ok());
    }
}
