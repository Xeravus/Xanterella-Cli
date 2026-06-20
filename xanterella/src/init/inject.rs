use log::{error, info};

use std::process::{self, Command};

use crate::utils::config::*;
use crate::utils::get::*;

pub fn inject_tailscale(ip: &str) {
    info!("[ RUN ] - Injeziere Tailscale Auth Key");

    let cmd = format!(
        "touch /mnt/etc/tailscale_key && echo '{}' > /mnt/etc/tailscale_key && chmod 600 /mnt/etc/tailscale_key",
        config_parse().tailkey
    );
    let inject = Command::new("ssh").args(get_sshstring(ip, User::Root)).arg(cmd).output().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte den Tailscale Key nicht injezieren: {}", err);
        process::exit(1);
    });
    if !inject.status.success() {
        error!("[ FAILED ] - Konnte den Tailscale Key nicht injezieren: {}", String::from_utf8_lossy(&inject.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Tailscale Auth Key erfolgreich injeziert");
}

pub fn inject_wifi(ip: &str) {
    info!("[ RUN ] - Injeziere Wlan Passwort");

    let cmd = format!(
        "touch /mnt/etc/wifi_secrets && echo 'PSK_HOME={}' > /mnt/etc/wifi_secrets && chmod 600 /mnt/etc/wifi_secrets",
        config_parse().wifi
    );
    let inject = Command::new("ssh").args(get_sshstring(ip, User::Root)).arg(cmd).output().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte das Wlan Passwort nicht injezieren: {}", err);
        process::exit(1);
    });
    if !inject.status.success() {
        error!("[ FAILED ] - Konnte das Wlan Passwort nicht injezieren: {}", String::from_utf8_lossy(&inject.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Wlan Passwort erfolgreich injeziert");
}
