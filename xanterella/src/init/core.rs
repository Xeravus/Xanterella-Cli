use log::info;

use crate::init::init::*;
use crate::init::inject::*;

pub fn init_git(ip: &str) {
    info!("[ RUN ] - Starte Git Authentikation");

    init_git_email(ip);
    init_git_name(ip);
    info!("[ OK ] - Git Authentikation erfolgreich");
}

pub fn inject(ip: &str) {
    info!("[ RUN ] - Starte Injezierung");

    inject_tailscale(ip);
    inject_wifi(ip);
    info!("[ OK ] - Injezierung erfolgreich");
}
