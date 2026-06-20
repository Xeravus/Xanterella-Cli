use log::{error, info};

use std::process::{self, Command};

use crate::utils::get::*;

pub fn ping(ip: &str) {
    info!("[ RUN ] - Starte Ping");

    let ping = Command::new("ping").args(["-c", "1"]).args(["-W", "1"]).arg(ip).output().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte Ping nicht starten: {}", err);
        process::exit(1);
    });

    if !ping.status.success() {
        error!("[ FAILED ] - Konnte das Gerät nicht pingen: {}", ip);
        process::exit(1);
    }
    info!("[ OK ] - Ping erfolgreich");
}

pub fn ping_ssh(ip: &str) {
    info!("[ RUN ] - Starte SSH Ping");

    let ssh = Command::new("ssh").args(get_sshstring(ip, User::Root)).output().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte Tailscale nicht starten: {}", err);
        process::exit(1);
    });
    if !ssh.status.success() {
        error!("[ FAILED ] - Konnte das Gerät nicht über ssh erreichen: {}", String::from_utf8_lossy(&ssh.stderr));
        process::exit(1);
    }
    info!("[ OK ] - SSH PING erfolgreich");
}

pub fn nix_check() {
    info!("[ RUN ] - Starte Flake Check");

    let check = Command::new("nixos-rebuild")
        .arg("dry-build")
        .arg("--flake")
        .arg(".#crylia")
        .current_dir(get_path(Paths::Nixconf))
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte Nixos-rebuild nicht starten: {}", err);
            process::exit(1);
        });
    if check.status.success() {
        info!("[ OK ] - Flake Check erfolgreich");
    } else {
        error!("[ FAILED ] - Die Nix Flake ist nicht funktionierend: {}", String::from_utf8_lossy(&check.stderr));
        process::exit(1);
    }
}
