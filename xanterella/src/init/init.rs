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
    let status = git.status()
        .unwrap_or_else(|err| {
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
