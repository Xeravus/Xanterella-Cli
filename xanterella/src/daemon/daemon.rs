use log::info;
use tokio::time::{self, sleep};

use std::collections::HashSet;

use crate::installer::core::*;
use crate::utils::get::*;

pub async fn start_daemon(automate: bool, fast: bool, init: bool, debug: bool) {
    info!(" [ RUN ] - Starte Daemon");
    if debug {
        info!(" [ OK ] - Daemon im debug Mode");
    }
    if init {
        info!(" [ OK ] - Daemon im init Mode");
    }

    let mut interval = time::interval(time::Duration::from_secs(1));
    let mut active_installs: HashSet<String> = HashSet::new();
    loop {
        interval.tick().await;
        for i in get_taildevices_specific(get_taildevices(), "installer", &active_installs) {
            info!("[ OK ] - Installer gefunden: {}", i);
            active_installs.insert(i.clone());
            let _ = sleep(time::Duration::from_secs(10));
            tokio::spawn(async move { daemon_install(automate, fast, i.clone().to_string(), debug) });
        }
        //check_for_crylia(*debug);
    }
}
