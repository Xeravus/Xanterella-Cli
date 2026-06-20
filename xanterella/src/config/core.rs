use log::{info};

use crate::config::query::*;

pub fn list_hosts() {
    info!("Hosts: ");
    for i in query_hosts() {
        info!("{}", i);
    }
}

pub fn list_modules() {
    info!("Modules: ");
    for i in query_modules_all() {
        info!("{}", i);
    }
}

pub fn list_profiles() {
    info!("Profiles: ");
    for i in query_profiles() {
        info!("{}", i);
    }
}
