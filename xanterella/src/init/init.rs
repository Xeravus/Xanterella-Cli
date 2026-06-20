use log::{error, info};

use std::process::{self, Command};

use crate::utils::get::*;

pub fn init_git_email(ip: &str) {
    info!("[ RUN ] - Git email wird eingestellt");

    let mut git = if ip != "127.0.0.1" {
        let mut c = Command::new("ssh");
        c.args(get_sshstring(ip, User::Cato));
        c.args(["git", "config", "--global", "user.email", "cato.jenisch@gmail.com"]);
        let _ = c.output();
        c
    } else {
        let mut c = Command::new("git");
        c.args(["config", "--global", "user.email", "cato.jenisch@gmail.com"]);
        let _ = c.output();
        c
    };
    let status = git.status().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte git nicht starten: {}", err);
        process::exit(1);
    });
    if !status.success() {
        error!("[ FAILED ] - Konnte Git nicht einstellen");
        process::exit(1);
    }
    info!("[ OK ] - Git email eingestellt");
}

pub fn init_git_name(ip: &str) {
    info!("[ RUN ] - Git name wird eingestellt");

    let mut git = if ip != "127.0.0.1" {
        let mut c = Command::new("ssh");
        c.args(get_sshstring(ip, User::Cato));
        c.args(["git", "config", "--global", "user.name", "Xeravus"]);
        let _ = c.output();
        c
    } else {
        let mut c = Command::new("git");
        c.args(["config", "--global", "user.name", "Xeravus"]);
        let _ = c.output();
        c
    };
    let status = git.status().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte git nicht starten: {}", err);
        process::exit(1);
    });
    if !status.success() {
        error!("[ FAILED ] - Konnte Git nicht einstellen");
        process::exit(1);
    }
    info!("[ OK ] - Git name eingestellt");
}

pub fn init_github(ip: &str) {
    info!("[ RUN ] - Starte Verbindung zwischen Git und GitHub");

    let mut gh = if ip != "127.0.0.1" {
        let mut c = Command::new("ssh");
        c.args(get_sshstring(ip, User::Cato));
        c.args(["gh", "auth", "login", "--hostname", "github.com", "-w", "-p", "https"]);
        c
    } else {
        let mut c = Command::new("gh");
        c.args(["auth", "login", "--hostname", "github.com", "-w", "-p", "https"]);
        c
    };
    let status = gh.status().unwrap_or_else(|err| {
        error!("[ FAILED ] - Konnte gh nicht starten: {}", err);
        process::exit(1);
    });
    if !status.success() {
        error!("[ FAILED ] - Konnte Git und GitHub nicht verknüpfen");
        process::exit(1);
    }
    info!("[ OK ] - GitHub Verknüpfung erfolgreich");
}

pub fn init_pull_xanterella(ip: &str) {
    info!("[ RUN ] - Pull Xanterella Git Repo");

    let pull = Command::new("ssh")
        .args(get_sshstring(ip, User::Root))
        .args(["git", "pull", "https://github.com/Xeravus/Xanterella.git"])
        .current_dir(get_path(Paths::Home))
        .output()
        .unwrap_or_else(|err| {
            error!("[ FAILED ] - Konnte das Repo nicht pullen: {}", err);
            process::exit(1);
        });
    if !pull.status.success() {
        error!("[ FAILED ] - Konnte das Repo nicht pullen: {}", String::from_utf8_lossy(&pull.stderr));
        process::exit(1);
    }
    info!("[ OK ] - Xanterella Git Repo erfolgreich gepullt");
}
