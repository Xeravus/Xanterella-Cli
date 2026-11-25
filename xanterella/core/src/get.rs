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

        let result = Xanterella::new(mock).get_taildevices().unwrap();
        assert!(result.devices.is_empty());
    }

    #[test]
    fn failed_tailscale_command_reports_stderr() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run_cmd().once().returning(|_, _, _| crate::test_helpers::mock_err("offline"));
        let result = Xanterella::new(mock).get_taildevices();
        assert!(matches!(result, Err(EventsFailed::Tailscale(message)) if message == "offline"));
    }

    #[test]
    fn invalid_peer_data_reports_parse_error() {
        let mut mock = MockCommandRunner::new();
        mock.expect_run_cmd().once().returning(|_, _, _| crate::test_helpers::mock_ok(r#"{"Peer":[]}"#));
        let result = Xanterella::new(mock).get_taildevices();
        assert!(matches!(result, Err(EventsFailed::SerdeJson(_))));
    }

    #[test]
    fn selects_first_address_of_available_matching_linux_peers() {
        let devices = serde_json::from_value(serde_json::json!({"Peer": {
            "available": {"HostName":"worker","OS":"linux","TailscaleIPs":["100.1.1.1","::1"]},
            "busy": {"HostName":"worker","OS":"linux","TailscaleIPs":["100.1.1.2"]},
            "other_os": {"HostName":"worker","OS":"windows","TailscaleIPs":["100.1.1.3"]},
            "other_name": {"HostName":"other","OS":"linux","TailscaleIPs":["100.1.1.4"]}
        }}))
        .unwrap();
        let active = HashSet::from(["100.1.1.2".to_owned()]);
        let ips = Xanterella::<MockCommandRunner>::get_taildevices_specific(devices, "worker", &active);
        assert_eq!(ips, ["100.1.1.1"]);
    }
}
