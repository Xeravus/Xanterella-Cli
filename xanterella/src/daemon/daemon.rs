use log::{info};
use tokio::time::{self, sleep};

use std::collections::HashSet;

use crate::utils::get::*;
use crate::installer::core::*;

pub async fn start_daemon(automate: &bool, fast: &bool, debug: &bool) {
    info!(" [ RUN ] - Starte Daemon");
    if *debug {
        info!(" [ OK ] - Daemon im debug Mode");
    }

    let mut interval = time::interval(time::Duration::from_secs(1));
    let mut active_installs: HashSet<String> = HashSet::new();
    loop {
        interval.tick().await;
        check_for_installer(&mut active_installs, *automate, *fast, *debug);
    }
}

pub fn check_for_installer(active_installs: &mut HashSet<String>, automate: bool, fast: bool, debug: bool) {
    let devices: Taildevices = get_taildevices();
    for (_nodekey, device) in devices.devices {
        if device.name == "installer" && device.os == "linux" {
            let ip = device.ip[0].clone();
            if !active_installs.contains(&ip) {
                info!("[ OK ] - Installer gefunden: {}", ip);
                active_installs.insert(ip.clone());
                let _ = sleep(time::Duration::from_secs(10));
                tokio::spawn(async move {
                    daemon_install(automate.clone(), fast.clone(), ip.to_string().clone(), debug.clone())
                });
            }
        }
    }
}
//    info!("[ OK ] - Daemon beendet");
